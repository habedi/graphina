use crate::PyGraph;
use graphina::community::connected_components::connected_components as connected_components_core;
use pyo3::prelude::*;

#[pyfunction]
pub fn connected_components(py_graph: &PyGraph) -> Vec<Vec<usize>> {
    let components = connected_components_core(&py_graph.graph);
    components
        .into_iter()
        .map(|comp| {
            comp.into_iter()
                .filter_map(|node_id| py_graph.internal_to_py.get(&node_id).copied())
                .collect()
        })
        .collect()
}

pub fn register_connected_components(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(connected_components, m)?)?;
    Ok(())
}
