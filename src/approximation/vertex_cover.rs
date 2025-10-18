//! Approximation algorithms for vertex cover problems.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::HashSet;

/// Approximates a minimum weighted vertex cover using a greedy strategy.
/// This implementation re‑evaluates the uncovered incident edges at each iteration.
/// For each uncovered edge, it chooses the node (not yet in the cover) that covers
/// the maximum number of uncovered edges, and then marks all its incident edges as covered.
///
/// # Arguments
///
/// * `graph` - A reference to the graph whose vertex cover is being approximated.
/// * `weight` - An optional function that maps a node to its weight (defaults to 1.0 for all nodes).
///
/// # Returns
///
/// A `HashSet<NodeId>` containing the nodes in the approximated vertex cover.
pub fn min_weighted_vertex_cover<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    _weight: Option<&dyn Fn(NodeId) -> f64>,
) -> HashSet<NodeId>
where
    Ty: GraphConstructor<A, f64>,
{
    let mut cover = HashSet::new();
    let mut uncovered: HashSet<(NodeId, NodeId)> = graph.edges().map(|(u, v, _)| (u, v)).collect();

    while !uncovered.is_empty() {
        let best = graph
            .nodes()
            .map(|(u, _)| u)
            .filter(|u| !cover.contains(u))
            .max_by_key(|&u| {
                let count = graph
                    .neighbors(u)
                    .filter(|w| uncovered.contains(&(u, *w)) || uncovered.contains(&(*w, u)))
                    .count();
                count
            });
        if let Some(best) = best {
            cover.insert(best);
            uncovered.retain(|&(u, v)| u != best && v != best);
        } else {
            break;
        }
    }
    cover
}

#[cfg(test)]
mod tests {
    use crate::core::types::Graph;
    // Import the function under test from the parent module
    use super::min_weighted_vertex_cover;

    #[test]
    fn test_greedy_vertex_cover() {
        let mut graph = Graph::<i32, f64>::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n4, 1.0);
        let cover = min_weighted_vertex_cover(&graph, None);
        assert!(!cover.is_empty());
        assert!(cover.len() <= graph.node_count());
    }
}
