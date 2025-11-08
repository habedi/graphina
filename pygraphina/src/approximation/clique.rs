use crate::PyGraph;
use graphina::approximation::clique::{
    clique_removal as clique_removal_core, large_clique_size as large_clique_size_core,
    max_clique as max_clique_core,
};
use pyo3::prelude::*;

#[pyfunction]
pub fn max_clique(py_graph: &PyGraph) -> Vec<usize> {
    let clique = max_clique_core(&py_graph.graph);
    clique
        .into_iter()
        .filter_map(|node_id| py_graph.internal_to_py.get(&node_id).copied())
        .collect()
}

#[pyfunction]
pub fn clique_removal(py_graph: &PyGraph) -> Vec<Vec<usize>> {
    let cliques = clique_removal_core(&py_graph.graph);
    cliques
        .into_iter()
        .map(|clique| {
            clique
                .into_iter()
                .filter_map(|node_id| py_graph.internal_to_py.get(&node_id).copied())
                .collect()
        })
        .collect()
}

#[pyfunction]
pub fn large_clique_size(py_graph: &PyGraph) -> usize {
    large_clique_size_core(&py_graph.graph)
}

pub fn register_clique(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(max_clique, m)?)?;
    m.add_function(wrap_pyfunction!(clique_removal, m)?)?;
    m.add_function(wrap_pyfunction!(large_clique_size, m)?)?;
    Ok(())
}
