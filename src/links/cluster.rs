//! Within-inter cluster link prediction algorithms.
//!
//! This module provides within-inter cluster based algorithms for link prediction.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};

/// Helper: If no ebunch is provided, generate all unordered pairs of nodes.
fn default_ebunch<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Vec<(NodeId, NodeId)>
where
    Ty: crate::core::types::GraphConstructor<A, W>,
{
    let nodes: Vec<NodeId> = graph.nodes().map(|(u, _)| u).collect();
    let mut ebunch = Vec::new();
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            ebunch.push((nodes[i], nodes[j]));
        }
    }
    ebunch
}

/// Within–Inter Cluster Ratio
/// For each pair (u, v), computes the ratio:
///    (within-cluster common neighbors + delta) / (inter-cluster common neighbors + delta)
/// where "within" means common neighbor w with community(u)==community(v)==community(w).
pub fn within_inter_cluster<A, Ty, F, C>(
    graph: &BaseGraph<A, f64, Ty>,
    ebunch: Option<&[(NodeId, NodeId)]>,
    community: F,
    delta: f64,
) -> Vec<((NodeId, NodeId), f64)>
where
    Ty: GraphConstructor<A, f64>,
    F: Fn(NodeId) -> C,
    C: Eq,
{
    let pairs = match ebunch {
        Some(p) => p.to_vec(),
        None => default_ebunch(graph),
    };
    let mut results = Vec::new();
    for (u, v) in pairs {
        let set_u: Vec<NodeId> = graph.neighbors(u).collect();
        let set_v: Vec<NodeId> = graph.neighbors(v).collect();
        let common: Vec<NodeId> = set_u.into_iter().filter(|w| set_v.contains(w)).collect();
        let within = common
            .iter()
            .filter(|&&w| community(u) == community(w) && community(v) == community(w))
            .count() as f64;
        let inter = (common.len() as f64) - within;
        let score = (within + delta) / (inter + delta);
        results.push(((u, v), score));
    }
    results
}
