use crate::PyGraph;
use graphina::centrality::other::{
    global_reaching_centrality as global_reaching_centrality_core,
    local_reaching_centrality as local_reaching_centrality_core,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyfunction]
#[pyo3(signature = (py_graph, distance))]
pub fn local_reaching_centrality(
    py_graph: &PyGraph,
    distance: usize,
) -> PyResult<HashMap<usize, f64>> {
    let centrality = local_reaching_centrality_core(&py_graph.graph, distance).map_err(|e| {
        PyValueError::new_err(format!(
            "Failed to compute local reaching centrality: {}",
            e
        ))
    })?;
    Ok(centrality
        .into_iter()
        .filter_map(|(node_id, score)| {
            py_graph
                .internal_to_py
                .get(&node_id)
                .map(|&py_id| (py_id, score))
        })
        .collect())
}

#[pyfunction]
pub fn global_reaching_centrality(py_graph: &PyGraph) -> PyResult<HashMap<usize, f64>> {
    let centrality = global_reaching_centrality_core(&py_graph.graph).map_err(|e| {
        PyValueError::new_err(format!(
            "Failed to compute global reaching centrality: {}",
            e
        ))
    })?;
    Ok(centrality
        .into_iter()
        .filter_map(|(node_id, score)| {
            py_graph
                .internal_to_py
                .get(&node_id)
                .map(|&py_id| (py_id, score))
        })
        .collect())
}

pub fn register_reaching_centrality(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(local_reaching_centrality, m)?)?;
    m.add_function(wrap_pyfunction!(global_reaching_centrality, m)?)?;
    Ok(())
}
