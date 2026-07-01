//! Louvain method algorithms.
//!
//! This module provides the Louvain method for community detection.

use crate::core::error::{GraphinaError, Result};
use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use rand::prelude::*;
use rand::{SeedableRng, rngs::StdRng};
use rustc_hash::FxHashMap;

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
/// Returns `GraphinaError::InvalidGraph` on empty input.
pub fn louvain<A, Ty>(graph: &BaseGraph<A, f64, Ty>, seed: Option<u64>) -> Result<Vec<Vec<NodeId>>>
where
    Ty: GraphConstructor<A, f64>,
{
    let n = graph.node_count();

    // Handle empty graph
    if n == 0 {
        return Err(GraphinaError::invalid_graph("Louvain: empty graph"));
    }

    // Handle single node
    if n == 1 {
        let node = graph
            .nodes()
            .next()
            .map(|(nid, _)| nid)
            .ok_or_else(|| GraphinaError::invalid_graph("Louvain: missing node"))?;
        return Ok(vec![vec![node]]);
    }

    let m: f64 = graph.edges().map(|(_u, _v, &w)| w).sum();

    // Handle graph with no edges
    if m == 0.0 {
        return Ok(graph.nodes().map(|(nid, _)| vec![nid]).collect());
    }

    // Map NodeId to contiguous indices so removed nodes and sparse ids are handled.
    let node_list: Vec<NodeId> = graph.nodes().map(|(nid, _)| nid).collect();
    let node_to_idx: FxHashMap<NodeId, usize> = node_list
        .iter()
        .enumerate()
        .map(|(idx, &nid)| (nid, idx))
        .collect();

    let two_m = 2.0 * m;

    // Initial working graph: weighted inter-node adjacency (both directions) plus each
    // node's weighted degree. A self-loop adds twice to the degree and is not stored as
    // a neighbor.
    let mut deg = vec![0.0f64; n];
    let mut adj: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n];
    for (u, v, &w) in graph.edges() {
        let ui = node_to_idx[&u];
        let vi = node_to_idx[&v];
        if ui == vi {
            deg[ui] += 2.0 * w;
            continue;
        }
        deg[ui] += w;
        deg[vi] += w;
        adj[ui].push((vi, w));
        adj[vi].push((ui, w));
    }

    // belongs[o] is the current super-node that original node o has been folded into.
    let mut belongs: Vec<usize> = (0..n).collect();
    let mut rng = create_rng(seed);

    // Multi-level loop: local moving, then aggregate the resulting communities into a
    // smaller graph, and repeat until a pass merges nothing. Aggregation is what lets a
    // community grow beyond a single node's neighborhood; local moving on its own leaves
    // many small communities and low modularity.
    let max_levels = 100;
    for _ in 0..max_levels {
        let (comm, k) = one_level(&adj, &deg, two_m, &mut rng);
        for b in belongs.iter_mut() {
            *b = comm[*b];
        }
        if k == adj.len() {
            break; // no community merged, so the partition has converged
        }
        let (new_adj, new_deg) = aggregate_graph(&adj, &deg, &comm, k);
        adj = new_adj;
        deg = new_deg;
        if adj.len() == 1 {
            break;
        }
    }

    // Group original nodes by their final super-node.
    let final_k = belongs.iter().copied().max().map_or(0, |c| c + 1);
    let mut new_comms: Vec<Vec<NodeId>> = vec![Vec::new(); final_k];
    for (o, &b) in belongs.iter().enumerate() {
        new_comms[b].push(node_list[o]);
    }
    new_comms.retain(|comm| !comm.is_empty());

    Ok(new_comms)
}

