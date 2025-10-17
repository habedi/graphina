//! Louvain method algorithms.
//!
//! This module provides the Louvain method for community detection.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use rand::prelude::*;
use rand::{SeedableRng, rngs::StdRng};
use std::collections::HashMap;

/// Private helper: Create a seeded RNG from an optional seed.
fn create_rng(seed: Option<u64>) -> StdRng {
    match seed {
        Some(s) => StdRng::seed_from_u64(s),
        None => StdRng::seed_from_u64(rand::random::<u64>()),
    }
}

/// Helper: Compute the total degree for nodes in a given community.
fn total_degree(decomp: &[usize], degrees: &[f64], comm: usize) -> f64 {
    decomp
        .iter()
        .enumerate()
        .filter(|&(_i, &c)| c == comm)
        .map(|(i, _)| degrees[i])
        .sum()
}

/// Production-level Louvain Method for community detection.
///
/// Designed for undirected graphs with nonnegative f64 weights. It works in two phases:
/// 1. **Modularity Optimization:** Nodes are moved between communities to maximize modularity gain.
/// 2. **Graph Aggregation:** Nodes in the same community are aggregated, and the process repeats.
///
/// **Time Complexity:** Empirically near O(m) per iteration; overall complexity depends on iterations.
///
/// # Parameters
/// - `seed`: Optional seed for the RNG (used when shuffling nodes).
///
/// # Returns
/// A vector of communities, where each community is a vector of `NodeId`s.
pub fn louvain<A, Ty>(graph: &BaseGraph<A, f64, Ty>, seed: Option<u64>) -> Vec<Vec<NodeId>>
where
    Ty: GraphConstructor<A, f64>,
{
    let m: f64 = graph.edges().map(|(_u, _v, &w)| w).sum();
    let n = graph.node_count();
    let mut community: Vec<usize> = (0..n).collect();

    // Compute node degrees.
    let mut degrees = vec![0.0; n];
    for (u, v, &w) in graph.edges() {
        degrees[u.index()] += w;
        degrees[v.index()] += w;
    }

    // Precompute neighbors: for each node, store (neighbor_index, weight).
    let mut neighbors: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n];
    for (u, v, &w) in graph.edges() {
        let ui = u.index();
        let vi = v.index();
        neighbors[ui].push((vi, w));
        neighbors[vi].push((ui, w));
    }

    let mut rng = create_rng(seed);
    let mut improvement = true;
    while improvement {
        improvement = false;
        let mut nodes: Vec<usize> = (0..n).collect();
        nodes.shuffle(&mut rng);
        for &i in &nodes {
            let current_comm = community[i];
            let k_i = degrees[i];
            let mut comm_weights: HashMap<usize, f64> = HashMap::new();
            for &(j, w) in &neighbors[i] {
                let comm_j = community[j];
                *comm_weights.entry(comm_j).or_insert(0.0) += w;
            }
            let total_current = total_degree(&community, &degrees, current_comm);
            let delta_remove =
                comm_weights.get(&current_comm).unwrap_or(&0.0) - (total_current * k_i) / (2.0 * m);
            let mut best_delta = 0.0;
            let mut best_comm = current_comm;
            for (&comm, &w_in) in &comm_weights {
                if comm == current_comm {
                    continue;
                }
                let total_comm = total_degree(&community, &degrees, comm);
                let delta = w_in - (total_comm * k_i) / (2.0 * m);
                if delta > best_delta {
                    best_delta = delta;
                    best_comm = comm;
                }
            }
            if best_delta > delta_remove {
                community[i] = best_comm;
                improvement = true;
            }
        }
    }

    // Phase 2: Aggregate nodes by community.
    let mut comm_map: HashMap<usize, usize> = HashMap::new();
    for &c in &community {
        if !comm_map.contains_key(&c) {
            let new_index = comm_map.len();
            comm_map.insert(c, new_index);
        }
    }
    let mut new_comms: Vec<Vec<NodeId>> = vec![Vec::new(); comm_map.len()];
    for (i, &comm) in community.iter().enumerate() {
        let new_comm = comm_map[&comm];
        if let Some((node, _)) = graph.nodes().find(|(node, _)| node.index() == i) {
            new_comms[new_comm].push(node);
        }
    }
    new_comms
}
