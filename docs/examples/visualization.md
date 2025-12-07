# Visualization Examples

## Quick ASCII Visualization

Useful for debugging small graphs directly in the terminal.

```rust
use graphina::core::types::Graph;

fn main() {
    let mut graph = Graph::<&str, f64>::new();
    let a = graph.add_node("Alice");
    let b = graph.add_node("Bob");
    graph.add_edge(a, b, 1.0);

    println!("{}", graph.to_ascii_art());
}
```

Output:
```text
Nodes:
  [0] Alice (degree: 1)
  [1] Bob (degree: 1)

Edges:
  [0] -- [1] (weight: 1)
```

## Interactive HTML Export

Generate a standalone HTML file with a force-directed layout.

```rust
use graphina::visualization::{LayoutAlgorithm, VisualizationConfig};
use graphina::core::types::Graph;

fn main() {
    let mut graph = Graph::<&str, f64>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    graph.add_edge(a, b, 1.0);

    let mut config = VisualizationConfig::default();
    config.layout = LayoutAlgorithm::ForceDirected;
    config.node_color = "#4CAF50".to_string(); // Custom Green
    config.edge_color = "#2196F3".to_string(); // Custom Blue

    // Creates 'graph.html' which you can open in a browser
    graph.save_as_html("graph.html", &config).unwrap();
}
```

## Static Image Export (PNG / SVG)

Requires the `visualization` feature enabled.

```rust
use graphina::visualization::{LayoutAlgorithm, VisualizationConfig};
use graphina::core::types::Graph;

fn main() {
    // Setup graph
    let mut graph = Graph::<&str, f64>::new();
    let n1 = graph.add_node("1");
    let n2 = graph.add_node("2");
    graph.add_edge(n1, n2, 1.0);

    // Force-directed PNG
    let mut config = VisualizationConfig::default();
    config.width = 1000;
    config.height = 800;
    graph.save_as_png("graph_force.png", &config).unwrap();

    // Circular SVG
    config.layout = LayoutAlgorithm::Circular;
    graph.save_as_svg("graph_circular.svg", &config).unwrap();
}
```

## D3.js JSON Export

Export raw data to build custom frontends.

```rust
use graphina::core::types::Graph;

fn main() {
    let mut graph = Graph::<&str, f64>::new();
    let n = graph.add_node("Node");

    let json_str = graph.to_d3_json().unwrap();
    std::fs::write("data.json", json_str).unwrap();
}
```
