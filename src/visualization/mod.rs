/*!
# Graph Visualization Extension

This extension provides multiple ways to visualize graphs:
- **D3.js Export**: Export graphs to D3.js-compatible JSON format for web visualization
- **Static Images**: Generate PNG/SVG images using the plotters crate
- **HTML Interactive Viewers**: Create standalone HTML files with interactive visualizations
- **ASCII Art**: Simple CLI debugging visualization

This module is independent of other extensions and only depends on the core library.

# Examples

```rust
use graphina::core::types::Graph;
use graphina::visualization::{VisualizationConfig, LayoutAlgorithm};

let mut g = Graph::<&str, f64>::new();
let n1 = g.add_node("A");
let n2 = g.add_node("B");
g.add_edge(n1, n2, 1.0);

// ASCII art for quick debugging
println!("{}", g.to_ascii_art());

// Export to D3.js format
let d3_json = g.to_d3_json().unwrap();

// Generate static image
let config = VisualizationConfig::default();
g.save_as_png("graph.png", &config).unwrap();

// Create interactive HTML viewer
g.save_as_html("graph.html", &config).unwrap();
```
*/

pub mod config;
pub mod d3;
pub mod layout;

// Re-export main types and functions for convenience
pub use config::VisualizationConfig;
pub use d3::{D3Graph, D3Link, D3Node};
pub use layout::{LayoutAlgorithm, LayoutEngine, NodePosition};
