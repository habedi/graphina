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

use crate::{PyDiGraph, PyGraph};

/// Perform parallel BFS from multiple starting nodes.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
/// starts : list of int
///     List of starting node IDs.
///
/// Returns
/// -------
/// list of list of int
///     List of visited node lists, one for each starting node.
///
/// Raises
/// ------
/// GraphinaError
///     If invalid start nodes are provided.
/// TypeError
///     If graph is not PyGraph or PyDiGraph.
#[pyfunction]
pub fn bfs_parallel(graph: &Bound<'_, PyAny>, starts: Vec<usize>) -> PyResult<Vec<Vec<usize>>> {
    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        let internal_starts: Vec<_> = starts
            .iter()
            .filter_map(|&py_id| py_graph.mapper.py_to_internal.get(&py_id).copied())
            .collect();

        if internal_starts.len() != starts.len() {
            return Err(crate::GraphinaError::new_err("Invalid node IDs in starts"));
        }

        let results = bfs_parallel_core(&py_graph.graph, &internal_starts);

        Ok(results
            .into_iter()
            .map(|visited| {
                visited
                    .into_iter()
                    .filter_map(|nid| py_graph.mapper.internal_to_py.get(&nid).copied())
                    .collect()
            })
            .collect())
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        let internal_starts: Vec<_> = starts
            .iter()
            .filter_map(|&py_id| py_graph.mapper.py_to_internal.get(&py_id).copied())
            .collect();

        if internal_starts.len() != starts.len() {
            return Err(crate::GraphinaError::new_err("Invalid node IDs in starts"));
        }

        let results = bfs_parallel_core(&py_graph.graph, &internal_starts);

        Ok(results
            .into_iter()
            .map(|visited| {
                visited
                    .into_iter()
                    .filter_map(|nid| py_graph.mapper.internal_to_py.get(&nid).copied())
                    .collect()
            })
            .collect())
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
}

/// Compute degrees of all nodes in parallel.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
///
/// Returns
/// -------
/// dict
///     Mapping of node ID to degree.
///
/// Raises
/// ------
/// TypeError
///     If graph is not PyGraph or PyDiGraph.
#[pyfunction]
pub fn degrees_parallel(graph: &Bound<'_, PyAny>) -> PyResult<HashMap<usize, usize>> {
    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        let internal_degrees = degrees_parallel_core(&py_graph.graph);

        let py_degrees: HashMap<usize, usize> = internal_degrees
            .into_iter()
            .filter_map(|(nid, deg)| {
                let py_id = py_graph.mapper.internal_to_py.get(&nid).copied()?;
                Some((py_id, deg))
            })
            .collect();

        Ok(py_degrees)
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        let internal_degrees = degrees_parallel_core(&py_graph.graph);

        let py_degrees: HashMap<usize, usize> = internal_degrees
            .into_iter()
            .filter_map(|(nid, deg)| {
                let py_id = py_graph.mapper.internal_to_py.get(&nid).copied()?;
                Some((py_id, deg))
            })
            .collect();

        Ok(py_degrees)
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
}

/// Find connected components in parallel.
///
/// Parameters
/// ----------
/// graph : PyGraph
///     The input graph.
///
/// Returns
/// -------
/// dict
///     Mapping of node ID to component ID.
///
/// Raises
/// ------
/// TypeError
///     If graph is not PyGraph.
#[pyfunction]
pub fn connected_components_parallel(graph: &PyGraph) -> PyResult<HashMap<usize, usize>> {
    let component_map = connected_components_parallel_core(&graph.graph);

    let py_component_map: HashMap<usize, usize> = component_map
        .into_iter()
        .filter_map(|(nid, comp_id)| {
            let py_id = graph.mapper.internal_to_py.get(&nid).copied()?;
            Some((py_id, comp_id))
        })
        .collect();

    Ok(py_component_map)
}

