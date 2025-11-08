//! Graph Visualization Examples
//!
//! This example shows all visualization-related features
//! - ASCII art
//! - D3.js JSON export
//! - Interactive HTML file
//! - PNG and SVG exports

use graphina::core::types::Graph;
use graphina::visualization::{LayoutAlgorithm, VisualizationConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Graph Visualization Example\n");

    // Create a sample graph
    let mut graph = Graph::<&str, f64>::new();

    // Add nodes
    let a = graph.add_node("Alice");
    let b = graph.add_node("Bob");
    let c = graph.add_node("Charlie");
    let d = graph.add_node("David");
    let e = graph.add_node("Eve");

    // Add edges with weights
    graph.add_edge(a, b, 1.0);
    graph.add_edge(a, c, 2.0);
    graph.add_edge(b, c, 1.5);
    graph.add_edge(b, d, 3.0);
    graph.add_edge(c, d, 2.5);
    graph.add_edge(c, e, 1.0);
    graph.add_edge(d, e, 2.0);

    // 1. ASCII Art Visualization (for CLI debugging)
    println!("📊 ASCII Art Visualization:");
    println!("{}\n", graph.to_ascii_art());

    // 2. Export to D3.js JSON format
    println!("Exporting to D3.js JSON format...");
    let d3_json = graph.to_d3_json()?;
    std::fs::write("graph_d3.json", &d3_json)?;
    println!("Saved to graph_d3.json\n");

    // 3. Generate Interactive HTML Viewer
    println!("Generating interactive HTML viewer...");
    let mut html_config = VisualizationConfig::default();
    html_config.layout = LayoutAlgorithm::ForceDirected;
    html_config.node_color = "#4CAF50".to_string();
    html_config.edge_color = "#2196F3".to_string();
    html_config.node_size = 15.0;

    graph.save_as_html("graph_interactive.html", &html_config)?;
    println!("✅ Saved to graph_interactive.html\n");

    // 4. Generate static images with different layouts
    println!("Generating static visualizations...");

    // Force-directed layout
    let mut config_force = VisualizationConfig::default();
    config_force.layout = LayoutAlgorithm::ForceDirected;
    config_force.width = 1000;
    config_force.height = 800;
    graph.save_as_png("graph_force_directed.png", &config_force)?;
    println!("Saved force-directed layout to graph_force_directed.png");

    // Circular layout
    let mut config_circular = VisualizationConfig::default();
    config_circular.layout = LayoutAlgorithm::Circular;
    config_circular.node_color = "#FF5722".to_string();
    graph.save_as_png("graph_circular.png", &config_circular)?;
    println!("Saved circular layout to graph_circular.png");

    // Hierarchical layout
    let mut config_hierarchical = VisualizationConfig::default();
    config_hierarchical.layout = LayoutAlgorithm::Hierarchical;
    config_hierarchical.node_color = "#9C27B0".to_string();
    graph.save_as_svg("graph_hierarchical.svg", &config_hierarchical)?;
    println!("Saved hierarchical layout to graph_hierarchical.svg");

    // Grid layout
    let mut config_grid = VisualizationConfig::default();
    config_grid.layout = LayoutAlgorithm::Grid;
    config_grid.node_color = "#FF9800".to_string();
    graph.save_as_svg("graph_grid.svg", &config_grid)?;
    println!("Saved grid layout to graph_grid.svg\n");

    println!("All visualizations generated successfully!");
    println!("\nFiles created:");
    println!("  - graph_d3.json (D3.js-compatible JSON)");
    println!("  - graph_interactive.html (Open in browser!)");
    println!("  - graph_force_directed.png");
    println!("  - graph_circular.png");
    println!("  - graph_hierarchical.svg");
    println!("  - graph_grid.svg");

    Ok(())
}
