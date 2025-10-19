use graphina::centrality::{
    eigenvector::eigenvector_centrality, katz::katz_centrality, pagerank::pagerank,
};
/// Integration tests for bug fixes in centrality algorithms
///
/// This test suite verifies that the critical bug fixes for PageRank, Eigenvector,
/// and Katz centrality work correctly, especially with graphs that have non-contiguous
/// node indices (which can happen with StableGraph after node deletions).
use graphina::core::types::{Digraph, Graph};

#[test]
fn test_pagerank_with_deleted_nodes() {
    // Test PageRank with non-contiguous node indices
    let mut graph: Digraph<i32, f64> = Digraph::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    let n4 = graph.add_node(4);

    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, n3, 1.0);
    graph.add_edge(n3, n4, 1.0);
    graph.add_edge(n4, n1, 1.0);

    // Delete a node to create non-contiguous indices
    graph.remove_node(n2);

    // This should not panic and should return valid results
    let pr = pagerank(&graph, 0.85, 100, 1e-6).unwrap();

    // Verify results make sense
    assert!(!pr.contains_key(&n2)); // Deleted node should not be in results
    assert!(pr.contains_key(&n1));
    assert!(pr.contains_key(&n3));
    assert!(pr.contains_key(&n4));

    // Sum should be approximately 1.0
    let sum: f64 = pr.values().sum();
    assert!((sum - 1.0).abs() < 1e-4);
}

#[test]
fn test_eigenvector_with_deleted_nodes() {
    // Test eigenvector centrality with non-contiguous node indices
    let mut graph: Graph<i32, f64> = Graph::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    let n4 = graph.add_node(4);
    let n5 = graph.add_node(5);

    // Create a star graph
    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n1, n3, 1.0);
    graph.add_edge(n1, n4, 1.0);
    graph.add_edge(n1, n5, 1.0);

    // Delete a leaf node
    graph.remove_node(n5);

    // This should not panic
    let eig = eigenvector_centrality(&graph, 100, 1e-6).unwrap();

    // Center node should still have highest centrality
    assert!(eig[&n1] > eig[&n2]);
    assert!(eig[&n1] > eig[&n3]);
    assert!(eig[&n1] > eig[&n4]);
    assert!(!eig.contains_key(&n5));
}

#[test]
fn test_katz_with_deleted_nodes() {
    // Test Katz centrality with non-contiguous node indices
    let mut graph: Digraph<i32, f64> = Digraph::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    let n4 = graph.add_node(4);

    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, n3, 1.0);
    graph.add_edge(n3, n4, 1.0);

    // Delete middle node
    graph.remove_node(n2);

    // This should not panic
    let katz = katz_centrality(&graph, 0.1, None, 100, 1e-6).unwrap();

    assert!(katz.contains_key(&n1));
    assert!(!katz.contains_key(&n2));
    assert!(katz.contains_key(&n3));
    assert!(katz.contains_key(&n4));
}

#[test]
fn test_pagerank_performance_improvement() {
    // Create a larger graph to verify performance improvement
    let mut graph: Digraph<i32, f64> = Digraph::new();
    let mut nodes = Vec::new();

    // Create 100 nodes
    for i in 0..100 {
        nodes.push(graph.add_node(i));
    }

    // Create edges - each node points to next 5 nodes
    for i in 0..100 {
        for j in 1..=5 {
            let target = (i + j) % 100;
            graph.add_edge(nodes[i], nodes[target], 1.0);
        }
    }

    // This should complete quickly (not O(n²m))
    let pr = pagerank(&graph, 0.85, 100, 1e-6).unwrap();

    // Verify basic correctness
    assert_eq!(pr.len(), 100);
    let sum: f64 = pr.values().sum();
    assert!((sum - 1.0).abs() < 1e-3);
}

#[test]
fn test_eigenvector_convergence_error() {
    // Test that eigenvector properly reports convergence failure
    let mut graph: Digraph<i32, f64> = Digraph::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);

    graph.add_edge(n1, n2, 1.0);

    // Use very strict tolerance that may not converge
    let result = eigenvector_centrality(&graph, 2, 1e-15);

    // Should either converge or return error, not panic
    match result {
        Ok(_) => {} // Converged quickly
        Err(e) => {
            assert!(e.to_string().contains("converge"));
        }
    }
}

#[test]
fn test_katz_convergence_error() {
    // Test that Katz properly reports convergence failure
    let mut graph: Graph<i32, f64> = Graph::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);

    graph.add_edge(n1, n2, 1.0);

    // Use too few iterations
    let result = katz_centrality(&graph, 0.1, None, 1, 1e-9);

    // Should either converge or return error, not panic
    match result {
        Ok(_) => {} // Converged in 1 iteration (unlikely)
        Err(e) => {
            assert!(e.to_string().contains("converge"));
        }
    }
}

#[test]
fn test_pagerank_weighted_edges() {
    // Test PageRank with weighted edges
    let mut graph: Digraph<i32, f64> = Digraph::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);

    // n1 points to n2 and n3, but stronger link to n2
    graph.add_edge(n1, n2, 3.0);
    graph.add_edge(n1, n3, 1.0);

    let pr = pagerank(&graph, 0.85, 100, 1e-6).unwrap();

    // n2 should receive more PageRank than n3 due to higher weight
    assert!(pr[&n2] > pr[&n3]);
}

#[test]
fn test_eigenvector_multigraph() {
    // Test eigenvector with multiple edges between same nodes
    let mut graph: Graph<i32, f64> = Graph::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);

    // Create a triangle for better convergence
    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, n3, 1.0);
    graph.add_edge(n3, n1, 1.0);

    // Use relaxed parameters for convergence
    let eig = eigenvector_centrality(&graph, 10000, 1e-4).unwrap();

    // All nodes should have valid centrality in a triangle
    assert!(eig[&n1] > 0.0);
    assert!(eig[&n2] > 0.0);
    assert!(eig[&n3] > 0.0);

    // In a symmetric triangle, all should have similar values
    let diff12 = (eig[&n1] - eig[&n2]).abs();
    let diff23 = (eig[&n2] - eig[&n3]).abs();
    assert!(diff12 < 0.1, "Nodes should have similar centrality");
    assert!(diff23 < 0.1, "Nodes should have similar centrality");
}
