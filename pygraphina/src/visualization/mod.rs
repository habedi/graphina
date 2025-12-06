use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::PyGraph;
use graphina::visualization::layout::{LayoutAlgorithm, LayoutEngine};

/// Compute node positions for graph visualization.
///
/// Args:
///     graph: Input graph
///     algorithm: Layout algorithm name. Options: "force_directed", "circular",
///                "hierarchical", "grid", "random"
///     width: Canvas width (default 800)
///     height: Canvas height (default 600)
///
/// Returns:
///     dict: Mapping of node ID to (x, y) position tuple
///
/// Example:
///     >>> g = pygraphina.PyGraph()
///     >>> n0 = g.add_node(0)
///     >>> n1 = g.add_node(1)
///     >>> g.add_edge(n0, n1, 1.0)
///     >>> positions = pygraphina.visualization.compute_layout(g, "circular", 800, 600)
///     >>> print(positions[n0])  # (x, y) tuple
#[pyfunction]
#[pyo3(signature = (graph, algorithm="force_directed", width=800.0, height=600.0))]
pub fn compute_layout(
    graph: &PyGraph,
    algorithm: &str,
    width: f64,
    height: f64,
) -> PyResult<HashMap<usize, (f64, f64)>> {
    let layout_algo = match algorithm.to_lowercase().as_str() {
        "force_directed" | "force-directed" | "forcedirected" => LayoutAlgorithm::ForceDirected,
        "circular" | "circle" => LayoutAlgorithm::Circular,
        "hierarchical" | "tree" => LayoutAlgorithm::Hierarchical,
        "grid" => LayoutAlgorithm::Grid,
        "random" => LayoutAlgorithm::Random,
        _ => {
            return Err(PyValueError::new_err(format!(
                "Unknown layout algorithm: '{}'. Valid options: force_directed, circular, hierarchical, grid, random",
                algorithm
            )));
        }
    };

    let positions = LayoutEngine::compute_layout(&graph.graph, layout_algo, width, height);

    let mut out = HashMap::new();
    for (nid, pos) in positions.into_iter() {
        if let Some(&pyid) = graph.internal_to_py.get(&nid) {
            out.insert(pyid, (pos.x, pos.y));
        }
    }
    Ok(out)
}

/// Export graph to D3.js-compatible JSON format.
///
/// Args:
///     graph: Input graph
///
/// Returns:
///     str: JSON string compatible with D3.js force-directed graphs
///
/// Example:
///     >>> g = pygraphina.PyGraph()
///     >>> n0 = g.add_node(0)
///     >>> n1 = g.add_node(1)
///     >>> g.add_edge(n0, n1, 1.0)
///     >>> json_str = pygraphina.visualization.to_d3_json(g)
#[pyfunction]
pub fn to_d3_json(graph: &PyGraph) -> PyResult<String> {
    graph
        .graph
        .to_d3_json()
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Generate ASCII art representation of the graph.
///
/// Useful for quick debugging and terminal visualization.
///
/// Args:
///     graph: Input graph
///
/// Returns:
///     str: ASCII art showing nodes, edges, and adjacency matrix (for small graphs)
///
/// Example:
///     >>> g = pygraphina.PyGraph()
///     >>> n0 = g.add_node(0)
///     >>> n1 = g.add_node(1)
///     >>> g.add_edge(n0, n1, 1.0)
///     >>> print(pygraphina.visualization.to_ascii_art(g))
#[pyfunction]
pub fn to_ascii_art(graph: &PyGraph) -> String {
    graph.graph.to_ascii_art()
}

/// Save graph as interactive HTML file with D3.js visualization.
///
/// Creates a standalone HTML file with zoomable, draggable nodes.
///
/// Args:
///     graph: Input graph
///     path: Output file path
///     layout: Layout algorithm (default "force_directed")
///     width: Canvas width (default 800)
///     height: Canvas height (default 600)
///     show_labels: Whether to show node labels (default True)
///
/// Example:
///     >>> g = pygraphina.PyGraph()
///     >>> n0 = g.add_node(0)
///     >>> n1 = g.add_node(1)
///     >>> g.add_edge(n0, n1, 1.0)
///     >>> pygraphina.visualization.save_as_html(g, "graph.html")
#[pyfunction]
#[pyo3(signature = (graph, path, layout="force_directed", width=800, height=600, show_labels=true))]
pub fn save_as_html(
    graph: &PyGraph,
    path: &str,
    layout: &str,
    width: u32,
    height: u32,
    show_labels: bool,
) -> PyResult<()> {
    use graphina::visualization::config::VisualizationConfig;

    let layout_algo = match layout.to_lowercase().as_str() {
        "force_directed" | "force-directed" | "forcedirected" => LayoutAlgorithm::ForceDirected,
        "circular" | "circle" => LayoutAlgorithm::Circular,
        "hierarchical" | "tree" => LayoutAlgorithm::Hierarchical,
        "grid" => LayoutAlgorithm::Grid,
        "random" => LayoutAlgorithm::Random,
        _ => LayoutAlgorithm::ForceDirected,
    };

    let config = VisualizationConfig {
        layout: layout_algo,
        width,
        height,
        show_labels,
        ..Default::default()
    };

    graph
        .graph
        .save_as_html(path, &config)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

pub fn register_visualization(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_layout, m)?)?;
    m.add_function(wrap_pyfunction!(to_d3_json, m)?)?;
    m.add_function(wrap_pyfunction!(to_ascii_art, m)?)?;
    m.add_function(wrap_pyfunction!(save_as_html, m)?)?;
    Ok(())
}
