/*!
# Minimum Spanning Tree Algorithms

Minimum Spanning Tree (MST) algorithms.
It provides the following algorithms:

- **Prim's Algorithm:**
  A greedy approach that grows the MST by adding the minimum edge at each step.
  It computes an MST forest (covering all connected components).

- **Kruskal's Algorithm:**
  Sorts all edges and uses a union–find data structure to avoid cycles.

- **Borůvka's Algorithm (Parallel):**
  A parallel implementation using Rayon to process each component concurrently.

**Note:** The weight type `W` must implement `Ord`. If you wish to use floating‑point weights (e.g. `f32` or `f64`), consider wrapping them in a type that provides a total order (e.g. [`ordered_float::OrderedFloat`](https://docs.rs/ordered-float/)).

All algorithms assume that the graph's nodes are indexed from 0 to \(n-1\) and that edge weights satisfy the required ordering and arithmetic properties.
They use a union–find (disjoint-set) data structure with path compression and union by rank for cycle detection and component merging.

## Error Handling

If the input graph is empty, algorithms will return a `Result` containing a `GraphinaError`.
If other required conditions are violated, the algorithm may also signal an error via a `Result`.
*/

use crate::core::error::{GraphinaError, Result};
use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use rayon::prelude::*;
use std::cmp::Ordering;
use std::convert::From;
use std::ops::{Add, AddAssign, Sub};

/// Edge-count floor above which Borůvka's per-round cheapest-edge search runs in
/// parallel. Below it the Rayon dispatch and per-worker table allocation cost
/// more than a single sequential pass, so the sequential path is used.
const BORUVKA_PARALLEL_MIN_EDGES: usize = 10_000;

/// Returns an upper bound on node indices, suitable for sizing a dense structure
/// indexed by `NodeId::index()`.
///
/// `BaseGraph` wraps a `StableGraph`, so indices are stable but not contiguous
/// after node removals; a remaining node's index can exceed `node_count()`.
/// Sizing by this bound (rather than `node_count`) keeps index-keyed access in
/// range for sparse graphs.
fn index_bound<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> usize
where
    Ty: GraphConstructor<A, W>,
{
    graph
        .node_ids()
        .map(|node| node.index())
        .max()
        .map_or(0, |m| m + 1)
}

/// A simple union–find (disjoint-set) data structure.
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    /// Creates a new union–find structure for `n` elements.
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    /// Finds the representative of the set that contains `i`, using path compression.
    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    /// Unions the sets containing `i` and `j` using union by rank.
    fn union(&mut self, i: usize, j: usize) {
        let i = self.find(i);
        let j = self.find(j);
        if i == j {
            return;
        }
        match self.rank[i].cmp(&self.rank[j]) {
            Ordering::Less => self.parent[i] = j,
            Ordering::Greater => self.parent[j] = i,
            Ordering::Equal => {
                self.parent[j] = i;
                self.rank[i] += 1;
            }
        }
    }
}

/// Represents an edge in the MST.
#[derive(Debug, Clone, Copy)]
pub struct MstEdge<W> {
    /// Source node
    pub u: NodeId,
    /// Target node
    pub v: NodeId,
    /// Edge weight
    pub weight: W,
}

