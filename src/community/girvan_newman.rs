//! Girvan-Newman algorithms.
//!
//! This module provides Girvan-Newman for community detection.

use crate::core::error::{GraphinaError, Result};
use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::{HashMap, HashSet, VecDeque};

/// Production-level Girvan–Newman Algorithm.
///
/// Uses Brandes’ algorithm to compute edge betweenness centrality, then iteratively removes the edge
/// with the highest betweenness until the graph splits into at least `target_communities`.
///
/// **Time Complexity:** Worst-case O(n*m) per iteration (practically often lower).
///
/// # Returns
/// A vector of communities, where each community is a vector of `NodeId`s.
///
/// # Note
/// This algorithm is computationally expensive for very large graphs.
/// Returns `GraphinaError::InvalidGraph` if `target_communities == 0` or no further splits possible.
pub fn girvan_newman<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    target_communities: usize,
) -> Result<Vec<Vec<NodeId>>>
where
    W: Copy + PartialOrd + Into<f64> + From<u8>,
    Ty: GraphConstructor<A, W>,
{
    if target_communities == 0 {
        return Err(GraphinaError::invalid_graph(
            "Girvan-Newman: target_communities=0",
        ));
    }
    // Build explicit node index mapping to avoid relying on StableGraph raw indices
    let node_list: Vec<NodeId> = graph.nodes().map(|(node, _)| node).collect();
    let mut node_to_idx: HashMap<NodeId, usize> = HashMap::new();
    for (idx, &node) in node_list.iter().enumerate() {
        node_to_idx.insert(node, idx);
    }

    // Store only the endpoints (usize pairs) using the compact indices.
    let mut active_edges: HashSet<(usize, usize)> = graph
        .edges()
        .map(|(u, v, _w)| {
            let ui = node_to_idx[&u];
            let vi = node_to_idx[&v];
            if ui < vi { (ui, vi) } else { (vi, ui) }
        })
        .collect();

    // Build initial connectivity (neighbors set) from active edges.
    let n = node_list.len();
    let mut neighbors: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for &(u, v) in &active_edges {
        neighbors[u].insert(v);
        neighbors[v].insert(u);
    }

    // Remove edges iteratively until we reach the desired number of components.
    while connected_components_count(&neighbors) < target_communities {
        let edge_btwn = compute_edge_betweenness(n, &neighbors);
        if let Some((&(u, v), _)) = edge_btwn
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        {
            neighbors[u].remove(&v);
            neighbors[v].remove(&u);
            active_edges.retain(|&(a, b)| !(a == u && b == v));
        } else {
            return Err(GraphinaError::invalid_graph(
                "Girvan-Newman: no edges to split further",
            ));
        }
    }
    Ok(compute_components_from_neighbors(&neighbors, &node_list))
}

/// Helper: Compute connected components from an adjacency list and map back to NodeId.
fn compute_components_from_neighbors(
    neighbors: &[HashSet<usize>],
    node_list: &[NodeId],
) -> Vec<Vec<NodeId>> {
    let n = neighbors.len();
    let mut visited = vec![false; n];
    let mut components = Vec::new();
    for i in 0..n {
        if !visited[i] {
            let mut comp = Vec::new();
            let mut queue = VecDeque::new();
            queue.push_back(i);
            visited[i] = true;
            while let Some(u) = queue.pop_front() {
                comp.push(u);
                for &v in &neighbors[u] {
                    if !visited[v] {
                        visited[v] = true;
                        queue.push_back(v);
                    }
                }
            }
            // Convert indices to NodeId using the captured node_list mapping.
            let component: Vec<NodeId> = comp.into_iter().map(|i| node_list[i]).collect();
            components.push(component);
        }
    }
    components
}

/// Helper: Return the number of connected components.
fn connected_components_count(neighbors: &[HashSet<usize>]) -> usize {
    let n = neighbors.len();
    let mut visited = vec![false; n];
    let mut count = 0;
    for i in 0..n {
        if !visited[i] {
            count += 1;
            let mut queue = VecDeque::new();
            queue.push_back(i);
            visited[i] = true;
            while let Some(u) = queue.pop_front() {
                for &v in &neighbors[u] {
                    if !visited[v] {
                        visited[v] = true;
                        queue.push_back(v);
                    }
                }
            }
        }
    }
    count
}

/// Helper: Compute edge betweenness centrality using Brandes’ algorithm.
fn compute_edge_betweenness(
    n: usize,
    neighbors: &[HashSet<usize>],
) -> HashMap<(usize, usize), f64> {
    let mut edge_btwn: HashMap<(usize, usize), f64> = HashMap::new();
    for s in 0..n {
        let mut stack = Vec::new();
        let mut preds: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut sigma = vec![0.0; n];
        sigma[s] = 1.0;
        let mut dist = vec![-1; n];
        dist[s] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(s);
        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for &w in &neighbors[v] {
                if dist[w] < 0 {
                    dist[w] = dist[v] + 1;
                    queue.push_back(w);
                }
                if dist[w] == dist[v] + 1 {
                    sigma[w] += sigma[v];
                    preds[w].push(v);
                }
            }
        }
        let mut delta = vec![0.0; n];
        while let Some(w) = stack.pop() {
            for &v in &preds[w] {
                let c = (sigma[v] / sigma[w]) * (1.0 + delta[w]);
                let key = if v < w { (v, w) } else { (w, v) };
                *edge_btwn.entry(key).or_insert(0.0) += c;
                delta[v] += c;
            }
        }
    }
    edge_btwn
}
