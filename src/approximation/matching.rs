//! Approximation algorithms for matching problems.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::HashSet;

/// Approximate the minimum maximal matching using a greedy algorithm.
pub fn min_maximal_matching<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> HashSet<(NodeId, NodeId)>
where
    Ty: GraphConstructor<A, f64>,
{
    let mut matching = HashSet::new();
    let mut matched = HashSet::new();
    for (u, v, _) in graph.edges() {
        if !matched.contains(&u) && !matched.contains(&v) {
            matching.insert((u, v));
            matched.insert(u);
            matched.insert(v);
        }
    }
    matching
}
