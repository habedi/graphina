use crate::PyGraph;
use graphina::approximation::treewidth::{
    treewidth_min_degree as treewidth_min_degree_core,
    treewidth_min_fill_in as treewidth_min_fill_in_core,
};
use pyo3::prelude::*;

#[pyfunction]
pub fn treewidth_min_degree(py_graph: &PyGraph) -> (usize, Vec<usize>) {
    let (width, elimination_order) = treewidth_min_degree_core(&py_graph.graph);
    let py_order = elimination_order
        .into_iter()
        .filter_map(|node_id| py_graph.internal_to_py.get(&node_id).copied())
        .collect();
    (width, py_order)
}

#[pyfunction]
pub fn treewidth_min_fill_in(py_graph: &PyGraph) -> (usize, Vec<usize>) {
    let (width, elimination_order) = treewidth_min_fill_in_core(&py_graph.graph);
    let py_order = elimination_order
        .into_iter()
        .filter_map(|node_id| py_graph.internal_to_py.get(&node_id).copied())
        .collect();
    (width, py_order)
}

pub fn register_treewidth(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(treewidth_min_degree, m)?)?;
    m.add_function(wrap_pyfunction!(treewidth_min_fill_in, m)?)?;
    Ok(())
}
