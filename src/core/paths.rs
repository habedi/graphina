/*!
# Shortest Paths Algorithms

This module provides a collection of shortest‑paths algorithms for the Graphina library.
It supports single‑source and all‑pairs computations via (classical) algorithms including:

- **Dijkstra’s Algorithm:**
  Computes single‑source shortest paths for graphs with nonnegative weights.

- **Bellman–Ford Algorithm:**
  Computes single‑source shortest paths even with negative weights and detects negative cycles.

- **A\* (A-Star) Algorithm:**
  Finds a shortest path from a source to a target using an admissible heuristic.

- **Floyd–Warshall Algorithm:**
  Computes all‑pairs shortest paths using dynamic programming.

- **Johnson’s Algorithm:**
  Computes all‑pairs shortest paths for sparse graphs (even with negative edge weights) by re-weighting the graph and then running Dijkstra’s algorithm from each node.

- **Iterative Deepening A\* (IDA\*):**
  A recursive, depth‑first variant of A\* search specialized for graphs with `f64` weights.
  The f64 is used instead of a generic weight type to simplify the implementation.

## Error Handling

Preconditions for each algorithm are enforced at runtime using custom exceptions from `graphina::core::exceptions`.
For example, algorithms that require nonnegative edge weights will return a `Result` containing a `GraphinaException`
if a negative weight is encountered. Users should handle these `Result` types accordingly.

*/

use crate::core::exceptions::GraphinaException;
use crate::core::types::{BaseGraph, GraphConstructor, GraphinaGraph, NodeId, NodeMap};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::ops::{Add, Sub};

use ordered_float::NotNan;

pub type PathFindResult = (NodeMap<Option<f64>>, NodeMap<Option<NodeId>>);

/// Returns an iterator over outgoing edges from a given node as `(target, weight)`.
fn outgoing_edges<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    u: NodeId,
) -> impl Iterator<Item = (NodeId, W)> + '_
where
    W: Copy,
    Ty: GraphConstructor<A, W>,
{
    graph
        .edges()
        .filter(move |(src, _tgt, _w)| *src == u)
        .map(|(_src, tgt, w)| (tgt, *w))
}

// ============================
// Dijkstra’s Algorithm
// ============================
//