/// One level of Louvain local moving on a weighted graph given as inter-node adjacency
/// (both directions) and node degrees. Returns each node's community label, compacted to
/// `0..k`, together with `k`. Neighbor communities are visited in sorted order and ties
/// are broken toward the lower community id, so the result is deterministic for a given
/// RNG sequence.
fn one_level(
    adj: &[Vec<(usize, f64)>],
    deg: &[f64],
    two_m: f64,
    rng: &mut StdRng,
) -> (Vec<usize>, usize) {
    let n = adj.len();
    let mut community: Vec<usize> = (0..n).collect();
    let mut tot: Vec<f64> = deg.to_vec();

    let mut improvement = true;
    let mut iter = 0;
    while improvement && iter < 100 {
        improvement = false;
        iter += 1;
        let mut order: Vec<usize> = (0..n).collect();
        order.shuffle(rng);
        for &i in &order {
            let ki = deg[i];
            if ki == 0.0 {
                continue;
            }
            let ci = community[i];

            // Weight from i to each neighboring community.
            let mut cw: FxHashMap<usize, f64> = FxHashMap::default();
            for &(j, w) in &adj[i] {
                *cw.entry(community[j]).or_insert(0.0) += w;
            }
            let mut candidates: Vec<(usize, f64)> = cw.into_iter().collect();
            candidates.sort_unstable_by_key(|&(c, _)| c);

            // Remove i from its community, then pick the community with the best gain.
            // Staying put is the baseline, so a move needs a strictly larger gain.
            tot[ci] -= ki;
            let w_to_ci = candidates
                .iter()
                .find(|&&(c, _)| c == ci)
                .map_or(0.0, |&(_, w)| w);
            let mut best_c = ci;
            let mut best_gain = w_to_ci - tot[ci] * ki / two_m;
            for &(c, w) in &candidates {
                if c == ci {
                    continue;
                }
                let gain = w - tot[c] * ki / two_m;
                if gain > best_gain + 1e-12 {
                    best_gain = gain;
                    best_c = c;
                }
            }

            tot[best_c] += ki;
            community[i] = best_c;
            if best_c != ci {
                improvement = true;
            }
        }
    }

    // Compact labels to 0..k.
    let mut relabel: FxHashMap<usize, usize> = FxHashMap::default();
    let mut out = vec![0usize; n];
    for (i, slot) in out.iter_mut().enumerate() {
        let next = relabel.len();
        *slot = *relabel.entry(community[i]).or_insert(next);
    }
    let k = relabel.len();
    (out, k)
}

/// Aggregate a weighted graph by community: each community becomes one node whose degree
/// is the sum of its members' degrees, with inter-community edge weights summed. Edges
/// internal to a community are dropped, since they are already reflected in the summed
/// degree; this keeps the total degree, and hence 2m, invariant across levels.
fn aggregate_graph(
    adj: &[Vec<(usize, f64)>],
    deg: &[f64],
    comm: &[usize],
    k: usize,
) -> (Vec<Vec<(usize, f64)>>, Vec<f64>) {
    let mut new_deg = vec![0.0f64; k];
    for (i, &d) in deg.iter().enumerate() {
        new_deg[comm[i]] += d;
    }

    let mut maps: Vec<FxHashMap<usize, f64>> = vec![FxHashMap::default(); k];
    for (i, nbrs) in adj.iter().enumerate() {
        let ci = comm[i];
        for &(j, w) in nbrs {
            let cj = comm[j];
            if ci != cj {
                *maps[ci].entry(cj).or_insert(0.0) += w;
            }
        }
    }
    let new_adj: Vec<Vec<(usize, f64)>> =
        maps.into_iter().map(|m| m.into_iter().collect()).collect();
    (new_adj, new_deg)
}

#[cfg(test)]
mod tests {
    use super::louvain;
    use crate::core::types::{Graph, NodeId};
    use std::collections::HashMap;

