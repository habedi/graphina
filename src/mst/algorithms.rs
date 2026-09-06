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

**Note:** The weight type `W` needs only `PartialOrd`, so plain `f32` and `f64` weights work directly. The weights must still be totally ordered in practice: an unordered weight such as `NaN` is rejected with `GraphinaError::InvalidArgument`.

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
/// Rejects a graph whose weights are not totally ordered. `PartialOrd` admits
/// values such as `NaN` that compare unequal to themselves; sorting or heap
/// ordering with such a value is inconsistent, so the algorithms refuse it up
/// front instead of producing an arbitrary tree.
fn reject_unordered_weights<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Result<()>
where
    W: PartialOrd,
    Ty: GraphConstructor<A, W>,
{
    if graph
        .edges()
        .any(|(_, _, w)| w.partial_cmp(w) != Some(Ordering::Equal))
    {
        return Err(GraphinaError::invalid_argument(
            "MST weights must be totally ordered; found an unordered (NaN) weight.",
        ));
    }
    Ok(())
}

/// Indexed binary min-heap over node indices for Prim's algorithm. Each node
/// appears at most once, keyed by the lightest edge found so far into it, and
/// `decrease` moves it up in place, so the heap never holds more than one entry
/// per node and no stale entries need to be skipped. Keys are compared with
/// `PartialOrd`; the weights are known to be totally ordered by the time the heap
/// is used (see `reject_unordered_weights`).
struct IndexedMinHeap<W> {
    heap: Vec<usize>,
    pos: Vec<usize>,
    key: Vec<Option<W>>,
}

impl<W: Copy + PartialOrd> IndexedMinHeap<W> {
    const ABSENT: usize = usize::MAX;

    fn new(bound: usize) -> Self {
        Self {
            heap: Vec::new(),
            pos: vec![Self::ABSENT; bound],
            key: vec![None; bound],
        }
    }

    fn contains(&self, node: usize) -> bool {
        self.pos[node] != Self::ABSENT
    }

    fn key(&self, node: usize) -> Option<W> {
        self.key[node]
    }

    fn less(&self, a: usize, b: usize) -> bool {
        match (self.key[a], self.key[b]) {
            (Some(x), Some(y)) => x < y,
            _ => false,
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
        self.pos[self.heap[i]] = i;
        self.pos[self.heap[j]] = j;
    }

    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.less(self.heap[i], self.heap[parent]) {
                self.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut i: usize) {
        let len = self.heap.len();
        loop {
            let left = 2 * i + 1;
            let right = left + 1;
            let mut smallest = i;
            if left < len && self.less(self.heap[left], self.heap[smallest]) {
                smallest = left;
            }
            if right < len && self.less(self.heap[right], self.heap[smallest]) {
                smallest = right;
            }
            if smallest == i {
                break;
            }
            self.swap(i, smallest);
            i = smallest;
        }
    }

    /// Inserts `node` with `weight`, or lowers its key if it is already present
    /// with a heavier one. Returns whether the key changed.
    fn push_or_decrease(&mut self, node: usize, weight: W) -> bool {
        if self.contains(node) {
            match self.key[node] {
                Some(current) if weight < current => {
                    self.key[node] = Some(weight);
                    self.sift_up(self.pos[node]);
                    true
                }
                _ => false,
            }
        } else {
            self.key[node] = Some(weight);
            self.pos[node] = self.heap.len();
            self.heap.push(node);
            self.sift_up(self.heap.len() - 1);
            true
        }
    }

    fn pop_min(&mut self) -> Option<usize> {
        if self.heap.is_empty() {
            return None;
        }
        let last = self.heap.len() - 1;
        self.swap(0, last);
        let node = self.heap.pop()?;
        self.pos[node] = Self::ABSENT;
        if !self.heap.is_empty() {
            self.sift_down(0);
        }
        Some(node)
    }
}

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
    reject_unordered_weights(graph)?;

