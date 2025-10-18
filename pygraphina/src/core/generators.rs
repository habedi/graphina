use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use graphina::core::generators::{
    barabasi_albert_graph, bipartite_graph, complete_graph as complete_graph_core,
    cycle_graph as cycle_graph_core, erdos_renyi_graph, star_graph as star_graph_core,
    watts_strogatz_graph,
};
use graphina::core::types::GraphMarker;

use crate::PyGraph;

/// Generate an Erdős-Rényi random graph (undirected only).
///
/// Args:
///     n: Number of nodes
///     p: Probability of edge creation (0.0 to 1.0)
///     seed: Random seed for reproducibility
///
/// Returns:
///     PyGraph: The generated random graph
///
/// Example:
///     >>> g = pygraphina.erdos_renyi(100, 0.1, 42)
#[pyfunction]
pub fn erdos_renyi(n: usize, p: f64, seed: u64) -> PyResult<PyGraph> {
    let result = erdos_renyi_graph::<GraphMarker>(n, p, seed);

    match result {
        Ok(graph) => {
            let mut py_graph = PyGraph::new();
            py_graph.populate_from_internal(graph);
            Ok(py_graph)
        }
        Err(e) => Err(PyValueError::new_err(format!(
            "Failed to generate graph: {}",
            e
        ))),
    }
}

/// Generate a complete graph where all nodes are connected (undirected only).
///
/// Args:
///     n: Number of nodes
///
/// Returns:
///     PyGraph: The complete graph
///
/// Example:
///     >>> g = pygraphina.complete_graph(10)
#[pyfunction]
pub fn complete_graph(n: usize) -> PyResult<PyGraph> {
    let result = complete_graph_core::<GraphMarker>(n);

    match result {
        Ok(graph) => {
            let mut py_graph = PyGraph::new();
            py_graph.populate_from_internal(graph);
            Ok(py_graph)
        }
        Err(e) => Err(PyValueError::new_err(format!(
            "Failed to generate graph: {}",
            e
        ))),
    }
}

/// Generate a bipartite graph (undirected only).
///
/// Args:
///     n1: Number of nodes in first partition
///     n2: Number of nodes in second partition
///     p: Probability of edge creation between partitions
///     seed: Random seed for reproducibility
///
/// Returns:
///     PyGraph: The generated bipartite graph
///
/// Example:
///     >>> g = pygraphina.bipartite(10, 15, 0.3, 42)
#[pyfunction]
pub fn bipartite(n1: usize, n2: usize, p: f64, seed: u64) -> PyResult<PyGraph> {
    let result = bipartite_graph::<GraphMarker>(n1, n2, p, seed);

    match result {
        Ok(graph) => {
            let mut py_graph = PyGraph::new();
            py_graph.populate_from_internal(graph);
            Ok(py_graph)
        }
        Err(e) => Err(PyValueError::new_err(format!(
            "Failed to generate graph: {}",
            e
        ))),
    }
}

/// Generate a star graph with one central node connected to all others (undirected only).
///
/// Args:
///     n: Total number of nodes (including center)
///
/// Returns:
///     PyGraph: The star graph
///
/// Example:
///     >>> g = pygraphina.star_graph(10)
#[pyfunction]
pub fn star_graph(n: usize) -> PyResult<PyGraph> {
    let result = star_graph_core::<GraphMarker>(n);

    match result {
        Ok(graph) => {
            let mut py_graph = PyGraph::new();
            py_graph.populate_from_internal(graph);
            Ok(py_graph)
        }
        Err(e) => Err(PyValueError::new_err(format!(
            "Failed to generate graph: {}",
            e
        ))),
    }
}

/// Generate a cycle graph where nodes form a ring (undirected only).
///
/// Args:
///     n: Number of nodes (must be >= 3)
///
/// Returns:
///     PyGraph: The cycle graph
///
/// Example:
///     >>> g = pygraphina.cycle_graph(10)
#[pyfunction]
pub fn cycle_graph(n: usize) -> PyResult<PyGraph> {
    let result = cycle_graph_core::<GraphMarker>(n);

    match result {
        Ok(graph) => {
            let mut py_graph = PyGraph::new();
            py_graph.populate_from_internal(graph);
            Ok(py_graph)
        }
        Err(e) => Err(PyValueError::new_err(format!(
            "Failed to generate graph: {}",
            e
        ))),
    }
}

/// Generate a Watts-Strogatz small-world graph (undirected only).
///
/// Args:
///     n: Number of nodes
///     k: Each node is connected to k nearest neighbors (must be even)
///     beta: Rewiring probability (0.0 to 1.0)
///     seed: Random seed for reproducibility
///
/// Returns:
///     PyGraph: The small-world graph
///
/// Example:
///     >>> g = pygraphina.watts_strogatz(100, 6, 0.3, 42)
#[pyfunction]
pub fn watts_strogatz(n: usize, k: usize, beta: f64, seed: u64) -> PyResult<PyGraph> {
    let result = watts_strogatz_graph::<GraphMarker>(n, k, beta, seed);

    match result {
        Ok(graph) => {
            let mut py_graph = PyGraph::new();
            py_graph.populate_from_internal(graph);
            Ok(py_graph)
        }
        Err(e) => Err(PyValueError::new_err(format!(
            "Failed to generate graph: {}",
            e
        ))),
    }
}

/// Generate a Barabási-Albert scale-free graph using preferential attachment (undirected only).
///
/// Args:
///     n: Number of nodes
///     m: Number of edges to attach from new node to existing nodes
///     seed: Random seed for reproducibility
///
/// Returns:
///     PyGraph: The scale-free graph
///
/// Example:
///     >>> g = pygraphina.barabasi_albert(100, 3, 42)
#[pyfunction]
pub fn barabasi_albert(n: usize, m: usize, seed: u64) -> PyResult<PyGraph> {
    let result = barabasi_albert_graph::<GraphMarker>(n, m, seed);

    match result {
        Ok(graph) => {
            let mut py_graph = PyGraph::new();
            py_graph.populate_from_internal(graph);
            Ok(py_graph)
        }
        Err(e) => Err(PyValueError::new_err(format!(
            "Failed to generate graph: {}",
            e
        ))),
    }
}

pub fn register_generators(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(erdos_renyi, m)?)?;
    m.add_function(wrap_pyfunction!(complete_graph, m)?)?;
    m.add_function(wrap_pyfunction!(bipartite, m)?)?;
    m.add_function(wrap_pyfunction!(star_graph, m)?)?;
    m.add_function(wrap_pyfunction!(cycle_graph, m)?)?;
    m.add_function(wrap_pyfunction!(watts_strogatz, m)?)?;
    m.add_function(wrap_pyfunction!(barabasi_albert, m)?)?;
    Ok(())
}
