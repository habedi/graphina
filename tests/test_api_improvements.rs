// Tests for API improvements and consistency fixes

use graphina::core::types::{Digraph, EdgeId, Graph, NodeId};

#[test]
fn test_edge_weight_consistent_naming() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let e = g.add_edge(n1, n2, 3.5);

    // New consistent naming
    assert_eq!(g.edge_weight(e), Some(&3.5));

    // Mutable reference
    if let Some(w) = g.edge_weight_mut(e) {
        *w = 4.5;
    }
    assert_eq!(g.edge_weight(e), Some(&4.5));
}

#[test]
fn test_builder_pattern() {
    let graph = Graph::<i32, f64>::builder()
        .add_node(1)
        .add_node(2)
        .add_node(3)
        .add_edge(0, 1, 1.0)
        .add_edge(1, 2, 2.0)
        .build();

    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 2);
}

#[test]
fn test_is_empty() {
    let mut g = Graph::<i32, f64>::new();
    assert!(g.is_empty());

    let n = g.add_node(1);
    assert!(!g.is_empty());

    g.remove_node(n);
    assert!(g.is_empty());
}

#[test]
fn test_density_undirected() {
    let mut g = Graph::<i32, f64>::new();

    // Empty graph
    assert_eq!(g.density(), 0.0);

    // Single node
    g.add_node(1);
    assert_eq!(g.density(), 0.0);

    // Two nodes, no edges
    let n1 = g.add_node(2);
    let n2 = g.add_node(3);
    assert_eq!(g.density(), 0.0);

    // Two nodes, one edge - density should be 1.0
    g.add_edge(n1, n2, 1.0);
    assert_eq!(g.density(), 1.0);
}

#[test]
fn test_density_directed() {
    let mut g = Digraph::<i32, f64>::new();

    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    // 3 nodes, 0 edges: density = 0 / 6 = 0.0
    assert_eq!(g.density(), 0.0);

    // 3 nodes, 2 edges: density = 2 / 6 = 0.333...
    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);
    assert!((g.density() - 0.3333333).abs() < 0.0001);
}

#[test]
fn test_contains_node() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);

    assert!(g.contains_node(n1));
    assert!(g.contains_node(n2));

    g.remove_node(n1);
    assert!(!g.contains_node(n1));
    assert!(g.contains_node(n2));
}

#[test]
fn test_contains_edge() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    assert!(!g.contains_edge(n1, n2));

    g.add_edge(n1, n2, 1.0);
    assert!(g.contains_edge(n1, n2));
    assert!(g.contains_edge(n2, n1)); // Undirected
    assert!(!g.contains_edge(n1, n3));
}

#[test]
fn test_degree_methods() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    // No edges
    assert_eq!(g.degree(n1), Some(0));
    assert_eq!(g.in_degree(n1), Some(0));
    assert_eq!(g.out_degree(n1), Some(0));

    // Add edges
    g.add_edge(n1, n2, 1.0);
    g.add_edge(n1, n3, 1.0);

    assert_eq!(g.degree(n1), Some(2));
    assert_eq!(g.degree(n2), Some(1));
    assert_eq!(g.degree(n3), Some(1));
}

#[test]
fn test_degree_methods_directed() {
    let mut g = Digraph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n1, n3, 1.0);
    g.add_edge(n2, n1, 1.0);

    // n1: 2 out, 1 in, degree = 3
    assert_eq!(g.out_degree(n1), Some(2));
    assert_eq!(g.in_degree(n1), Some(1));
    assert_eq!(g.degree(n1), Some(3));

    // n2: 1 out, 1 in, degree = 2
    assert_eq!(g.out_degree(n2), Some(1));
    assert_eq!(g.in_degree(n2), Some(1));
    assert_eq!(g.degree(n2), Some(2));
}

#[test]
fn test_clear() {
    let mut g = Graph::<i32, f64>::new();
    g.add_node(1);
    g.add_node(2);

    assert_eq!(g.node_count(), 2);

    g.clear();

    assert_eq!(g.node_count(), 0);
    assert_eq!(g.edge_count(), 0);
    assert!(g.is_empty());
}

