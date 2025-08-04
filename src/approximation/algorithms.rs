//! This module provides fast approximations and heuristic methods for NP‑hard graph problems,
//! optimized for large, sparse graphs. Functions that rely on shortest–path computations require
//! that the graph’s weight type is `ordered_float::OrderedFloat<f64>`.
//!
//! Import with:
//! ```rust
//! use graphina::approximation::algorithms::*;
//! ```

use crate::core::exceptions::GraphinaException;
use crate::core::paths::dijkstra;
use crate::core::types::{BaseGraph, EdgeType, NodeId};
use ordered_float::OrderedFloat;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

/// -------------------------------
/// Helper: Find a path from `source` to `target` avoiding nodes in `blocked` using BFS.
fn find_path<A, Ty>(
    graph: &BaseGraph<A, OrderedFloat<f64>, Ty>,
    source: NodeId,
    target: NodeId,
    blocked: &HashSet<NodeId>,
) -> Option<Vec<NodeId>>
where
    Ty: EdgeType,
{
    let n = graph.node_count();
    let mut prev: Vec<Option<NodeId>> = vec![None; n];
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    visited[source.index()] = true;
    queue.push_back(source);
    while let Some(u) = queue.pop_front() {
        if u == target {
            let mut path = Vec::new();
            let mut cur = u;
            path.push(cur);
            while let Some(p) = prev[cur.index()] {
                cur = p;
                path.push(cur);
            }
            path.reverse();
            return Some(path);
        }
        for v in graph.neighbors(u) {
            if !visited[v.index()] && !blocked.contains(&v) {
                visited[v.index()] = true;
                prev[v.index()] = Some(u);
                queue.push_back(v);
            }
        }
    }
    None
}

/// Compute an approximate local node connectivity between source and target by
/// repeatedly finding vertex-disjoint paths using BFS.
pub fn local_node_connectivity<A, Ty>(
    graph: &BaseGraph<A, OrderedFloat<f64>, Ty>,
    source: NodeId,
    target: NodeId,
) -> usize
where
    Ty: EdgeType,
{
    let mut connectivity = 0;
    let mut blocked = HashSet::new();
    while let Some(path) = find_path(graph, source, target, &blocked) {
        // Block all intermediate nodes (exclude source and target)
        for &node in path.iter().skip(1).take(path.len() - 2) {
            blocked.insert(node);
        }
        connectivity += 1;
    }
    connectivity
}

/// -------------------------------
/// Approximate a maximum independent set using a greedy algorithm with neighbor caching.
pub fn maximum_independent_set<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> HashSet<NodeId>
where
    Ty: EdgeType,
{
    let mut mis = HashSet::new();
    let mut nodes: Vec<NodeId> = graph.nodes().map(|(u, _)| u).collect();
    let neighbor_cache: HashMap<NodeId, HashSet<NodeId>> = nodes
        .iter()
        .map(|&u| (u, graph.neighbors(u).collect()))
        .collect();
    nodes.sort_by_key(|&u| neighbor_cache.get(&u).unwrap().len());
    let mut used = HashSet::new();
    for u in nodes {
        if !used.contains(&u) {
            mis.insert(u);
            if let Some(neighbors) = neighbor_cache.get(&u) {
                for &v in neighbors {
                    used.insert(v);
                }
            }
        }
    }
    mis
}

/// -------------------------------
/// Approximate a maximum clique using a greedy heuristic with neighbor caching.
pub fn max_clique<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> HashSet<NodeId>
where
    Ty: EdgeType,
{
    let mut best = HashSet::new();
    let neighbor_cache: HashMap<NodeId, HashSet<NodeId>> = graph
        .nodes()
        .map(|(u, _)| (u, graph.neighbors(u).collect()))
        .collect();
    for (node, _) in graph.nodes() {
        let mut clique = HashSet::new();
        clique.insert(node);
        let mut neighbors: Vec<NodeId> =
            neighbor_cache.get(&node).unwrap().iter().cloned().collect();
        neighbors.sort_by_key(|u| std::cmp::Reverse(neighbor_cache.get(u).unwrap().len()));
        for v in neighbors {
            if clique
                .iter()
                .all(|&w| neighbor_cache.get(&w).unwrap().contains(&v))
            {
                clique.insert(v);
            }
        }
        if clique.len() > best.len() {
            best = clique;
        }
    }
    best
}

