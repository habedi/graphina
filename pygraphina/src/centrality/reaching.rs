use crate::{PyDiGraph, PyGraph};
use graphina::centrality::other::{
    global_reaching_centrality as global_reaching_centrality_core,
    local_reaching_centrality as local_reaching_centrality_core,
};
use pyo3::prelude::*;
use std::collections::HashMap;

/// Compute local reaching centrality for nodes/graph.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
/// distance : int
///     Maximum number of hops to consider.
///
/// Returns
/// -------
/// dict
///     Dictionary mapping node IDs to local reaching centrality scores.
///
/// Raises
/// ------
/// GraphinaError
///     If the algorithm fails.
/// TypeError
///     If graph is not PyGraph or PyDiGraph.
#[pyfunction]
#[pyo3(signature = (graph, distance))]
pub fn local_reaching_centrality(
    graph: &Bound<'_, PyAny>,
    distance: usize,
) -> PyResult<HashMap<usize, f64>> {
    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        let centrality =
            local_reaching_centrality_core(&py_graph.graph, distance).map_err(|e| {
                crate::GraphinaError::new_err(format!(
                    "Failed to compute local reaching centrality: {}",
                    e
                ))
            })?;
        Ok(centrality
            .into_iter()
            .filter_map(|(node_id, score)| {
                py_graph
                    .mapper
                    .internal_to_py
                    .get(&node_id)
                    .map(|&py_id| (py_id, score))
            })
            .collect())
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        let centrality =
            local_reaching_centrality_core(&py_graph.graph, distance).map_err(|e| {
                crate::GraphinaError::new_err(format!(
                    "Failed to compute local reaching centrality: {}",
                    e
                ))
            })?;
        Ok(centrality
            .into_iter()
            .filter_map(|(node_id, score)| {
                py_graph
                    .mapper
                    .internal_to_py
                    .get(&node_id)
                    .map(|&py_id| (py_id, score))
            })
            .collect())
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
}

/// Compute global reaching centrality for the graph.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
///
/// Returns
/// -------
/// dict
///     Dictionary mapping node IDs to global reaching centrality scores,
///     equivalent to local reaching centrality with an unbounded distance.
///
/// Raises
/// ------
/// GraphinaError
///     If the algorithm fails.
/// TypeError
///     If graph is not PyGraph or PyDiGraph.
#[pyfunction]
pub fn global_reaching_centrality(graph: &Bound<'_, PyAny>) -> PyResult<HashMap<usize, f64>> {
    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        let centrality = global_reaching_centrality_core(&py_graph.graph).map_err(|e| {
            crate::GraphinaError::new_err(format!(
                "Failed to compute global reaching centrality: {}",
                e
            ))
        })?;
        Ok(centrality
            .into_iter()
            .filter_map(|(node_id, score)| {
                py_graph
                    .mapper
                    .internal_to_py
                    .get(&node_id)
                    .map(|&py_id| (py_id, score))
            })
            .collect())
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        let centrality = global_reaching_centrality_core(&py_graph.graph).map_err(|e| {
            crate::GraphinaError::new_err(format!(
                "Failed to compute global reaching centrality: {}",
                e
            ))
        })?;
        Ok(centrality
            .into_iter()
            .filter_map(|(node_id, score)| {
                py_graph
                    .mapper
                    .internal_to_py
                    .get(&node_id)
                    .map(|&py_id| (py_id, score))
            })
            .collect())
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
}

pub fn register_reaching_centrality(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(local_reaching_centrality, m)?)?;
    m.add_function(wrap_pyfunction!(global_reaching_centrality, m)?)?;
    Ok(())
}
