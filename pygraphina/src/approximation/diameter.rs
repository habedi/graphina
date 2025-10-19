use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::PyGraph;
use crate::centrality::utils::to_ordered_graph;
use graphina::approximation::diameter::diameter as diameter_core;

#[pyfunction]
pub fn diameter(py_graph: &PyGraph) -> PyResult<f64> {
    let (og, _map) = to_ordered_graph(py_graph);
    let d =
        diameter_core(&og).map_err(|e| PyValueError::new_err(format!("diameter failed: {}", e)))?;
    Ok(d)
}

pub fn register_diameter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(diameter, m)?)?;
    Ok(())
}
