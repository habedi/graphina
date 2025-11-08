use crate::PyGraph;
use graphina::community::label_propagation::label_propagation as label_propagation_core;
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyfunction]
#[pyo3(signature = (py_graph, max_iter, seed=None))]
pub fn label_propagation(
    py_graph: &PyGraph,
    max_iter: usize,
    seed: Option<u64>,
) -> HashMap<usize, usize> {
    let labels = label_propagation_core(&py_graph.graph, max_iter, seed);
    let mut result = HashMap::new();
    for (py_id, node_id) in &py_graph.py_to_internal {
        let idx = node_id.index();
        if idx < labels.len() {
            result.insert(*py_id, labels[idx]);
        }
    }
    result
}

pub fn register_label_propagation(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(label_propagation, m)?)?;
    Ok(())
}