/// -------------------------------
/// Repeatedly remove a clique (found via max_clique) from the graph until no nodes remain.
pub fn clique_removal<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> Vec<HashSet<NodeId>>
where
    Ty: EdgeType,
{
    let mut cliques = Vec::new();
    let mut remaining: HashSet<NodeId> = graph.nodes().map(|(u, _)| u).collect();
    while !remaining.is_empty() {
        let clique = max_clique(graph)
            .into_iter()
            .filter(|u| remaining.contains(u))
            .collect::<HashSet<_>>();
        if clique.is_empty() {
            break;
        }
        for u in &clique {
            remaining.remove(u);
        }
        cliques.push(clique);
    }
    cliques
}

/// -------------------------------
/// Return the size of a large clique approximated by the max_clique heuristic.
pub fn large_clique_size<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> usize
where
    Ty: EdgeType,
{
    max_clique(graph).len()
}

/// -------------------------------
/// Estimate the average clustering coefficient using cached neighbor sets.
pub fn average_clustering<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> f64
where
    Ty: EdgeType,
{
    let mut total = 0.0;
    let mut count = 0;
    let neighbor_cache: HashMap<NodeId, HashSet<NodeId>> = graph
        .nodes()
        .map(|(u, _)| (u, graph.neighbors(u).collect()))
        .collect();
    for (u, _) in graph.nodes() {
        let neighbors = neighbor_cache.get(&u).unwrap();
        let k = neighbors.len();
        if k < 2 {
            continue;
        }
        let mut links = 0;
        let neighbor_vec: Vec<&NodeId> = neighbors.iter().collect();
        for i in 0..neighbor_vec.len() {
            for j in (i + 1)..neighbor_vec.len() {
                if neighbor_cache
                    .get(neighbor_vec[i])
                    .unwrap()
                    .contains(neighbor_vec[j])
                {
                    links += 1;
                }
            }
        }
        let possible = k * (k - 1) / 2;
        total += links as f64 / possible as f64;
        count += 1;
    }
    if count > 0 { total / count as f64 } else { 0.0 }
}

/// -------------------------------
/// Approximate the densest subgraph using a greedy peeling algorithm.
/// Uses a min-heap (with OrderedFloat) for efficiency.
pub fn densest_subgraph<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    _iterations: Option<usize>,
) -> HashSet<NodeId>
where
    Ty: EdgeType,
{
    let mut best_density = 0.0;
    let mut best_set = HashSet::new();
    let mut current_nodes: HashSet<NodeId> = graph.nodes().map(|(u, _)| u).collect();

    let mut degrees: HashMap<NodeId, usize> = current_nodes
        .iter()
        .map(|&u| {
            (
                u,
                graph
                    .neighbors(u)
                    .filter(|v| current_nodes.contains(v))
                    .count(),
            )
        })
        .collect();

    let mut heap: BinaryHeap<Reverse<(ordered_float::OrderedFloat<f64>, NodeId)>> =
        BinaryHeap::new();
    for (&u, &deg) in &degrees {
        heap.push(Reverse((OrderedFloat(deg as f64), u)));
    }

    while !current_nodes.is_empty() {
        let total_edges: usize = degrees.values().sum::<usize>() / 2;
        let density = total_edges as f64 / current_nodes.len() as f64;
        if density > best_density {
            best_density = density;
            best_set = current_nodes.clone();
        }
        if let Some(Reverse((_, u))) = heap.pop() {
            if !current_nodes.contains(&u) {
                continue;
            }
            current_nodes.remove(&u);
            let neighbors = graph.neighbors(u).collect::<HashSet<_>>();
            for v in neighbors {
                if current_nodes.contains(&v) {
                    if let Some(d) = degrees.get_mut(&v) {
                        *d = d.saturating_sub(1);
                        heap.push(Reverse((OrderedFloat(*d as f64), v)));
                    }
                }
            }
        } else {
            break;
        }
    }
    best_set
}

/// -------------------------------
/// Compute a lower bound on the diameter using BFS from an arbitrary node.
pub fn diameter<A, Ty>(
    graph: &BaseGraph<A, OrderedFloat<f64>, Ty>,
) -> Result<f64, GraphinaException>
where
    Ty: EdgeType,
{
    if let Some((start, _)) = graph.nodes().next() {
        let distances = dijkstra(graph, start)?;
        let max_dist = distances
            .into_iter()
            .filter_map(|d| d.map(|od| od.0))
            .fold(0.0, f64::max);
        Ok(max_dist)
    } else {
        Ok(0.0)
    }
}

