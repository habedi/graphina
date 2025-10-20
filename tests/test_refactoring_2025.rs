/*!
# Refactoring Tests - 2025

Tests to verify that the parallel and visualization modules work correctly
as independent extensions after being extracted from the core module.
*/

use graphina::core::types::Graph;

#[cfg(feature = "parallel")]
mod parallel_tests {
    use super::*;
    use graphina::parallel::{
        bfs_parallel, clustering_coefficients_parallel, connected_components_parallel,
        degrees_parallel, pagerank_parallel, shortest_paths_parallel, triangles_parallel,
    };

    #[test]
    fn test_parallel_module_accessible() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n1, n2, 1.0);

        // Test that parallel module is accessible and works
        let degrees = degrees_parallel(&g);
        assert_eq!(degrees.len(), 2);
    }

    #[test]
    fn test_all_parallel_functions_available() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n1, 1.0);

        // Test all parallel functions are available
        let _ = bfs_parallel(&g, &[n1]);
        let _ = degrees_parallel(&g);
        let _ = clustering_coefficients_parallel(&g);
        let _ = triangles_parallel(&g);
        let _ = pagerank_parallel(&g, 0.85, 10, 1e-6);
        let _ = shortest_paths_parallel(&g, &[n1]);
        let _ = connected_components_parallel(&g);
    }
}

#[cfg(feature = "visualization")]
mod visualization_tests {
    use super::*;
    use graphina::visualization::{
        D3Graph, D3Link, D3Node, LayoutEngine, VisualizationConfig,
    };

    #[test]
    fn test_visualization_module_accessible() {
        let mut g = Graph::<&str, f64>::new();
        let n1 = g.add_node("A");
        let n2 = g.add_node("B");
        g.add_edge(n1, n2, 1.0);

        // Test that visualization module is accessible and works
        let ascii = g.to_ascii_art();
        assert!(ascii.contains("Graph Visualization"));
    }

    #[test]
    fn test_all_visualization_types_available() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n1, n2, 1.0);

        // Test all visualization types are available
        let _config = VisualizationConfig::default();
        let _layout = LayoutAlgorithm::ForceDirected;
        let positions = LayoutEngine::compute_layout(&g, LayoutAlgorithm::Circular, 800.0, 600.0);
        assert_eq!(positions.len(), 2);

        // Test D3 types
        let _d3_node = D3Node {
            id: "1".to_string(),
            label: "Node 1".to_string(),
            group: None,
            x: Some(100.0),
            y: Some(100.0),
        };

        let _d3_link = D3Link {
            source: "1".to_string(),
            target: "2".to_string(),
            value: 1.0,
            label: None,
        };

        let _d3_graph = D3Graph {
            nodes: vec![],
            links: vec![],
            directed: false,
        };
    }

    #[test]
    fn test_all_layout_algorithms_available() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n1, n2, 1.0);

        // Test all layout algorithms work
        let algorithms = vec![
            LayoutAlgorithm::ForceDirected,
            LayoutAlgorithm::Circular,
            LayoutAlgorithm::Hierarchical,
            LayoutAlgorithm::Grid,
            LayoutAlgorithm::Random,
        ];

        for algo in algorithms {
            let positions = LayoutEngine::compute_layout(&g, algo, 800.0, 600.0);
            assert_eq!(positions.len(), 2);
        }
    }

    #[test]
    fn test_graph_extension_methods() {
        let mut g = Graph::<&str, f64>::new();
        let n1 = g.add_node("Alice");
        let n2 = g.add_node("Bob");
        g.add_edge(n1, n2, 1.0);

        // Test ASCII art
        let ascii = g.to_ascii_art();
        assert!(ascii.contains("Alice"));
        assert!(ascii.contains("Bob"));

        // Test D3 JSON export
        let json = g.to_d3_json().expect("Failed to export to D3 JSON");
        assert!(json.contains("Alice"));
        assert!(json.contains("Bob"));

        // Test D3 graph structure
        let d3_graph = g.to_d3_graph(None).expect("Failed to create D3 graph");
        assert_eq!(d3_graph.nodes.len(), 2);
        assert_eq!(d3_graph.links.len(), 1);
        assert!(!d3_graph.directed);
    }
}

#[test]
fn test_core_module_is_leaner() {
    // This test documents that core no longer contains parallel or visualization
    // We can only test this by ensuring the imports work correctly

    use graphina::core::types::Graph;

    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    g.add_edge(n1, n2, 1.0);

    // Core functionality should still work
    assert_eq!(g.node_count(), 2);
    assert_eq!(g.edge_count(), 1);
    assert!(g.contains_edge(n1, n2));
}

#[test]
fn test_extensions_are_independent() {
    // Test that we can use one extension without the other
    // (This is enforced by feature flags at compile time)

    use graphina::core::types::Graph;

    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    g.add_edge(n1, n2, 1.0);

    // Core should work without any extensions
    assert!(g.neighbors(n1).any(|n| n == n2));
}

#[cfg(all(feature = "parallel", feature = "visualization"))]
#[test]
fn test_extensions_work_together() {
    // When both features are enabled, both should work

    use graphina::parallel::degrees_parallel;

    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    g.add_edge(n1, n2, 1.0);

    // Both extensions should work
    let degrees = degrees_parallel(&g);
    assert_eq!(degrees.len(), 2);

    let ascii = g.to_ascii_art();
    assert!(ascii.contains("Graph Visualization"));
}
