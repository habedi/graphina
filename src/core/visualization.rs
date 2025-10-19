/*!
# Graph Visualization Module

This module provides multiple ways to visualize graphs:
- **D3.js Export**: Export graphs to D3.js-compatible JSON format for web visualization
- **Static Images**: Generate PNG/SVG images using the plotters crate
- **HTML Interactive Viewers**: Create standalone HTML files with interactive visualizations
- **ASCII Art**: Simple CLI debugging visualization

# Examples

```rust
use graphina::core::types::Graph;
use graphina::core::visualization::{VisualizationConfig, LayoutAlgorithm};

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

use crate::core::error::GraphinaError;
use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use petgraph::EdgeType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// D3.js-compatible node representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D3Node {
    pub id: String,
    pub label: String,
    pub group: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}

/// D3.js-compatible edge representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D3Link {
    pub source: String,
    pub target: String,
    pub value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// D3.js-compatible graph representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D3Graph {
    pub nodes: Vec<D3Node>,
    pub links: Vec<D3Link>,
    pub directed: bool,
}

/// Layout algorithms for graph visualization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayoutAlgorithm {
    /// Force-directed layout (default)
    #[default]
    ForceDirected,
    /// Circular layout
    Circular,
    /// Hierarchical/tree layout
    Hierarchical,
    /// Grid layout
    Grid,
    /// Random layout
    Random,
}

/// Configuration for graph visualization
#[derive(Debug, Clone)]
pub struct VisualizationConfig {
    /// Width of the visualization in pixels
    pub width: u32,
    /// Height of the visualization in pixels
    pub height: u32,
    /// Layout algorithm to use
    pub layout: LayoutAlgorithm,
    /// Node color (hex format, e.g., "#69b3a2")
    pub node_color: String,
    /// Edge color (hex format)
    pub edge_color: String,
    /// Node size
    pub node_size: f64,
    /// Edge width
    pub edge_width: f64,
    /// Whether to show node labels
    pub show_labels: bool,
    /// Whether to show edge labels
    pub show_edge_labels: bool,
    /// Background color
    pub background_color: String,
    /// Font size for labels
    pub font_size: u32,
}

impl Default for VisualizationConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            layout: LayoutAlgorithm::ForceDirected,
            node_color: "#69b3a2".to_string(),
            edge_color: "#999999".to_string(),
            node_size: 10.0,
            edge_width: 2.0,
            show_labels: true,
            show_edge_labels: false,
            background_color: "#ffffff".to_string(),
            font_size: 12,
        }
    }
}

/// Node position in 2D space
#[derive(Debug, Clone, Copy)]
pub struct NodePosition {
    pub x: f64,
    pub y: f64,
}

/// Layout engine for computing node positions
pub struct LayoutEngine;

impl LayoutEngine {
    /// Compute node positions using the specified layout algorithm
    pub fn compute_layout<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
        graph: &BaseGraph<A, W, Ty>,
        algorithm: LayoutAlgorithm,
        width: f64,
        height: f64,
    ) -> HashMap<NodeId, NodePosition> {
        match algorithm {
            LayoutAlgorithm::ForceDirected => Self::force_directed_layout(graph, width, height),
            LayoutAlgorithm::Circular => Self::circular_layout(graph, width, height),
            LayoutAlgorithm::Hierarchical => Self::hierarchical_layout(graph, width, height),
            LayoutAlgorithm::Grid => Self::grid_layout(graph, width, height),
            LayoutAlgorithm::Random => Self::random_layout(graph, width, height),
        }
    }

    /// Force-directed layout using Fruchterman-Reingold algorithm
    fn force_directed_layout<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
        graph: &BaseGraph<A, W, Ty>,
        width: f64,
        height: f64,
    ) -> HashMap<NodeId, NodePosition> {
        let mut positions = HashMap::new();
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();

        if nodes.is_empty() {
            return positions;
        }

        // Initialize with random positions
        use rand::Rng;
        let mut rng = rand::rng();
        for node in &nodes {
            positions.insert(
                *node,
                NodePosition {
                    x: rng.random_range(0.0..width),
                    y: rng.random_range(0.0..height),
                },
            );
        }

        // Fruchterman-Reingold parameters
        let area = width * height;
        let k = (area / nodes.len() as f64).sqrt();
        let iterations = 50;
        let mut temperature = width.max(height) / 10.0;
        let cooling_factor = 0.95;

        for _ in 0..iterations {
            let mut displacements = HashMap::new();

            // Repulsive forces between all pairs
            for i in 0..nodes.len() {
                let mut dx = 0.0;
                let mut dy = 0.0;

                for j in 0..nodes.len() {
                    if i != j {
                        let pos_i = positions[&nodes[i]];
                        let pos_j = positions[&nodes[j]];

                        let delta_x = pos_i.x - pos_j.x;
                        let delta_y = pos_i.y - pos_j.y;
                        let distance = (delta_x * delta_x + delta_y * delta_y).sqrt().max(0.01);

                        let force = k * k / distance;
                        dx += (delta_x / distance) * force;
                        dy += (delta_y / distance) * force;
                    }
                }

                displacements.insert(nodes[i], (dx, dy));
            }

            // Attractive forces for edges
            for (src, tgt, _) in graph.edges() {
                let pos_src = positions[&src];
                let pos_tgt = positions[&tgt];

                let delta_x = pos_tgt.x - pos_src.x;
                let delta_y = pos_tgt.y - pos_src.y;
                let distance = (delta_x * delta_x + delta_y * delta_y).sqrt().max(0.01);

                let force = distance * distance / k;

                let (dx_src, dy_src) = displacements.get_mut(&src).unwrap();
                *dx_src += (delta_x / distance) * force;
                *dy_src += (delta_y / distance) * force;

                let (dx_tgt, dy_tgt) = displacements.get_mut(&tgt).unwrap();
                *dx_tgt -= (delta_x / distance) * force;
                *dy_tgt -= (delta_y / distance) * force;
            }

            // Apply displacements
            for node in &nodes {
                let (dx, dy) = displacements[node];
                let pos = positions.get_mut(node).unwrap();

                let displacement = (dx * dx + dy * dy).sqrt();
                if displacement > 0.0 {
                    let limited = displacement.min(temperature);
                    pos.x += (dx / displacement) * limited;
                    pos.y += (dy / displacement) * limited;

                    // Keep within bounds
                    pos.x = pos.x.max(0.0).min(width);
                    pos.y = pos.y.max(0.0).min(height);
                }
            }

            temperature *= cooling_factor;
        }

        positions
    }

    /// Circular layout
    fn circular_layout<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
        graph: &BaseGraph<A, W, Ty>,
        width: f64,
        height: f64,
    ) -> HashMap<NodeId, NodePosition> {
        let mut positions = HashMap::new();
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();

        if nodes.is_empty() {
            return positions;
        }

        let center_x = width / 2.0;
        let center_y = height / 2.0;
        let radius = width.min(height) / 2.5;

        for (i, node) in nodes.iter().enumerate() {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / nodes.len() as f64;
            positions.insert(
                *node,
                NodePosition {
                    x: center_x + radius * angle.cos(),
                    y: center_y + radius * angle.sin(),
                },
            );
        }

        positions
    }

    /// Hierarchical/tree layout
    fn hierarchical_layout<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
        graph: &BaseGraph<A, W, Ty>,
        width: f64,
        height: f64,
    ) -> HashMap<NodeId, NodePosition> {
        let mut positions = HashMap::new();
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();

        if nodes.is_empty() {
            return positions;
        }

        // Simple BFS-based layering
        let mut layers: Vec<Vec<NodeId>> = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();

        // Start from nodes with no incoming edges, or just the first node
        let start_nodes: Vec<_> = nodes
            .iter()
            .filter(|&&n| graph.in_degree(n).unwrap_or(0) == 0)
            .copied()
            .collect();

        if start_nodes.is_empty() {
            queue.push_back(nodes[0]);
        } else {
            for node in start_nodes {
                queue.push_back(node);
            }
        }

        while !queue.is_empty() {
            let layer_size = queue.len();
            let mut current_layer = Vec::new();

            for _ in 0..layer_size {
                if let Some(node) = queue.pop_front() {
                    if visited.insert(node) {
                        current_layer.push(node);

                        // Add neighbors to queue
                        for (_, neighbor, _) in graph.edges().filter(|(src, _, _)| *src == node) {
                            if !visited.contains(&neighbor) {
                                queue.push_back(neighbor);
                            }
                        }
                    }
                }
            }

            if !current_layer.is_empty() {
                layers.push(current_layer);
            }
        }

        // Add any remaining unvisited nodes
        for node in nodes {
            if !visited.contains(&node) {
                layers.push(vec![node]);
            }
        }

        // Position nodes
        let layer_height = if layers.len() > 1 {
            height / (layers.len() - 1) as f64
        } else {
            height / 2.0
        };

        for (layer_idx, layer) in layers.iter().enumerate() {
            let y = layer_idx as f64 * layer_height;
            let layer_width = if layer.len() > 1 {
                width / (layer.len() - 1) as f64
            } else {
                width / 2.0
            };

            for (node_idx, &node) in layer.iter().enumerate() {
                let x = if layer.len() > 1 {
                    node_idx as f64 * layer_width
                } else {
                    width / 2.0
                };

                positions.insert(node, NodePosition { x, y });
            }
        }

        positions
    }

    /// Grid layout
    fn grid_layout<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
        graph: &BaseGraph<A, W, Ty>,
        width: f64,
        height: f64,
    ) -> HashMap<NodeId, NodePosition> {
        let mut positions = HashMap::new();
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();

        if nodes.is_empty() {
            return positions;
        }

        let cols = (nodes.len() as f64).sqrt().ceil() as usize;
        let rows = (nodes.len() as f64 / cols as f64).ceil() as usize;

        let cell_width = width / cols as f64;
        let cell_height = height / rows as f64;

        for (i, node) in nodes.iter().enumerate() {
            let row = i / cols;
            let col = i % cols;

            positions.insert(
                *node,
                NodePosition {
                    x: (col as f64 + 0.5) * cell_width,
                    y: (row as f64 + 0.5) * cell_height,
                },
            );
        }

        positions
    }

    /// Random layout
    fn random_layout<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
        graph: &BaseGraph<A, W, Ty>,
        width: f64,
        height: f64,
    ) -> HashMap<NodeId, NodePosition> {
        use rand::Rng;
        let mut rng = rand::rng();
        let mut positions = HashMap::new();

        for (node, _) in graph.nodes() {
            positions.insert(
                node,
                NodePosition {
                    x: rng.random_range(0.0..width),
                    y: rng.random_range(0.0..height),
                },
            );
        }

        positions
    }
}

impl<A, W, Ty> BaseGraph<A, W, Ty>
where
    A: Display + Clone,
    W: Display + Clone + Into<f64>,
    Ty: GraphConstructor<A, W> + EdgeType,
{
    /// Export graph to D3.js-compatible JSON format
    pub fn to_d3_json(&self) -> Result<String, GraphinaError> {
        let d3_graph = self.to_d3_graph(None)?;
        serde_json::to_string_pretty(&d3_graph).map_err(GraphinaError::from)
    }

    /// Export graph to D3Graph structure with optional positions
    pub fn to_d3_graph(
        &self,
        positions: Option<&HashMap<NodeId, NodePosition>>,
    ) -> Result<D3Graph, GraphinaError> {
        let mut nodes = Vec::new();
        let mut links = Vec::new();

        // Build nodes
        for (node_id, attr) in self.nodes() {
            let pos = positions.and_then(|p| p.get(&node_id));
            nodes.push(D3Node {
                id: format!("{}", node_id.index()),
                label: format!("{}", attr),
                group: None,
                x: pos.map(|p| p.x),
                y: pos.map(|p| p.y),
            });
        }

        // Build links
        for (src, tgt, weight) in self.edges() {
            links.push(D3Link {
                source: format!("{}", src.index()),
                target: format!("{}", tgt.index()),
                value: (*weight).clone().into(),
                label: Some(format!("{}", weight)),
            });
        }

        Ok(D3Graph {
            nodes,
            links,
            directed: self.is_directed(),
        })
    }

    /// Generate ASCII art representation of the graph
    pub fn to_ascii_art(&self) -> String {
        let mut output = String::new();
        output.push_str("Graph Visualization (ASCII)\n");
        output.push_str(&format!(
            "Nodes: {}, Edges: {}, Type: {}\n",
            self.node_count(),
            self.edge_count(),
            if self.is_directed() {
                "Directed"
            } else {
                "Undirected"
            }
        ));
        output.push_str(&"=".repeat(50));
        output.push('\n');

        // List nodes
        output.push_str("\nNodes:\n");
        for (node_id, attr) in self.nodes() {
            let degree = self.degree(node_id).unwrap_or(0);
            output.push_str(&format!(
                "  [{}] {} (degree: {})\n",
                node_id.index(),
                attr,
                degree
            ));
        }

        // List edges
        output.push_str("\nEdges:\n");
        for (src, tgt, weight) in self.edges() {
            let arrow = if self.is_directed() { "->" } else { "--" };
            output.push_str(&format!(
                "  [{}] {} [{}] (weight: {})\n",
                src.index(),
                arrow,
                tgt.index(),
                weight
            ));
        }

        // Simple adjacency visualization for small graphs
        if self.node_count() <= 20 {
            output.push_str("\nAdjacency Matrix:\n");
            output.push_str("    ");
            let nodes: Vec<_> = self.nodes().map(|(id, _)| id).collect();
            for node in &nodes {
                output.push_str(&format!("{:3} ", node.index()));
            }
            output.push('\n');

            for src in &nodes {
                output.push_str(&format!("{:3} ", src.index()));
                for tgt in &nodes {
                    if self.contains_edge(*src, *tgt) {
                        output.push_str("  X ");
                    } else {
                        output.push_str("  . ");
                    }
                }
                output.push('\n');
            }
        }

        output
    }

    /// Save graph as interactive HTML file
    pub fn save_as_html<P: AsRef<Path>>(
        &self,
        path: P,
        config: &VisualizationConfig,
    ) -> Result<(), GraphinaError> {
        let positions = LayoutEngine::compute_layout(
            self,
            config.layout,
            config.width as f64,
            config.height as f64,
        );

        let d3_graph = self.to_d3_graph(Some(&positions))?;
        let graph_json = serde_json::to_string(&d3_graph).map_err(GraphinaError::from)?;

        let html = Self::generate_html_template(config, &graph_json);

        let mut file = File::create(path.as_ref()).map_err(GraphinaError::from)?;

        file.write_all(html.as_bytes())
            .map_err(GraphinaError::from)?;

        Ok(())
    }

    /// Generate HTML template for interactive visualization
    fn generate_html_template(config: &VisualizationConfig, graph_json: &str) -> String {
        let html_head = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Graph Visualization</title>
    <script src="https://d3js.org/d3.v7.min.js"></script>
    <style>
        body {{
            margin: 0;
            padding: 20px;
            font-family: Arial, sans-serif;
            background-color: {};
        }}
        #graph {{
            border: 1px solid #ddd;
            background-color: white;
        }}
        .controls {{
            margin-bottom: 10px;
        }}
        .controls button {{
            margin-right: 10px;
            padding: 5px 15px;
            cursor: pointer;
        }}
        .info {{
            margin-top: 10px;
            padding: 10px;
            background-color: #f0f0f0;
            border-radius: 5px;
        }}
    </style>
</head>
<body>
    <h1>Interactive Graph Visualization</h1>
    <div class="controls">
        <button onclick="resetZoom()">Reset Zoom</button>
        <button onclick="centerGraph()">Center</button>
        <button onclick="toggleLabels()">Toggle Labels</button>
    </div>
    <svg id="graph" width="{}" height="{}"></svg>
    <div class="info">
        <div><strong>Nodes:</strong> <span id="node-count">-</span></div>
        <div><strong>Edges:</strong> <span id="edge-count">-</span></div>
        <div><strong>Type:</strong> <span id="graph-type">-</span></div>
        <div id="selected-info"></div>
    </div>
    <script>"#,
            config.background_color, config.width, config.height
        );

        // Build JavaScript with proper escaping
        let script_body = format!(
            r##"
        var graphData = {};
        var showLabels = {};
        var svg = d3.select("#graph");
        var width = {};
        var height = {};
        var g = svg.append("g");

        var zoom = d3.zoom()
            .scaleExtent([0.1, 10])
            .on("zoom", function(event) {{
                g.attr("transform", event.transform);
            }});
        svg.call(zoom);

        if (graphData.directed) {{
            svg.append("defs").append("marker")
                .attr("id", "arrowhead")
                .attr("viewBox", "-0 -5 10 10")
                .attr("refX", 20)
                .attr("refY", 0)
                .attr("orient", "auto")
                .attr("markerWidth", 6)
                .attr("markerHeight", 6)
                .append("path")
                .attr("d", "M 0,-5 L 10,0 L 0,5")
                .attr("fill", "{}");
        }}

        var link = g.append("g")
            .selectAll("line")
            .data(graphData.links)
            .enter().append("line")
            .attr("stroke", "{}")
            .attr("stroke-width", {})
            .attr("marker-end", graphData.directed ? "url(#arrowhead)" : null);

        var node = g.append("g")
            .selectAll("circle")
            .data(graphData.nodes)
            .enter().append("circle")
            .attr("r", {})
            .attr("fill", "{}")
            .attr("stroke", "#fff")
            .attr("stroke-width", 2)
            .on("mouseover", handleMouseOver)
            .on("mouseout", handleMouseOut)
            .on("click", handleClick)
            .call(d3.drag()
                .on("start", dragstarted)
                .on("drag", dragged)
                .on("end", dragended));

        var label = g.append("g")
            .selectAll("text")
            .data(graphData.nodes)
            .enter().append("text")
            .text(function(d) {{ return d.label; }})
            .attr("font-size", {})
            .attr("dx", 12)
            .attr("dy", 4)
            .style("display", showLabels ? "block" : "none");

        function updatePositions() {{
            link
                .attr("x1", function(d) {{ return graphData.nodes.find(function(n) {{ return n.id === d.source; }}).x; }})
                .attr("y1", function(d) {{ return graphData.nodes.find(function(n) {{ return n.id === d.source; }}).y; }})
                .attr("x2", function(d) {{ return graphData.nodes.find(function(n) {{ return n.id === d.target; }}).x; }})
                .attr("y2", function(d) {{ return graphData.nodes.find(function(n) {{ return n.id === d.target; }}).y; }});
            node
                .attr("cx", function(d) {{ return d.x; }})
                .attr("cy", function(d) {{ return d.y; }});
            label
                .attr("x", function(d) {{ return d.x; }})
                .attr("y", function(d) {{ return d.y; }});
        }}

        updatePositions();

        function handleMouseOver(event, d) {{
            d3.select(this).transition().duration(200).attr("r", {} * 1.5);
            document.getElementById("selected-info").innerHTML = "<strong>Selected Node:</strong> " + d.label + " (ID: " + d.id + ")";
        }}

        function handleMouseOut(event, d) {{
            d3.select(this).transition().duration(200).attr("r", {});
        }}

        function handleClick(event, d) {{
            console.log("Clicked node:", d);
        }}

        function dragstarted(event, d) {{
            d3.select(this).raise().attr("stroke", "black");
        }}

        function dragged(event, d) {{
            d.x = event.x;
            d.y = event.y;
            updatePositions();
        }}

        function dragended(event, d) {{
            d3.select(this).attr("stroke", "#fff");
        }}

        function resetZoom() {{
            svg.transition().duration(750).call(zoom.transform, d3.zoomIdentity);
        }}

        function centerGraph() {{
            var bounds = g.node().getBBox();
            var fullWidth = bounds.width;
            var fullHeight = bounds.height;
            var midX = bounds.x + fullWidth / 2;
            var midY = bounds.y + fullHeight / 2;
            var scale = 0.9 / Math.max(fullWidth / width, fullHeight / height);
            var translate = [width / 2 - scale * midX, height / 2 - scale * midY];
            svg.transition().duration(750).call(zoom.transform, d3.zoomIdentity.translate(translate[0], translate[1]).scale(scale));
        }}

        function toggleLabels() {{
            showLabels = !showLabels;
            label.style("display", showLabels ? "block" : "none");
        }}

        document.getElementById("node-count").textContent = graphData.nodes.length;
        document.getElementById("edge-count").textContent = graphData.links.length;
        document.getElementById("graph-type").textContent = graphData.directed ? "Directed" : "Undirected";
    </script>
</body>
</html>"##,
            graph_json,
            if config.show_labels { "true" } else { "false" },
            config.width,
            config.height,
            config.edge_color,
            config.edge_color,
            config.edge_width,
            config.node_size,
            config.node_color,
            config.font_size,
            config.node_size,
            config.node_size
        );

        format!("{}{}", html_head, script_body)
    }

    /// Save graph as PNG image using plotters
    pub fn save_as_png<P: AsRef<Path>>(
        &self,
        path: P,
        config: &VisualizationConfig,
    ) -> Result<(), GraphinaError> {
        use plotters::prelude::*;

        let positions = LayoutEngine::compute_layout(
            self,
            config.layout,
            config.width as f64,
            config.height as f64,
        );

        let root =
            BitMapBackend::new(path.as_ref(), (config.width, config.height)).into_drawing_area();

        root.fill(&WHITE)
            .map_err(|e| GraphinaError::AlgorithmError(e.to_string()))?;

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .build_cartesian_2d(0.0..config.width as f64, 0.0..config.height as f64)
            .map_err(|e| GraphinaError::AlgorithmError(e.to_string()))?;

        // Draw edges
        for (src, tgt, _weight) in self.edges() {
            if let (Some(pos_src), Some(pos_tgt)) = (positions.get(&src), positions.get(&tgt)) {
                chart
                    .draw_series(std::iter::once(PathElement::new(
                        vec![(pos_src.x, pos_src.y), (pos_tgt.x, pos_tgt.y)],
                        ShapeStyle::from(&RGBColor(150, 150, 150))
                            .stroke_width(config.edge_width as u32),
                    )))
                    .map_err(|e| GraphinaError::AlgorithmError(e.to_string()))?;
            }
        }

        // Draw nodes
        for (node_id, attr) in self.nodes() {
            if let Some(pos) = positions.get(&node_id) {
                chart
                    .draw_series(std::iter::once(Circle::new(
                        (pos.x, pos.y),
                        config.node_size as i32,
                        ShapeStyle::from(&RGBColor(105, 179, 162)).filled(),
                    )))
                    .map_err(|e| GraphinaError::AlgorithmError(e.to_string()))?;

                // Draw label if enabled
                if config.show_labels {
                    chart
                        .draw_series(std::iter::once(Text::new(
                            format!("{}", attr),
                            (pos.x + config.node_size + 2.0, pos.y),
                            ("sans-serif", config.font_size).into_font(),
                        )))
                        .map_err(|e| GraphinaError::AlgorithmError(e.to_string()))?;
                }
            }
        }

        root.present()
            .map_err(|e| GraphinaError::AlgorithmError(e.to_string()))?;

        Ok(())
    }

    /// Save graph as SVG image using plotters
    pub fn save_as_svg<P: AsRef<Path>>(
        &self,
        path: P,
        config: &VisualizationConfig,
    ) -> Result<(), GraphinaError> {
        use plotters::prelude::*;

        let positions = LayoutEngine::compute_layout(
            self,
            config.layout,
            config.width as f64,
            config.height as f64,
        );

        let root =
            SVGBackend::new(path.as_ref(), (config.width, config.height)).into_drawing_area();

        root.fill(&WHITE)
            .map_err(|e| GraphinaError::AlgorithmError(e.to_string()))?;

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .build_cartesian_2d(0.0..config.width as f64, 0.0..config.height as f64)
            .map_err(|e| GraphinaError::AlgorithmError(e.to_string()))?;

        // Draw edges
        for (src, tgt, _weight) in self.edges() {
            if let (Some(pos_src), Some(pos_tgt)) = (positions.get(&src), positions.get(&tgt)) {
                chart
                    .draw_series(std::iter::once(PathElement::new(
                        vec![(pos_src.x, pos_src.y), (pos_tgt.x, pos_tgt.y)],
                        ShapeStyle::from(&RGBColor(150, 150, 150))
                            .stroke_width(config.edge_width as u32),
                    )))
                    .map_err(|e| GraphinaError::AlgorithmError(e.to_string()))?;
            }
        }

        // Draw nodes
        for (node_id, attr) in self.nodes() {
            if let Some(pos) = positions.get(&node_id) {
                chart
                    .draw_series(std::iter::once(Circle::new(
                        (pos.x, pos.y),
                        config.node_size as i32,
                        ShapeStyle::from(&RGBColor(105, 179, 162)).filled(),
                    )))
                    .map_err(|e| GraphinaError::AlgorithmError(e.to_string()))?;

                // Draw label if enabled
                if config.show_labels {
                    chart
                        .draw_series(std::iter::once(Text::new(
                            format!("{}", attr),
                            (pos.x + config.node_size + 2.0, pos.y),
                            ("sans-serif", config.font_size).into_font(),
                        )))
                        .map_err(|e| GraphinaError::AlgorithmError(e.to_string()))?;
                }
            }
        }

        root.present()
            .map_err(|e| GraphinaError::AlgorithmError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn ascii_art_edge_format_is_correct() {
        let mut g = Graph::<&str, f64>::new();
        let a = g.add_node("A");
        let b = g.add_node("B");
        g.add_edge(a, b, 1.0);

        let ascii = g.to_ascii_art();
        // For undirected graphs by default, arrow is "--"
        assert!(ascii.contains(&format!("[{}] -- [{}]", a.index(), b.index())));
    }
}
