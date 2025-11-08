/*!
D3.js export, HTML generation, and ASCII visualization
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

use super::config::VisualizationConfig;
use super::layout::{LayoutEngine, NodePosition};

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

        let html = generate_html_template(config, &graph_json);

        let mut file = File::create(path.as_ref()).map_err(GraphinaError::from)?;
        file.write_all(html.as_bytes())
            .map_err(GraphinaError::from)?;

        Ok(())
    }

    /// Save graph as PNG image using plotters
    #[cfg(feature = "visualization")]
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
    #[cfg(feature = "visualization")]
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

/// Generate HTML template for interactive visualization (helper function)
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

#[cfg(test)]
mod tests {
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
