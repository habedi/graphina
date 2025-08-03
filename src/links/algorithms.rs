// File: src/links/algorithms.rs

//! Link Prediction Algorithms
//!
//! This module implements several link prediction algorithms. Each function returns a vector of
//! pairs ((u, v), score). The optional parameter `ebunch` allows you to supply a slice of node pairs
//! for which the scores should be computed. If `ebunch` is None, the functions default to using all
//! unordered node pairs.

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

/// Jaccard Coefficient
/// For each pair (u, v), Jaccard = |N(u) ∩ N(v)| / |N(u) ∪ N(v)|
pub fn jaccard_coefficient<A, Ty>(
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
        let set_u: HashSet<_> = graph.neighbors(u).collect();
        let set_v: HashSet<_> = graph.neighbors(v).collect();
        let intersection = set_u.intersection(&set_v).count();
        let union = set_u.union(&set_v).count();
        let score = if union > 0 {
            intersection as f64 / union as f64
        } else {
            0.0
        };
        results.push(((u, v), score));
    }
    results
}

/// Adamic–Adar Index
/// For each pair (u, v), AA = sum_{w in N(u) ∩ N(v)} (1 / log(degree(w)))
pub fn adamic_adar_index<A, Ty>(
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
        let set_u: HashSet<_> = graph.neighbors(u).collect();
        let set_v: HashSet<_> = graph.neighbors(v).collect();
        let common: Vec<_> = set_u.intersection(&set_v).cloned().collect();
        let score: f64 = common
            .iter()
            .filter_map(|w| {
                let deg = graph.neighbors(*w).count();
                if deg > 1 {
                    Some(1.0 / (deg as f64).ln())
                } else {
                    None
                }
            })
            .sum();
        results.push(((u, v), score));
    }
    results
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

/// Within–Inter Cluster Ratio
/// For each pair (u, v), computes the ratio:
///    (within-cluster common neighbors + delta) / (inter-cluster common neighbors + delta)
/// where “within” means common neighbor w with community(u)==community(v)==community(w).
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
