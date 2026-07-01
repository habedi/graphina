//! Approximation algorithms for vertex cover problems.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::{BinaryHeap, HashMap, HashSet};

/// Approximates a minimum vertex cover with the greedy maximum-degree heuristic:
/// repeatedly add the node covering the most still-uncovered edges, then mark all its
/// incident edges as covered.
///
/// The uncovered degree of every node is maintained incrementally through a
/// lazy-deletion max-heap, so each edge is touched a constant number of times. The run
/// time is O((V + E) log V), replacing the earlier version that rescanned every node
/// and all of its incident edges on each iteration (O(V * E)).
///
/// # Arguments
///
/// * `graph` - A reference to the graph whose vertex cover is being approximated.
///
/// # Returns
///
/// A `HashSet<NodeId>` containing the nodes in the approximated vertex cover.
pub fn min_weighted_vertex_cover<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> HashSet<NodeId>
where
    Ty: GraphConstructor<A, f64>,
{
    // Deduplicated undirected incidence. `adj[u]` holds u's distinct neighbors and
    // `deg[u]` counts its still-uncovered incident edges; a self-loop counts once
    // toward its own endpoint, which keeps that endpoint eligible until it is chosen.
    let mut adj: HashMap<NodeId, HashSet<NodeId>> = HashMap::new();
    let mut deg: HashMap<NodeId, usize> = HashMap::new();
    let mut self_loops: HashSet<NodeId> = HashSet::new();
    for (u, v, _) in graph.edges() {
        if u == v {
            if self_loops.insert(u) {
                *deg.entry(u).or_insert(0) += 1;
            }
            continue;
        }
        if adj.entry(u).or_default().insert(v) {
            *deg.entry(u).or_insert(0) += 1;
            adj.entry(v).or_default().insert(u);
            *deg.entry(v).or_insert(0) += 1;
        }
    }

    // Max-heap keyed by (uncovered degree, node id). Ties break toward the higher node
    // id, matching the previous `max_by_key` scan over ascending node order.
    let mut heap: BinaryHeap<(usize, NodeId)> = deg.iter().map(|(&u, &d)| (d, u)).collect();
    let mut cover: HashSet<NodeId> = HashSet::new();

    while let Some((d, u)) = heap.pop() {
        if cover.contains(&u) {
            continue; // already chosen
        }
        if deg.get(&u).copied().unwrap_or(0) != d {
            continue; // stale heap entry, a fresher one exists
        }
        if d == 0 {
            break; // the maximum uncovered degree is 0, so every edge is covered
        }
        cover.insert(u);
        // Every uncovered edge incident to u is now covered, so each still-uncovered
        // neighbor loses one incident uncovered edge.
        if let Some(neighbors) = adj.get(&u) {
            for &w in neighbors {
                if !cover.contains(&w) {
                    if let Some(dw) = deg.get_mut(&w) {
                        *dw -= 1;
                        heap.push((*dw, w));
                    }
                }
            }
        }
    }
    cover
}

#[cfg(test)]
mod tests {
    // Import the function under test from the parent module
    use super::min_weighted_vertex_cover;
    use crate::core::types::{Graph, NodeId};
    use std::collections::HashSet;

    /// Every edge must have at least one endpoint in the cover.
    fn covers_all_edges(graph: &Graph<i32, f64>, cover: &HashSet<NodeId>) -> bool {
        graph
            .edges()
            .all(|(u, v, _)| cover.contains(&u) || cover.contains(&v))
    }

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
        let cover = min_weighted_vertex_cover(&graph);
        assert!(!cover.is_empty());
        assert!(cover.len() <= graph.node_count());
        assert!(covers_all_edges(&graph, &cover));
    }

    #[test]
    fn test_vertex_cover_empty_graph() {
        let graph = Graph::<i32, f64>::new();
        let cover = min_weighted_vertex_cover(&graph);
        assert!(cover.is_empty());
    }

    #[test]
    fn test_vertex_cover_star_picks_center() {
        // A star's center covers every edge, so the greedy maximum-degree heuristic
        // selects it alone.
        let mut graph = Graph::<i32, f64>::new();
        let center = graph.add_node(0);
        let leaves: Vec<_> = (1..=5).map(|i| graph.add_node(i)).collect();
        for &leaf in &leaves {
            graph.add_edge(center, leaf, 1.0);
        }
        let cover = min_weighted_vertex_cover(&graph);
        assert_eq!(cover, HashSet::from([center]));
    }

    #[test]
    fn test_vertex_cover_complete_graph() {
        // A complete graph on n nodes needs all but one node in any vertex cover.
        let mut graph = Graph::<i32, f64>::new();
        let nodes: Vec<_> = (0..5).map(|i| graph.add_node(i)).collect();
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                graph.add_edge(nodes[i], nodes[j], 1.0);
            }
        }
        let cover = min_weighted_vertex_cover(&graph);
        assert!(covers_all_edges(&graph, &cover));
        assert_eq!(cover.len(), nodes.len() - 1);
    }

    #[test]
    fn test_vertex_cover_self_loop_forces_endpoint() {
        // Only the self-loop's own endpoint can cover it, so it must be in the cover.
        let mut graph = Graph::<i32, f64>::new();
        let a = graph.add_node(0);
        let b = graph.add_node(1);
        graph.add_edge(a, b, 1.0);
        graph.add_edge(a, a, 1.0);
        let cover = min_weighted_vertex_cover(&graph);
        assert!(cover.contains(&a));
        assert!(covers_all_edges(&graph, &cover));
    }

    #[test]
    fn test_vertex_cover_sparse_indices_after_removal() {
        // A cover over a graph with a removed node: node ids are keyed through hash
        // maps, so a non-contiguous index range must still produce a valid cover.
        let mut graph = Graph::<i32, f64>::new();
        let nodes: Vec<_> = (0..4).map(|i| graph.add_node(i)).collect();
        graph.remove_node(nodes[1]);
        graph.add_edge(nodes[0], nodes[2], 1.0);
        graph.add_edge(nodes[2], nodes[3], 1.0);
        let cover = min_weighted_vertex_cover(&graph);
        assert!(covers_all_edges(&graph, &cover));
        assert!(cover.contains(&nodes[2]));
    }
}
