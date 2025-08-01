// File: src/centrality/algorithms.rs

//! Centrality algorithms module.
//!
//! This module provides implementations for a selection of centrality measures.
//! Measures included are:
//! - Degree centrality (total, in–, out–)
//! - Eigenvector centrality (wrapper: takes graph and max_iter)
//! - Katz centrality (wrapper: takes graph, alpha, beta, max_iter)
//! - Closeness centrality (using Dijkstra’s algorithm)
//! - PageRank (wrapper: takes graph, damping, max_iter)
//! - Betweenness centrality (node and edge)
//! - Harmonic centrality
//! - Local and global reaching centrality
//! - VoteRank
//! - Laplacian centrality

use crate::core::paths::dijkstra;
use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::{HashMap, VecDeque};

//
// -----------------------------
// Degree Centralities
// -----------------------------
//

/// Degree centrality: sum of a node’s in–degree and out–degree.
pub fn degree_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Vec<f64>
where
    W: Copy,
    Ty: GraphConstructor<A, W>,
{
    if !<Ty as GraphConstructor<A, W>>::is_directed() {
        return out_degree_centrality(&graph);
    }
    let n = graph.node_count();
    let mut degree = vec![0; n];
    // Out–degree.
    for (node, _) in graph.nodes() {
        degree[node.index()] += graph.neighbors(node).count();
    }
    // In–degree.
    for (_u, v, _w) in graph.edges() {
        degree[v.index()] += 1;
    }
    degree.into_iter().map(|d| d as f64).collect()
}

/// In–degree centrality.
pub fn in_degree_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Vec<f64>
where
    W: Copy,
    Ty: GraphConstructor<A, W>,
{
    if !<Ty as GraphConstructor<A, W>>::is_directed() {
        return out_degree_centrality(&graph);
    }
    let n = graph.node_count();
    let mut cent = vec![0.0; n];
    for (_u, v, _w) in graph.edges() {
        cent[v.index()] += 1.0;
    }
    cent
}

/// Out–degree centrality.
pub fn out_degree_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Vec<f64>
where
    W: Copy,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    let mut cent = vec![0.0; n];
    for (u, _) in graph.nodes() {
        cent[u.index()] = graph.neighbors(u).count() as f64;
    }
    cent
}

//
// -----------------------------
// Eigenvector Centrality
// -----------------------------
//

/// Full implementation of eigenvector centrality with convergence tolerance.
pub fn eigenvector_centrality_impl<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    max_iter: usize,
    tol: f64,
) -> Vec<f64>
where
    Ty: GraphConstructor<A, f64>,
{
    let n = graph.node_count();
    let mut centrality = vec![1.0; n];
    for _ in 0..max_iter {
        let mut next = vec![0.0; n];
        for (node, _) in graph.nodes() {
            for neighbor in graph.neighbors(node) {
                next[neighbor.index()] += centrality[node.index()];
            }
        }
        let norm = next.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 0.0 {
            for x in &mut next {
                *x /= norm;
            }
        }
        let diff: f64 = centrality
            .iter()
            .zip(next.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        centrality = next;
        if diff < tol * n as f64 {
            break;
        }
    }
    centrality
}

/// Wrapper for eigenvector centrality with default tolerance (1e-6).
pub fn eigenvector_centrality<A, Ty>(graph: &BaseGraph<A, f64, Ty>, max_iter: usize) -> Vec<f64>
where
    Ty: GraphConstructor<A, f64>,
{
    eigenvector_centrality_impl(graph, max_iter, 1e-6_f64)
}

/// NumPy–style eigenvector centrality (alias to the above).
pub fn eigenvector_centrality_numpy<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    max_iter: usize,
    tol: f64,
) -> Vec<f64>
where
    Ty: GraphConstructor<A, f64>,
{
    eigenvector_centrality_impl(graph, max_iter, tol)
}

//
// -----------------------------
// Katz Centrality
// -----------------------------
//

