/*!
# Integration Tests: Architecture & Refactoring

This test suite validates architectural improvements and refactoring efforts across the project.
These tests ensure that:
- Module independence is maintained
- Validation utilities work correctly
- Builder patterns function as expected
- Performance improvements are effective
- Extensions (parallel, visualization) work as independent modules

## Coverage:
- Architecture fixes from 2025
- Module refactoring validation
- Builder patterns and topology generators
- Performance regression prevention
- Cross-module integration
*/

use graphina::core::types::{Digraph, Graph, NodeId};
use graphina::core::validation::{
    count_components, has_negative_weights, is_bipartite, is_connected, is_dag, is_empty,
    validate_is_dag, validate_node_exists, validate_non_empty, validate_non_negative_weights,
};

// ============================================================================
// Architecture Fixes - Validation Utilities
// ============================================================================

#[test]
fn test_validation_utilities_empty_graph() {
    let g = Graph::<i32, f64>::new();

    assert!(is_empty(&g));
    assert!(!is_connected(&g)); // Empty graph is not connected
    assert!(is_dag(&g));
    assert!(!has_negative_weights(&g));
    assert_eq!(count_components(&g), 0);
}

#[test]
fn test_validation_utilities_single_node() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);

    assert!(!is_empty(&g));
    assert!(is_connected(&g));
    assert!(is_dag(&g));
    assert_eq!(count_components(&g), 1);

    assert!(validate_node_exists(&g, n1).is_ok());
    assert!(validate_non_empty(&g).is_ok());
}

#[test]
fn test_validation_negative_weights() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);

    g.add_edge(n1, n2, 5.0);
    assert!(!has_negative_weights(&g));
    assert!(validate_non_negative_weights(&g).is_ok());

    g.add_edge(n2, n1, -1.0);
    assert!(has_negative_weights(&g));
    assert!(validate_non_negative_weights(&g).is_err());
}

#[test]
fn test_validation_dag() {
    let mut g = Digraph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);

    assert!(is_dag(&g));
    assert!(validate_is_dag(&g).is_ok());

    g.add_edge(n3, n1, 1.0); // Create cycle

    assert!(!is_dag(&g));
    assert!(validate_is_dag(&g).is_err());
}

#[test]
fn test_bipartite_detection() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    let n4 = g.add_node(4);

    // Create bipartite graph: 1,2 on one side, 3,4 on other
    g.add_edge(n1, n3, 1.0);
    g.add_edge(n1, n4, 1.0);
    g.add_edge(n2, n3, 1.0);
    g.add_edge(n2, n4, 1.0);

    assert!(is_bipartite(&g));

    // Add edge within same side - no longer bipartite
    g.add_edge(n1, n2, 1.0);
    assert!(!is_bipartite(&g));
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

    // Create components of size 10
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

#[test]
fn test_connected_components_non_contiguous_indices() {
    use graphina::community::connected_components::connected_components;

    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    let n4 = g.add_node(4);
    let _n5 = g.add_node(5);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n3, n4, 1.0);

    // Remove node to create non-contiguous indices
    g.remove_node(n3);

    let components = connected_components(&g);

    // Should have 3 components: {n1,n2}, {n4}, {n5}
    assert_eq!(components.len(), 3);
}

#[test]
fn test_component_count_after_operations() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    assert_eq!(count_components(&g), 3);

    g.add_edge(n1, n2, 1.0);
    assert_eq!(count_components(&g), 2);

    g.add_edge(n2, n3, 1.0);
    assert_eq!(count_components(&g), 1);

    g.remove_node(n2);
    assert_eq!(count_components(&g), 2);
}

// ============================================================================
// Graph Operations & Consistency
// ============================================================================

#[test]
fn test_graph_density_calculation() {
    let mut g = Graph::<i32, f64>::new();

    assert_eq!(g.density(), 0.0); // Empty graph

    let n1 = g.add_node(1);
    assert_eq!(g.density(), 0.0); // Single node

    let n2 = g.add_node(2);
    assert_eq!(g.density(), 0.0); // No edges

    g.add_edge(n1, n2, 1.0);
    assert_eq!(g.density(), 1.0); // Complete graph with 2 nodes

    let _n3 = g.add_node(3);
    // 1 edge out of 3 possible (undirected)
    assert!((g.density() - 1.0 / 3.0).abs() < 1e-10);
}

#[test]
fn test_directed_graph_density() {
    let mut g = Digraph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);
    g.add_edge(n3, n1, 1.0);

    // 3 edges out of 6 possible (directed: n*(n-1))
    assert!((g.density() - 0.5).abs() < 1e-10);
}

#[test]
fn test_degree_calculations_consistency() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n1, n3, 1.0);
    g.add_edge(n2, n3, 1.0);

    assert_eq!(g.degree(n1), Some(2));
    assert_eq!(g.degree(n2), Some(2));
    assert_eq!(g.degree(n3), Some(2));

    // Total degree should be 2 * edge_count
    let total_degree: usize = g.nodes().map(|(id, _)| g.degree(id).unwrap()).sum();
    assert_eq!(total_degree, 2 * g.edge_count());
}

