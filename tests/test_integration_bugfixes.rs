/*!
# Integration Tests: Bug Fixes

This test suite contains comprehensive integration tests for all bug fixes across the project.
These tests verify that bugs found in various modules (core, centrality, approximation, etc.)
have been properly fixed and continue to work correctly across module boundaries.

## Coverage:
- Core graph operations with non-contiguous node indices
- Centrality algorithms with deleted nodes
- Approximation algorithms edge cases
- Cross-module bug fixes from 2025 analysis
- Visualization layout safety
- Traversal algorithm robustness
*/

use graphina::core::types::{Digraph, Graph};

#[cfg(feature = "subgraphs")]
use graphina::subgraphs::SubgraphOps;

// ============================================================================
// Core Module Bug Fixes
// ============================================================================

#[test]
#[cfg(feature = "community")]
fn test_louvain_with_removed_nodes() {
    use graphina::community::louvain::louvain;

    let mut g = Graph::<i32, f64>::new();
    let n0 = g.add_node(0);
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    let n4 = g.add_node(4);

    g.add_edge(n0, n1, 1.0);
    g.add_edge(n1, n2, 1.0);
    g.add_edge(n3, n4, 1.0);

    g.remove_node(n2);

    let communities = louvain(&g, Some(42)).unwrap();
    assert!(!communities.is_empty());
}

#[test]
fn test_undirected_degree_consistency() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);

    g.add_edge(n1, n2, 5.0);

    assert_eq!(g.degree(n1), Some(1));
    assert_eq!(g.degree(n2), Some(1));
    assert_eq!(g.edge_count(), 1);
}

#[test]
#[cfg(feature = "centrality")]
fn test_centrality_empty_graph() {
    use graphina::centrality::degree::degree_centrality;

    let g = Graph::<i32, f64>::new();
    let result = degree_centrality(&g);

    assert!(result.is_ok());
    let centrality = result.unwrap();
    assert_eq!(centrality.len(), 0);
}

#[test]
fn test_metrics_single_node() {
    use graphina::metrics::{average_clustering_coefficient, diameter};

    let mut g = Graph::<i32, f64>::new();
    g.add_node(1);

    assert_eq!(diameter(&g), Some(0));
    assert_eq!(average_clustering_coefficient(&g), 0.0);
}

#[test]
fn test_dijkstra_negative_weights() {
    use graphina::core::paths::dijkstra_path_f64;

    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);

    g.add_edge(n1, n2, -5.0);

    let result = dijkstra_path_f64(&g, n1, None);
    assert!(result.is_err());
}

#[test]
fn test_self_loop_handling() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);

    g.add_edge(n1, n1, 1.0);

    assert!(g.degree(n1).unwrap() > 0);
    assert!(g.contains_edge(n1, n1));
}

#[test]
fn test_directed_edge_finding() {
    let mut g = Digraph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);

    g.add_edge(n1, n2, 1.0);

    assert!(g.contains_edge(n1, n2));
    assert!(!g.contains_edge(n2, n1));
}

#[test]
fn test_iterator_safety() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);

    let nodes: Vec<_> = g.nodes().map(|(id, _)| id).collect();

    for node in nodes {
        g.remove_node(node);
    }

    assert_eq!(g.node_count(), 0);
}

#[test]
fn test_parallel_vs_sequential_consistency() {
    use graphina::parallel::degrees_parallel;

    let mut g = Graph::<i32, f64>::new();
    let nodes: Vec<_> = (0..100).map(|i| g.add_node(i)).collect();

    for i in 0..100 {
        for j in (i + 1)..100 {
            if (i * j) % 7 == 0 {
                g.add_edge(nodes[i], nodes[j], 1.0);
            }
        }
    }

    let parallel_degrees = degrees_parallel(&g);

    let sequential_degrees: std::collections::HashMap<_, _> = g
        .nodes()
        .map(|(id, _)| (id, g.degree(id).unwrap()))
        .collect();

    for (node, deg) in &sequential_degrees {
        assert_eq!(parallel_degrees.get(node), Some(deg));
    }
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_graph_builder_invalid_edge() {
    let _g = Graph::<i32, f64>::builder()
        .add_node(1)
        .add_node(2)
        .add_edge(0, 5, 1.0)
        .build();
}

#[test]
fn test_nodemap_with_deleted_nodes() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    let map = g.to_nodemap(|_, val| *val * 2);

    g.remove_node(n2);

    assert_eq!(map.get(&n1), Some(&2));
    assert_eq!(map.get(&n3), Some(&6));
}

