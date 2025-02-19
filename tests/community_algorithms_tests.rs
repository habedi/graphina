// tests/community_algorithms_tests.rs

use graphina::community::algorithms::*;
use graphina::core::types::Graph;
use std::collections::HashSet;

// Create a sample disconnected graph for label propagation and infomap tests.
// The graph consists of two disconnected triangles.
fn sample_disconnected_graph() -> Graph<&'static str, f64> {
    let mut g = Graph::<&str, f64>::new();
    // Create six nodes labeled A-F.
    let n0 = g.add_node("A");
    let n1 = g.add_node("B");
    let n2 = g.add_node("C");
    let n3 = g.add_node("D");
    let n4 = g.add_node("E");
    let n5 = g.add_node("F");

    // First triangle: A-B-C.
    g.add_edge(n0, n1, 1.0);
    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n0, 1.0);

    // Second triangle: D-E-F.
    g.add_edge(n3, n4, 1.0);
    g.add_edge(n4, n5, 1.0);
    g.add_edge(n5, n3, 1.0);

    g
}

// Create a sample connected graph for other tests.
// This graph consists of two triangles connected by an edge.
fn sample_graph() -> Graph<&'static str, f64> {
    let mut g = Graph::<&str, f64>::new();
    // Create six nodes labeled A-F.
    let n0 = g.add_node("A");
    let n1 = g.add_node("B");
    let n2 = g.add_node("C");
    let n3 = g.add_node("D");
    let n4 = g.add_node("E");
    let n5 = g.add_node("F");

    // First triangle: A-B-C.
    g.add_edge(n0, n1, 1.0);
    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n0, 1.0);

    // Second triangle: D-E-F.
    g.add_edge(n3, n4, 1.0);
    g.add_edge(n4, n5, 1.0);
    g.add_edge(n5, n3, 1.0);

    // Connect the two triangles: edge from B to D.
    g.add_edge(n1, n3, 1.0);

    g
}

#[test]
fn test_label_propagation() {
    let graph = sample_disconnected_graph();
    let labels = label_propagation(&graph, 100, Some(42));
    assert_eq!(labels.len(), graph.node_count());
    // Check that at least two different community labels exist.
    let unique: HashSet<_> = labels.iter().collect();
    assert!(
        unique.len() >= 2,
        "Expected at least 2 communities, got {:?}",
        unique
    );
}

#[test]
fn test_louvain() {
    let graph = sample_graph();
    let communities = louvain(&graph, Some(42));
    // Ensure that all nodes are assigned to some community.
    let total: usize = communities.iter().map(|c| c.len()).sum();
    assert_eq!(total, graph.node_count());
}

#[test]
fn test_girvan_newman() {
    let graph = sample_graph();
    // Force the algorithm to stop when the graph splits into 2 components.
    let communities = girvan_newman(&graph, 2);
    let total: usize = communities.iter().map(|c| c.len()).sum();
    assert_eq!(total, graph.node_count());
    assert_eq!(communities.len(), 2);
}

#[test]
fn test_spectral_clustering() {
    let graph = sample_graph();
    let communities = spectral_clustering(&graph, 2, Some(42));
    let total: usize = communities.iter().map(|c| c.len()).sum();
    assert_eq!(total, graph.node_count());
    assert_eq!(communities.len(), 2);
}

#[test]
fn test_personalized_page_rank() {
    let graph = sample_graph();
    let pr = personalized_page_rank(&graph, None, 0.85, 1e-6, 100);
    assert_eq!(pr.len(), graph.node_count());
    // Check that the PageRank vector sums to approximately 1.
    let sum: f64 = pr.iter().sum();
    assert!((sum - 1.0).abs() < 1e-3);
}

#[test]
fn test_infomap() {
    let graph = sample_disconnected_graph();
    let modules = infomap(&graph, 100, Some(42));
    assert_eq!(modules.len(), graph.node_count());
    // Ensure at least two different module assignments.
    let unique: HashSet<_> = modules.iter().collect();
    assert!(unique.len() >= 2, "Infomap modules: {:?}", unique);
}

#[test]
fn test_connected_components() {
    let graph = sample_graph();
    let comps = connected_components(&graph);
    let total: usize = comps.iter().map(|c| c.len()).sum();
    assert_eq!(total, graph.node_count());
    // For our sample graph (which is connected), we expect one component.
    assert_eq!(comps.len(), 1);
}
