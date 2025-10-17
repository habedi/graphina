//! Approximation algorithms for independent set problems.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::{HashMap, HashSet};

/// Approximate a maximum independent set using a greedy algorithm with neighbor caching.
pub fn maximum_independent_set<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> HashSet<NodeId>
where
    Ty: GraphConstructor<A, f64>,
{
    let mut mis = HashSet::new();
    let mut nodes: Vec<NodeId> = graph.nodes().map(|(u, _)| u).collect();
    let neighbor_cache: HashMap<NodeId, HashSet<NodeId>> = nodes
        .iter()
        .map(|&u| (u, graph.neighbors(u).collect()))
        .collect();
    nodes.sort_by_key(|&u| neighbor_cache.get(&u).unwrap().len());
    let mut used = HashSet::new();
    for u in nodes {
        if !used.contains(&u) {
            mis.insert(u);
            if let Some(neighbors) = neighbor_cache.get(&u) {
                for &v in neighbors {
                    used.insert(v);
                }
            }
        }
    }
    mis
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_greedy_independent_set() {
        let mut graph = Graph::<i32, f64>::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);
        let indep_set = maximum_independent_set(&graph);
        assert!(!indep_set.is_empty());
    }
}
