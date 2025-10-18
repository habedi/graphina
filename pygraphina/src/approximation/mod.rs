use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashSet;

use crate::PyGraph;
use crate::centrality::utils::to_ordered_graph;
use graphina::approximation::clique::{
    clique_removal as clique_removal_core, large_clique_size as large_clique_size_core,
    max_clique as max_clique_core,
};
use graphina::approximation::diameter::diameter as diameter_core;
use graphina::approximation::vertex_cover::min_weighted_vertex_cover as min_weighted_vertex_cover_core;
use graphina::core::types::NodeId;

#[pyfunction]
pub fn max_clique(py_graph: &PyGraph) -> PyResult<Vec<usize>> {
    let clique: HashSet<NodeId> = max_clique_core(&py_graph.graph);
    let mut out: Vec<usize> = Vec::with_capacity(clique.len());
    for nid in clique.into_iter() {
        let pyid = py_graph
            .internal_to_py
            .get(&nid)
            .ok_or_else(|| PyValueError::new_err("Internal node id missing mapping"))?;
        out.push(*pyid);
    }
    Ok(out)
}

#[pyfunction]
pub fn clique_removal(py_graph: &PyGraph) -> PyResult<Vec<Vec<usize>>> {
    let parts = clique_removal_core(&py_graph.graph);
    let mut out: Vec<Vec<usize>> = Vec::with_capacity(parts.len());
    for set in parts.into_iter() {
        let mut group: Vec<usize> = Vec::with_capacity(set.len());
        for nid in set.into_iter() {
            let pyid = py_graph
                .internal_to_py
                .get(&nid)
                .ok_or_else(|| PyValueError::new_err("Internal node id missing mapping"))?;
            group.push(*pyid);
        }
        out.push(group);
    }
    Ok(out)
}

#[pyfunction]
pub fn large_clique_size(py_graph: &PyGraph) -> PyResult<usize> {
    Ok(large_clique_size_core(&py_graph.graph))
}

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

#[pyfunction]
pub fn diameter(py_graph: &PyGraph) -> PyResult<f64> {
    let (og, _map) = to_ordered_graph(py_graph);
    let d =
        diameter_core(&og).map_err(|e| PyValueError::new_err(format!("diameter failed: {}", e)))?;
    Ok(d)
}

pub fn register_approximation(m: &pyo3::prelude::Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(max_clique, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(clique_removal, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(large_clique_size, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(min_weighted_vertex_cover, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(diameter, m)?)?;
    Ok(())
}
