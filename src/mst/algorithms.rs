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
/// The algorithm iteratively finds, in parallel, the cheapest edge connecting each component to a different component.
/// These candidate edges are then processed sequentially using a union–find structure. The process continues until
/// a single component remains or no connecting edges are found (i.e. the graph is disconnected).
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

    let n = graph.node_count();
    let all_edges: Vec<(NodeId, NodeId, W)> = graph.edges().map(|(u, v, w)| (u, v, *w)).collect();

    let mut uf = UnionFind::new(n);
    let mut mst_edges = Vec::new();
    let mut total_weight = W::from(0u8);
    let mut components = n;

    while components > 1 {
        // Use the canonical component root for each node, not the raw parent
        // pointer: after unions the parent array is not path-compressed, so two
        // nodes in the same component can have different parents.
        let roots: Vec<usize> = (0..n).map(|i| uf.find(i)).collect();
        let cheapest: Vec<Option<(NodeId, NodeId, W)>> = (0..n)
            .into_par_iter()
            .map(|comp| {
                let mut min_edge: Option<(NodeId, NodeId, W)> = None;
                for &(u, v, w) in &all_edges {
                    let comp_u = roots[u.index()];
                    let comp_v = roots[v.index()];
                    if (comp_u == comp && comp_v != comp) || (comp_v == comp && comp_u != comp) {
                        match min_edge {
                            Some((_, _, current)) if w < current => min_edge = Some((u, v, w)),
                            None => min_edge = Some((u, v, w)),
                            _ => {}
                        }
                    }
                }
                min_edge
            })
            .collect();

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

    let n = graph.node_count();
    let mut edges: Vec<(NodeId, NodeId, W)> = graph.edges().map(|(u, v, w)| (u, v, *w)).collect();
    edges.sort_by(|a, b| a.2.cmp(&b.2));

    let mut uf = UnionFind::new(n);
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
    let mut in_tree: std::collections::HashSet<NodeId> = std::collections::HashSet::new();

    // Build an undirected incident-edge adjacency once (O(E)). The previous
    // version filtered the full edge list for every node added to the tree, which
    // was O(V * E); this makes expansion O(degree) per node. Each edge is stored
    // from both endpoints to mirror the original both-directions seeding.
    let mut adjacency: std::collections::HashMap<NodeId, Vec<(NodeId, W)>> =
        std::collections::HashMap::new();
    for (u, v, w) in graph.edges() {
        adjacency.entry(u).or_default().push((v, *w));
        adjacency.entry(v).or_default().push((u, *w));
    }
    let empty: Vec<(NodeId, W)> = Vec::new();

    // Process each connected component.
    for start in graph.nodes().map(|(node, _)| node) {
        if in_tree.contains(&start) {
            continue;
        }
        in_tree.insert(start);
        let mut heap = std::collections::BinaryHeap::new();

        for &(neighbor, weight) in adjacency.get(&start).unwrap_or(&empty) {
            heap.push(std::cmp::Reverse((weight, start, neighbor)));
        }

        while let Some(std::cmp::Reverse((w, u, v))) = heap.pop() {
            // Skip if both endpoints are already in the MST.
            if in_tree.contains(&u) && in_tree.contains(&v) {
                continue;
            }
            let (from, to) = if in_tree.contains(&u) { (u, v) } else { (v, u) };
            if !in_tree.contains(&to) {
                in_tree.insert(to);
                mst_edges.push(MstEdge {
                    u: from,
                    v: to,
                    weight: w,
                });
                total_weight += w;
                // Add all edges incident to the newly added node.
                for &(neighbor, weight) in adjacency.get(&to).unwrap_or(&empty) {
                    if !in_tree.contains(&neighbor) {
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
