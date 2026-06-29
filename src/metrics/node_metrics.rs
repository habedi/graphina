/*!
# Node-level Metrics

Node-level metrics for network analysis.
*/

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use petgraph::EdgeType;
use std::collections::{HashMap, HashSet};

/// Builds, for each distinct neighbor of `node`, the set of its adjacent nodes.
/// This turns the inner adjacency test in the triangle loops into an O(1) hash
/// lookup instead of an O(degree) `contains_edge` call. Membership in
/// `adj[&a]` matches `graph.contains_edge(a, b)`: for undirected graphs the
/// neighbor set is symmetric, and for directed graphs it holds the outgoing
/// neighbors, exactly what `contains_edge(a, b)` tests.
fn neighbor_adjacency<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
    neighbors: &[NodeId],
) -> HashMap<NodeId, HashSet<NodeId>> {
    let mut adj: HashMap<NodeId, HashSet<NodeId>> = HashMap::new();
    for &nb in neighbors {
        adj.entry(nb)
            .or_insert_with(|| graph.neighbors(nb).collect());
    }
    adj
}

/// Computes the local clustering coefficient for a specific node.
///
/// Measures the probability that two neighbors of a node are also connected.
///
/// # Time Complexity
/// O(d²) where d is the node's degree
pub fn clustering_coefficient<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
    node: NodeId,
) -> f64 {
    let neighbors: Vec<NodeId> = graph.neighbors(node).collect();
    let k = neighbors.len();

    if k < 2 {
        return 0.0;
    }

    let adj = neighbor_adjacency(graph, &neighbors);
    let mut triangles = 0;
    for i in 0..k {
        let si = &adj[&neighbors[i]];
        for other in &neighbors[i + 1..] {
            if si.contains(other) {
                triangles += 1;
            }
        }
    }

    let possible_edges = k * (k - 1) / 2;
    triangles as f64 / possible_edges as f64
}

/// Counts the number of triangles containing a specific node.
///
/// # Time Complexity
/// O(d²) where d is the node's degree
pub fn triangles<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
    node: NodeId,
) -> usize {
    let neighbors: Vec<NodeId> = graph.neighbors(node).collect();
    let k = neighbors.len();
    if k < 2 {
        return 0;
    }

    let adj = neighbor_adjacency(graph, &neighbors);
    let mut count = 0;
    for i in 0..k {
        let si = &adj[&neighbors[i]];
        for other in &neighbors[i + 1..] {
            if si.contains(other) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_clustering_coefficient_triangle() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n1, 1.0);

        // All nodes in triangle have clustering coefficient 1.0
        assert!((clustering_coefficient(&g, n1) - 1.0).abs() < 0.001);
        assert!((clustering_coefficient(&g, n2) - 1.0).abs() < 0.001);
        assert!((clustering_coefficient(&g, n3) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_triangles() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n1, 1.0);
        g.add_edge(n1, n4, 1.0);

        assert_eq!(triangles(&g, n1), 1);
        assert_eq!(triangles(&g, n2), 1);
        assert_eq!(triangles(&g, n3), 1);
        assert_eq!(triangles(&g, n4), 0);
    }
}
