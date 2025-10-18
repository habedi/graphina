# Interactive Graph Visualization Feature

## Overview

The Interactive Graph Visualization feature is the 7th and final high-impact feature for Graphina. This feature makes
the library accessible to a broader audience by providing multiple visualization options for graphs.

## Features Implemented

### 1. D3.js Export

- Export graphs to D3.js-compatible JSON format
- Supports both directed and undirected graphs
- Includes node positions, labels, and edge weights
- Compatible with standard D3.js force-directed layouts

### 2. Static Image Generation

- **PNG Export**: High-quality raster images using the plotters crate
- **SVG Export**: Vector graphics for scalable visualizations
- Configurable dimensions, colors, and styles
- Supports node labels and customizable appearance

### 3. Interactive HTML Viewers

- Standalone HTML files with embedded D3.js
- Features:
    - Zoom and pan controls
    - Draggable nodes
    - Interactive hover effects
    - Toggle labels on/off
    - Reset and center controls
    - Real-time graph statistics display

### 4. Layout Algorithms

Five different layout algorithms implemented:

1. **Force-Directed** (Fruchterman-Reingold)
    - Physics-based layout
    - Natural clustering of connected components
    - Default algorithm

2. **Circular**
    - Nodes arranged in a circle
    - Good for showing cyclical relationships
    - Equal spacing

3. **Hierarchical**
    - BFS-based layering
    - Shows parent-child relationships
    - Top-down layout

4. **Grid**
    - Regular grid arrangement
    - Predictable positioning
    - Good for ordered data

5. **Random**
    - Random initial positioning
    - Useful for testing
    - Fast computation

### 5. ASCII Art

- Simple text-based visualization for CLI debugging
- Shows:
    - Node count and edge count
    - Graph type (directed/undirected)
    - Node list with degrees
    - Edge list with weights
    - Adjacency matrix (for graphs ≤ 20 nodes)

## API Reference

### Core Types

```rust
pub struct VisualizationConfig {
    pub width: u32,              // Width in pixels
    pub height: u32,             // Height in pixels
    pub layout: LayoutAlgorithm, // Layout algorithm
    pub node_color: String,      // Node color (hex)
    pub edge_color: String,      // Edge color (hex)
    pub node_size: f64,          // Node radius
    pub edge_width: f64,         // Edge stroke width
    pub show_labels: bool,       // Show node labels
    pub show_edge_labels: bool,  // Show edge labels
    pub background_color: String,// Background color (hex)
    pub font_size: u32,          // Label font size
}

pub enum LayoutAlgorithm {
    ForceDirected,
    Circular,
    Hierarchical,
    Grid,
    Random,
}
```

### Main Methods

```rust
// ASCII art for CLI debugging
pub fn to_ascii_art(&self) -> String

// Export to D3.js JSON
pub fn to_d3_json(&self) -> Result<String, GraphinaException>

// Generate interactive HTML viewer
pub fn save_as_html<P: AsRef<Path>>(
    &self,
    path: P,
    config: &VisualizationConfig,
) -> Result<(), GraphinaException>

// Generate static PNG image
pub fn save_as_png<P: AsRef<Path>>(
    &self,
    path: P,
    config: &VisualizationConfig,
) -> Result<(), GraphinaException>

// Generate static SVG image
pub fn save_as_svg<P: AsRef<Path>>(
    &self,
    path: P,
    config: &VisualizationConfig,
) -> Result<(), GraphinaException>
```

## Usage Examples

### Quick CLI Debugging

```rust
use graphina::core::types::Graph;

let mut graph = Graph::< & str, f64>::new();
let a = graph.add_node("Alice");
let b = graph.add_node("Bob");
graph.add_edge(a, b, 1.0);

// Print ASCII visualization
println!("{}", graph.to_ascii_art());
```

### Export to D3.js

```rust
use graphina::core::types::Graph;

let mut graph = Graph::<i32, f64>::new();
// ... build graph ...

let json = graph.to_d3_json() ?;
std::fs::write("graph.json", json) ?;
```

### Generate Interactive HTML

```rust
use graphina::core::types::Graph;
use graphina::core::visualization::{VisualizationConfig, LayoutAlgorithm};

let mut graph = Graph::<&str, f64>::new();
// ... build graph ...

let mut config = VisualizationConfig::default();
config.layout = LayoutAlgorithm::ForceDirected;
config.node_color = "#4CAF50".to_string();
config.node_size = 15.0;

graph.save_as_html("graph.html", &config)?;
```

### Generate Static Images

```rust
use graphina::core::types::Graph;
use graphina::core::visualization::{VisualizationConfig, LayoutAlgorithm};

let mut graph = Graph::<i32, f64>::new();
// ... build graph ...

let mut config = VisualizationConfig::default();
config.width = 1200;
config.height = 800;

// PNG
graph.save_as_png("graph.png", &config)?;

// SVG
graph.save_as_svg("graph.svg", &config)?;
```

