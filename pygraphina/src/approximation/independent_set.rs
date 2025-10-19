use crate::PyGraph;
use graphina::approximation::independent_set::maximum_independent_set as maximum_independent_set_core;
use pyo3::prelude::*;

#[pyfunction]
pub fn maximum_independent_set(py_graph: &PyGraph) -> Vec<usize> {
    let mis = maximum_independent_set_core(&py_graph.graph);
    mis.into_iter()
        .filter_map(|node_id| py_graph.internal_to_py.get(&node_id).copied())
        .collect()
}

pub fn register_independent_set(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(maximum_independent_set, m)?)?;
    Ok(())
}