#[test]
fn test_directed_degree_calculations() {
    let mut g = Digraph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n1, n3, 1.0);
    g.add_edge(n2, n3, 1.0);

    // For directed graphs, degree = in_degree + out_degree
    assert_eq!(g.degree(n1), Some(2)); // out: 2, in: 0
    assert_eq!(g.degree(n2), Some(2)); // out: 1, in: 1
    assert_eq!(g.degree(n3), Some(2)); // out: 0, in: 2
}

#[test]
fn test_node_removal_edge_cleanup() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);
    g.add_edge(n1, n3, 1.0);

    assert_eq!(g.edge_count(), 3);

    g.remove_node(n2);

    // Should have removed 2 edges connected to n2
    assert_eq!(g.edge_count(), 1);
    assert!(g.contains_edge(n1, n3));
}

#[test]
fn test_graph_clear() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    g.add_edge(n1, n2, 1.0);

    g.clear();

    assert_eq!(g.node_count(), 0);
    assert_eq!(g.edge_count(), 0);
    assert!(g.is_empty());
}

#[test]
fn test_bulk_operations() {
    let mut g = Graph::<i32, f64>::new();

    // Bulk add nodes
    let nodes: Vec<NodeId> = (0..100).map(|i| g.add_node(i)).collect();

    assert_eq!(g.node_count(), 100);

    // Bulk add edges
    for i in 0..99 {
        g.add_edge(nodes[i], nodes[i + 1], 1.0);
    }

    assert_eq!(g.edge_count(), 99);
}

#[test]
fn test_retain_operations() {
    let mut g = Graph::<i32, f64>::new();
    let nodes: Vec<_> = (0..10).map(|i| g.add_node(i)).collect();

    for i in 0..9 {
        g.add_edge(nodes[i], nodes[i + 1], 1.0);
    }

    // Remove even-numbered nodes
    let to_remove: Vec<_> = g
        .nodes()
        .filter(|(_, val)| *val % 2 == 0)
        .map(|(id, _)| id)
        .collect();

    for id in to_remove {
        g.remove_node(id);
    }

    assert_eq!(g.node_count(), 5);
}

// ============================================================================
// Module Independence Tests - Parallel & Visualization
// ============================================================================

#[test]
#[cfg(feature = "parallel")]
fn test_parallel_module_accessible() {
    use graphina::parallel::degrees_parallel;

    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    g.add_edge(n1, n2, 1.0);

    let degrees = degrees_parallel(&g);
    assert_eq!(degrees.len(), 2);
}

#[test]
#[cfg(feature = "visualization")]
fn test_visualization_module_accessible() {
    use graphina::visualization::{LayoutAlgorithm, LayoutEngine};

    let mut g = Graph::<&str, f64>::new();
    let n1 = g.add_node("A");
    let n2 = g.add_node("B");
    g.add_edge(n1, n2, 1.0);

    let positions = LayoutEngine::compute_layout(&g, LayoutAlgorithm::Circular, 800.0, 600.0);

    assert_eq!(positions.len(), 2);
}

#[test]
#[cfg(feature = "visualization")]
fn test_all_layout_algorithms_available() {
    use graphina::visualization::{LayoutAlgorithm, LayoutEngine};

    let mut g = Graph::<i32, f64>::new();
    for i in 0..5 {
        g.add_node(i);
    }

    let algorithms = vec![
        LayoutAlgorithm::ForceDirected,
        LayoutAlgorithm::Circular,
        LayoutAlgorithm::Hierarchical,
        LayoutAlgorithm::Grid,
        LayoutAlgorithm::Random,
    ];

    for algo in algorithms {
        let positions = LayoutEngine::compute_layout(&g, algo, 800.0, 600.0);
        assert_eq!(positions.len(), 5);
    }
}

#[test]
#[cfg(all(feature = "parallel", feature = "visualization"))]
fn test_extensions_work_together() {
    use graphina::parallel::degrees_parallel;
    use graphina::visualization::LayoutAlgorithm;

    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    g.add_edge(n1, n2, 1.0);

    // Both extensions should work
    let degrees = degrees_parallel(&g);
    assert_eq!(degrees.len(), 2);

    // Should be able to create layout algorithms
    let _layout = LayoutAlgorithm::ForceDirected;
}

// ============================================================================
// Builder Patterns
// ============================================================================

#[test]
fn test_graph_builder_basic() {
    let g = Graph::<i32, f64>::builder()
        .add_node(1)
        .add_node(2)
        .add_node(3)
        .add_edge(0, 1, 1.0)
        .add_edge(1, 2, 2.0)
        .build();

    assert_eq!(g.node_count(), 3);
    assert_eq!(g.edge_count(), 2);
}

#[test]
fn test_digraph_builder() {
    let g = Digraph::<String, f64>::builder()
        .add_node("A".to_string())
        .add_node("B".to_string())
        .add_edge(0, 1, 5.0)
        .build();

    assert!(g.is_directed());
    assert_eq!(g.node_count(), 2);
    assert_eq!(g.edge_count(), 1);
}

#[test]
fn test_builder_with_capacity() {
    let g = Graph::<i32, f64>::with_capacity(100, 200);

    assert_eq!(g.node_count(), 0);
    assert_eq!(g.edge_count(), 0);
}
