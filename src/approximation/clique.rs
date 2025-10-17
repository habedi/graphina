//! Approximation algorithms for clique problems.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::{HashMap, HashSet};

/// Approximate a maximum clique using a greedy heuristic with neighbor caching.
pub fn max_clique<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> HashSet<NodeId>
where
    Ty: GraphConstructor<A, f64>,
{
    let mut best = HashSet::new();
    let neighbor_cache: HashMap<NodeId, HashSet<NodeId>> = graph
        .nodes()
        .map(|(u, _)| (u, graph.neighbors(u).collect()))
        .collect();
    for (node, _) in graph.nodes() {
        let mut clique = HashSet::new();
        clique.insert(node);
        let mut neighbors: Vec<NodeId> =
            neighbor_cache.get(&node).unwrap().iter().cloned().collect();
        neighbors.sort_by_key(|u| std::cmp::Reverse(neighbor_cache.get(u).unwrap().len()));
        for v in neighbors {
            if clique
                .iter()
                .all(|&w| neighbor_cache.get(&w).unwrap().contains(&v))
            {
                clique.insert(v);
            }
        }
        if clique.len() > best.len() {
            best = clique;
        }
    }
    best
}

/// Repeatedly remove a clique (found via max_clique) from the graph until no nodes remain.
pub fn clique_removal<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> Vec<HashSet<NodeId>>
where
    Ty: GraphConstructor<A, f64>,
{
    let mut cliques = Vec::new();
    let mut remaining: HashSet<NodeId> = graph.nodes().map(|(u, _)| u).collect();
    while !remaining.is_empty() {
        let clique = max_clique(graph)
            .into_iter()
            .filter(|u| remaining.contains(u))
            .collect::<HashSet<_>>();
        if clique.is_empty() {
            break;
        }
        for u in &clique {
            remaining.remove(u);
        }
        cliques.push(clique);
    }
    cliques
}

/// Return the size of a large clique approximated by the max_clique heuristic.
pub fn large_clique_size<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> usize
where
    Ty: GraphConstructor<A, f64>,
{
    max_clique(graph).len()
}