    let bound = index_bound(graph);
    let all_edges: Vec<(NodeId, NodeId, W)> = graph.edges().map(|(u, v, w)| (u, v, *w)).collect();
    // One candidate table per worker chunk. Rayon's `fold` would allocate a fresh
    // node-sized table for every internal split, which dominated the runtime on
    // graphs with a few hundred thousand edges.
    let chunk_len = all_edges
        .len()
        .div_ceil(rayon::current_num_threads().max(1))
        .max(1);

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
                    .par_chunks(chunk_len)
                    .map(|chunk| {
                        let mut acc = vec![None::<(NodeId, NodeId, W)>; bound];
                        for &(u, v, w) in chunk {
                            let ru = roots[u.index()];
                            let rv = roots[v.index()];
                            if ru != rv {
                                keep_lighter(&mut acc[ru], (u, v, w));
                                keep_lighter(&mut acc[rv], (u, v, w));
                            }
                        }
                        acc
                    })
                    .reduce_with(|mut a, b| {
                        for (slot, other) in a.iter_mut().zip(b) {
                            if let Some(cand) = other {
                                keep_lighter(slot, cand);
                            }
                        }
                        a
                    })
                    .unwrap_or_else(|| vec![None::<(NodeId, NodeId, W)>; bound])
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
/// - `W` must implement `Copy`, `PartialOrd`, `Add`, `AddAssign`, and `From<u8>`; weights must be
///   totally ordered in practice (a `NaN` weight is rejected with `InvalidArgument`).
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
    W: Copy + PartialOrd + Add<Output = W> + AddAssign + From<u8>,
    Ty: GraphConstructor<A, W>,
{
    if graph.node_count() == 0 {
        return Err(GraphinaError::invalid_graph(
            "Graph is empty, cannot compute MST.",
        ));
    }

    reject_unordered_weights(graph)?;

    let mut edges: Vec<(NodeId, NodeId, W)> = graph.edges().map(|(u, v, w)| (u, v, *w)).collect();
    // Weights are totally ordered after the check above, so the fallback never fires.
    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(Ordering::Equal));

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
/// - `W` must implement `Copy`, `PartialOrd`, `Add`, `AddAssign`, and `From<u8>`; weights must be
///   totally ordered in practice (a `NaN` weight is rejected with `InvalidArgument`).
/// - `Ty` must implement `GraphConstructor`.
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
    W: Copy + PartialOrd + Add<Output = W> + AddAssign + From<u8>,
    Ty: GraphConstructor<A, W>,
{
    if graph.node_count() == 0 {
        return Err(GraphinaError::invalid_graph(
            "Graph is empty, cannot compute MST.",
        ));
    }
    reject_unordered_weights(graph)?;

    let mut mst_edges = Vec::new();
    let mut total_weight = W::from(0u8);

    let bound = index_bound(graph);
    let mut in_tree = vec![false; bound];
    let mut parent: Vec<Option<NodeId>> = vec![None; bound];
    let mut adjacency: Vec<Vec<(NodeId, W)>> = vec![Vec::new(); bound];
    for (u, v, w) in graph.edges() {
        adjacency[u.index()].push((v, *w));
        adjacency[v.index()].push((u, *w));
    }

    // One tree per connected component. The indexed heap holds each frontier
    // node once, keyed by the lightest edge into it, so a pop always yields a
    // tree edge and the heap never exceeds the node count.
    let mut heap = IndexedMinHeap::new(bound);
    for start in graph.node_ids() {
        if in_tree[start.index()] {
            continue;
        }
        in_tree[start.index()] = true;
        prim_relax(&adjacency, &in_tree, &mut parent, &mut heap, start);

        while let Some(vi) = heap.pop_min() {
            in_tree[vi] = true;
            let to = NodeId::new(petgraph::graph::NodeIndex::new(vi));
            if let (Some(from), Some(weight)) = (parent[vi], heap.key(vi)) {
                mst_edges.push(MstEdge {
                    u: from,
                    v: to,
                    weight,
                });
                total_weight += weight;
            }
            prim_relax(&adjacency, &in_tree, &mut parent, &mut heap, to);
        }
    }

    Ok((mst_edges, total_weight))
}

/// Offers every edge leaving `node` to the frontier heap: a neighbor outside the
/// tree is inserted, or has its key lowered, when this edge is lighter than the
/// best one seen so far, and `parent` records where that edge came from.
fn prim_relax<W: Copy + PartialOrd>(
    adjacency: &[Vec<(NodeId, W)>],
    in_tree: &[bool],
    parent: &mut [Option<NodeId>],
    heap: &mut IndexedMinHeap<W>,
    node: NodeId,
) {
    for &(neighbor, weight) in &adjacency[node.index()] {
        let ni = neighbor.index();
        if !in_tree[ni] && heap.push_or_decrease(ni, weight) {
            parent[ni] = Some(node);
        }
    }
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

    #[test]
    fn test_mst_accepts_plain_f64_weights() {
        use crate::core::types::Graph;

        // Floating-point weights no longer need an `OrderedFloat` wrapper.
        let mut g = Graph::<i32, f64>::new();
        let a = g.add_node(0);
        let b = g.add_node(1);
        let c = g.add_node(2);
        g.add_edge(a, b, 1.5);
        g.add_edge(b, c, 2.5);
        g.add_edge(a, c, 10.0);
        let (k_edges, k_total) = kruskal_mst(&g).expect("kruskal");
        let (p_edges, p_total) = prim_mst(&g).expect("prim");
        let (b_edges, b_total) = boruvka_mst(&g).expect("boruvka");
        assert_eq!((k_edges.len(), p_edges.len(), b_edges.len()), (2, 2, 2));
        assert_eq!(k_total, 4.0);
        assert_eq!(p_total, 4.0);
        assert_eq!(b_total, 4.0);
    }

    #[test]
    fn test_mst_rejects_unordered_weights() {
        use crate::core::types::Graph;

        // A NaN weight has no place in a total order, so every algorithm reports
        // an invalid argument instead of sorting inconsistently.
        let mut g = Graph::<i32, f64>::new();
        let a = g.add_node(0);
        let b = g.add_node(1);
        g.add_edge(a, b, f64::NAN);
        assert!(kruskal_mst(&g).is_err());
        assert!(prim_mst(&g).is_err());
        assert!(boruvka_mst(&g).is_err());
    }
}