/// Generic, Full implementationof Dijkstra's algorithm for finding shortest paths in a graph
/// with non-negative weights.
///
/// # Arguments
///
/// * `graph`: the target graph.
/// * `source`: the source of path finding.
/// * `cutoff`: the maximum total cost before stopping search.
/// * `eval_cost`: callback to evaluate the cost of possible edges in the graph, returning
///     - `Some(f64)` for cost,
///     - `None` for not passable edge.
///
/// # Returns
///
/// - `Vec<Option<f64>>` in which `None` for unreachable, and `Some(cost)` for the total path cost.
/// - `Vec<Option<NodeID>>` in which `None` for no traceback (i.e. is source or unreachable),
///   and `Some(NodeId)` for the previous node visited in the path.
///
/// # Error
///
/// return error, if encounter negative cost, or encounter `NaN` weight.
///
/// # Example
/// ```rust
/// use graphina::core::types::Digraph;
///
/// use graphina::core::paths::dijkstra_path_impl;
///
/// let mut graph: Digraph<String, (f64, String)> = Digraph::new();
/// //                             ^^^^^^^^^^^^^
/// //                                         L arbitrary type as edge
///
/// let cities = ["ATL", "PEK", "LHR", "HND", "CDG", "FRA", "HKG"];
///
/// let ids = cities
///     .iter()
///     .map(|s| graph.add_node(s.to_string()))
///     .collect::<Vec<_>>();
///
/// let edges = [
///     //
///     ("ATL", "PEK", (900.0, "boeing")),
///     ("ATL", "LHR", (500.0, "airbus")),
///     ("ATL", "HND", (700.0, "airbus")),
///     //
///     ("PEK", "LHR", (800.0, "boeing")),
///     ("PEK", "HND", (100.0, "airbus")),
///     ("PEK", "HKG", (100.0, "airbus")),
///     //
///     ("LHR", "CDG", (100.0, "airbus")),
///     ("LHR", "FRA", (200.0, "boeing")),
///     ("LHR", "HND", (600.0, "airbus")),
///     //
///     ("HND", "ATL", (700.0, "airbus")),
///     ("HND", "FRA", (600.0, "airbus")),
///     ("HND", "HKG", (100.0, "airbus")),
///     //
/// ];
///
/// for (s, d, w) in edges {
///     let depart = cities.iter().position(|city| s == *city).unwrap();
///     let destin = cities.iter().position(|city| d == *city).unwrap();
///     graph.add_edge(ids[depart], ids[destin], (w.0, w.1.to_string()));
/// }
///
/// // function for evaluating possible cost for the edge
/// // Some(f64) for cost
/// // None for impassable
/// let eval_cost = |(price, manufactuer): &(f64, String)| match manufactuer.as_str() {
///     "boeing" => None,  // avoid boeing plane
///     _ => Some(*price), // return price as the cost
/// };
///
/// let (cost, trace) = dijkstra_path_impl(&graph, ids[0], Some(1000.0), eval_cost).unwrap();
/// println!("cost : {:?}", cost);
/// println!("trace: {:?}", trace);
///
/// let expected_cost = [
///     Some(0.0),
///     None,
///     Some(500.0),
///     Some(700.0),
///     Some(600.0),
///     None,
///     Some(800.0),
/// ];
/// let expected_trace = [
///     None,
///     None,
///     Some(ids[0]),
///     Some(ids[0]),
///     Some(ids[2]),
///     None,
///     Some(ids[3]),
/// ];
///
/// for id in ids {
///     assert_eq!(cost[&id], expected_cost[id.index()]);
///     assert_eq!(trace[&id], expected_trace[id.index()]);
/// }
/// ```
pub fn dijkstra_path_impl<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    source: NodeId,
    cutoff: Option<f64>,
    eval_cost: impl Fn(&W) -> Option<f64>,
) -> Result<PathFindResult, GraphinaException>
where
    W: Debug,
    A: Debug,
    Ty: GraphConstructor<A, W>,
    NodeId: Ord,
    BaseGraph<A, W, Ty>: GraphinaGraph<A, W>,
{
    let mut dist = graph.to_nodemap_default();
    let mut trace = graph.to_nodemap_default();
    let mut heap = BinaryHeap::new();

    dist.insert(source, Some(0.0));
    heap.push(Reverse((NotNan::new(0.0).unwrap(), source)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if let Some(current) = dist[&u] {
            if *d > current {
                continue;
            }
        }
        for (v, edge) in graph.outgoing_edges(u) {
            let Some(w) = eval_cost(edge) else {
                continue;
            };
            if w.is_sign_negative() {
                return Err(GraphinaException::new(&format!(
                    "Dijkstra requires nonnegative costs, but found cost: {:?}, src: {:?}, dst: {:?}, edge: {:?}",
                    w, u, v, edge
                )));
            }
            let Ok(w) = NotNan::new(w) else {
                return Err(GraphinaException::new(&format!(
                    "Dijkstra requires not NaN costs, but found cost: {:?}, src: {:?}, dst: {:?}, edge: {:?}",
                    w, u, v, edge
                )));
            };
            let next = d + w;
            if let Some(cutoff) = cutoff {
                if *next > cutoff {
                    continue;
                }
            }
            if dist[&v].is_none() || Some(*next) < dist[&v] {
                dist.insert(v, Some(*next));
                trace.insert(v, Some(u));
                heap.push(Reverse((next, v)));
            }
        }
    }
    Ok((dist, trace))
}

/// Full implementation of Dijkstra's algorithm for finding shortest paths in a graph
/// for graph with edge type `f64`
/// with non-negative weights.
///
/// # Arguments
///
/// * `graph`: the target graph.
/// * `source`: the source of path finding.
/// * `cutoff`: the maximum total cost before stopping search.
///
/// # Returns
///
/// - `Vec<Option<f64>>` in which `None` for unreachable, and `Some(cost)` for the total path cost.
/// - `Vec<Option<NodeID>>` in which `None` for no traceback (i.e. is source or unreachable),
///   and `Some(NodeId)` for the previous node visited in the path.
///
/// # Error
///
/// return error, if encounter negative cost, or encounter `NaN` weight.
///
/// # Example
/// ```rust
/// use graphina::core::types::Graph;
///
/// use graphina::core::paths::dijkstra_path_f64;
///
/// let mut graph = Graph::new();
/// let ids = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();
/// let edges = [(0, 1, 1.0), (1, 2, 1.0), (2, 3, 2.0), (3, 4, 1.0)];
/// for (s, d, w) in edges {
///     graph.add_edge(ids[s], ids[d], w);
/// }
///
/// let (cost, trace) = dijkstra_path_f64(&graph, ids[0], None).unwrap();
///
/// println!("cost : {:?}", cost);
/// println!("trace: {:?}", trace);
/// let expected_cost = [Some(0.0), Some(1.0), Some(2.0), Some(4.0), Some(5.0)];
/// let expected_trace = [None, Some(ids[0]), Some(ids[1]), Some(ids[2]), Some(ids[3])];
///
/// for id in ids {
///     assert_eq!(cost[&id], expected_cost[id.index()]);
///     assert_eq!(trace[&id], expected_trace[id.index()]);
/// }
/// ```
pub fn dijkstra_path_f64<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    source: NodeId,
    cutoff: Option<f64>,
) -> Result<PathFindResult, GraphinaException>
where
    A: Debug,
    Ty: GraphConstructor<A, f64>,
    BaseGraph<A, f64, Ty>: GraphinaGraph<A, f64>,
{
    dijkstra_path_impl(graph, source, cutoff, |f| Some(*f))
}