/// Compute PageRank scores in parallel.
///
/// Uses multi-threaded computation for faster PageRank on large graphs.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
/// damping : float
///     Damping factor (typically 0.85).
/// max_iterations : int
///     Maximum number of iterations.
/// tolerance : float
///     Convergence threshold.
/// nstart : dict, optional
///     Starting value for PageRank iteration for each node.
///
/// Returns
/// -------
/// dict
///     Mapping of node ID to PageRank score.
///
/// Raises
/// ------
/// TypeError
///     If graph is not PyGraph or PyDiGraph.
#[pyfunction]
#[pyo3(signature = (graph, damping=0.85, max_iterations=100, tolerance=1e-6, nstart=None))]
pub fn pagerank_parallel(
    graph: &Bound<'_, PyAny>,
    damping: f64,
    max_iterations: usize,
    tolerance: f64,
    nstart: Option<HashMap<usize, f64>>,
) -> PyResult<HashMap<usize, f64>> {
    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        let nstart_map = if let Some(ns) = nstart {
            let mut map = HashMap::new();
            for (py_id, val) in ns {
                if let Some(&internal_id) = py_graph.mapper.py_to_internal.get(&py_id) {
                    map.insert(internal_id, val);
                }
            }
            Some(map)
        } else {
            None
        };

        let ranks = pagerank_parallel_core(
            &py_graph.graph,
            damping,
            max_iterations,
            tolerance,
            nstart_map.as_ref(),
        );
        let py_ranks: HashMap<usize, f64> = ranks
            .into_iter()
            .filter_map(|(nid, rank)| {
                let py_id = py_graph.mapper.internal_to_py.get(&nid)?;
                Some((*py_id, rank))
            })
            .collect();
        Ok(py_ranks)
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        let nstart_map = if let Some(ns) = nstart {
            let mut map = HashMap::new();
            for (py_id, val) in ns {
                if let Some(&internal_id) = py_graph.mapper.py_to_internal.get(&py_id) {
                    map.insert(internal_id, val);
                }
            }
            Some(map)
        } else {
            None
        };

        let ranks = pagerank_parallel_core(
            &py_graph.graph,
            damping,
            max_iterations,
            tolerance,
            nstart_map.as_ref(),
        );
        let py_ranks: HashMap<usize, f64> = ranks
            .into_iter()
            .filter_map(|(nid, rank)| {
                let py_id = py_graph.mapper.internal_to_py.get(&nid)?;
                Some((*py_id, rank))
            })
            .collect();
        Ok(py_ranks)
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
}

/// Count triangles per node in parallel.
///
/// Parameters
/// ----------
/// graph : PyGraph
///     The input graph.
///
/// Returns
/// -------
/// dict
///     Mapping of node ID to triangle count.
///
/// Raises
/// ------
/// TypeError
///     If graph is not PyGraph.
#[pyfunction]
pub fn triangles_parallel(graph: &PyGraph) -> PyResult<HashMap<usize, usize>> {
    let triangles = triangles_parallel_core(&graph.graph);

    let py_triangles: HashMap<usize, usize> = triangles
        .into_iter()
        .filter_map(|(nid, count)| {
            let py_id = graph.mapper.internal_to_py.get(&nid).copied()?;
            Some((py_id, count))
        })
        .collect();

    Ok(py_triangles)
}

/// Compute clustering coefficients for all nodes in parallel.
///
/// Parameters
/// ----------
/// graph : PyGraph
///     The input graph.
///
/// Returns
/// -------
/// dict
///     Mapping of node ID to clustering coefficient.
///
/// Raises
/// ------
/// TypeError
///     If graph is not PyGraph.
#[pyfunction]
pub fn clustering_coefficients_parallel(graph: &PyGraph) -> PyResult<HashMap<usize, f64>> {
    let coeffs = clustering_coefficients_parallel_core(&graph.graph);

    let py_coeffs: HashMap<usize, f64> = coeffs
        .into_iter()
        .filter_map(|(nid, coeff)| {
            let py_id = graph.mapper.internal_to_py.get(&nid).copied()?;
            Some((py_id, coeff))
        })
        .collect();

    Ok(py_coeffs)
}

/// Compute shortest paths from multiple sources in parallel.
///
/// Computes BFS-based shortest path distances (hop counts) from multiple source nodes.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
/// sources : list of int
///     List of source node IDs.
///
/// Returns
/// -------
/// list of dict
///     List of dicts, one per source, mapping target node ID to distance (hop count).
///
/// Raises
/// ------
/// GraphinaError
///     If invalid source nodes are provided.
/// TypeError
///     If graph is not PyGraph or PyDiGraph.
#[pyfunction]
pub fn shortest_paths_parallel(
    graph: &Bound<'_, PyAny>,
    sources: Vec<usize>,
) -> PyResult<Vec<HashMap<usize, usize>>> {
    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        let internal_sources: Vec<_> = sources
            .iter()
            .filter_map(|&py_id| py_graph.mapper.py_to_internal.get(&py_id).copied())
            .collect();

        if internal_sources.len() != sources.len() {
            return Err(crate::GraphinaError::new_err("Invalid node IDs in sources"));
        }

        let paths = shortest_paths_parallel_core(&py_graph.graph, &internal_sources);

        Ok(paths
            .into_iter()
            .map(|dists| {
                dists
                    .into_iter()
                    .filter_map(|(tgt_nid, dist)| {
                        let py_tgt = py_graph.mapper.internal_to_py.get(&tgt_nid).copied()?;
                        Some((py_tgt, dist))
                    })
                    .collect()
            })
            .collect())
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        let internal_sources: Vec<_> = sources
            .iter()
            .filter_map(|&py_id| py_graph.mapper.py_to_internal.get(&py_id).copied())
            .collect();

        if internal_sources.len() != sources.len() {
            return Err(crate::GraphinaError::new_err("Invalid node IDs in sources"));
        }

        let paths = shortest_paths_parallel_core(&py_graph.graph, &internal_sources);

        Ok(paths
            .into_iter()
            .map(|dists| {
                dists
                    .into_iter()
                    .filter_map(|(tgt_nid, dist)| {
                        let py_tgt = py_graph.mapper.internal_to_py.get(&tgt_nid).copied()?;
                        Some((py_tgt, dist))
                    })
                    .collect()
            })
            .collect())
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
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
