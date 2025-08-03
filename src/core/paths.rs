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
use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;
use std::ops::{Add, Sub};

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

/// ============================
/// Dijkstra’s Algorithm
/// ============================
///
/// Computes single‑source shortest paths for graphs with nonnegative weights.
///
/// # Returns
///
/// A `Result` containing a vector of length equal to the number of nodes, where each element is:
/// - `Some(cost)` if the node is reachable from the source, or
/// - `None` if it is unreachable.
///
/// Returns an `Err(GraphinaException)` if a negative edge weight is found.
///
/// # Complexity
///
/// - **Time:** \(O(E \log V)\)
/// - **Space:** \(O(V)\)
pub fn dijkstra<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    source: NodeId,
) -> Result<Vec<Option<W>>, GraphinaException>
where
    W: Copy + PartialOrd + Add<Output = W> + Sub<Output = W> + From<u8> + Ord + Debug,
    Ty: GraphConstructor<A, W>,
    NodeId: Ord,
{
    let n = graph.node_count();
    let mut dist = vec![None; n];
    let mut heap = BinaryHeap::new();

    dist[source.index()] = Some(W::from(0u8));
    heap.push(Reverse((W::from(0u8), source)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if let Some(current) = dist[u.index()] {
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
            if dist[v.index()].is_none() || Some(next) < dist[v.index()] {
                dist[v.index()] = Some(next);
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
/// - **Time:** \(O(VE)\)
/// - **Space:** \(O(V)\)
pub fn bellman_ford<A, W, Ty>(graph: &BaseGraph<A, W, Ty>, source: NodeId) -> Option<Vec<Option<W>>>
where
    W: Copy + PartialOrd + Add<Output = W> + From<u8>,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    let mut dist = vec![None; n];
    dist[source.index()] = Some(W::from(0u8));

    for _ in 0..n.saturating_sub(1) {
        let mut updated = false;
        for (u, v, &w) in graph.edges() {
            if let Some(du) = dist[u.index()] {
                let candidate = du + w;
                if dist[v.index()].is_none() || Some(candidate) < dist[v.index()] {
                    dist[v.index()] = Some(candidate);
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
        if let Some(du) = dist[u.index()] {
            if let Some(dv) = dist[v.index()] {
                if du + w < dv {
                    return None;
                }
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
/// Returns `Some(matrix)` where `matrix[i][j]` is:
///     - `Some(cost)` if a path from node `i` to `j` exists, or
///     - `None` if `j` is unreachable from `i`.
/// Returns `None` if a negative cycle is detected.
///
/// # Complexity
///
/// - **Time:** \(O(V^3)\)
/// - **Space:** \(O(V^2)\)
pub fn floyd_warshall<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Option<Vec<Vec<Option<W>>>>
where
    W: Copy + PartialOrd + Add<Output = W> + From<u8>,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
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
    for (i, row) in dist.iter().enumerate().take(n) {
        if let Some(dii) = row[i] {
            if dii < W::from(0u8) {
                return None;
            }
        }
    }
    Some(dist)
}

/// ============================
/// Johnson’s Algorithm
/// ============================
///
/// Computes all‑pairs shortest paths for sparse graphs (even with negative edge weights)
/// by reweighting the graph to eliminate negatives and then running Dijkstra’s algorithm from each node.
/// Returns `Some(matrix)` if no negative cycle is detected, or `None` otherwise.
///
/// # Complexity
///
/// - **Time:** \(O(VE \log V)\) (implementation uses a binary heap)
/// - **Space:** \(O(V^2)\)
pub fn johnson<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Option<Vec<Vec<Option<W>>>>
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
    Some(dist)
}

/// ============================
/// Iterative Deepening A* (IDA*)
/// ============================
///
/// Finds a path from `source` to `target` using the IDA* search with an admissible heuristic.
/// This implementation is specialized for graphs with `f64` weights.
///
/// # Returns
///
/// A `Result` which is `Ok(Some((total_cost, path)))` if a path is found, `Ok(None)` if no path exists,
/// or an `Err(GraphinaException)` if a negative edge weight is found.
///
/// # Complexity
///
/// - **Time:** Exponential in the worst‑case
/// - **Space:** \(O(V)\)
pub fn ida_star<A, Ty, F>(
    graph: &BaseGraph<A, f64, Ty>,
    source: NodeId,
    target: NodeId,
    heuristic: F,
) -> Result<Option<(f64, Vec<NodeId>)>, GraphinaException>
where
    Ty: GraphConstructor<A, f64>,
    F: Fn(NodeId) -> f64,
{
    for (_u, _v, &w) in graph.edges() {
        if w < 0.0 {
            return Err(GraphinaException::new(&format!(
                "IDA* requires nonnegative weights, but found weight: {}",
                w
            )));
        }
    }

    // Recursive search using a HashSet for fast cycle membership checks.
    #[allow(clippy::too_many_arguments)]
    fn search<A, Ty, F>(
        graph: &BaseGraph<A, f64, Ty>,
        current: NodeId,
        target: NodeId,
        g: f64,
        threshold: f64,
        heuristic: &F,
        path: &mut Vec<NodeId>,
        visited: &mut HashSet<NodeId>,
    ) -> Result<f64, f64>
    where
        Ty: GraphConstructor<A, f64>,
        F: Fn(NodeId) -> f64,
    {
        let f = g + heuristic(current);
        if f > threshold {
            return Err(f);
        }
        if current == target {
            return Ok(g);
        }
        let mut min = f64::INFINITY;
        for (neighbor, w) in outgoing_edges(graph, current) {
            if visited.contains(&neighbor) {
                continue;
            }
            path.push(neighbor);
            visited.insert(neighbor);
            match search(
                graph,
                neighbor,
                target,
                g + w,
                threshold,
                heuristic,
                path,
                visited,
            ) {
                Ok(cost) => return Ok(cost),
                Err(t) => {
                    if t < min {
                        min = t;
                    }
                }
            }
            visited.remove(&neighbor);
            path.pop();
        }
        Err(min)
    }

    let mut threshold = heuristic(source);
    let mut path = vec![source];
    let mut visited = HashSet::new();
    visited.insert(source);
    loop {
        match search(
            graph,
            source,
            target,
            0.0,
            threshold,
            &heuristic,
            &mut path,
            &mut visited,
        ) {
            Ok(cost) => return Ok(Some((cost, path))),
            Err(t) if t.is_infinite() => return Ok(None),
            Err(t) => threshold = t,
        }
    }
}