    /// Newman modularity of a partition, used to check partition quality.
    fn modularity(graph: &Graph<i32, f64>, comms: &[Vec<NodeId>]) -> f64 {
        let m: f64 = graph.edges().map(|(_, _, &w)| w).sum();
        if m == 0.0 {
            return 0.0;
        }
        let mut comm_of: HashMap<NodeId, usize> = HashMap::new();
        for (ci, c) in comms.iter().enumerate() {
            for &v in c {
                comm_of.insert(v, ci);
            }
        }
        let mut intra = vec![0.0; comms.len()];
        let mut dsum = vec![0.0; comms.len()];
        for (u, v, &w) in graph.edges() {
            dsum[comm_of[&u]] += w;
            dsum[comm_of[&v]] += w;
            if comm_of[&u] == comm_of[&v] {
                intra[comm_of[&u]] += w;
            }
        }
        let two_m = 2.0 * m;
        (0..comms.len())
            .map(|c| intra[c] / m - (dsum[c] / two_m).powi(2))
            .sum()
    }

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

        let communities = louvain(&graph, Some(42)).unwrap();
        assert!(!communities.is_empty());
    }

    #[test]
    fn test_louvain_empty_graph() {
        let graph: Graph<i32, f64> = Graph::new();
        let communities = louvain(&graph, Some(42)).unwrap_err();
        assert!(matches!(
            communities,
            crate::core::error::GraphinaError::InvalidGraph { .. }
        ));
        // empty graph now returns error
    }

    #[test]
    fn test_louvain_single_node() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);

        let communities = louvain(&graph, Some(42)).unwrap();
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

        let communities = louvain(&graph, Some(42)).unwrap();
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
        let communities = louvain(&graph, Some(42)).unwrap();

        // Should have valid communities
        assert!(!communities.is_empty());

        // Total nodes in communities should match graph node count
        let total_nodes: usize = communities.iter().map(|c| c.len()).sum();
        assert_eq!(total_nodes, graph.node_count());
    }

    #[test]
    fn test_louvain_multilevel_path_merges_segments() {
        // A path has no dense clusters, so a single pass of local moving only pairs up
        // adjacent nodes and leaves many tiny communities with low modularity.
        // Recovering the long contiguous segments (high modularity) requires the
        // aggregation step, so this pins the multi-level behavior against a regression
        // to a single-level implementation.
        let mut g = Graph::<i32, f64>::new();
        let nodes: Vec<_> = (0..50).map(|i| g.add_node(i)).collect();
        for i in 0..49 {
            g.add_edge(nodes[i], nodes[i + 1], 1.0);
        }
        let comms = louvain(&g, Some(0)).unwrap();
        let q = modularity(&g, &comms);
        assert!(
            q >= 0.6,
            "expected modularity >= 0.6 from multi-level Louvain, got {q}"
        );
        let total: usize = comms.iter().map(|c| c.len()).sum();
        assert_eq!(total, 50);
    }

    #[test]
    fn test_louvain_recovers_clique_chain() {
        // Four size-8 cliques joined by single bridge edges: the modularity-optimal
        // partition is exactly the four cliques.
        let mut g = Graph::<i32, f64>::new();
        let nodes: Vec<_> = (0..32).map(|i| g.add_node(i)).collect();
        for cl in 0..4 {
            let base = cl * 8;
            for i in 0..8 {
                for j in (i + 1)..8 {
                    g.add_edge(nodes[base + i], nodes[base + j], 1.0);
                }
            }
        }
        for cl in 0..3 {
            g.add_edge(nodes[cl * 8 + 7], nodes[(cl + 1) * 8], 1.0);
        }
        let comms = louvain(&g, Some(0)).unwrap();
        assert_eq!(comms.len(), 4);
        assert!(modularity(&g, &comms) > 0.7);
    }

    #[test]
    fn test_louvain_performance_smoke() {
        // Generate a moderately sized graph and guarantee louvain completes quickly
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
        let comms = louvain(&g, Some(123)).unwrap();
        let dur = start.elapsed();
        assert!(!comms.is_empty());
        // very lenient bound to avoid flakiness in CI
        assert!(dur.as_secs_f32() < 1.5, "Louvain took too long: {:?}", dur);
    }
}
