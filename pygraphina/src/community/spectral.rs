use crate::PyGraph;
use graphina::community::spectral::spectral_clustering as spectral_clustering_core;
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(signature = (py_graph, k, seed=None))]
pub fn spectral_clustering(py_graph: &PyGraph, k: usize, seed: Option<u64>) -> Vec<Vec<usize>> {
    let communities = spectral_clustering_core(&py_graph.graph, k, seed);
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

pub fn register_spectral(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(spectral_clustering, m)?)?;
    Ok(())
}