#[test]
fn test_node_ids_iterator() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    let ids: Vec<NodeId> = g.node_ids().collect();
    assert_eq!(ids.len(), 3);
    assert!(ids.contains(&n1));
    assert!(ids.contains(&n2));
    assert!(ids.contains(&n3));
}

#[test]
fn test_edge_ids_iterator() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let e1 = g.add_edge(n1, n2, 1.0);

    let ids: Vec<EdgeId> = g.edge_ids().collect();
    assert_eq!(ids.len(), 1);
    assert!(ids.contains(&e1));
}

#[test]
fn test_retain_nodes() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    let n4 = g.add_node(4);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);
    g.add_edge(n3, n4, 1.0);

    // Keep only even-valued nodes
    g.retain_nodes(|_id, attr| *attr % 2 == 0);

    assert_eq!(g.node_count(), 2);
    assert!(g.contains_node(n2));
    assert!(g.contains_node(n4));
    assert!(!g.contains_node(n1));
    assert!(!g.contains_node(n3));

    // Edges involving removed nodes should be gone
    assert_eq!(g.edge_count(), 0);
}

#[test]
fn test_retain_edges() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 2.0);
    g.add_edge(n1, n3, 3.0);

    // Keep only edges with weight > 1.5
    g.retain_edges(|_src, _dst, w| **w > 1.5);

    assert_eq!(g.edge_count(), 2);
    assert!(!g.contains_edge(n1, n2));
    assert!(g.contains_edge(n2, n3));
    assert!(g.contains_edge(n1, n3));
}

#[test]
fn test_with_capacity() {
    let g = Graph::<i32, f64>::with_capacity(100, 200);
    assert!(g.is_empty());
    assert_eq!(g.node_count(), 0);
}

#[test]
fn test_builder_empty_graph() {
    let graph = Graph::<i32, f64>::builder().build();
    assert!(graph.is_empty());
}

#[test]
fn test_builder_directed() {
    let graph = Digraph::<i32, f64>::builder()
        .add_node(1)
        .add_node(2)
        .add_edge(0, 1, 5.0)
        .build();

    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 1);
    assert!(graph.is_directed());
}

#[test]
fn test_degree_nonexistent_node() {
    let g = Graph::<i32, f64>::new();
    let fake_node = NodeId::new(petgraph::graph::NodeIndex::new(999));

    assert_eq!(g.degree(fake_node), None);
    assert_eq!(g.in_degree(fake_node), None);
    assert_eq!(g.out_degree(fake_node), None);
}

#[test]
fn test_contains_edge_directed() {
    let mut g = Digraph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);

    g.add_edge(n1, n2, 1.0);

    assert!(g.contains_edge(n1, n2));
    assert!(!g.contains_edge(n2, n1)); // Directed - reverse edge doesn't exist
}

#[test]
fn test_density_complete_graph() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    // Complete graph: all possible edges
    g.add_edge(n1, n2, 1.0);
    g.add_edge(n1, n3, 1.0);
    g.add_edge(n2, n3, 1.0);

    // For undirected: max_edges = n*(n-1)/2 = 3
    // density = (2*3) / (3*2) = 1.0
    assert_eq!(g.density(), 1.0);
}

#[test]
fn test_retain_nodes_preserves_valid_edges() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 2.0);

    // Remove only n1
    g.retain_nodes(|_id, attr| *attr != 1);

    assert_eq!(g.node_count(), 2);
    assert_eq!(g.edge_count(), 1); // n2-n3 edge should remain
    assert!(g.contains_edge(n2, n3));
}

#[test]
fn test_find_edge() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    let e = g.add_edge(n1, n2, 1.0);

    assert_eq!(g.find_edge(n1, n2), Some(e));
    assert_eq!(g.find_edge(n2, n1), Some(e)); // Undirected
    assert_eq!(g.find_edge(n1, n3), None);
}

#[test]
fn test_builder_multiple_edges_same_nodes() {
    // Test that builder can handle creating edges between same nodes
    let graph = Graph::<i32, f64>::builder()
        .add_node(1)
        .add_node(2)
        .add_edge(0, 1, 1.0)
        .add_edge(0, 1, 2.0) // Second edge between same nodes
        .build();

    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 2); // Both edges should exist
}
