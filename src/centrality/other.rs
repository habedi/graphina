//! Other centrality algorithms.
//!
//! This module provides additional centrality measures like local/global reaching, VoteRank, Laplacian centrality.
//!
//! Convention: most functions return `Result<_, crate::core::error::GraphinaError>` for
//! observability and error propagation. Selector-style routines that return node lists (e.g.,
//! `voterank`) may return plain values.

use crate::core::error::Result;
use crate::core::types::{BaseGraph, GraphConstructor, NodeId, NodeMap};
use std::collections::HashMap;
use std::collections::HashSet;

/// Local reaching centrality: measures the ability of a node to reach other nodes within a certain distance.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `distance`: the maximum distance to consider.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing local reaching centralities of each node in the graph.
pub fn local_reaching_centrality<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    distance: usize,
) -> Result<NodeMap<f64>>
where
    Ty: GraphConstructor<A, W>,
{
    let mut centrality = NodeMap::new();
    for (node, _) in graph.nodes() {
        let mut reached = HashSet::new();
        let mut current = HashSet::new();
        current.insert(node);
        reached.insert(node);

        for _ in 0..distance {
            let mut next = HashSet::new();
            for &n in &current {
                for neighbor in graph.neighbors(n) {
                    if !reached.contains(&neighbor) {
                        reached.insert(neighbor);
                        next.insert(neighbor);
                    }
                }
            }
            current = next;
        }

        centrality.insert(node, reached.len() as f64);
    }
    Ok(centrality)
}

/// Global reaching centrality: similar to local but considers the entire graph.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing global reaching centralities of each node in the graph.
pub fn global_reaching_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Result<NodeMap<f64>>
where
    Ty: GraphConstructor<A, W>,
{
    local_reaching_centrality(graph, graph.node_count())
}

/// VoteRank: a centrality measure based on voting.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `num_seeds`: number of seeds to select.
///
/// # Returns
///
/// A vector of `NodeId` representing the top nodes by VoteRank.
pub fn voterank<A, W, Ty>(graph: &BaseGraph<A, W, Ty>, num_seeds: usize) -> Vec<NodeId>
where
    Ty: GraphConstructor<A, W>,
{
    // Use a compact mapping to avoid relying on raw StableGraph indices
    let node_list: Vec<NodeId> = graph.nodes().map(|(u, _)| u).collect();
    let mut node_to_idx: HashMap<NodeId, usize> = HashMap::new();
    for (i, &nid) in node_list.iter().enumerate() {
        node_to_idx.insert(nid, i);
    }
    let mut votes = vec![0.0; node_list.len()];
    let mut selected = Vec::new();
    let mut remaining: HashSet<NodeId> = node_list.iter().copied().collect();

    for _ in 0..num_seeds.min(node_list.len()) {
        let mut max_vote = -1.0;
        let mut candidate = None;
        for &node in &remaining {
            let vote = graph
                .neighbors(node)
                .filter(|n| remaining.contains(n))
                .count() as f64;
            if vote > max_vote {
                max_vote = vote;
                candidate = Some(node);
            }
        }
        if let Some(node) = candidate {
            selected.push(node);
            remaining.remove(&node);
            for neighbor in graph.neighbors(node) {
                if remaining.contains(&neighbor) {
                    if let Some(&idx) = node_to_idx.get(&neighbor) {
                        votes[idx] -= 1.0;
                    }
                }
            }
        }
    }
    selected
}

/// Laplacian centrality: based on the Laplacian matrix of the graph.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing Laplacian centralities of each node in the graph.
pub fn laplacian_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Result<NodeMap<f64>>
where
    W: Copy + PartialOrd + Into<f64>,
    Ty: GraphConstructor<A, W>,
{
    let mut centrality = NodeMap::new();
    for (node, _) in graph.nodes() {
        let degree = graph.neighbors(node).count() as f64;
        let mut sum = degree * degree;
        for neighbor in graph.neighbors(node) {
            sum += graph.neighbors(neighbor).count() as f64;
        }
        centrality.insert(node, sum);
    }
    Ok(centrality)
}