#[test]
fn test_dag_validation() {
    use graphina::core::validation::is_dag;

    let mut g = Digraph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);

    assert!(is_dag(&g));

    g.add_edge(n3, n1, 1.0);

    assert!(!is_dag(&g));
}

#[test]
fn test_subgraph_attribute_preservation() {
    let mut g = Graph::<String, f64>::new();
    let n1 = g.add_node("Alice".to_string());
    let n2 = g.add_node("Bob".to_string());
    let n3 = g.add_node("Charlie".to_string());

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);

    let subgraph = g
        .subgraph(&[n1, n2])
        .expect("Subgraph creation should succeed");

    assert_eq!(subgraph.node_count(), 2);

    for (_, attr) in subgraph.nodes() {
        assert!(attr == "Alice" || attr == "Bob");
    }
}

#[test]
fn test_serialization_special_values() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);

    g.add_edge(n1, n2, 0.0);

    let json = g.to_serializable();
    let json_str = serde_json::to_string(&json).unwrap();

    assert!(!json_str.contains("NaN"));
    assert!(!json_str.contains("Infinity"));
}

// ============================================================================
// Centrality Module Bug Fixes
// ============================================================================

#[test]
#[cfg(feature = "centrality")]
fn test_pagerank_with_deleted_nodes() {
    use graphina::centrality::pagerank::pagerank;

    let mut graph: Digraph<i32, f64> = Digraph::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    let n4 = graph.add_node(4);

    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, n3, 1.0);
    graph.add_edge(n3, n4, 1.0);
    graph.add_edge(n4, n1, 1.0);

    graph.remove_node(n2);

    let pr = pagerank(&graph, 0.85, 100, 1e-6).unwrap();

    assert!(!pr.contains_key(&n2));
    assert!(pr.contains_key(&n1));
    assert!(pr.contains_key(&n3));
    assert!(pr.contains_key(&n4));

    let sum: f64 = pr.values().sum();
    assert!((sum - 1.0).abs() < 1e-4);
}

#[test]
#[cfg(feature = "centrality")]
fn test_eigenvector_with_deleted_nodes() {
    use graphina::centrality::eigenvector::eigenvector_centrality;

    let mut graph: Graph<i32, f64> = Graph::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    let n4 = graph.add_node(4);
    let n5 = graph.add_node(5);

    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n1, n3, 1.0);
    graph.add_edge(n1, n4, 1.0);
    graph.add_edge(n1, n5, 1.0);

    graph.remove_node(n5);

    let eig = eigenvector_centrality(&graph, 100, 1e-6).unwrap();

    assert!(eig[&n1] > eig[&n2]);
    assert!(eig[&n1] > eig[&n3]);
    assert!(eig[&n1] > eig[&n4]);
    assert!(!eig.contains_key(&n5));
}

#[test]
#[cfg(feature = "centrality")]
fn test_katz_with_deleted_nodes() {
    use graphina::centrality::katz::katz_centrality;

    let mut graph: Digraph<i32, f64> = Digraph::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    let n4 = graph.add_node(4);

    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, n3, 1.0);
    graph.add_edge(n3, n4, 1.0);

    graph.remove_node(n2);

    let katz = katz_centrality(&graph, 0.1, None, 100, 1e-6).unwrap();

    assert!(katz.contains_key(&n1));
    assert!(!katz.contains_key(&n2));
    assert!(katz.contains_key(&n3));
    assert!(katz.contains_key(&n4));
}

#[test]
#[cfg(feature = "centrality")]
fn test_betweenness_centrality_two_nodes_division_by_zero_fix() {
    use graphina::centrality::betweenness::betweenness_centrality;
    use ordered_float::OrderedFloat;

    let mut graph = Graph::<i32, OrderedFloat<f64>>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);

    graph.add_edge(n1, n2, OrderedFloat(1.0));

    let result = betweenness_centrality(&graph, true);
    assert!(result.is_ok());

    let centrality = result.unwrap();
    assert_eq!(centrality.len(), 2);
    assert_eq!(*centrality.get(&n1).unwrap(), 0.0);
    assert_eq!(*centrality.get(&n2).unwrap(), 0.0);
}