/// Full implementation of Katz centrality with convergence tolerance.
/// Formula: x = alpha * A * x + beta.
pub fn katz_centrality_impl<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    alpha: f64,
    beta: f64,
    max_iter: usize,
    tol: f64,
) -> Vec<f64>
where
    Ty: GraphConstructor<A, f64>,
{
    let n = graph.node_count();
    let mut centrality = vec![beta; n];
    for _ in 0..max_iter {
        let mut next = vec![beta; n];
        for (node, _) in graph.nodes() {
            for neighbor in graph.neighbors(node) {
                next[neighbor.index()] += alpha * centrality[node.index()];
            }
        }
        let norm = next.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 0.0 {
            for x in &mut next {
                *x /= norm;
            }
        }
        let diff: f64 = centrality
            .iter()
            .zip(next.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        centrality = next;
        if diff < tol * n as f64 {
            break;
        }
    }
    centrality
}

/// Wrapper for Katz centrality with default tolerance (1e-6).
/// This wrapper takes 4 arguments: graph, alpha, beta, max_iter.
pub fn katz_centrality<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    alpha: f64,
    beta: f64,
    max_iter: usize,
) -> Vec<f64>
where
    Ty: crate::core::types::GraphConstructor<A, f64>,
{
    katz_centrality_impl(graph, alpha, beta, max_iter, 1e-6_f64)
}

/// NumPy–style Katz centrality.
pub fn katz_centrality_numpy<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    alpha: f64,
    beta: f64,
) -> Vec<f64>
where
    Ty: crate::core::types::GraphConstructor<A, f64>,
{
    // Default parameters: 100 iterations, tol = 1e-6.
    katz_centrality_impl(graph, alpha, beta, 100, 1e-6_f64)
}

//
// -----------------------------
// Closeness Centrality
// -----------------------------
//

/// Compute closeness centrality using Dijkstra’s algorithm.
/// Closeness = (n - 1) / (sum of shortest-path distances).
pub fn closeness_centrality<A, Ty>(
    graph: &BaseGraph<A, ordered_float::OrderedFloat<f64>, Ty>,
) -> Vec<f64>
where
    Ty: GraphConstructor<A, ordered_float::OrderedFloat<f64>>,
{
    let n = graph.node_count();
    let mut closeness = vec![0.0; n];
    for (node, _) in graph.nodes() {
        let distances = dijkstra(graph, node);
        let sum: f64 = distances.iter().filter_map(|&d| d.map(|od| od.0)).sum();
        if sum > 0.0 {
            closeness[node.index()] = (n as f64 - 1.0) / sum;
        }
    }
    closeness
}

//
// -----------------------------
// PageRank
// -----------------------------
//

