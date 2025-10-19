use crate::PyGraph;
use graphina::community::louvain::louvain as louvain_core;
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(signature = (py_graph, seed=None))]
pub fn louvain(py_graph: &PyGraph, seed: Option<u64>) -> Vec<Vec<usize>> {
    let communities = louvain_core(&py_graph.graph, seed);
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

pub fn register_louvain(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(louvain, m)?)?;
    Ok(())
}
