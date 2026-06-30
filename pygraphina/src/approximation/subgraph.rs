use crate::PyGraph;
use graphina::approximation::subgraph::densest_subgraph as densest_subgraph_core;
use pyo3::prelude::*;

#[pyfunction]
pub fn densest_subgraph(py_graph: &PyGraph) -> Vec<usize> {
    let subgraph = densest_subgraph_core(&py_graph.graph);
    subgraph
        .into_iter()
        .filter_map(|node_id| py_graph.mapper.internal_to_py.get(&node_id).copied())
        .collect()
}

pub fn register_subgraph(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(densest_subgraph, m)?)?;
    Ok(())
}
