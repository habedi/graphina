use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::PyGraph;
use crate::centrality::utils::to_ordered_graph;
use graphina::centrality::betweenness::{betweenness_centrality, edge_betweenness_centrality};
use graphina::core::types::NodeId;

#[pyfunction]
pub fn betweenness(py_graph: &PyGraph, normalized: bool) -> PyResult<HashMap<usize, f64>> {
    let (og, old_to_new) = to_ordered_graph(py_graph);
    let mut new_to_old: std::collections::HashMap<NodeId, NodeId> =
        std::collections::HashMap::new();
    for (old, new) in old_to_new.iter() {
        new_to_old.insert(*new, *old);
    }

    match betweenness_centrality(&og, normalized) {
        Ok(map) => {
            let mut out = HashMap::new();
            for (new_nid, val) in map.into_iter() {
                let old_nid = new_to_old.get(&new_nid).ok_or_else(|| {
                    PyValueError::new_err("missing mapping back to original node")
                })?;
                let pyid = py_graph
                    .internal_to_py
                    .get(old_nid)
                    .ok_or_else(|| PyValueError::new_err("Internal node id missing mapping"))?;
                out.insert(*pyid, val);
            }
            Ok(out)
        }
        Err(e) => Err(PyValueError::new_err(format!("betweenness failed: {}", e))),
    }
}

#[pyfunction]
pub fn edge_betweenness(
    py_graph: &PyGraph,
    normalized: bool,
) -> PyResult<HashMap<(usize, usize), f64>> {
    let (og, old_to_new) = to_ordered_graph(py_graph);
    let mut new_to_old: std::collections::HashMap<NodeId, NodeId> =
        std::collections::HashMap::new();
    for (old, new) in old_to_new.iter() {
        new_to_old.insert(*new, *old);
    }

    match edge_betweenness_centrality(&og, normalized) {
        Ok(map) => {
            let mut out = HashMap::new();
            for ((nu, nv), val) in map.into_iter() {
                let ou = new_to_old.get(&nu).ok_or_else(|| {
                    PyValueError::new_err("missing mapping back to original node u")
                })?;
                let ov = new_to_old.get(&nv).ok_or_else(|| {
                    PyValueError::new_err("missing mapping back to original node v")
                })?;
                let pu = py_graph
                    .internal_to_py
                    .get(ou)
                    .ok_or_else(|| PyValueError::new_err("missing node mapping for u"))?;
                let pv = py_graph
                    .internal_to_py
                    .get(ov)
                    .ok_or_else(|| PyValueError::new_err("missing node mapping for v"))?;
                out.insert((*pu, *pv), val);
            }
            Ok(out)
        }
        Err(e) => Err(PyValueError::new_err(format!(
            "edge_betweenness failed: {}",
            e
        ))),
    }
}

pub fn register_betweenness(m: &pyo3::prelude::Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(betweenness, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(edge_betweenness, m)?)?;
    Ok(())
}