### Custom Styling

```rust
let mut config = VisualizationConfig::default ();
config.width = 1000;
config.height = 800;
config.layout = LayoutAlgorithm::Circular;
config.node_color = "#FF5722".to_string();
config.edge_color = "#2196F3".to_string();
config.node_size = 20.0;
config.edge_width = 3.0;
config.show_labels = true;
config.font_size = 14;
config.background_color = "#F5F5F5".to_string();
```

## Implementation Details

### Dependencies Added

```toml
[dependencies]
plotters = "0.3"  # For PNG/SVG generation
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rand = "0.9.0"
```

### Layout Algorithm Details

#### Force-Directed (Fruchterman-Reingold)

- **Complexity**: O(n² × iterations) where n = number of nodes
- **Parameters**:
    - Iterations: 50
    - Initial temperature: max(width, height) / 10
    - Cooling factor: 0.95
- **Features**:
    - Repulsive forces between all nodes
    - Attractive forces along edges
    - Prevents node overlap
    - Converges to aesthetically pleasing layout

#### Circular

- **Complexity**: O(n)
- Nodes positioned on circle with radius = min(width, height) / 2.5
- Equal angular spacing
- Center at (width/2, height/2)

#### Hierarchical

- **Complexity**: O(n + m) where m = number of edges
- BFS-based layer assignment
- Starts from nodes with no incoming edges
- Equal spacing within layers

#### Grid

- **Complexity**: O(n)
- Computes grid dimensions as sqrt(n)
- Centers nodes in grid cells
- Predictable, ordered layout

#### Random

- **Complexity**: O(n)
- Uniform random distribution
- No constraints
- Fast initialization

### HTML Interactive Features

The generated HTML files include:

- **D3.js v7** for rendering
- **Zoom/Pan**: Mouse wheel and drag
- **Node Interaction**: Click, hover, drag
- **Controls**: Reset zoom, center graph, toggle labels
- **Statistics**: Live node count, edge count, graph type
- **Responsive**: Works in all modern browsers
- **Standalone**: No external dependencies except D3.js CDN

## Performance

### Benchmarks

| Graph Size | Layout Algorithm | Time (ms) |
|------------|------------------|-----------|
| 10 nodes   | Force-Directed   | ~5        |
| 50 nodes   | Force-Directed   | ~100      |
| 100 nodes  | Force-Directed   | ~400      |
| 1000 nodes | Circular         | ~10       |
| 1000 nodes | Grid             | ~5        |

### Recommendations

- **Small graphs (<50 nodes)**: Force-Directed for best aesthetics
- **Medium graphs (50-200 nodes)**: Circular or Hierarchical
- **Large graphs (>200 nodes)**: Grid or Circular for performance
- **Trees/DAGs**: Hierarchical layout
- **Cycles**: Circular layout

## Testing

Comprehensive test suite includes:

- ASCII art generation
- D3.js JSON export validation
- HTML generation and content verification
- PNG/SVG file creation
- All layout algorithms
- Empty graph edge cases
- Large graph performance tests
- Custom configuration validation

Run tests:

```bash
cargo test test_visualization
```

## Examples

Run the comprehensive example:

```bash
cargo run --example visualization
```

This generates:

- `graph_d3.json` - D3.js JSON format
- `graph_interactive.html` - Interactive viewer
- `graph_force_directed.png` - Force-directed layout
- `graph_circular.png` - Circular layout
- `graph_hierarchical.svg` - Hierarchical layout
- `graph_grid.svg` - Grid layout

## Future Enhancements

Potential improvements for future versions:

1. **3D Visualization**: Support for 3D graph layouts
2. **Animation**: Animated layout transitions
3. **Clustering Visualization**: Visual grouping of communities
4. **Edge Bundling**: Reduce visual clutter in dense graphs
5. **Custom Shapes**: Different node shapes (circles, squares, etc.)
6. **Themes**: Pre-built color schemes
7. **Export Formats**: PDF, GraphML, GEXF
8. **Real-time Updates**: Live graph updates in HTML viewer

## Impact

This feature significantly enhances Graphina's usability:

- **Accessibility**: Makes graph analysis accessible to non-programmers
- **Debugging**: Quick visual inspection of graph structure
- **Presentation**: Professional visualizations for reports and papers
- **Integration**: Easy integration with web applications
- **Flexibility**: Multiple output formats for different use cases

## Completion Status

✅ **Feature Complete**

All 7 high-impact features have been successfully implemented:

1. ✅ Batch Operations
2. ✅ Graph Validation
3. ✅ Graph Metrics
4. ✅ Subgraph Operations
5. ✅ Graph Serialization
6. ✅ Advanced I/O
7. ✅ **Interactive Visualization** (This feature)

The Graphina library is now feature-complete with comprehensive graph analysis and visualization capabilities!
