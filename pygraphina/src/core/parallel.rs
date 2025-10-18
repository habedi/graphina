use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use graphina::core::parallel::{
    bfs_parallel as bfs_parallel_core,
    connected_components_parallel as connected_components_parallel_core,
    degrees_parallel as degrees_parallel_core,
};

use crate::PyGraph;

/// Perform parallel BFS from multiple starting nodes.
///
/// Args:
///     graph: Input graph
///     starts: List of starting node IDs
///
/// Returns:
///     list: List of visited node lists, one for each starting node
///
/// Example:
///     >>> g = pygraphina.PyGraph()
///     >>> n0 = g.add_node(0)
///     >>> n1 = g.add_node(1)
///     >>> g.add_edge(n0, n1, 1.0)
///     >>> results = pygraphina.bfs_parallel(g, [n0, n1])
#[pyfunction]
pub fn bfs_parallel(graph: &PyGraph, starts: Vec<usize>) -> PyResult<Vec<Vec<usize>>> {
    let internal_starts: Vec<_> = starts
        .iter()
        .filter_map(|&py_id| graph.py_to_internal.get(&py_id).copied())
        .collect();

    if internal_starts.len() != starts.len() {
        return Err(PyValueError::new_err("Invalid node IDs in starts"));
    }

    let results = bfs_parallel_core(&graph.graph, &internal_starts);

    let py_results: Vec<Vec<usize>> = results
        .into_iter()
        .map(|visited| {
            visited
                .into_iter()
                .filter_map(|nid| graph.internal_to_py.get(&nid).copied())
                .collect()
        })
        .collect();

    Ok(py_results)
}

/// Compute degrees of all nodes in parallel.
///
/// Args:
///     graph: Input graph
///
/// Returns:
///     dict: Mapping of node ID to degree
///
/// Example:
///     >>> g = pygraphina.PyGraph()
///     >>> n0 = g.add_node(0)
///     >>> n1 = g.add_node(1)
///     >>> g.add_edge(n0, n1, 1.0)
///     >>> degrees = pygraphina.degrees_parallel(g)
#[pyfunction]
pub fn degrees_parallel(graph: &PyGraph) -> PyResult<HashMap<usize, usize>> {
    let internal_degrees = degrees_parallel_core(&graph.graph);

    let py_degrees: HashMap<usize, usize> = internal_degrees
        .into_iter()
        .filter_map(|(nid, deg)| {
            let py_id = graph.internal_to_py.get(&nid).copied()?;
            Some((py_id, deg))
        })
        .collect();

    Ok(py_degrees)
}

/// Find connected components in parallel.
///
/// Args:
///     graph: Input graph
///
/// Returns:
///     dict: Mapping of node ID to component ID
///
/// Example:
///     >>> g = pygraphina.PyGraph()
///     >>> n0 = g.add_node(0)
///     >>> n1 = g.add_node(1)
///     >>> n2 = g.add_node(2)
///     >>> g.add_edge(n0, n1, 1.0)
///     >>> components = pygraphina.connected_components_parallel(g)
#[pyfunction]
pub fn connected_components_parallel(graph: &PyGraph) -> PyResult<HashMap<usize, usize>> {
    let component_map = connected_components_parallel_core(&graph.graph);

    let py_component_map: HashMap<usize, usize> = component_map
        .into_iter()
        .filter_map(|(nid, comp_id)| {
            let py_id = graph.internal_to_py.get(&nid).copied()?;
            Some((py_id, comp_id))
        })
        .collect();

    Ok(py_component_map)
}

pub fn register_parallel(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bfs_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(degrees_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(connected_components_parallel, m)?)?;
    Ok(())
}
