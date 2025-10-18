/*!
# Tests for Batch Operations

This module tests the batch operations feature that allows adding multiple nodes
and edges at once for improved performance.
*/

use graphina::core::types::{Digraph, Graph};

#[test]
fn test_add_nodes_bulk() {
    let mut g = Graph::<i32, f64>::new();

    let nodes = g.add_nodes_bulk(&[1, 2, 3, 4, 5]);

    assert_eq!(nodes.len(), 5);
    assert_eq!(g.node_count(), 5);

    // Verify all nodes exist
    for node_id in &nodes {
        assert!(g.contains_node(*node_id));
    }

    // Verify attributes
    assert_eq!(*g.node_attr(nodes[0]).unwrap(), 1);
    assert_eq!(*g.node_attr(nodes[4]).unwrap(), 5);
}

#[test]
fn test_add_nodes_bulk_empty() {
    let mut g = Graph::<i32, f64>::new();

    let nodes = g.add_nodes_bulk(&[]);

    assert_eq!(nodes.len(), 0);
    assert_eq!(g.node_count(), 0);
    assert!(g.is_empty());
}

#[test]
fn test_add_edges_bulk() {
    let mut g = Graph::<i32, f64>::new();

    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    let n4 = g.add_node(4);

    let edges = g.add_edges_bulk(&[(n1, n2, 1.0), (n2, n3, 2.0), (n3, n4, 3.0), (n4, n1, 4.0)]);

    assert_eq!(edges.len(), 4);
    assert_eq!(g.edge_count(), 4);

    // Verify all edges exist
    for edge_id in &edges {
        assert!(g.edge_weight(*edge_id).is_some());
    }

    // Verify edge weights
    assert_eq!(*g.edge_weight(edges[0]).unwrap(), 1.0);
    assert_eq!(*g.edge_weight(edges[3]).unwrap(), 4.0);
}

#[test]
fn test_add_edges_bulk_empty() {
    let mut g = Graph::<i32, f64>::new();

    let edges = g.add_edges_bulk(&[]);

    assert_eq!(edges.len(), 0);
    assert_eq!(g.edge_count(), 0);
}

#[test]
fn test_extend_nodes() {
    let mut g = Graph::<i32, f64>::new();

    let node_attrs = vec![10, 20, 30, 40, 50];
    let nodes = g.extend_nodes(node_attrs);

    assert_eq!(nodes.len(), 5);
    assert_eq!(g.node_count(), 5);

    // Verify attributes
    assert_eq!(*g.node_attr(nodes[0]).unwrap(), 10);
    assert_eq!(*g.node_attr(nodes[2]).unwrap(), 30);
    assert_eq!(*g.node_attr(nodes[4]).unwrap(), 50);
}

#[test]
fn test_extend_nodes_from_range() {
    let mut g = Graph::<i32, f64>::new();

    let nodes = g.extend_nodes(1..=10);

    assert_eq!(nodes.len(), 10);
    assert_eq!(g.node_count(), 10);

    // Verify attributes
    assert_eq!(*g.node_attr(nodes[0]).unwrap(), 1);
    assert_eq!(*g.node_attr(nodes[9]).unwrap(), 10);
}

#[test]
fn test_extend_edges() {
    let mut g = Graph::<i32, f64>::new();

    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    let n4 = g.add_node(4);

    let edge_data = vec![(n1, n2, 1.5), (n2, n3, 2.5), (n3, n4, 3.5)];

    let edges = g.extend_edges(edge_data);

    assert_eq!(edges.len(), 3);
    assert_eq!(g.edge_count(), 3);

    // Verify edge weights
    assert_eq!(*g.edge_weight(edges[0]).unwrap(), 1.5);
    assert_eq!(*g.edge_weight(edges[1]).unwrap(), 2.5);
    assert_eq!(*g.edge_weight(edges[2]).unwrap(), 3.5);
}