///
/// ## Borůvka's MST Algorithm (Parallel)
///
/// Computes the Minimum Spanning Tree (MST) using a parallel variant of Borůvka's algorithm.
///
/// Each round finds the cheapest edge leaving every component in a single pass
/// over the edges (parallelized with Rayon for large graphs, sequential below an
/// edge-count threshold where dispatch would cost more than it saves). These
/// candidate edges are then processed sequentially using a union–find structure.
/// The process continues until a single component remains or no connecting edges
/// are found (i.e. the graph is disconnected).
///
/// # Type Bounds
///
/// - `W` must implement `Copy`, `PartialOrd`, `Add`, `AddAssign`, `Sub`, `From<u8>`, and also `Send + Sync`
///   to enable parallel processing.
/// - `Ty` must implement `GraphConstructor` for the given node attribute and weight types.
///
/// # Complexity
///
/// - **Time Complexity:** Approximately \(O(E \log V)\) in practice (using parallelism can reduce runtime).
/// - **Space Complexity:** \(O(E + V)\)
///
/// # Returns
///
/// A `Result` containing a tuple with:
/// - A vector of MST edges (`MstEdge<W>`).
/// - The total weight of the MST.
///
/// Returns an `Err(GraphinaError)` if the input graph is empty.
///
/// # Example
///
/// ```rust
/// use graphina::mst::boruvka_mst;
/// use graphina::core::types::{Graph, NodeId};
/// use ordered_float::OrderedFloat;
///
/// let mut g = Graph::<i32, OrderedFloat<f64>>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// g.add_edge(n1, n2, OrderedFloat(1.0));
///
/// let (mst_edges, total_weight) = boruvka_mst(&g).unwrap();
/// ```
pub fn boruvka_mst<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Result<(Vec<MstEdge<W>>, W)>
where
    W: Copy + PartialOrd + Add<Output = W> + AddAssign + Sub<Output = W> + From<u8> + Send + Sync,
    Ty: GraphConstructor<A, W>,
{
    if graph.node_count() == 0 {
        return Err(GraphinaError::invalid_graph(
            "Graph is empty, cannot compute MST.",
        ));
    }

    // `bound` sizes index-keyed structures for the possibly-sparse index space;
    // `components` counts actual nodes, so it reaches 1 when the real nodes are
    // merged even though gap indices remain singletons in the union-find.
    let bound = index_bound(graph);
    let all_edges: Vec<(NodeId, NodeId, W)> = graph.edges().map(|(u, v, w)| (u, v, *w)).collect();

    let mut uf = UnionFind::new(bound);
    let mut mst_edges = Vec::new();
    let mut total_weight = W::from(0u8);
    let mut components = graph.node_count();

    while components > 1 {
        // Use the canonical component root for each node, not the raw parent
        // pointer: after unions the parent array is not path-compressed, so two
        // nodes in the same component can have different parents.
        let roots: Vec<usize> = (0..bound).map(|i| uf.find(i)).collect();

        // Cheapest outgoing edge per component root, found in a single parallel
        // pass over the edges (O(E)) rather than one full edge scan per component
        // (O(V * E)). Each worker folds edges into a local per-component table
        // keyed by root, then the tables are reduced by keeping the lighter edge
        // for each component.
        let keep_lighter =
            |slot: &mut Option<(NodeId, NodeId, W)>, cand: (NodeId, NodeId, W)| match slot {
                Some((_, _, current)) if cand.2 < *current => *slot = Some(cand),
                None => *slot = Some(cand),
                _ => {}
            };
        // Parallelism pays off only when the edge scan is large enough to cover
        // Rayon's dispatch and the per-worker table allocation; below the
        // threshold a single sequential pass over the edges is faster.
        let cheapest: Vec<Option<(NodeId, NodeId, W)>> =
            if all_edges.len() >= BORUVKA_PARALLEL_MIN_EDGES {
                all_edges
                    .par_iter()
                    .fold(
                        || vec![None::<(NodeId, NodeId, W)>; bound],
                        |mut acc, &(u, v, w)| {
                            let ru = roots[u.index()];
                            let rv = roots[v.index()];
                            if ru != rv {
                                keep_lighter(&mut acc[ru], (u, v, w));
                                keep_lighter(&mut acc[rv], (u, v, w));
                            }
                            acc
                        },
                    )
                    .reduce(
                        || vec![None::<(NodeId, NodeId, W)>; bound],
                        |mut a, b| {
                            for (slot, other) in a.iter_mut().zip(b) {
                                if let Some(cand) = other {
                                    keep_lighter(slot, cand);
                                }
                            }
                            a
                        },
                    )
            } else {
                let mut acc = vec![None::<(NodeId, NodeId, W)>; bound];
                for &(u, v, w) in &all_edges {
                    let ru = roots[u.index()];
                    let rv = roots[v.index()];
                    if ru != rv {
                        keep_lighter(&mut acc[ru], (u, v, w));
                        keep_lighter(&mut acc[rv], (u, v, w));
                    }
                }
                acc
            };

        let mut found = false;
        for (u, v, w) in cheapest.into_iter().flatten() {
            let ru = uf.find(u.index());
            let rv = uf.find(v.index());
            if ru != rv {
                uf.union(ru, rv);
                mst_edges.push(MstEdge { u, v, weight: w });
                total_weight += w;
                components -= 1;
                found = true;
            }
        }
        if !found {
            break;
        }
    }

    Ok((mst_edges, total_weight))
}