/// Computes single‑source shortest paths for graphs with nonnegative weights.
///
/// # Returns
///
/// A `Result` containing a NodeMap keyed by node IDs, where each value is:
/// - `Some(cost)` if the node is reachable from the source, or
/// - `None` if it is unreachable.
///
/// Returns an `Err(GraphinaException)` if a negative edge weight is found.
///
/// # Complexity
///
/// - Time: O(E log V)
/// - Space: O(V)
pub fn dijkstra<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    source: NodeId,
) -> Result<NodeMap<Option<W>>, GraphinaException>
where
    W: Copy + PartialOrd + Add<Output = W> + Sub<Output = W> + From<u8> + Ord + Debug,
    Ty: GraphConstructor<A, W>,
    NodeId: Ord,
{
    // Use a NodeMap instead of Vec to avoid relying on contiguous NodeIndex values from StableGraph
    let mut dist: NodeMap<Option<W>> = graph.to_nodemap_default();
    let mut heap = BinaryHeap::new();

    dist.insert(source, Some(W::from(0u8)));
    heap.push(Reverse((W::from(0u8), source)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if let Some(current) = dist[&u] {
            if d > current {
                continue;
            }
        }
        for (v, w) in outgoing_edges(graph, u) {
            if w < W::from(0u8) {
                return Err(GraphinaException::new(&format!(
                    "Dijkstra requires nonnegative weights, but found weight: {:?}",
                    w
                )));
            }
            let next = d + w;
            if dist[&v].is_none() || Some(next) < dist[&v] {
                dist.insert(v, Some(next));
                heap.push(Reverse((next, v)));
            }
        }
    }
    Ok(dist)
}

/// ============================
/// Bellman–Ford Algorithm
/// ============================
///
/// Computes single‑source shortest paths for graphs with negative weights.
/// Returns `Some(distances)` if no negative cycle is detected, or `None` otherwise.
///
/// # Complexity
///
/// - **Time:** O(VE)
/// - **Space:** O(V)
pub fn bellman_ford<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    source: NodeId,
) -> Option<NodeMap<Option<W>>>
where
    W: Copy + PartialOrd + Add<Output = W> + From<u8>,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    let mut dist: NodeMap<Option<W>> = graph.to_nodemap_default();
    dist.insert(source, Some(W::from(0u8)));

    for _ in 0..n.saturating_sub(1) {
        let mut updated = false;
        for (u, v, &w) in graph.edges() {
            if let Some(du) = dist[&u] {
                let candidate = du + w;
                if dist[&v].is_none() || Some(candidate) < dist[&v] {
                    dist.insert(v, Some(candidate));
                    updated = true;
                }
            }
        }
        if !updated {
            break;
        }
    }
    // Check for negative cycles.
    for (u, v, &w) in graph.edges() {
        if let (Some(du), Some(dv)) = (dist[&u], dist[&v]) {
            if du + w < dv {
                return None;
            }
        }
    }
    Some(dist)
}

