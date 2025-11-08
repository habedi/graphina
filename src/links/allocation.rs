//! Resource allocation-based link prediction algorithms.
//!
//! This module provides resource allocation-based algorithms for link prediction.

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

/// Resource Allocation Index (RA)
/// For each pair (u, v), RA = sum_{w in N(u) ∩ N(v)} (1 / degree(w))
pub fn resource_allocation_index<A, Ty>(
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
        let neighbors_u: HashSet<_> = graph.neighbors(u).collect();
        let neighbors_v: HashSet<_> = graph.neighbors(v).collect();
        let common: Vec<_> = neighbors_u.intersection(&neighbors_v).cloned().collect();
        let score: f64 = common
            .iter()
            .map(|w| {
                let deg = graph.neighbors(*w).count();
                if deg > 0 { 1.0 / deg as f64 } else { 0.0 }
            })
            .sum();
        results.push(((u, v), score));
    }
    results
}

/// RA Index Soundarajan–Hopcroft
/// For each pair (u, v), returns the resource allocation index over common neighbors w
/// for which community(u)==community(v)==community(w).
pub fn ra_index_soundarajan_hopcroft<A, Ty, F, C>(
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
        let score: f64 = common
            .into_iter()
            .filter_map(|w| {
                if community(u) == community(w) && community(v) == community(w) {
                    let deg = graph.neighbors(w).count();
                    if deg > 0 {
                        Some(1.0 / (deg as f64))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum();
        results.push(((u, v), score));
    }
    results
}
