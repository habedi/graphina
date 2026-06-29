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
    let mut centrality = NodeMap::default();
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
    // Compact mapping to avoid relying on raw StableGraph indices.
    let node_list: Vec<NodeId> = graph.nodes().map(|(u, _)| u).collect();
    let n = node_list.len();
    let mut influential = Vec::new();
    if n == 0 {
        return influential;
    }
    let mut node_to_idx: HashMap<NodeId, usize> = HashMap::new();
    for (i, &nid) in node_list.iter().enumerate() {
        node_to_idx.insert(nid, i);
    }
    let directed = graph.is_directed();

    // Average degree (in-degree for directed graphs) sets the rate at which a
    // selected node's neighbors lose voting ability.
    let total_degree: usize = node_list
        .iter()
        .map(|&v| {
            if directed {
                graph.in_degree(v).unwrap_or(0)
            } else {
                graph.degree(v).unwrap_or(0)
            }
        })
        .sum();
    let avg_degree = total_degree as f64 / n as f64;
    let decay = if avg_degree > 0.0 {
        1.0 / avg_degree
    } else {
        0.0
    };

    let mut ability = vec![1.0f64; n];
    let mut selected = vec![false; n];

    for _ in 0..num_seeds.min(n) {
        // Tally votes: each node's score is the sum of the voting ability of the
        // nodes that vote for it (its neighbors, or in-neighbors when directed).
        let mut score = vec![0.0f64; n];
        for (u, v, _) in graph.edges() {
            let (ui, vi) = (node_to_idx[&u], node_to_idx[&v]);
            score[vi] += ability[ui];
            if !directed {
                score[ui] += ability[vi];
            }
        }
        for (i, &sel) in selected.iter().enumerate() {
            if sel {
                score[i] = 0.0;
            }
        }

        // Select the highest-scoring node, breaking ties by node order.
        let mut best = 0usize;
        let mut best_score = -1.0;
        for (i, &s) in score.iter().enumerate() {
            if s > best_score {
                best_score = s;
                best = i;
            }
        }
        // No remaining node has any votes: stop electing.
        if best_score <= 0.0 {
            break;
        }

        selected[best] = true;
        ability[best] = 0.0;
        influential.push(node_list[best]);

        // Weaken the voting ability of the selected node's neighbors.
        for neighbor in graph.neighbors(node_list[best]) {
            if let Some(&j) = node_to_idx.get(&neighbor) {
                ability[j] = (ability[j] - decay).max(0.0);
            }
        }
    }
    influential
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
    // Precompute every node's degree once (O(E) total) so the neighbor-degree
    // sum below is O(1) per neighbor. The previous version recomputed each
    // neighbor's degree with `neighbors(neighbor).count()` inside the inner loop,
    // making the whole function roughly O(sum of degree^2).
    let degrees: HashMap<NodeId, f64> = graph
        .nodes()
        .map(|(node, _)| (node, graph.neighbors(node).count() as f64))
        .collect();

    let mut centrality = NodeMap::default();
    for (node, _) in graph.nodes() {
        let degree = degrees[&node];
        // Unnormalized Laplacian centrality (Qi et al.): the drop in Laplacian
        // energy when the node is removed. For an unweighted graph this is
        // d^2 + d + 2 * sum of neighbor degrees.
        let mut sum = degree * degree + degree;
        for neighbor in graph.neighbors(node) {
            sum += 2.0 * degrees[&neighbor];
        }
        centrality.insert(node, sum);
    }
    Ok(centrality)
}
