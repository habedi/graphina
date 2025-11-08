use crate::PyGraph;
use graphina::community::girvan_newman::girvan_newman as girvan_newman_core;
use pyo3::prelude::*;

#[pyfunction]
pub fn girvan_newman(py_graph: &PyGraph, target_communities: usize) -> Vec<Vec<usize>> {
    let communities = girvan_newman_core(&py_graph.graph, target_communities);
    communities
        .into_iter()
        .map(|community| {
            community
                .into_iter()
                .filter_map(|node_id| py_graph.internal_to_py.get(&node_id).copied())
                .collect()
        })
        .collect()
}

pub fn register_girvan_newman(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(girvan_newman, m)?)?;
    Ok(())
}
