use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::PyGraph;
use graphina::centrality::degree::{
    degree_centrality, in_degree_centrality, out_degree_centrality,
};

#[pyfunction]
pub fn degree(py_graph: &PyGraph) -> PyResult<HashMap<usize, f64>> {
    let res =
        degree_centrality(&py_graph.graph).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let mut out = HashMap::new();
    for (nid, val) in res.into_iter() {
        let pyid = py_graph
            .internal_to_py
            .get(&nid)
            .ok_or_else(|| PyValueError::new_err("Internal node id missing mapping"))?;
        out.insert(*pyid, val);
    }
    Ok(out)
}

#[pyfunction]
pub fn in_degree(py_graph: &PyGraph) -> PyResult<HashMap<usize, f64>> {
    let res =
        in_degree_centrality(&py_graph.graph).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let mut out = HashMap::new();
    for (nid, val) in res.into_iter() {
        let pyid = py_graph
            .internal_to_py
            .get(&nid)
            .ok_or_else(|| PyValueError::new_err("Internal node id missing mapping"))?;
        out.insert(*pyid, val);
    }
    Ok(out)
}

#[pyfunction]
pub fn out_degree(py_graph: &PyGraph) -> PyResult<HashMap<usize, f64>> {
    let res =
        out_degree_centrality(&py_graph.graph).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let mut out = HashMap::new();
    for (nid, val) in res.into_iter() {
        let pyid = py_graph
            .internal_to_py
            .get(&nid)
            .ok_or_else(|| PyValueError::new_err("Internal node id missing mapping"))?;
        out.insert(*pyid, val);
    }
    Ok(out)
}

pub fn register_degree(m: &pyo3::prelude::Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(degree, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(in_degree, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(out_degree, m)?)?;
    Ok(())
}
