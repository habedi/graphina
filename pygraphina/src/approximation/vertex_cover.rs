use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashSet;

use crate::PyGraph;
use graphina::approximation::vertex_cover::min_weighted_vertex_cover as min_weighted_vertex_cover_core;
use graphina::core::types::NodeId;

#[pyfunction]
pub fn min_weighted_vertex_cover(py_graph: &PyGraph) -> PyResult<Vec<usize>> {
    let cover: HashSet<NodeId> = min_weighted_vertex_cover_core(&py_graph.graph, None);
    let mut out: Vec<usize> = Vec::with_capacity(cover.len());
    for nid in cover.into_iter() {
        let pyid = py_graph
            .internal_to_py
            .get(&nid)
            .ok_or_else(|| PyValueError::new_err("Internal node id missing mapping"))?;
        out.push(*pyid);
    }
    Ok(out)
}

pub fn register_vertex_cover(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(min_weighted_vertex_cover, m)?)?;
    Ok(())
}
