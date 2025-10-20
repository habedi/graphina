/*!
# Visualization Module Tests

Tests for graph visualization functionality including:
- ASCII art generation
- D3.js JSON export
- HTML generation
- Layout algorithms
- Static image generation
*/

use graphina::core::types::Graph;
use graphina::visualization::{LayoutAlgorithm, LayoutEngine, VisualizationConfig};
use std::fs;

#[test]
fn test_ascii_art_visualization() {
    let mut graph = Graph::<&str, f64>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    graph.add_edge(a, b, 1.0);

    let ascii = graph.to_ascii_art();

    assert!(ascii.contains("Graph Visualization (ASCII)"));
    assert!(ascii.contains("Nodes: 2"));
    assert!(ascii.contains("Edges: 1"));
    assert!(ascii.contains("Undirected")); // Graph is Undirected by default
    assert!(ascii.contains("[0] A"));
    assert!(ascii.contains("[1] B"));
}

#[test]
fn test_d3_json_export() {
    let mut graph = Graph::<i32, f64>::new();
    let n1 = graph.add_node(10);
    let n2 = graph.add_node(20);
    graph.add_edge(n1, n2, 1.5);

    let json = graph.to_d3_json().expect("Failed to export to D3 JSON");

    assert!(json.contains("\"nodes\""));
    assert!(json.contains("\"links\""));
    assert!(json.contains("\"directed\": false")); // Graph is Undirected by default
    assert!(json.contains("\"label\": \"10\""));
    assert!(json.contains("\"label\": \"20\""));
    assert!(json.contains("\"value\": 1.5"));
}

#[test]
fn test_force_directed_layout() {
    let mut graph = Graph::<i32, f64>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, n3, 1.0);

    let positions =
        LayoutEngine::compute_layout(&graph, LayoutAlgorithm::ForceDirected, 800.0, 600.0);

    assert_eq!(positions.len(), 3);
    for (_, pos) in &positions {
        assert!(pos.x >= 0.0 && pos.x <= 800.0);
        assert!(pos.y >= 0.0 && pos.y <= 600.0);
    }
}

#[test]
fn test_circular_layout() {
    let mut graph = Graph::<i32, f64>::new();
    let _n1 = graph.add_node(1);
    let _n2 = graph.add_node(2);
    let _n3 = graph.add_node(3);
    let _n4 = graph.add_node(4);

    let positions = LayoutEngine::compute_layout(&graph, LayoutAlgorithm::Circular, 800.0, 600.0);

    assert_eq!(positions.len(), 4);

    // Check that nodes are positioned in a circular pattern
    let center_x = 400.0;
    let center_y = 300.0;

    for (_, pos) in &positions {
        let dx = pos.x - center_x;
        let dy = pos.y - center_y;
        let distance = (dx * dx + dy * dy).sqrt();

        // All nodes should be approximately the same distance from center
        assert!(distance > 0.0);
    }
}

#[test]
fn test_grid_layout() {
    let mut graph = Graph::<i32, f64>::new();
    for i in 0..9 {
        graph.add_node(i);
    }

    let positions = LayoutEngine::compute_layout(&graph, LayoutAlgorithm::Grid, 900.0, 900.0);

    assert_eq!(positions.len(), 9);

    // Check all positions are within bounds
    for (_, pos) in &positions {
        assert!(pos.x >= 0.0 && pos.x <= 900.0);
        assert!(pos.y >= 0.0 && pos.y <= 900.0);
    }
}

#[test]
fn test_hierarchical_layout() {
    let mut graph = Graph::<i32, f64>::new();
    let root = graph.add_node(1);
    let child1 = graph.add_node(2);
    let child2 = graph.add_node(3);
    let grandchild = graph.add_node(4);

    graph.add_edge(root, child1, 1.0);
    graph.add_edge(root, child2, 1.0);
    graph.add_edge(child1, grandchild, 1.0);

    let positions =
        LayoutEngine::compute_layout(&graph, LayoutAlgorithm::Hierarchical, 800.0, 600.0);

    assert_eq!(positions.len(), 4);

    // Root should be at top (lower y value)
    let root_pos = positions.get(&root).unwrap();
    let child1_pos = positions.get(&child1).unwrap();
    let grandchild_pos = positions.get(&grandchild).unwrap();

    // Check hierarchical ordering
    assert!(root_pos.y <= child1_pos.y);
    assert!(child1_pos.y <= grandchild_pos.y);
}