/// Full implementation of PageRank with convergence tolerance.
pub fn pagerank_impl<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    damping: f64,
    max_iter: usize,
    tol: f64,
) -> Vec<f64>
where
    Ty: GraphConstructor<A, f64>,
{
    let n = graph.node_count();
    let mut rank = vec![1.0 / n as f64; n];
    let teleport = (1.0 - damping) / n as f64;

    // Precompute out–degree.
    let mut out_deg = vec![0usize; n];
    for (node, _) in graph.nodes() {
        out_deg[node.index()] = graph.neighbors(node).count();
    }

    for _ in 0..max_iter {
        let mut new_rank = vec![teleport; n];
        for (u, _) in graph.nodes() {
            let r = rank[u.index()];
            if out_deg[u.index()] > 0 {
                let share = damping * r / out_deg[u.index()] as f64;
                for v in graph.neighbors(u) {
                    new_rank[v.index()] += share;
                }
            } else {
                // Dangling node: distribute uniformly.
                for x in new_rank.iter_mut() {
                    *x += damping * r / n as f64;
                }
            }
        }
        let diff: f64 = rank
            .iter()
            .zip(new_rank.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        rank = new_rank;
        if diff < tol * n as f64 {
            break;
        }
    }
    rank
}

/// Wrapper for PageRank with default tolerance (1e-6).
/// This wrapper takes 3 arguments: graph, damping, max_iter.
pub fn pagerank<A, Ty>(graph: &BaseGraph<A, f64, Ty>, damping: f64, max_iter: usize) -> Vec<f64>
where
    Ty: GraphConstructor<A, f64>,
{
    pagerank_impl(graph, damping, max_iter, 1e-6_f64)
}

//
// -----------------------------
// Betweenness Centrality
// -----------------------------
//

/// Compute betweenness centrality (node version) using Brandes’ algorithm.
pub fn betweenness_centrality<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> Vec<f64>
where
    Ty: GraphConstructor<A, f64>,
{
    let n = graph.node_count();
    let mut bc = vec![0.0; n];

    for (s, _) in graph.nodes() {
        let mut stack = Vec::with_capacity(n);
        let mut pred = vec![Vec::new(); n];
        let mut sigma = vec![0.0; n];
        let mut dist = vec![-1.0_f64; n];
        sigma[s.index()] = 1.0;
        dist[s.index()] = 0.0;
        let mut queue = VecDeque::new();
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for w in graph.neighbors(v) {
                if dist[w.index()] < 0.0 {
                    dist[w.index()] = dist[v.index()] + 1.0_f64;
                    queue.push_back(w);
                }
                if (dist[w.index()] - (dist[v.index()] + 1.0_f64)).abs() < 1e-6_f64 {
                    sigma[w.index()] += sigma[v.index()];
                    pred[w.index()].push(v);
                }
            }
        }

        let mut delta = vec![0.0; n];
        while let Some(w) = stack.pop() {
            for &v in &pred[w.index()] {
                delta[v.index()] +=
                    (sigma[v.index()] / sigma[w.index()]) * (1.0 + delta[w.index()]);
            }
            if w != s {
                bc[w.index()] += delta[w.index()];
            }
        }
    }
    bc
}

/// Compute edge betweenness centrality using a modified Brandes’ algorithm.
/// Returns a map from (source_index, target_index) to betweenness score.
pub fn edge_betweenness_centrality<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
) -> HashMap<(usize, usize), f64>
where
    Ty: GraphConstructor<A, f64>,
{
    let n = graph.node_count();
    let mut eb: HashMap<(usize, usize), f64> = HashMap::new();

    // Initialize each edge's betweenness to 0.
    for (u, v, _) in graph.edges() {
        eb.insert((u.index(), v.index()), 0.0);
    }

    for (s, _) in graph.nodes() {
        let mut stack = Vec::with_capacity(n);
        let mut pred = vec![Vec::new(); n];
        let mut sigma = vec![0.0; n];
        let mut dist = vec![-1.0_f64; n];
        sigma[s.index()] = 1.0;
        dist[s.index()] = 0.0;
        let mut queue = VecDeque::new();
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for w in graph.neighbors(v) {
                if dist[w.index()] < 0.0 {
                    dist[w.index()] = dist[v.index()] + 1.0_f64;
                    queue.push_back(w);
                }
                if (dist[w.index()] - (dist[v.index()] + 1.0_f64)).abs() < 1e-6_f64 {
                    sigma[w.index()] += sigma[v.index()];
                    pred[w.index()].push(v);
                }
            }
        }

        let mut delta = vec![0.0; n];
        while let Some(w) = stack.pop() {
            for &v in &pred[w.index()] {
                let c = (sigma[v.index()] / sigma[w.index()]) * (1.0 + delta[w.index()]);
                delta[v.index()] += c;
                if let Some(val) = eb.get_mut(&(v.index(), w.index())) {
                    *val += c;
                }
            }
        }
    }
    eb
}

//
// -----------------------------
// Harmonic Centrality
// -----------------------------
//

