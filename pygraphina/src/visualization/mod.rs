use pyo3::prelude::*;
use std::collections::HashMap;

use crate::{PyDiGraph, PyGraph};
use graphina::visualization::layout::{LayoutAlgorithm, LayoutEngine};

/// Compute node positions for graph visualization.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
/// algorithm : str, optional
///     Layout algorithm name. Options: "force_directed", "circular",
///     "hierarchical", "grid", "random". Default "force_directed".
/// width : float, optional
///     Canvas width (default 800).
/// height : float, optional
///     Canvas height (default 600).
///
/// Returns
/// -------
/// dict
///     Mapping of node ID to (x, y) position tuple.
///
/// Raises
/// ------
/// GraphinaError
///     If algorithm is unknown.
/// TypeError
///     If graph is not PyGraph or PyDiGraph.
#[pyfunction]
#[pyo3(signature = (graph, algorithm="force_directed", width=800.0, height=600.0))]
pub fn compute_layout(
    graph: &Bound<'_, PyAny>,
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
            return Err(crate::GraphinaError::new_err(format!(
                "Unknown layout algorithm: '{}'. Valid options: force_directed, circular, hierarchical, grid, random",
                algorithm
            )));
        }
    };

    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        let positions = LayoutEngine::compute_layout(&py_graph.graph, layout_algo, width, height);

        let mut out = HashMap::new();
        for (nid, pos) in positions.into_iter() {
            if let Some(&pyid) = py_graph.mapper.internal_to_py.get(&nid) {
                out.insert(pyid, (pos.x, pos.y));
            }
        }
        Ok(out)
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        let positions = LayoutEngine::compute_layout(&py_graph.graph, layout_algo, width, height);

        let mut out = HashMap::new();
        for (nid, pos) in positions.into_iter() {
            if let Some(&pyid) = py_graph.mapper.internal_to_py.get(&nid) {
                out.insert(pyid, (pos.x, pos.y));
            }
        }
        Ok(out)
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
}

/// Export graph to D3.js-compatible JSON format.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
///
/// Returns
/// -------
/// str
///     JSON string compatible with D3.js force-directed graphs.
///
/// Raises
/// ------
/// TypeError
///     If graph is not PyGraph or PyDiGraph.
#[pyfunction]
pub fn to_d3_json(graph: &Bound<'_, PyAny>) -> PyResult<String> {
    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        py_graph
            .graph
            .to_d3_json()
            .map_err(|e| crate::GraphinaError::new_err(e.to_string()))
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        py_graph
            .graph
            .to_d3_json()
            .map_err(|e| crate::GraphinaError::new_err(e.to_string()))
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
}

/// Generate ASCII art representation of the graph.
///
/// Useful for quick debugging and terminal visualization.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
///
/// Returns
/// -------
/// str
///     ASCII art showing nodes, edges, and adjacency matrix (for small graphs).
///
/// Raises
/// ------
/// TypeError
///     If graph is not PyGraph or PyDiGraph.
#[pyfunction]
pub fn to_ascii_art(graph: &Bound<'_, PyAny>) -> PyResult<String> {
    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        Ok(py_graph.graph.to_ascii_art())
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        Ok(py_graph.graph.to_ascii_art())
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
}

/// Save graph as interactive HTML file with D3.js visualization.
///
/// Creates a standalone HTML file with zoomable, draggable nodes.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
/// path : str
///     Output file path.
/// layout : str, optional
///     Layout algorithm (default "force_directed").
/// width : int, optional
///     Canvas width (default 800).
/// height : int, optional
///     Canvas height (default 600).
/// show_labels : bool, optional
///     Whether to show node labels (default True).
///
/// Raises
/// ------
/// GraphinaError
///     If saving fails.
/// TypeError
///     If graph is not PyGraph or PyDiGraph.
#[pyfunction]
#[pyo3(signature = (graph, path, layout="force_directed", width=800, height=600, show_labels=true))]
pub fn save_as_html(
    graph: &Bound<'_, PyAny>,
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

    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        py_graph
            .graph
            .save_as_html(path, &config)
            .map_err(|e| crate::GraphinaError::new_err(e.to_string()))
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        py_graph
            .graph
            .save_as_html(path, &config)
            .map_err(|e| crate::GraphinaError::new_err(e.to_string()))
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
}

pub fn register_visualization(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_layout, m)?)?;
    m.add_function(wrap_pyfunction!(to_d3_json, m)?)?;
    m.add_function(wrap_pyfunction!(to_ascii_art, m)?)?;
    m.add_function(wrap_pyfunction!(save_as_html, m)?)?;
    Ok(())
}
