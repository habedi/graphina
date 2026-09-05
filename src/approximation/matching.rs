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
        if u != v && !matched.contains(&u) && !matched.contains(&v) {
            matching.insert((u, v));
            matched.insert(u);
            matched.insert(v);
        }
    }
    matching
}

#[cfg(test)]
mod tests {
    use super::min_maximal_matching;
    use crate::core::types::Graph;

    #[test]
    fn self_loops_are_not_matching_edges() {
        let mut g = Graph::<i32, f64>::new();
        let a = g.add_node(0);
        let b = g.add_node(1);
        g.add_edge(a, a, 1.0);
        g.add_edge(a, b, 1.0);
        let matching = min_maximal_matching(&g);
        assert_eq!(matching.len(), 1);
        assert!(matching.contains(&(a, b)) || matching.contains(&(b, a)));
    }
}
