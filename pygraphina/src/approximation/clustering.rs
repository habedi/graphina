use crate::PyGraph;
use graphina::approximation::clustering::average_clustering as average_clustering_core;
use pyo3::prelude::*;

#[pyfunction]
pub fn average_clustering_approx(py_graph: &PyGraph) -> f64 {
    average_clustering_core(&py_graph.graph)
}

pub fn register_clustering(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(average_clustering_approx, m)?)?;
    Ok(())
}
