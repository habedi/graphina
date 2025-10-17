//! Preferential attachment-based link prediction algorithms.
//!
//! This module provides preferential attachment-based algorithms for link prediction.

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

/// Preferential Attachment
/// For each pair (u, v), PA = degree(u) * degree(v)
pub fn preferential_attachment<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    ebunch: Option<&[(NodeId, NodeId)]>,
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
        let deg_u = graph.neighbors(u).count();
        let deg_v = graph.neighbors(v).count();
        let score = (deg_u * deg_v) as f64;
        results.push(((u, v), score));
    }
    results
}