///
/// ## Kruskal's MST Algorithm
///
/// Computes the MST by first sorting all edges by weight and then selecting the smallest
/// edges one by one while avoiding cycles using a union–find data structure.
///
/// # Type Bounds
///
/// - `W` must implement `Copy`, `PartialOrd`, `Add`, `AddAssign`, `From<u8>`, and `Ord`.
/// - `Ty` must implement `GraphConstructor`.
///
/// # Complexity
///
/// - **Time Complexity:** \(O(E \log E)\), dominated by the sorting step.
/// - **Space Complexity:** \(O(E + V)\)
///
/// # Returns
///
/// A `Result` containing a tuple with:
/// - A vector of MST edges (`MstEdge<W>`).
/// - The total weight of the MST.
///
/// Returns an `Err(GraphinaError)` if the input graph is empty.
///
/// # Example
///
/// ```rust
/// use graphina::mst::kruskal_mst;
/// use graphina::core::types::{Graph, NodeId};
/// use ordered_float::OrderedFloat;
///
/// let mut g = Graph::<i32, OrderedFloat<f64>>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// g.add_edge(n1, n2, OrderedFloat(1.0));
///
/// let (mst_edges, total_weight) = kruskal_mst(&g).unwrap();
/// ```
pub fn kruskal_mst<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Result<(Vec<MstEdge<W>>, W)>
where
    W: Copy + PartialOrd + Add<Output = W> + AddAssign + From<u8> + Ord,
    Ty: GraphConstructor<A, W>,
{
    if graph.node_count() == 0 {
        return Err(GraphinaError::invalid_graph(
            "Graph is empty, cannot compute MST.",
        ));
    }

    let mut edges: Vec<(NodeId, NodeId, W)> = graph.edges().map(|(u, v, w)| (u, v, *w)).collect();
    edges.sort_by(|a, b| a.2.cmp(&b.2));

    // Size union-find by the index bound, not `node_count`: after node removals a
    // remaining node's index can exceed the count, and `find(index)` must stay in
    // range.
    let mut uf = UnionFind::new(index_bound(graph));
    let mut mst_edges = Vec::new();
    let mut total_weight = W::from(0u8);

    for (u, v, w) in edges {
        let ru = uf.find(u.index());
        let rv = uf.find(v.index());
        if ru != rv {
            uf.union(ru, rv);
            mst_edges.push(MstEdge { u, v, weight: w });
            total_weight += w;
        }
    }
    Ok((mst_edges, total_weight))
}

///
/// ## Prim's MST Algorithm
///
/// Computes the MST using Prim's algorithm. This version processes all connected components
/// (i.e. computes an MST forest) by iterating over nodes not yet included in the MST.
///
/// # Type Bounds
///
/// - `W` must implement `Copy`, `PartialOrd`, `Add`, `AddAssign`, `From<u8>`, and `Ord`.
/// - `Ty` must implement `GraphConstructor`.
/// - `NodeId` must implement `Ord`.
///
/// # Complexity
///
/// - **Time Complexity:** \(O(E \log V)\) per connected component.
/// - **Space Complexity:** \(O(V)\)
///
/// # Returns
///
/// A `Result` containing a tuple with:
/// - A vector of MST edges (`MstEdge<W>`).
/// - The total weight of the MST.
///
/// Returns an `Err(GraphinaError)` if the input graph is empty.
///
/// # Example
///
/// ```rust
/// use graphina::mst::prim_mst;
/// use graphina::core::types::{Graph, NodeId};
/// use ordered_float::OrderedFloat;
///
/// let mut g = Graph::<i32, OrderedFloat<f64>>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// g.add_edge(n1, n2, OrderedFloat(1.0));
///
/// let (mst_edges, total_weight) = prim_mst(&g).unwrap();
/// ```
pub fn prim_mst<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Result<(Vec<MstEdge<W>>, W)>
where
    W: Copy + PartialOrd + Add<Output = W> + AddAssign + From<u8> + Ord,
    Ty: GraphConstructor<A, W>,
    NodeId: Ord,
{
    if graph.node_count() == 0 {
        return Err(GraphinaError::invalid_graph(
            "Graph is empty, cannot compute MST.",
        ));
    }

    let mut mst_edges = Vec::new();
    let mut total_weight = W::from(0u8);

    // Dense, index-keyed state (`BaseGraph` wraps a `StableGraph`, so indices are
    // stable but sparse after removals; size by the index bound). `in_tree` and
    // the incident-edge adjacency are plain `Vec`s indexed by `NodeId::index()`,
    // so the inner-loop membership checks and neighbor lookups are hash-free. The
    // previous version used `HashSet`/`HashMap` keyed by `NodeId`, whose default
    // SipHash hashing dominated the runtime. Adjacency is built once (O(E)) with
    // each edge stored from both endpoints.
    let bound = index_bound(graph);
    let mut in_tree = vec![false; bound];
    let mut adjacency: Vec<Vec<(NodeId, W)>> = vec![Vec::new(); bound];
    for (u, v, w) in graph.edges() {
        adjacency[u.index()].push((v, *w));
        adjacency[v.index()].push((u, *w));
    }

    // Process each connected component.
    for start in graph.node_ids() {
        if in_tree[start.index()] {
            continue;
        }
        in_tree[start.index()] = true;
        let mut heap = std::collections::BinaryHeap::new();

        for &(neighbor, weight) in &adjacency[start.index()] {
            heap.push(std::cmp::Reverse((weight, start, neighbor)));
        }

        while let Some(std::cmp::Reverse((w, u, v))) = heap.pop() {
            // Skip if both endpoints are already in the MST.
            if in_tree[u.index()] && in_tree[v.index()] {
                continue;
            }
            let (from, to) = if in_tree[u.index()] { (u, v) } else { (v, u) };
            if !in_tree[to.index()] {
                in_tree[to.index()] = true;
                mst_edges.push(MstEdge {
                    u: from,
                    v: to,
                    weight: w,
                });
                total_weight += w;
                // Add all edges incident to the newly added node.
                for &(neighbor, weight) in &adjacency[to.index()] {
                    if !in_tree[neighbor.index()] {
                        heap.push(std::cmp::Reverse((weight, to, neighbor)));
                    }
                }
            }
        }
    }

    Ok((mst_edges, total_weight))
}

