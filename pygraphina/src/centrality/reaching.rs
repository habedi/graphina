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
/// distance : float
///     Distance threshold.
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
/// f64
///     The global reaching centrality score. (Wait, core returns HashMap used in existing code?)
///     Existing code filtered map. So it returns dict?
///     Standard global reaching centrality is a single scalar.
///     Let's check existing return type: HashMap<usize, f64>.
///     NetworkX global reaching is a float.
///     But graphina core `global_reaching_centrality` return type?
///     The existing code iterates and maps. So it returns node scores?
///     Maybe it's generalized reaching centrality per node?
///     NetworkX: `local_reaching_centrality` (node score), `global_reaching_centrality` (single float).
///     The existing binding returns `HashMap<usize, f64>`.
///     So maybe core implements node-level metric.
///     I will document return as dict.
///
/// Returns
/// -------
/// dict
///     Dictionary mapping node IDs to scores.
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
