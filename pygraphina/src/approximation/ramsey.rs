use crate::PyGraph;
use graphina::approximation::ramsey::ramsey_r2 as ramsey_r2_core;
use pyo3::prelude::*;

#[pyfunction]
pub fn ramsey_r2(py_graph: &PyGraph) -> (Vec<usize>, Vec<usize>) {
    let (clique, independent_set) = ramsey_r2_core(&py_graph.graph);
    let py_clique = clique
        .into_iter()
        .filter_map(|node_id| py_graph.internal_to_py.get(&node_id).copied())
        .collect();
    let py_independent = independent_set
        .into_iter()
        .filter_map(|node_id| py_graph.internal_to_py.get(&node_id).copied())
        .collect();
    (py_clique, py_independent)
}

pub fn register_ramsey(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ramsey_r2, m)?)?;
    Ok(())
}
