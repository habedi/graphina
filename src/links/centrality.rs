//! Common neighbor centrality link prediction algorithms.
//!
//! This module provides common neighbor centrality based algorithms for link prediction.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::HashSet;

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

/// Common Neighbor Centrality (CCPA)
/// For each pair (u, v), returns (|N(u) ∩ N(v)|)^alpha.
pub fn common_neighbor_centrality<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    ebunch: Option<&[(NodeId, NodeId)]>,
    alpha: f64,
) -> Vec<((NodeId, NodeId), f64)>
where
    Ty: GraphConstructor<A, f64>,
{
    let pairs = match ebunch {
        Some(p) => p.to_vec(),
        None => default_ebunch(graph),
    };
    let mut results = Vec::new();
    for (u, v) in pairs {
        let set_u: HashSet<_> = graph.neighbors(u).collect();
        let set_v: HashSet<_> = graph.neighbors(v).collect();
        let common = set_u.intersection(&set_v).count();
        let score = (common as f64).powf(alpha);
        results.push(((u, v), score));
    }
    results
}