/// ============================
/// A* (A-Star) Algorithm
/// ============================
///
/// Finds a shortest path from `source` to `target` using an admissible heuristic.
///
/// # Preconditions
///
/// - The heuristic must be admissible (i.e., it never overestimates the true cost).
///
/// # Returns
///
/// A `Result` which is `Ok(Some((total_cost, path)))` if a path is found, `Ok(None)` if no path exists,
/// or an `Err(GraphinaException)` if a negative edge weight is found.
///
/// # Complexity
///
/// - **Time:** Worst-case \(O(E \log V)\)
/// - **Space:** \(O(V)\)
pub fn a_star<A, W, Ty, F>(
    graph: &BaseGraph<A, W, Ty>,
    source: NodeId,
    target: NodeId,
    heuristic: F,
) -> Result<Option<(W, Vec<NodeId>)>, GraphinaException>
where
    W: Copy + PartialOrd + Add<Output = W> + Sub<Output = W> + From<u8> + Ord + Debug,
    Ty: GraphConstructor<A, W>,
    F: Fn(NodeId) -> W,
    NodeId: Ord,
{
    let n = graph.node_count();
    let mut dist = vec![None; n];
    let mut prev = vec![None; n];
    let mut heap = BinaryHeap::new();

    dist[source.index()] = Some(W::from(0u8));
    heap.push(Reverse((W::from(0u8) + heuristic(source), source)));

    while let Some(Reverse((f, u))) = heap.pop() {
        if u == target {
            break;
        }
        if let Some(current) = dist[u.index()] {
            if f - heuristic(u) > current {
                continue;
            }
        }
        for (v, w) in outgoing_edges(graph, u) {
            if w < W::from(0u8) {
                return Err(GraphinaException::new(&format!(
                    "A* requires nonnegative weights, but found weight: {:?}",
                    w
                )));
            }
            let tentative = dist[u.index()].unwrap() + w;
            if dist[v.index()].is_none() || Some(tentative) < dist[v.index()] {
                dist[v.index()] = Some(tentative);
                prev[v.index()] = Some(u);
                let priority = tentative + heuristic(v);
                heap.push(Reverse((priority, v)));
            }
        }
    }

    if let Some(goal_cost) = dist[target.index()] {
        let mut path = Vec::new();
        let mut cur = target;
        while cur != source {
            path.push(cur);
            cur = prev[cur.index()].ok_or_else(|| {
                GraphinaException::new("Path reconstruction failed unexpectedly.")
            })?;
        }
        path.push(source);
        path.reverse();
        Ok(Some((goal_cost, path)))
    } else {
        Ok(None)
    }
}

/// ============================
/// Floyd–Warshall Algorithm
/// ============================
///
/// Computes all‑pairs shortest paths using dynamic programming.
/// Returns `Some(map)` where `map[u][v]` is:
///     - `Some(cost)` if a path from node `u` to `v` exists, or
///     - `None` if `v` is unreachable from `u`.
/// Returns `None` if a negative cycle is detected.
///
/// # Complexity
///
/// - **Time:** O(V^3)
/// - **Space:** O(V^2)
pub fn floyd_warshall<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Option<NodeMap<NodeMap<Option<W>>>>
where
    W: Copy + PartialOrd + Add<Output = W> + From<u8>,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    let nodes: Vec<NodeId> = graph.node_ids().collect();

    let mut dist = vec![vec![None; n]; n];
    for (i, row) in dist.iter_mut().enumerate().take(n) {
        row[i] = Some(W::from(0u8));
    }
    for (u, v, &w) in graph.edges() {
        let ui = u.index();
        let vi = v.index();
        match dist[ui][vi] {
            Some(current) if w < current => dist[ui][vi] = Some(w),
            None => dist[ui][vi] = Some(w),
            _ => {}
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let (Some(dik), Some(dkj)) = (dist[i][k], dist[k][j]) {
                    let candidate = dik + dkj;
                    match dist[i][j] {
                        Some(dij) if candidate < dij => dist[i][j] = Some(candidate),
                        None => dist[i][j] = Some(candidate),
                        _ => {}
                    }
                }
            }
        }
    }
    for (i, row) in dist.iter_mut().enumerate().take(n) {
        row[i] = Some(W::from(0u8));
    }
    // Convert to NodeMap form
    let mut outer: NodeMap<NodeMap<Option<W>>> = NodeMap::new();
    for (i, u) in nodes.iter().enumerate() {
        let mut inner: NodeMap<Option<W>> = NodeMap::new();
        for (j, v) in nodes.iter().enumerate() {
            inner.insert(*v, dist[i][j]);
        }
        outer.insert(*u, inner);
    }
    Some(outer)
}

