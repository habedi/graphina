//! Soundarajan-Hopcroft link prediction algorithms.
//!
//! This module provides Soundarajan-Hopcroft based algorithms for link prediction.

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

/// CN Soundarajan–Hopcroft
/// For each pair (u, v), returns the number of common neighbors w such that
/// community(u) == community(v) == community(w).
pub fn cn_soundarajan_hopcroft<A, Ty, F, C>(
    graph: &BaseGraph<A, f64, Ty>,
    ebunch: Option<&[(NodeId, NodeId)]>,
    community: F,
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
        let score = common
            .into_iter()
            .filter(|w| community(u) == community(*w) && community(v) == community(*w))
            .count() as f64;
        results.push(((u, v), score));
    }
    results
}
