//! Approximation algorithms for clustering problems.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::{HashMap, HashSet};

/// Estimate the average clustering coefficient using cached neighbor sets.
pub fn average_clustering<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> f64
where
    Ty: GraphConstructor<A, f64>,
{
    let mut total = 0.0;
    let mut count = 0;
    let neighbor_cache: HashMap<NodeId, HashSet<NodeId>> = graph
        .nodes()
        .map(|(u, _)| (u, graph.neighbors(u).collect()))
        .collect();
    for (u, _) in graph.nodes() {
        if let Some(neighbors) = neighbor_cache.get(&u) {
            let k = neighbors.len();
            if k < 2 {
                continue;
            }
            let mut links = 0;
            let neighbor_vec: Vec<&NodeId> = neighbors.iter().collect();
            for i in 0..neighbor_vec.len() {
                for j in (i + 1)..neighbor_vec.len() {
                    if let Some(set_i) = neighbor_cache.get(neighbor_vec[i]) {
                        if set_i.contains(neighbor_vec[j]) {
                            links += 1;
                        }
                    }
                }
            }
            let possible = k * (k - 1) / 2;
            total += links as f64 / possible as f64;
            count += 1;
        }
    }
    if count > 0 { total / count as f64 } else { 0.0 }
}