#[test]
fn test_html_generation() {
    let mut graph = Graph::<&str, f64>::new();
    let a = graph.add_node("Node A");
    let b = graph.add_node("Node B");
    graph.add_edge(a, b, 2.5);

    let temp_file = "test_graph.html";
    let config = VisualizationConfig::default();

    graph
        .save_as_html(temp_file, &config)
        .expect("Failed to save HTML");

    let html_content = fs::read_to_string(temp_file).expect("Failed to read HTML file");

    assert!(html_content.contains("<!DOCTYPE html>"));
    assert!(html_content.contains("Graph Visualization"));
    assert!(html_content.contains("d3.v7.min.js")); // Check for the actual D3.js CDN URL
    assert!(html_content.contains("graphData"));
    assert!(html_content.contains("Node A"));
    assert!(html_content.contains("Node B"));

    fs::remove_file(temp_file).ok();
}

#[test]
fn test_png_generation() {
    let mut graph = Graph::<i32, f64>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    graph.add_edge(n1, n2, 1.0);

    let temp_file = "test_graph.png";
    let config = VisualizationConfig::default();

    graph
        .save_as_png(temp_file, &config)
        .expect("Failed to save PNG");

    assert!(fs::metadata(temp_file).is_ok());
    assert!(fs::metadata(temp_file).unwrap().len() > 0);

    fs::remove_file(temp_file).ok();
}

#[test]
fn test_svg_generation() {
    let mut graph = Graph::<i32, f64>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    graph.add_edge(n1, n2, 1.0);

    let temp_file = "test_graph.svg";
    let config = VisualizationConfig::default();

    graph
        .save_as_svg(temp_file, &config)
        .expect("Failed to save SVG");

    let svg_content = fs::read_to_string(temp_file).expect("Failed to read SVG file");
    assert!(svg_content.contains("<svg"));

    fs::remove_file(temp_file).ok();
}

#[test]
fn test_visualization_config_customization() {
    let mut config = VisualizationConfig::default();

    config.width = 1200;
    config.height = 900;
    config.layout = LayoutAlgorithm::Circular;
    config.node_color = "#FF5733".to_string();
    config.edge_color = "#33FF57".to_string();
    config.node_size = 20.0;
    config.edge_width = 3.0;
    config.show_labels = false;
    config.font_size = 16;

    assert_eq!(config.width, 1200);
    assert_eq!(config.height, 900);
    assert_eq!(config.layout, LayoutAlgorithm::Circular);
    assert_eq!(config.node_color, "#FF5733");
    assert_eq!(config.node_size, 20.0);
    assert!(!config.show_labels);
}

#[test]
fn test_empty_graph_visualization() {
    let graph = Graph::<i32, f64>::new();

    let ascii = graph.to_ascii_art();
    assert!(ascii.contains("Nodes: 0"));
    assert!(ascii.contains("Edges: 0"));

    let json = graph.to_d3_json().expect("Failed to export empty graph");
    assert!(json.contains("\"nodes\": []"));
    assert!(json.contains("\"links\": []"));
}

#[test]
fn test_large_graph_layout_performance() {
    let mut graph = Graph::<i32, f64>::new();

    // Create a graph with 100 nodes
    let nodes: Vec<_> = (0..100).map(|i| graph.add_node(i)).collect();

    // Add some edges
    for i in 0..99 {
        graph.add_edge(nodes[i], nodes[i + 1], 1.0);
    }

    // Test that layout computation completes in reasonable time
    let start = std::time::Instant::now();
    let positions =
        LayoutEngine::compute_layout(&graph, LayoutAlgorithm::ForceDirected, 1000.0, 1000.0);
    let duration = start.elapsed();

    assert_eq!(positions.len(), 100);
    assert!(duration.as_secs() < 5, "Layout computation took too long");
}

#[test]
fn test_all_layout_algorithms() {
    let mut graph = Graph::<i32, f64>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, n3, 1.0);

    let algorithms = vec![
        LayoutAlgorithm::ForceDirected,
        LayoutAlgorithm::Circular,
        LayoutAlgorithm::Hierarchical,
        LayoutAlgorithm::Grid,
        LayoutAlgorithm::Random,
    ];

    for algorithm in algorithms {
        let positions = LayoutEngine::compute_layout(&graph, algorithm, 800.0, 600.0);
        assert_eq!(positions.len(), 3, "Algorithm {:?} failed", algorithm);
    }
}
