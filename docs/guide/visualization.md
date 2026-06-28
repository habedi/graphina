# Visualization

Graphina provides tools to visualize graphs directly or export them to other formats.

## ASCII Visualization

For small graphs or quick debugging, you can print the graph structure to the console.

```rust
use graphina::core::types::Graph;
use graphina::visualization::d3::D3Graph;

// Setup
let mut graph = Graph::<&str, f64>::new();
graph.add_node("A");
println!("{}", graph.to_ascii_art());
```

## Interactive HTML / D3.js

Graphina can generate a standalone HTML file containing an interactive force-directed graph visualization powered by
D3.js.

```rust
use graphina::visualization::config::VisualizationConfig;
use graphina::visualization::d3::BaseGraphExt; // Import trait extensions if needed

let config = VisualizationConfig::default();

// Save to file
g.save_as_html("graph_view.html", &config).expect("Failed to save HTML");
```

Open `graph_view.html` in your browser to explore the graph (zoom, pan, drag nodes).

## Exporting for Other Tools

### DOT / Graphviz

Graphina plans to support DOT export.

### D3 JSON

You can export the raw JSON structure required for D3.js if you want to build a custom frontend.

```rust
let json_string = g.to_d3_json().unwrap();
```