/// Approximates a minimum weighted vertex cover using a greedy strategy.
/// This implementation re‑evaluates the uncovered incident edges at each iteration.
/// For each uncovered edge, it chooses the node (not yet in the cover) that covers
/// the maximum number of uncovered edges, and then marks all its incident edges as covered.
///
/// # Arguments
///
/// * `graph` - A reference to the graph whose vertex cover is being approximated.
/// * `weight` - An optional function that maps a node to its weight (defaults to 1.0 for all nodes).
///
/// # Returns
///
/// A `HashSet<NodeId>` containing the nodes in the approximated vertex cover.
pub fn min_weighted_vertex_cover<A, Ty>(
    graph: &crate::core::types::BaseGraph<A, f64, Ty>,
    _weight: Option<&dyn Fn(NodeId) -> f64>,
) -> std::collections::HashSet<NodeId>
where
    Ty: crate::core::types::EdgeType,
{
    let mut cover = HashSet::new();
    let mut uncovered: HashSet<(NodeId, NodeId)> = graph.edges().map(|(u, v, _)| (u, v)).collect();

    while !uncovered.is_empty() {
        let best = graph
            .nodes()
            .map(|(u, _)| u)
            .filter(|u| !cover.contains(u))
            .max_by_key(|&u| {
                let count = graph
                    .neighbors(u)
                    .filter(|w| uncovered.contains(&(u, *w)) || uncovered.contains(&(*w, u)))
                    .count();
                count
            });
        if let Some(best) = best {
            cover.insert(best);
            uncovered.retain(|&(u, v)| u != best && v != best);
        } else {
            break;
        }
    }
    cover
}

/// -------------------------------
/// Approximate the minimum maximal matching using a greedy algorithm.
pub fn min_maximal_matching<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> HashSet<(NodeId, NodeId)>
where
    Ty: EdgeType,
{
    let mut matching = HashSet::new();
    let mut matched = HashSet::new();
    for (u, v, _) in graph.edges() {
        if !matched.contains(&u) && !matched.contains(&v) {
            matching.insert((u, v));
            matched.insert(u);
            matched.insert(v);
        }
    }
    matching
}

/// -------------------------------
/// Approximate Ramsey R2 by computing a maximum clique and a maximum independent set.
pub fn ramsey_r2<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> (HashSet<NodeId>, HashSet<NodeId>)
where
    Ty: EdgeType,
{
    let clique = max_clique(graph);
    let independent_set = maximum_independent_set(graph);
    (clique, independent_set)
}

/// -------------------------------
/// Approximate a solution to the TSP using Christofides' algorithm (placeholder).
pub fn christofides<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
) -> Result<(Vec<NodeId>, f64), GraphinaException>
where
    A: Clone,
    Ty: EdgeType + EdgeType,
{
    let start_node = graph
        .nodes()
        .next()
        .map(|(u, _)| u)
        .ok_or_else(|| GraphinaException::new("Cannot run TSP on an empty graph."))?;
    greedy_tsp(&graph.convert::<OrderedFloat<f64>>(), start_node)
}

/// -------------------------------
/// Approximate the TSP solution using a greedy algorithm.
pub fn traveling_salesman_problem<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
) -> Result<(Vec<NodeId>, f64), GraphinaException>
where
    A: Clone,
    Ty: EdgeType + EdgeType,
{
    let start_node = graph
        .nodes()
        .next()
        .map(|(u, _)| u)
        .ok_or_else(|| GraphinaException::new("Cannot run TSP on an empty graph."))?;
    greedy_tsp(&graph.convert::<OrderedFloat<f64>>(), start_node)
}

/// -------------------------------
/// Greedy TSP: starting at `source`, repeatedly go to the nearest unvisited node.
/// This function requires the graph to use OrderedFloat<f64> weights.
pub fn greedy_tsp<A, Ty>(
    graph: &BaseGraph<A, OrderedFloat<f64>, Ty>,
    source: NodeId,
) -> Result<(Vec<NodeId>, f64), GraphinaException>
where
    Ty: EdgeType,
{
    let mut unvisited: HashSet<NodeId> = graph.nodes().map(|(u, _)| u).collect();
    let mut tour = Vec::new();
    let mut cost = 0.0;
    let mut current = source;
    tour.push(current);
    unvisited.remove(&current);
    while !unvisited.is_empty() {
        let distances = dijkstra(graph, current)?;
        let (next_node, next_cost) = unvisited
            .iter()
            .filter_map(|v| distances[v.index()].map(|d| (*v, d.0)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .ok_or_else(|| {
                GraphinaException::new("Could not find a path to any unvisited node.")
            })?;
        tour.push(next_node);
        cost += next_cost;
        current = next_node;
        unvisited.remove(&current);
    }
    let distances = dijkstra(graph, current)?;
    if let Some(d) = distances[source.index()] {
        cost += d.0;
        tour.push(source);
    }
    Ok((tour, cost))
}

/// -------------------------------
/// Simulated Annealing TSP (placeholder): returns the initial cycle.
pub fn simulated_annealing_tsp<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    init_cycle: Vec<NodeId>,
) -> Result<(Vec<NodeId>, f64), GraphinaException>
where
    A: Clone,
    Ty: EdgeType + EdgeType,
{
    let cost = tour_cost(&graph.convert::<OrderedFloat<f64>>(), &init_cycle)?;
    Ok((init_cycle, cost))
}

/// -------------------------------
/// Threshold Accepting TSP (placeholder): returns the initial cycle.
pub fn threshold_accepting_tsp<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    init_cycle: Vec<NodeId>,
) -> Result<(Vec<NodeId>, f64), GraphinaException>
where
    A: Clone,
    Ty: EdgeType + EdgeType,
{
    let cost = tour_cost(&graph.convert::<OrderedFloat<f64>>(), &init_cycle)?;
    Ok((init_cycle, cost))
}