#[test]
fn test_bulk_operations_directed_graph() {
    let mut g = Digraph::<i32, f64>::new();

    // Add nodes in bulk
    let nodes = g.add_nodes_bulk(&[100, 200, 300]);
    assert_eq!(g.node_count(), 3);

    // Add edges in bulk
    let edges = g.add_edges_bulk(&[(nodes[0], nodes[1], 10.0), (nodes[1], nodes[2], 20.0)]);
    assert_eq!(g.edge_count(), 2);

    // Verify directed graph properties
    assert!(g.is_directed());
    assert!(g.contains_edge(nodes[0], nodes[1]));
    assert!(!g.contains_edge(nodes[1], nodes[0]));
}

#[test]
fn test_mixed_bulk_and_single_operations() {
    let mut g = Graph::<i32, f64>::new();

    // Add some nodes individually
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);

    // Add more nodes in bulk
    let bulk_nodes = g.add_nodes_bulk(&[3, 4, 5]);

    assert_eq!(g.node_count(), 5);

    // Add edges using bulk and individual operations
    g.add_edge(n1, n2, 1.0);

    let bulk_edges = g.add_edges_bulk(&[
        (n2, bulk_nodes[0], 2.0),
        (bulk_nodes[0], bulk_nodes[1], 3.0),
    ]);

    assert_eq!(g.edge_count(), 3);

    // Verify all operations worked
    assert!(g.contains_edge(n1, n2));
    assert!(g.contains_edge(n2, bulk_nodes[0]));
    assert_eq!(*g.edge_weight(bulk_edges[1]).unwrap(), 3.0);
}

#[test]
fn test_large_bulk_operation() {
    let mut g = Graph::<i32, f64>::new();

    // Create a large vector of attributes
    let large_attrs: Vec<i32> = (0..1000).collect();
    let nodes = g.add_nodes_bulk(&large_attrs);

    assert_eq!(nodes.len(), 1000);
    assert_eq!(g.node_count(), 1000);

    // Create edges between consecutive nodes
    let mut edge_data = Vec::new();
    for i in 0..999 {
        edge_data.push((nodes[i], nodes[i + 1], i as f64));
    }

    let edges = g.add_edges_bulk(&edge_data);

    assert_eq!(edges.len(), 999);
    assert_eq!(g.edge_count(), 999);
}

#[test]
fn test_extend_with_capacity_hint() {
    let mut g = Graph::<String, f64>::new();

    // Create a large iterator
    let data = (0..500).map(|i| format!("Node{}", i));
    let nodes = g.extend_nodes(data);

    assert_eq!(nodes.len(), 500);
    assert_eq!(g.node_count(), 500);
}

#[test]
fn test_bulk_operations_preserve_attributes() {
    let mut g = Graph::<String, f64>::new();

    let attrs = vec![
        "Alice".to_string(),
        "Bob".to_string(),
        "Charlie".to_string(),
    ];

    let nodes = g.add_nodes_bulk(&attrs);

    // Verify attributes are correctly stored
    assert_eq!(g.node_attr(nodes[0]).unwrap(), "Alice");
    assert_eq!(g.node_attr(nodes[1]).unwrap(), "Bob");
    assert_eq!(g.node_attr(nodes[2]).unwrap(), "Charlie");
}

#[test]
fn test_extend_nodes_empty_iterator() {
    let mut g = Graph::<i32, f64>::new();

    let empty_vec: Vec<i32> = vec![];
    let nodes = g.extend_nodes(empty_vec);

    assert_eq!(nodes.len(), 0);
    assert!(g.is_empty());
}

#[test]
fn test_extend_edges_empty_iterator() {
    let mut g = Graph::<i32, f64>::new();

    let n1 = g.add_node(1);
    let n2 = g.add_node(2);

    let empty_vec: Vec<_> = vec![];
    let edges = g.extend_edges(empty_vec);

    assert_eq!(edges.len(), 0);
    assert_eq!(g.edge_count(), 0);
}
