//! Approximation algorithms for clique problems.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::{HashMap, HashSet};

/// Approximate a maximum clique using a greedy heuristic with neighbor caching.
pub fn max_clique<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> HashSet<NodeId>
where
    Ty: GraphConstructor<A, f64>,
{
    let nodes = graph.nodes().map(|(u, _)| u).collect();
    max_clique_in_nodes(graph, &nodes)
}

fn max_clique_in_nodes<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    nodes: &HashSet<NodeId>,
) -> HashSet<NodeId>
where
    Ty: GraphConstructor<A, f64>,
{
    let mut best = HashSet::new();

    // Build neighbor cache - this is guaranteed to contain all nodes
    let neighbor_cache: HashMap<NodeId, HashSet<NodeId>> = graph
        .nodes()
        .filter(|(u, _)| nodes.contains(u))
        .map(|(u, _)| {
            (
                u,
                graph.neighbors(u).filter(|v| nodes.contains(v)).collect(),
            )
        })
        .collect();

    for (node, _) in graph.nodes().filter(|(u, _)| nodes.contains(u)) {
        let mut clique = HashSet::new();
        clique.insert(node);

        // Safe: node is guaranteed to be in the cache since we just iterated over it
        let node_neighbors = match neighbor_cache.get(&node) {
            Some(neighbors) => neighbors,
            None => continue, // Defensive: should never happen
        };

        let mut neighbors: Vec<NodeId> = node_neighbors.iter().cloned().collect();
        neighbors.sort_by_key(|u| {
            std::cmp::Reverse(
                neighbor_cache.get(u).map(|n| n.len()).unwrap_or(0), // Defensive: use 0 if not found
            )
        });

        for v in neighbors {
            let is_connected_to_all = clique.iter().all(|&w| {
                neighbor_cache
                    .get(&w)
                    .map(|neighbors| neighbors.contains(&v))
                    .unwrap_or(false) // Defensive: if not found, not connected
            });

            if is_connected_to_all {
                clique.insert(v);
            }
        }

        if clique.len() > best.len() {
            best = clique;
        }
    }
    best
}

/// Repeatedly select a clique from the remaining nodes until every node is covered.
/// The graph is unchanged, and the cliques contain its original node IDs.
pub fn clique_removal<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> Vec<HashSet<NodeId>>
where
    Ty: GraphConstructor<A, f64>,
{
    let mut cliques = Vec::new();
    let mut remaining: HashSet<NodeId> = graph.nodes().map(|(u, _)| u).collect();
    while !remaining.is_empty() {
        let clique = max_clique_in_nodes(graph, &remaining);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_max_clique_empty_graph() {
        let graph: Graph<i32, f64> = Graph::new();
        let clique = max_clique(&graph);
        assert!(clique.is_empty());
    }

    #[test]
    fn test_max_clique_single_node() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let clique = max_clique(&graph);
        assert_eq!(clique.len(), 1);
        assert!(clique.contains(&n1));
    }

    #[test]
    fn test_max_clique_triangle() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n1, 1.0);

        let clique = max_clique(&graph);
        assert_eq!(clique.len(), 3);
    }

    #[test]
    fn test_clique_removal_empty() {
        let graph: Graph<i32, f64> = Graph::new();
        let cliques = clique_removal(&graph);
        assert!(cliques.is_empty());
    }

    #[test]
    fn test_clique_removal_covers_disjoint_edges_with_deleted_node() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let deleted = graph.add_node(0);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n3, n4, 1.0);
        graph.remove_node(deleted);

        let cliques = clique_removal(&graph);

        assert_eq!(cliques.len(), 2);
        assert!(cliques.contains(&HashSet::from([n1, n2])));
        assert!(cliques.contains(&HashSet::from([n3, n4])));
        let covered: HashSet<_> = cliques.iter().flatten().copied().collect();
        assert_eq!(covered, HashSet::from([n1, n2, n3, n4]));
        assert_eq!(cliques.iter().map(HashSet::len).sum::<usize>(), 4);
        assert_eq!(
            graph.nodes().map(|(u, _)| u).collect::<HashSet<_>>(),
            covered
        );
    }

    #[test]
    fn test_large_clique_size() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        graph.add_edge(n1, n2, 1.0);

        assert_eq!(large_clique_size(&graph), 2);
    }
}