/// -------------------------------
/// Asadpour ATSP (not implemented).
pub fn asadpour_atsp<A, Ty>(_graph: &BaseGraph<A, f64, Ty>) -> (Vec<NodeId>, f64)
where
    Ty: EdgeType,
{
    unimplemented!("Asadpour ATSP algorithm is not implemented yet")
}

/// -------------------------------
/// Helper: Compute the total cost of a given tour.
fn tour_cost<A, Ty>(
    graph: &BaseGraph<A, OrderedFloat<f64>, Ty>,
    tour: &[NodeId],
) -> Result<f64, GraphinaException>
where
    Ty: EdgeType,
{
    let mut cost = 0.0;
    for i in 0..tour.len() - 1 {
        let distances = dijkstra(graph, tour[i])?;
        if let Some(d) = distances[tour[i + 1].index()] {
            cost += d.0;
        }
    }
    Ok(cost)
}

/// -------------------------------
/// Compute a treewidth decomposition using the Minimum Degree heuristic with a min-heap.
pub fn treewidth_min_degree<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> (usize, Vec<NodeId>)
where
    Ty: EdgeType,
{
    let mut order = Vec::new();
    let mut remaining: HashSet<NodeId> = graph.nodes().map(|(u, _)| u).collect();
    let mut neighbor_cache: HashMap<NodeId, HashSet<NodeId>> = graph
        .nodes()
        .map(|(u, _)| (u, graph.neighbors(u).collect()))
        .collect();
    let mut treewidth = 0;
    let mut heap: BinaryHeap<Reverse<(usize, NodeId)>> = BinaryHeap::new();
    for (&u, neighbors) in &neighbor_cache {
        heap.push(Reverse((neighbors.len(), u)));
    }
    while !remaining.is_empty() {
        let Reverse((deg, u)) = heap.pop().unwrap();
        if !remaining.contains(&u) {
            continue;
        }
        if deg > treewidth {
            treewidth = deg;
        }
        order.push(u);
        remaining.remove(&u);
        let neighbors = neighbor_cache.get(&u).unwrap().clone();
        for &v in &neighbors {
            if remaining.contains(&v) {
                if let Some(entry) = neighbor_cache.get_mut(&v) {
                    entry.remove(&u);
                    heap.push(Reverse((entry.len(), v)));
                }
            }
        }
    }
    (treewidth, order)
}

/// -------------------------------
/// Compute a treewidth decomposition using the Minimum Fill-in heuristic.
pub fn treewidth_min_fill_in<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> (usize, Vec<NodeId>)
where
    Ty: EdgeType,
{
    let mut order = Vec::new();
    let mut remaining: HashSet<NodeId> = graph.nodes().map(|(u, _)| u).collect();
    let mut treewidth = 0;
    while !remaining.is_empty() {
        let &u = remaining
            .iter()
            .min_by_key(|&&u| {
                let neighbors: Vec<NodeId> = graph
                    .neighbors(u)
                    .filter(|v| remaining.contains(v))
                    .collect();
                let mut fill_in = 0;
                for i in 0..neighbors.len() {
                    for j in i + 1..neighbors.len() {
                        if !graph.neighbors(neighbors[i]).any(|x| x == neighbors[j]) {
                            fill_in += 1;
                        }
                    }
                }
                fill_in
            })
            .unwrap();
        let deg = graph.neighbors(u).filter(|v| remaining.contains(v)).count();
        if deg > treewidth {
            treewidth = deg;
        }
        order.push(u);
        remaining.remove(&u);
    }
    (treewidth, order)
}