/// Harmonic centrality: sum of reciprocals of shortest-path distances (ignoring unreachable nodes).
pub fn harmonic_centrality<A, Ty>(
    graph: &BaseGraph<A, ordered_float::OrderedFloat<f64>, Ty>,
) -> Vec<f64>
where
    Ty: GraphConstructor<A, ordered_float::OrderedFloat<f64>>,
{
    let n = graph.node_count();
    let mut centrality = vec![0.0; n];
    for (node, _) in graph.nodes() {
        let distances = dijkstra(graph, node);
        let sum: f64 = distances
            .iter()
            .filter_map(|&d| d.map(|od| od.0))
            .filter(|&d| d > 0.0)
            .map(|d| 1.0 / d)
            .sum();
        centrality[node.index()] = sum;
    }
    centrality
}

//
// -----------------------------
// Reaching Centralities
// -----------------------------
//

/// Local reaching centrality: fraction of nodes reachable from a given node (via BFS).
pub fn local_reaching_centrality<A, Ty>(graph: &BaseGraph<A, f64, Ty>, v: NodeId) -> f64
where
    Ty: GraphConstructor<A, f64>,
{
    let n = graph.node_count();
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    visited[v.index()] = true;
    queue.push_back(v);
    let mut count = 0;
    while let Some(u) = queue.pop_front() {
        count += 1;
        for w in graph.neighbors(u) {
            if !visited[w.index()] {
                visited[w.index()] = true;
                queue.push_back(w);
            }
        }
    }
    if n > 1 {
        (count - 1) as f64 / (n as f64 - 1.0)
    } else {
        0.0
    }
}

/// Global reaching centrality: average difference between the maximum local reaching centrality and each node’s value.
pub fn global_reaching_centrality<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> f64
where
    Ty: GraphConstructor<A, f64>,
{
    let lrc: Vec<f64> = graph
        .nodes()
        .map(|(v, _)| local_reaching_centrality(graph, v))
        .collect();
    let max = lrc.iter().cloned().fold(f64::NEG_INFINITY, |a, b| a.max(b));
    let n = lrc.len() as f64;
    lrc.iter().map(|&x| max - x).sum::<f64>() / n
}

//
// -----------------------------
// VoteRank
// -----------------------------
//

/// VoteRank: iteratively select a set of influential nodes using a voting mechanism.
/// Initial scores are the nodes’ out–degrees; after selection, the scores of their neighbors are reduced.
pub fn voterank<A, Ty>(graph: &BaseGraph<A, f64, Ty>, number_of_nodes: usize) -> Vec<NodeId>
where
    Ty: GraphConstructor<A, f64>,
{
    let n = graph.node_count();
    let mut scores = vec![0.0; n];
    for (u, _) in graph.nodes() {
        scores[u.index()] = graph.neighbors(u).count() as f64;
    }
    let mut selected = Vec::new();
    let mut voted = vec![false; n];
    while selected.len() < number_of_nodes && selected.len() < n {
        let (max_idx, _) = scores
            .iter()
            .enumerate()
            .filter(|(i, _)| !voted[*i])
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();
        selected.push(NodeId::new(petgraph::graph::NodeIndex::new(max_idx)));
        voted[max_idx] = true;
        for nb in graph.neighbors(NodeId::new(petgraph::graph::NodeIndex::new(max_idx))) {
            scores[nb.index()] *= 0.8;
        }
    }
    selected
}

//
// -----------------------------
// Laplacian Centrality
// -----------------------------
//

/// Laplacian centrality for nodes, computed from local degree information.
/// Here LC(v) = d(v)^2 + 2 * (sum of 1’s for each neighbor).
pub fn laplacian_centrality<A, Ty>(graph: &BaseGraph<A, f64, Ty>, _normalized: bool) -> Vec<f64>
where
    Ty: GraphConstructor<A, f64>,
{
    graph
        .nodes()
        .map(|(u, _)| {
            let deg = graph.neighbors(u).count() as f64;
            let sum: f64 = graph.neighbors(u).map(|_| 1.0).sum();
            deg * deg + 2.0 * sum
        })
        .collect()
}
