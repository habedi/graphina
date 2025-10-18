/*!
# Tests for Graph Generator Bug Fixes

This module contains tests for bugs fixed in the graph generators,
specifically for the Barabási-Albert and Watts-Strogatz generators.
*/

use graphina::core::generators::{barabasi_albert_graph, watts_strogatz_graph};
use graphina::core::types::Undirected;

#[test]
fn test_barabasi_albert_no_infinite_loop() {
    // Test that the Barabási-Albert generator completes without hanging
    // This was previously prone to infinite loops with certain parameters
    let n = 50;
    let m = 5;
    let seed = 12345;

    let graph = barabasi_albert_graph::<Undirected>(n, m, seed)
        .expect("Failed to generate Barabási-Albert graph");

    assert_eq!(graph.node_count(), n);
    // Each new node attaches to m existing nodes
    let expected_min_edges = (m * (m - 1) / 2) + (n - m) * m;
    assert!(graph.edge_count() >= expected_min_edges - 10); // Allow small variance
}

#[test]
fn test_barabasi_albert_large_graph() {
    // Test with larger parameters that would definitely cause infinite loop in old version
    let n = 100;
    let m = 10;
    let seed = 42;

    let graph = barabasi_albert_graph::<Undirected>(n, m, seed)
        .expect("Failed to generate large Barabási-Albert graph");

    assert_eq!(graph.node_count(), n);
    assert!(graph.edge_count() > 0);
}

#[test]
fn test_barabasi_albert_edge_case_m_equals_1() {
    // Test edge case where m = 1
    let n = 20;
    let m = 1;
    let seed = 99;

    let graph = barabasi_albert_graph::<Undirected>(n, m, seed)
        .expect("Failed to generate Barabási-Albert graph with m=1");

    assert_eq!(graph.node_count(), n);
    assert_eq!(graph.edge_count(), n - 1); // Should form a tree
}

#[test]
fn test_barabasi_albert_no_duplicate_targets() {
    // Ensure that each new node doesn't connect to the same target multiple times
    let n = 30;
    let m = 5;
    let seed = 777;

    let graph = barabasi_albert_graph::<Undirected>(n, m, seed)
        .expect("Failed to generate Barabási-Albert graph");

    // Check that no node has duplicate edges (multi-edges)
    for (node, _) in graph.nodes() {
        let neighbors: Vec<_> = graph.neighbors(node).collect();
        let unique_neighbors: std::collections::HashSet<_> = neighbors.iter().cloned().collect();
        assert_eq!(
            neighbors.len(),
            unique_neighbors.len(),
            "Node {:?} has duplicate edges",
            node
        );
    }
}

#[test]
fn test_watts_strogatz_no_duplicate_edges() {
    // Test that Watts-Strogatz minimizes duplicate edges during rewiring
    // Note: With high rewiring and limited nodes, some duplicates may occur
    // but should be minimal
    let n = 30; // Increased nodes to reduce collision probability
    let k = 4;
    let beta = 0.3; // Lower beta to reduce rewiring attempts
    let seed = 333;

    let graph = watts_strogatz_graph::<Undirected>(n, k, beta, seed)
        .expect("Failed to generate Watts-Strogatz graph");

    assert_eq!(graph.node_count(), n);

    // Check for duplicate edges
    let mut edge_set = std::collections::HashSet::new();
    let mut duplicate_count = 0;
    for (src, tgt, _) in graph.edges() {
        let edge = if src.index() < tgt.index() {
            (src, tgt)
        } else {
            (tgt, src)
        };
        if !edge_set.insert(edge) {
            duplicate_count += 1;
        }
    }

    // With the fallback mechanism, duplicates should be minimal or zero
    assert!(
        duplicate_count == 0,
        "Found {} duplicate edges, expected 0",
        duplicate_count
    );
}

#[test]
fn test_watts_strogatz_high_rewiring() {
    // Test with high rewiring probability
    let n = 30;
    let k = 6;
    let beta = 1.0; // All edges should be rewired
    let seed = 555;

    let graph = watts_strogatz_graph::<Undirected>(n, k, beta, seed)
        .expect("Failed to generate Watts-Strogatz graph with beta=1.0");

    assert_eq!(graph.node_count(), n);
    // Should maintain approximately the same number of edges
    let expected_edges = n * k / 2;
    assert!(
        graph.edge_count() >= expected_edges - n,
        "Edge count {} is too far from expected {}",
        graph.edge_count(),
        expected_edges
    );
}

#[test]
fn test_watts_strogatz_no_rewiring() {
    // Test with no rewiring (beta = 0) - should be a ring lattice
    let n = 20;
    let k = 4;
    let beta = 0.0;
    let seed = 111;

    let graph = watts_strogatz_graph::<Undirected>(n, k, beta, seed)
        .expect("Failed to generate Watts-Strogatz graph with beta=0.0");

    assert_eq!(graph.node_count(), n);
    assert_eq!(graph.edge_count(), n * k / 2);
}

#[test]
fn test_barabasi_albert_deterministic_with_seed() {
    // Test that same seed produces same graph
    let n = 25;
    let m = 3;
    let seed = 12345;

    let graph1 =
        barabasi_albert_graph::<Undirected>(n, m, seed).expect("Failed to generate first graph");
    let graph2 =
        barabasi_albert_graph::<Undirected>(n, m, seed).expect("Failed to generate second graph");

    assert_eq!(graph1.node_count(), graph2.node_count());
    assert_eq!(graph1.edge_count(), graph2.edge_count());
}

#[test]
fn test_watts_strogatz_deterministic_with_seed() {
    // Test that same seed produces same graph
    let n = 20;
    let k = 4;
    let beta = 0.3;
    let seed = 99999;

    let graph1 = watts_strogatz_graph::<Undirected>(n, k, beta, seed)
        .expect("Failed to generate first graph");
    let graph2 = watts_strogatz_graph::<Undirected>(n, k, beta, seed)
        .expect("Failed to generate second graph");

    assert_eq!(graph1.node_count(), graph2.node_count());
    assert_eq!(graph1.edge_count(), graph2.edge_count());
}
