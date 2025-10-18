use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::PyGraph;
use graphina::community::connected_components::connected_components as connected_components_core;
use graphina::community::label_propagation::label_propagation as label_propagation_core;
use graphina::community::louvain::louvain as louvain_core;
use graphina::core::types::NodeId;

#[pyfunction]
pub fn connected_components(py_graph: &PyGraph) -> PyResult<Vec<Vec<usize>>> {
    let comps: Vec<Vec<NodeId>> = connected_components_core(&py_graph.graph);
    let mut out: Vec<Vec<usize>> = Vec::with_capacity(comps.len());
    for comp in comps.into_iter() {
        let mut group: Vec<usize> = Vec::with_capacity(comp.len());
        for nid in comp.into_iter() {
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
pub fn label_propagation(
    py_graph: &PyGraph,
    max_iter: usize,
    seed: Option<u64>,
) -> PyResult<HashMap<usize, usize>> {
    let labels = label_propagation_core(&py_graph.graph, max_iter, seed);
    let n = py_graph.graph.node_count();
    // Map index -> NodeId by index lookup
    let mut result: HashMap<usize, usize> = HashMap::with_capacity(n);
    for (node, _attr) in py_graph.graph.nodes() {
        let idx = node.index();
        let lbl = *labels.get(idx).unwrap_or(&0);
        let pyid = py_graph
            .internal_to_py
            .get(&node)
            .ok_or_else(|| PyValueError::new_err("Internal node id missing mapping"))?;
        result.insert(*pyid, lbl);
    }
    Ok(result)
}

#[pyfunction]
pub fn louvain(py_graph: &PyGraph, seed: Option<u64>) -> PyResult<Vec<Vec<usize>>> {
    let comms = louvain_core(&py_graph.graph, seed);
    let mut out: Vec<Vec<usize>> = Vec::with_capacity(comms.len());
    for group in comms.into_iter() {
        let mut mapped: Vec<usize> = Vec::with_capacity(group.len());
        for nid in group.into_iter() {
            let pyid = py_graph
                .internal_to_py
                .get(&nid)
                .ok_or_else(|| PyValueError::new_err("Internal node id missing mapping"))?;
            mapped.push(*pyid);
        }
        out.push(mapped);
    }
    Ok(out)
}

pub fn register_community(m: &pyo3::prelude::Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(connected_components, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(label_propagation, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(louvain, m)?)?;
    Ok(())
}