#[cfg(test)]
mod tests {

    // Regression: prim_mst dropped edges incident to a freshly added node when the
    // edge was stored with that node as the target. On a connected graph it
    // returned a partial tree (here 2 edges instead of 5). The spanning tree of
    // this connected, 6-node graph must have 5 edges and total weight 19.
    #[test]
    fn test_prim_mst_undirected_target_edges() {
        use crate::core::types::Graph;
        use crate::mst::{kruskal_mst, prim_mst};
        use ordered_float::OrderedFloat;

        let mut g: Graph<i32, OrderedFloat<f64>> = Graph::new();
        let nodes: Vec<_> = (0..6).map(|i| g.add_node(i)).collect();
        for (u, v, w) in [
            (0, 4, 5.0),
            (0, 5, 2.0),
            (1, 5, 1.0),
            (2, 4, 10.0),
            (3, 4, 1.0),
        ] {
            g.add_edge(nodes[u], nodes[v], OrderedFloat(w));
        }

        let (prim_edges, prim_weight) = prim_mst(&g).unwrap();
        assert_eq!(prim_edges.len(), 5);
        assert_eq!(prim_weight, OrderedFloat(19.0));

        let (kruskal_edges, kruskal_weight) = kruskal_mst(&g).unwrap();
        assert_eq!(prim_edges.len(), kruskal_edges.len());
        assert_eq!(prim_weight, kruskal_weight);
    }

    // Regression: boruvka_mst used the raw union-find parent pointer instead of the
    // canonical root to group nodes by component. After the first round the parent
    // array is not path-compressed, so cheapest-edge selection mis-grouped nodes,
    // missed valid merges, and returned a forest with too few edges (here 9 instead
    // of 10). This connected, 11-node graph must yield a spanning tree of 10 edges
    // and total weight 25.
    #[test]
    fn test_boruvka_mst_canonical_root_grouping() {
        use crate::core::types::Graph;
        use crate::mst::{boruvka_mst, kruskal_mst};
        use ordered_float::OrderedFloat;

        let edges = [
            (0, 2, 4.0),
            (0, 3, 1.0),
            (0, 4, 4.0),
            (0, 5, 4.0),
            (0, 6, 3.0),
            (1, 2, 8.0),
            (1, 3, 6.0),
            (1, 4, 5.0),
            (1, 5, 4.0),
            (1, 6, 10.0),
            (1, 7, 1.0),
            (1, 8, 7.0),
            (2, 3, 7.0),
            (2, 4, 7.0),
            (2, 5, 9.0),
            (2, 8, 8.0),
            (2, 9, 1.0),
            (2, 10, 3.0),
            (3, 4, 9.0),
            (3, 5, 10.0),
            (3, 10, 5.0),
            (4, 6, 5.0),
            (4, 9, 7.0),
            (4, 10, 5.0),
            (5, 6, 7.0),
            (5, 7, 7.0),
            (5, 8, 5.0),
            (5, 9, 4.0),
            (5, 10, 5.0),
            (6, 7, 6.0),
            (6, 8, 2.0),
            (6, 9, 5.0),
            (6, 10, 4.0),
            (7, 9, 2.0),
            (7, 10, 9.0),
            (8, 10, 9.0),
            (9, 10, 10.0),
        ];

        let mut g: Graph<i32, OrderedFloat<f64>> = Graph::new();
        let nodes: Vec<_> = (0..11).map(|i| g.add_node(i)).collect();
        for (u, v, w) in edges {
            g.add_edge(nodes[u], nodes[v], OrderedFloat(w));
        }

        let (boruvka_edges, boruvka_weight) = boruvka_mst(&g).unwrap();
        assert_eq!(boruvka_edges.len(), 10);
        assert_eq!(boruvka_weight, OrderedFloat(25.0));

        let (kruskal_edges, kruskal_weight) = kruskal_mst(&g).unwrap();
        assert_eq!(boruvka_edges.len(), kruskal_edges.len());
        assert_eq!(boruvka_weight, kruskal_weight);
    }
    use super::*;
    use crate::core::types::Graph;
    use ordered_float::OrderedFloat;

