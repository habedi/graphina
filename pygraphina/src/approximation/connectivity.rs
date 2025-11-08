use crate::PyGraph;
use crate::centrality::utils::to_ordered_graph;
use graphina::approximation::connectivity::local_node_connectivity as local_node_connectivity_core;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
pub fn local_node_connectivity(
    py_graph: &PyGraph,
    source: usize,
    target: usize,
) -> PyResult<usize> {
    let source_id = *py_graph
        .py_to_internal
        .get(&source)
        .ok_or_else(|| PyValueError::new_err("Invalid source node"))?;
    let target_id = *py_graph
        .py_to_internal
        .get(&target)
        .ok_or_else(|| PyValueError::new_err("Invalid target node"))?;

    let (og, node_map) = to_ordered_graph(py_graph);
    let mapped_source = node_map
        .get(&source_id)
        .ok_or_else(|| PyValueError::new_err("Source node not in graph"))?;
    let mapped_target = node_map
        .get(&target_id)
        .ok_or_else(|| PyValueError::new_err("Target node not in graph"))?;

    Ok(local_node_connectivity_core(
        &og,
        *mapped_source,
        *mapped_target,
    ))
}

pub fn register_connectivity(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(local_node_connectivity, m)?)?;
    Ok(())
}
