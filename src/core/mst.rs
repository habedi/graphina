/*!
# Minimum Spanning Tree Algorithms

This module implements multiple Minimum Spanning Tree (MST) algorithms for the Graphina library.
It provides the following algorithms:

- **Prim’s Algorithm:**
  A greedy approach that grows the MST by adding the minimum edge at each step.
  It computes an MST forest (covering all connected components).

- **Kruskal’s Algorithm:**
  Sorts all edges and uses a union–find data structure to avoid cycles.

- **Borůvka’s Algorithm (Parallel):**
  A parallel implementation using Rayon to process each component concurrently.

**Note:** The weight type `W` must implement `Ord`. If you wish to use floating‑point weights (e.g. `f32` or `f64`), consider wrapping them in a type that provides a total order (e.g. [`ordered_float::OrderedFloat`](https://docs.rs/ordered-float/)).

All algorithms assume that the graph's nodes are indexed from 0 to \(n-1\) and that edge weights satisfy the required ordering and arithmetic properties.
They use a union–find (disjoint-set) data structure with path compression and union by rank for cycle detection and component merging.

## Error Handling

If the input graph is empty, algorithms will return a `Result` containing a `GraphinaException`.
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
    pub u: NodeId,
    pub v: NodeId,
    pub weight: W,
}

///
/// ## Borůvka’s MST Algorithm (Parallel)
///
/// Computes the Minimum Spanning Tree (MST) using a parallel variant of Borůvka’s algorithm.
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
/// Returns an `Err(GraphinaException)` if the input graph is empty.
///
/// # Example
///
/// ```rust
/// use graphina::core::mst::boruvka_mst;
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
        let uf_snapshot = uf.parent.clone();
        let cheapest: Vec<Option<(NodeId, NodeId, W)>> = (0..n)
            .into_par_iter()
            .map(|comp| {
                let mut min_edge: Option<(NodeId, NodeId, W)> = None;
                for &(u, v, w) in &all_edges {
                    let comp_u = uf_snapshot[u.index()];
                    let comp_v = uf_snapshot[v.index()];
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
/// ## Kruskal’s MST Algorithm
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
/// Returns an `Err(GraphinaException)` if the input graph is empty.
///
/// # Example
///
/// ```rust
/// use graphina::core::mst::kruskal_mst;
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
/// ## Prim’s MST Algorithm
///
/// Computes the MST using Prim’s algorithm. This version processes all connected components
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
/// Returns an `Err(GraphinaException)` if the input graph is empty.
///
/// # Example
///
/// ```rust
/// use graphina::core::mst::prim_mst;
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

    let n = graph.node_count();
    let mut mst_edges = Vec::new();
    let mut total_weight = W::from(0u8);
    let mut in_tree = vec![false; n];

    // Process each connected component.
    for start in graph.nodes().map(|(node, _)| node) {
        if in_tree[start.index()] {
            continue;
        }
        in_tree[start.index()] = true;
        let mut heap = std::collections::BinaryHeap::new();

        // Add all edges incident to the starting node.
        for (_, v, weight) in graph
            .edges()
            .filter(|(u, _v, _w)| *u == start)
            .map(|(u, v, w)| (u, v, *w))
        {
            heap.push(std::cmp::Reverse((weight, start, v)));
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
                for (_, neighbor, weight) in graph
                    .edges()
                    .filter(|(x, _y, _w)| *x == to)
                    .map(|(x, y, w)| (x, y, *w))
                {
                    if !in_tree[neighbor.index()] {
                        heap.push(std::cmp::Reverse((weight, to, neighbor)));
                    }
                }
                // Also add edges where 'to' is the target.
                for (_, neighbor, weight) in graph
                    .edges()
                    .filter(|(_x, y, _w)| *y == to)
                    .map(|(x, y, w)| (x, y, *w))
                {
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