    #[test]
    fn test_kruskal_mst() {
        let mut graph = Graph::<i32, OrderedFloat<f64>>::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);
        graph.add_edge(n1, n2, OrderedFloat(1.0));
        graph.add_edge(n1, n3, OrderedFloat(3.0));
        graph.add_edge(n2, n3, OrderedFloat(2.0));
        graph.add_edge(n2, n4, OrderedFloat(4.0));
        graph.add_edge(n3, n4, OrderedFloat(5.0));
        let mst = kruskal_mst(&graph).expect("MST should exist");
        assert_eq!(mst.0.len(), 3);
        let total_weight: f64 = mst.0.iter().map(|e| e.weight.0).sum();
        assert!((total_weight - 7.0).abs() < 1e-6);
    }

    #[test]
    fn test_mst_disconnected_forest() {
        // Two disjoint components (0-1-2 and 3-4) form a spanning forest: three
        // edges total, and all three algorithms must agree on edge count and
        // weight rather than erroring on the disconnected input.
        let mut g: Graph<i32, OrderedFloat<f64>> = Graph::new();
        let nodes: Vec<_> = (0..5).map(|i| g.add_node(i)).collect();
        for (u, v, w) in [(0, 1, 1.0), (1, 2, 2.0), (0, 2, 5.0), (3, 4, 3.0)] {
            g.add_edge(nodes[u], nodes[v], OrderedFloat(w));
        }

        let (k_edges, k_weight) = kruskal_mst(&g).unwrap();
        let (p_edges, p_weight) = prim_mst(&g).unwrap();
        let (b_edges, b_weight) = boruvka_mst(&g).unwrap();

        assert_eq!(k_edges.len(), 3);
        assert_eq!(k_weight, OrderedFloat(6.0));
        assert_eq!(p_edges.len(), 3);
        assert_eq!(p_weight, k_weight);
        assert_eq!(b_edges.len(), 3);
        assert_eq!(b_weight, k_weight);
    }

    #[test]
    fn test_mst_sparse_indices_after_removal() {
        // Removing a node leaves stable but non-contiguous indices (`BaseGraph`
        // wraps a `StableGraph`), so a remaining node's index can exceed
        // `node_count()`. Sizing union-find or per-node buffers by `node_count()`
        // instead of the index bound indexes out of range and panics. All three
        // algorithms must handle the sparse graph and agree on the spanning tree.
        let mut g: Graph<i32, OrderedFloat<f64>> = Graph::new();
        let nodes: Vec<_> = (0..4).map(|i| g.add_node(i)).collect();
        g.remove_node(nodes[1]);
        // Remaining nodes 0, 2, 3 (indices 0, 2, 3; node_count is now 3).
        g.add_edge(nodes[0], nodes[2], OrderedFloat(1.0));
        g.add_edge(nodes[2], nodes[3], OrderedFloat(2.0));

        let (k_edges, k_weight) = kruskal_mst(&g).unwrap();
        let (p_edges, p_weight) = prim_mst(&g).unwrap();
        let (b_edges, b_weight) = boruvka_mst(&g).unwrap();

        assert_eq!(k_edges.len(), 2);
        assert_eq!(k_weight, OrderedFloat(3.0));
        assert_eq!(p_edges.len(), 2);
        assert_eq!(p_weight, k_weight);
        assert_eq!(b_edges.len(), 2);
        assert_eq!(b_weight, k_weight);
    }

    #[test]
    fn test_prim_mst() {
        let mut graph = Graph::<i32, OrderedFloat<f64>>::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, OrderedFloat(1.0));
        graph.add_edge(n1, n3, OrderedFloat(3.0));
        graph.add_edge(n2, n3, OrderedFloat(2.0));
        let mst = prim_mst(&graph).expect("MST should exist");
        assert_eq!(mst.0.len(), 2);
    }
}
