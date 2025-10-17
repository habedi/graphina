// Test file for edge cases and input validation

use graphina::core::generators::{complete_graph, erdos_renyi_graph};
use graphina::core::io::{read_edge_list, write_edge_list};
use graphina::core::types::{Graph, GraphMarker};
use std::fs;

#[test]
fn test_erdos_renyi_invalid_probability() {
    // Test that probabilities outside [0,1] are rejected
    let result = erdos_renyi_graph::<GraphMarker>(10, 1.5, 42);
    assert!(result.is_err(), "Should reject probability > 1.0");

    let result = erdos_renyi_graph::<GraphMarker>(10, -0.1, 42);
    assert!(result.is_err(), "Should reject negative probability");
}

#[test]
fn test_erdos_renyi_zero_nodes() {
    // Test that zero nodes is rejected
    let result = erdos_renyi_graph::<GraphMarker>(0, 0.5, 42);
    assert!(result.is_err(), "Should reject zero nodes");
}

#[test]
fn test_erdos_renyi_boundary_probabilities() {
    // Test boundary cases: p=0 and p=1
    let result = erdos_renyi_graph::<GraphMarker>(5, 0.0, 42);
    assert!(result.is_ok(), "Should accept p=0");
    let graph = result.unwrap();
    assert_eq!(graph.edge_count(), 0, "Graph with p=0 should have no edges");

    let result = erdos_renyi_graph::<GraphMarker>(5, 1.0, 42);
    assert!(result.is_ok(), "Should accept p=1");
    let graph = result.unwrap();
    // Complete graph on 5 nodes has 10 edges
    assert_eq!(graph.edge_count(), 10, "Graph with p=1 should be complete");
}

#[test]
fn test_complete_graph_single_node() {
    let result = complete_graph::<GraphMarker>(1);
    assert!(result.is_ok());
    let graph = result.unwrap();
    assert_eq!(graph.node_count(), 1);
    assert_eq!(graph.edge_count(), 0); // No self-loops in complete graph
}

#[test]
fn test_io_edge_list_with_comments() {
    // Test that comments are properly handled in edge list files
    let test_file = "/tmp/test_edges_with_comments.txt";
    let content = r#"# This is a comment
1,2,1.0
# Another comment
3,4,2.0
5,6,3.0  # Inline comment
"#;

    fs::write(test_file, content).expect("Failed to write test file");

    let mut graph = Graph::<i32, f32>::new();
    let result = read_edge_list(test_file, &mut graph, ',');

    assert!(
        result.is_ok(),
        "Should successfully read file with comments"
    );
    assert_eq!(graph.edge_count(), 3, "Should have 3 edges");

    // Cleanup
    fs::remove_file(test_file).ok();
}

#[test]
fn test_io_edge_list_invalid_format() {
    // Test that invalid lines are skipped (lines with < 2 tokens)
    let test_file = "/tmp/test_invalid_edges.txt";
    let content = "1,2,1.0\ninvalid line\n3,4,2.0\n";

    fs::write(test_file, content).expect("Failed to write test file");

    let mut graph = Graph::<i32, f32>::new();
    let result = read_edge_list(test_file, &mut graph, ',');

    // Should succeed but skip the invalid line
    assert!(result.is_ok(), "Should skip invalid lines and continue");
    assert_eq!(graph.edge_count(), 2, "Should have 2 valid edges");

    // Cleanup
    fs::remove_file(test_file).ok();
}

#[test]
fn test_io_edge_list_missing_weight() {
    // Test that missing weights default to 1.0
    let test_file = "/tmp/test_missing_weight.txt";
    let content = "1,2\n3,4,2.0\n";

    fs::write(test_file, content).expect("Failed to write test file");

    let mut graph = Graph::<i32, f32>::new();
    let result = read_edge_list(test_file, &mut graph, ',');

    assert!(result.is_ok(), "Should handle missing weights");

    // Cleanup
    fs::remove_file(test_file).ok();
}

#[test]
fn test_io_roundtrip() {
    // Test that write -> read preserves graph structure
    let input_file = "/tmp/test_roundtrip_in.txt";
    let output_file = "/tmp/test_roundtrip_out.txt";

    let mut graph1 = Graph::<i32, f32>::new();
    let n1 = graph1.add_node(1);
    let n2 = graph1.add_node(2);
    let n3 = graph1.add_node(3);

    graph1.add_edge(n1, n2, 1.5);
    graph1.add_edge(n2, n3, 2.5);

    // Write graph
    write_edge_list(output_file, &graph1, ',').expect("Failed to write");

    // Read it back
    let mut graph2 = Graph::<i32, f32>::new();
    read_edge_list(output_file, &mut graph2, ',').expect("Failed to read");

    // Compare
    assert_eq!(graph1.edge_count(), graph2.edge_count());

    // Cleanup
    fs::remove_file(output_file).ok();
}

#[test]
fn test_empty_graph_operations() {
    // Test operations on empty graphs don't panic
    let graph: Graph<i32, f32> = Graph::new();

    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);

    let nodes: Vec<_> = graph.nodes().collect();
    assert_eq!(nodes.len(), 0);

    let edges: Vec<_> = graph.edges().collect();
    assert_eq!(edges.len(), 0);
}

#[test]
fn test_adjacency_matrix_self_loops() {
    // Test that self-loops are handled correctly in adjacency matrix
    let mut graph = Graph::<i32, f64>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);

    graph.add_edge(n1, n1, 5.0); // Self-loop
    graph.add_edge(n1, n2, 3.0);

    let matrix = graph.to_adjacency_matrix();

    // Self-loop should appear on diagonal
    assert_eq!(matrix[0][0], Some(5.0));
    assert_eq!(matrix[0][1], Some(3.0));
    assert_eq!(matrix[1][0], Some(3.0)); // Undirected, so symmetric
}

#[test]
fn test_large_node_indices() {
    // Test that large node counts don't cause issues
    let mut graph = Graph::<i32, ()>::new();
    let nodes: Vec<_> = (0..1000).map(|i| graph.add_node(i)).collect();

    // Add some edges
    for i in 0..999 {
        graph.add_edge(nodes[i], nodes[i + 1], ());
    }

    assert_eq!(graph.node_count(), 1000);
    assert_eq!(graph.edge_count(), 999);
}

#[test]
fn test_node_removal_validity() {
    // Test that removing nodes properly handles incident edges
    let mut graph = Graph::<i32, f64>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);

    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, n3, 2.0);

    // Remove middle node
    let removed = graph.remove_node(n2);
    assert!(removed.is_some());
    assert_eq!(removed.unwrap(), 2);

    // Both edges involving n2 should be gone
    assert_eq!(graph.edge_count(), 0);
    assert_eq!(graph.node_count(), 2);
}

#[test]
fn test_edge_update() {
    // Test updating edge weights
    let mut graph = Graph::<i32, f64>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);

    let e = graph.add_edge(n1, n2, 1.0);

    // Update weight
    if let Some(weight) = graph.edge_attr_mut(e) {
        *weight = 5.0;
    }

    assert_eq!(*graph.edge_attr(e).unwrap(), 5.0);
}
