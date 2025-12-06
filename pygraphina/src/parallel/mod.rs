use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use graphina::parallel::{
    bfs_parallel as bfs_parallel_core,
    clustering_coefficients_parallel as clustering_coefficients_parallel_core,
    connected_components_parallel as connected_components_parallel_core,
    degrees_parallel as degrees_parallel_core, pagerank_parallel as pagerank_parallel_core,
    shortest_paths_parallel as shortest_paths_parallel_core,
    triangles_parallel as triangles_parallel_core,
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

/// Compute PageRank scores in parallel.
///
/// Uses multi-threaded computation for faster PageRank on large graphs.
///
/// Args:
///     graph: Input graph
///     damping: Damping factor (typically 0.85)
///     max_iterations: Maximum number of iterations
///     tolerance: Convergence threshold
///
/// Returns:
///     dict: Mapping of node ID to PageRank score
///
/// Example:
///     >>> g = pygraphina.PyGraph()
///     >>> n0 = g.add_node(0)
///     >>> n1 = g.add_node(1)
///     >>> n2 = g.add_node(2)
///     >>> g.add_edge(n0, n1, 1.0)
///     >>> g.add_edge(n1, n2, 1.0)
///     >>> g.add_edge(n2, n0, 1.0)
///     >>> scores = pygraphina.parallel.pagerank_parallel(g, 0.85, 100, 1e-6)
#[pyfunction]
pub fn pagerank_parallel(
    graph: &PyGraph,
    damping: f64,
    max_iterations: usize,
    tolerance: f64,
) -> PyResult<HashMap<usize, f64>> {
    let ranks = pagerank_parallel_core(&graph.graph, damping, max_iterations, tolerance);

    let py_ranks: HashMap<usize, f64> = ranks
        .into_iter()
        .filter_map(|(nid, rank)| {
            let py_id = graph.internal_to_py.get(&nid).copied()?;
            Some((py_id, rank))
        })
        .collect();

    Ok(py_ranks)
}

/// Count triangles per node in parallel.
///
/// Args:
///     graph: Input graph
///
/// Returns:
///     dict: Mapping of node ID to triangle count
///
/// Example:
///     >>> g = pygraphina.PyGraph()
///     >>> n0, n1, n2 = g.add_node(0), g.add_node(1), g.add_node(2)
///     >>> g.add_edge(n0, n1, 1.0)
///     >>> g.add_edge(n1, n2, 1.0)
///     >>> g.add_edge(n2, n0, 1.0)
///     >>> triangles = pygraphina.parallel.triangles_parallel(g)
#[pyfunction]
pub fn triangles_parallel(graph: &PyGraph) -> PyResult<HashMap<usize, usize>> {
    let triangles = triangles_parallel_core(&graph.graph);

    let py_triangles: HashMap<usize, usize> = triangles
        .into_iter()
        .filter_map(|(nid, count)| {
            let py_id = graph.internal_to_py.get(&nid).copied()?;
            Some((py_id, count))
        })
        .collect();

    Ok(py_triangles)
}

/// Compute clustering coefficients for all nodes in parallel.
///
/// Args:
///     graph: Input graph
///
/// Returns:
///     dict: Mapping of node ID to clustering coefficient
///
/// Example:
///     >>> g = pygraphina.PyGraph()
///     >>> n0, n1, n2 = g.add_node(0), g.add_node(1), g.add_node(2)
///     >>> g.add_edge(n0, n1, 1.0)
///     >>> g.add_edge(n1, n2, 1.0)
///     >>> g.add_edge(n2, n0, 1.0)
///     >>> coeffs = pygraphina.parallel.clustering_coefficients_parallel(g)
#[pyfunction]
pub fn clustering_coefficients_parallel(graph: &PyGraph) -> PyResult<HashMap<usize, f64>> {
    let coeffs = clustering_coefficients_parallel_core(&graph.graph);

    let py_coeffs: HashMap<usize, f64> = coeffs
        .into_iter()
        .filter_map(|(nid, coeff)| {
            let py_id = graph.internal_to_py.get(&nid).copied()?;
            Some((py_id, coeff))
        })
        .collect();

    Ok(py_coeffs)
}

/// Compute shortest paths from multiple sources in parallel.
///
/// Computes BFS-based shortest path distances (hop counts) from multiple source nodes.
///
/// Args:
///     graph: Input graph
///     sources: List of source node IDs
///
/// Returns:
///     list: List of dicts, one per source, mapping target node ID to distance (hop count)
///
/// Example:
///     >>> g = pygraphina.PyGraph()
///     >>> n0, n1, n2 = g.add_node(0), g.add_node(1), g.add_node(2)
///     >>> g.add_edge(n0, n1, 1.0)
///     >>> g.add_edge(n1, n2, 1.0)
///     >>> paths = pygraphina.parallel.shortest_paths_parallel(g, [n0, n2])
///     >>> # paths[0] contains distances from n0, paths[1] from n2
#[pyfunction]
pub fn shortest_paths_parallel(
    graph: &PyGraph,
    sources: Vec<usize>,
) -> PyResult<Vec<HashMap<usize, usize>>> {
    let internal_sources: Vec<_> = sources
        .iter()
        .filter_map(|&py_id| graph.py_to_internal.get(&py_id).copied())
        .collect();

    if internal_sources.len() != sources.len() {
        return Err(PyValueError::new_err("Invalid node IDs in sources"));
    }

    let paths = shortest_paths_parallel_core(&graph.graph, &internal_sources);

    let py_paths: Vec<HashMap<usize, usize>> = paths
        .into_iter()
        .map(|dists| {
            dists
                .into_iter()
                .filter_map(|(tgt_nid, dist)| {
                    let py_tgt = graph.internal_to_py.get(&tgt_nid).copied()?;
                    Some((py_tgt, dist))
                })
                .collect()
        })
        .collect();

    Ok(py_paths)
}

pub fn register_parallel(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bfs_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(degrees_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(connected_components_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(pagerank_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(triangles_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(clustering_coefficients_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(shortest_paths_parallel, m)?)?;
    Ok(())
}