/// ============================
/// Johnson’s Algorithm
/// ============================
///
/// Computes all‑pairs shortest paths for sparse graphs (even with negative edge weights)
/// by reweighting the graph to eliminate negatives and then running Dijkstra’s algorithm from each node.
/// Returns `Some(map)` if no negative cycle is detected, or `None` otherwise.
///
/// # Complexity
///
/// - **Time:** O(VE \log V) (implementation uses a binary heap)
/// - **Space:** O(V^2)
pub fn johnson<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Option<NodeMap<NodeMap<Option<W>>>>
where
    W: Copy + PartialOrd + Add<Output = W> + Sub<Output = W> + From<u8> + Ord,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    let mut h = vec![W::from(0u8); n];

    // Relax edges for n - 1 iterations.
    for _ in 0..n.saturating_sub(1) {
        let mut updated = false;
        for (u, v, &w) in graph.edges() {
            let ui = u.index();
            let vi = v.index();
            if h[ui] + w < h[vi] {
                h[vi] = h[ui] + w;
                updated = true;
            }
        }
        if !updated {
            break;
        }
    }
    // Check for negative cycles.
    for (u, v, &w) in graph.edges() {
        let ui = u.index();
        let vi = v.index();
        if h[ui] + w < h[vi] {
            return None;
        }
    }

    // Precompute mapping from contiguous indices to NodeId.
    let nodes: Vec<NodeId> = graph.nodes().map(|(node, _)| node).collect();

    let mut dist = vec![vec![None; n]; n];
    for u in 0..n {
        let start = nodes[u];
        let mut d = vec![None; n];
        d[u] = Some(W::from(0u8));
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((W::from(0u8), start)));
        while let Some(Reverse((du, current))) = heap.pop() {
            let ci = current.index();
            if let Some(cur) = d[ci] {
                if du > cur {
                    continue;
                }
            }
            for (v, w) in outgoing_edges(graph, current) {
                let vi = v.index();
                let new_w = w + h[current.index()] - h[vi];
                let nd = du + new_w;
                if d[vi].is_none() || Some(nd) < d[vi] {
                    d[vi] = Some(nd);
                    heap.push(Reverse((nd, v)));
                }
            }
        }
        for v in 0..n {
            if let Some(dprime) = d[v] {
                dist[u][v] = Some(dprime - h[u] + h[v]);
            }
        }
    }
    // Convert to NodeMap form
    let mut outer: NodeMap<NodeMap<Option<W>>> = NodeMap::new();
    for (i, u) in nodes.iter().enumerate() {
        let mut inner: NodeMap<Option<W>> = NodeMap::new();
        for (j, v) in nodes.iter().enumerate() {
            inner.insert(*v, dist[i][j]);
        }
        outer.insert(*u, inner);
    }
    Some(outer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{Digraph, NodeId};
    use ordered_float::OrderedFloat;
    use std::collections::HashMap;
    fn build_test_graph_ordered() -> (Digraph<i32, OrderedFloat<f64>>, HashMap<i32, NodeId>) {
        let mut graph: Digraph<i32, OrderedFloat<f64>> = Digraph::default();
        let mut nodes = HashMap::new();
        nodes.insert(0, graph.add_node(0));
        nodes.insert(1, graph.add_node(1));
        nodes.insert(2, graph.add_node(2));
        nodes.insert(3, graph.add_node(3));
        graph.add_edge(nodes[&0], nodes[&1], OrderedFloat(1.0));
        graph.add_edge(nodes[&0], nodes[&2], OrderedFloat(4.0));
        graph.add_edge(nodes[&1], nodes[&2], OrderedFloat(2.0));
        graph.add_edge(nodes[&1], nodes[&3], OrderedFloat(6.0));
        graph.add_edge(nodes[&2], nodes[&3], OrderedFloat(3.0));
        (graph, nodes)
    }
    #[test]
    fn test_dijkstra_directed() {
        let (graph, nodes) = build_test_graph_ordered();
        let n0 = nodes[&0];
        let n3 = nodes[&3];
        let dist = dijkstra(&graph, n0).unwrap();
        assert_eq!(dist[&n3], Some(OrderedFloat(6.0)));
    }
    #[test]
    fn test_bellman_ford_directed() {
        let (graph, nodes) = build_test_graph_ordered();
        let n0 = nodes[&0];
        let n3 = nodes[&3];
        let dist = bellman_ford(&graph, n0).expect("No negative cycle");
        assert_eq!(dist[&n3], Some(OrderedFloat(6.0)));
    }
    #[test]
    fn test_a_star_directed() {
        let (graph, nodes) = build_test_graph_ordered();
        let n0 = nodes[&0];
        let n1 = nodes[&1];
        let n2 = nodes[&2];
        let n3 = nodes[&3];
        let result = a_star(&graph, n0, n3, |_| OrderedFloat(0.0));
        assert!(result.is_ok());
        let path_opt = result.unwrap();
        assert!(path_opt.is_some());
        let (cost, path) = path_opt.unwrap();
        assert_eq!(cost, OrderedFloat(6.0));
        assert_eq!(path, vec![n0, n1, n2, n3]);
    }
    #[test]
    fn test_floyd_warshall_directed() {
        let (graph, nodes) = build_test_graph_ordered();
        let n0 = nodes[&0];
        let n3 = nodes[&3];
        let matrix = floyd_warshall(&graph).expect("No negative cycle");
        assert_eq!(matrix[&n0][&n3], Some(OrderedFloat(6.0)));
    }
}
