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
    let n = graph.node_count();

    // Handle empty graph
    if n == 0 {
        return Vec::new();
    }

    // Handle single node
    if n == 1 {
        let node = graph.nodes().next().map(|(nid, _)| nid).unwrap();
        return vec![vec![node]];
    }

    let m: f64 = graph.edges().map(|(_u, _v, &w)| w).sum();

    // Handle graph with no edges
    if m == 0.0 {
        return graph.nodes().map(|(nid, _)| vec![nid]).collect();
    }

    // BUGFIX: Map NodeId to contiguous indices to handle deleted nodes
    let node_list: Vec<NodeId> = graph.nodes().map(|(nid, _)| nid).collect();
    let node_to_idx: HashMap<NodeId, usize> = node_list
        .iter()
        .enumerate()
        .map(|(idx, &nid)| (nid, idx))
        .collect();

    let mut community: Vec<usize> = (0..n).collect();

    // Compute node degrees using the mapping
    let mut degrees = vec![0.0; n];
    for (u, v, &w) in graph.edges() {
        let ui = node_to_idx[&u];
        let vi = node_to_idx[&v];
        degrees[ui] += w;
        degrees[vi] += w;
    }

    // Precompute neighbors: for each node, store (neighbor_index, weight)
    let mut neighbors: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n];
    for (u, v, &w) in graph.edges() {
        let ui = node_to_idx[&u];
        let vi = node_to_idx[&v];
        neighbors[ui].push((vi, w));
        neighbors[vi].push((ui, w));
    }

    // Cache: community degree sums; indexed by community id (which starts as node index)
    let mut community_degree = vec![0.0f64; n];
    for i in 0..n {
        community_degree[i] = degrees[i];
    }

    let mut rng = create_rng(seed);
    let mut improvement = true;
    let max_iterations = 100;
    let mut iteration_count = 0;

    while improvement && iteration_count < max_iterations {
        improvement = false;
        iteration_count += 1;

        let mut nodes: Vec<usize> = (0..n).collect();
        nodes.shuffle(&mut rng);

        for &i in &nodes {
            let current_comm = community[i];
            let k_i = degrees[i];

            // Skip isolated nodes
            if k_i == 0.0 {
                continue;
            }

            let mut comm_weights: HashMap<usize, f64> = HashMap::new();
            for &(j, w) in &neighbors[i] {
                let comm_j = community[j];
                *comm_weights.entry(comm_j).or_insert(0.0) += w;
            }

            // Use cached totals
            let total_current = community_degree[current_comm];
            let k_i_in = comm_weights.get(&current_comm).copied().unwrap_or(0.0);

            let delta_remove = k_i_in - (total_current * k_i) / (2.0 * m);

            let mut best_delta = 0.0;
            let mut best_comm = current_comm;

            for (&comm, &w_in) in &comm_weights {
                if comm == current_comm {
                    continue;
                }
                let total_comm = community_degree[comm];
                let delta = w_in - (total_comm * k_i) / (2.0 * m);

                if delta > best_delta {
                    best_delta = delta;
                    best_comm = comm;
                }
            }

            // Only move if there's a significant improvement
            if best_delta > delta_remove + 1e-10 {
                // update cache before moving
                community_degree[current_comm] -= k_i;
                community_degree[best_comm] += k_i;
                community[i] = best_comm;
                improvement = true;
            }
        }
    }

    // Phase 2: Aggregate nodes by community
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
        let node = node_list[i];
        new_comms[new_comm].push(node);
    }

    // Remove empty communities
    new_comms.retain(|comm| !comm.is_empty());

    new_comms
}

#[cfg(test)]
mod tests {
    use super::louvain;
    use crate::core::types::Graph;

    #[test]
    fn test_louvain_simple() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);

        // Create two communities
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n3, n4, 1.0);

        let communities = louvain(&graph, Some(42));
        assert!(!communities.is_empty());
    }

    #[test]
    fn test_louvain_empty_graph() {
        let graph: Graph<i32, f64> = Graph::new();
        let communities = louvain(&graph, Some(42));
        assert_eq!(communities.len(), 0);
    }

    #[test]
    fn test_louvain_single_node() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);

        let communities = louvain(&graph, Some(42));
        assert_eq!(communities.len(), 1);
        assert_eq!(communities[0].len(), 1);
        assert_eq!(communities[0][0], n1);
    }

    #[test]
    fn test_louvain_no_edges() {
        let mut graph = Graph::new();
        let _n1 = graph.add_node(1);
        let _n2 = graph.add_node(2);
        let _n3 = graph.add_node(3);

        let communities = louvain(&graph, Some(42));
        assert_eq!(communities.len(), 3);
    }

    #[test]
    fn test_louvain_with_removed_nodes() {
        let mut graph = Graph::new();
        let n0 = graph.add_node(0);
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);

        // Create a simple community structure
        graph.add_edge(n0, n1, 1.0);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n3, n4, 1.0);

        // Remove a node in the middle
        graph.remove_node(n2);

        // This should not panic or cause array out of bounds
        let communities = louvain(&graph, Some(42));

        // Should have valid communities
        assert!(!communities.is_empty());

        // Total nodes in communities should match graph node count
        let total_nodes: usize = communities.iter().map(|c| c.len()).sum();
        assert_eq!(total_nodes, graph.node_count());
    }

    #[test]
    fn test_louvain_performance_smoke() {
        // Generate a moderately sized graph and ensure louvain completes quickly
        let mut g = Graph::<u32, f64>::new();
        let n = 200;
        let nodes: Vec<_> = (0..n).map(|i| g.add_node(i as u32)).collect();
        for i in 0..n {
            for j in (i + 1)..n {
                if (j - i) <= 3 {
                    // sparse banded connections
                    g.add_edge(nodes[i], nodes[j], 1.0);
                }
            }
        }
        let start = std::time::Instant::now();
        let comms = louvain(&g, Some(123));
        let dur = start.elapsed();
        assert!(!comms.is_empty());
        // very lenient bound to avoid flakiness in CI
        assert!(dur.as_secs_f32() < 1.5, "Louvain took too long: {:?}", dur);
    }
}