// ============================================================================
// Approximation Module Bug Fixes
// ============================================================================

#[test]
#[cfg(feature = "approximation")]
fn test_max_clique_empty_graph() {
    use graphina::approximation::clique::max_clique;

    let graph: Graph<i32, f64> = Graph::new();
    let clique = max_clique(&graph);
    assert!(clique.is_empty());
}

#[test]
#[cfg(feature = "approximation")]
fn test_max_clique_with_deleted_nodes() {
    use graphina::approximation::clique::max_clique;

    let mut graph = Graph::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let _n3 = graph.add_node(3);
    let _n4 = graph.add_node(4);

    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, _n3, 1.0);
    graph.add_edge(_n3, _n4, 1.0);

    graph.remove_node(n2);

    let clique = max_clique(&graph);
    assert!(!clique.contains(&n2));
}

#[test]
#[cfg(feature = "approximation")]
fn test_treewidth_with_deleted_nodes() {
    use graphina::approximation::treewidth::{treewidth_min_degree, treewidth_min_fill_in};

    let mut graph = Graph::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    let n4 = graph.add_node(4);

    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, n3, 1.0);
    graph.add_edge(n3, n4, 1.0);

    graph.remove_node(n2);

    let (_tw1, order1) = treewidth_min_degree(&graph);
    let (_tw2, order2) = treewidth_min_fill_in(&graph);

    assert!(!order1.contains(&n2));
    assert!(!order2.contains(&n2));
    assert_eq!(order1.len(), 3);
    assert_eq!(order2.len(), 3);
}

// ============================================================================
// 2025 Bug Fixes - Visualization & Traversal
// ============================================================================

#[test]
#[cfg(feature = "visualization")]
fn test_force_directed_layout_sparse_graph() {
    use graphina::visualization::{LayoutAlgorithm, LayoutEngine};

    let mut graph = Graph::<i32, f64>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let _n3 = graph.add_node(3);
    let _n4 = graph.add_node(4);

    graph.add_edge(n1, n2, 1.0);

    let positions =
        LayoutEngine::compute_layout(&graph, LayoutAlgorithm::ForceDirected, 800.0, 600.0);
    assert_eq!(positions.len(), 4);
}

#[test]
#[cfg(feature = "traversal")]
fn test_bidirectional_search_disconnected() {
    use graphina::traversal::bidis;

    let mut graph = Graph::<i32, ()>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    let n4 = graph.add_node(4);

    graph.add_edge(n1, n2, ());
    graph.add_edge(n3, n4, ());

    let path = bidis(&graph, n1, n4);
    assert!(path.is_none());
}

// ============================================================================
// Performance Regression Tests
// ============================================================================

#[test]
#[cfg(feature = "centrality")]
fn test_pagerank_performance_improvement() {
    use graphina::centrality::pagerank::pagerank;

    let mut graph: Digraph<i32, f64> = Digraph::new();
    let mut nodes = Vec::new();

    for i in 0..100 {
        nodes.push(graph.add_node(i));
    }

    for i in 0..100 {
        for j in 1..=5 {
            let target = (i + j) % 100;
            graph.add_edge(nodes[i], nodes[target], 1.0);
        }
    }

    let pr = pagerank(&graph, 0.85, 100, 1e-6).unwrap();

    assert_eq!(pr.len(), 100);
    let sum: f64 = pr.values().sum();
    assert!((sum - 1.0).abs() < 1e-3);
}

#[test]
#[cfg(feature = "community")]
fn test_connected_components_performance_fix() {
    use graphina::community::connected_components::connected_components;

    let mut g = Graph::<i32, f64>::new();

    let mut nodes = Vec::new();
    for i in 0..1000 {
        nodes.push(g.add_node(i));
    }

    for i in (0..1000).step_by(10) {
        for j in 0..9 {
            if i + j + 1 < 1000 {
                g.add_edge(nodes[i + j], nodes[i + j + 1], 1.0);
            }
        }
    }

    let components = connected_components(&g);
    assert!(components.len() >= 100);
}
