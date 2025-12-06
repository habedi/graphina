use pyo3::prelude::*;
use std::collections::HashMap;

use crate::{PyDiGraph, PyGraph};
use graphina::centrality::degree::{
    degree_centrality, in_degree_centrality, out_degree_centrality,
};

/// Compute degree centrality for nodes.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
///
/// Returns
/// -------
/// dict
///     Dictionary mapping node IDs to degree centrality scores.
///
/// Raises
/// ------
/// TypeError
///     If graph is not PyGraph or PyDiGraph.
#[pyfunction]
pub fn degree(graph: &Bound<'_, PyAny>) -> PyResult<HashMap<usize, f64>> {
    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        let res = degree_centrality(&py_graph.graph)
            .map_err(|e| crate::GraphinaError::new_err(e.to_string()))?;
        let mut out = HashMap::new();
        for (nid, val) in res.into_iter() {
            let pyid =
                py_graph.mapper.internal_to_py.get(&nid).ok_or_else(|| {
                    crate::GraphinaError::new_err("Internal node id missing mapping")
                })?;
            out.insert(*pyid, val);
        }
        Ok(out)
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        let res = degree_centrality(&py_graph.graph)
            .map_err(|e| crate::GraphinaError::new_err(e.to_string()))?;
        let mut out = HashMap::new();
        for (nid, val) in res.into_iter() {
            let pyid =
                py_graph.mapper.internal_to_py.get(&nid).ok_or_else(|| {
                    crate::GraphinaError::new_err("Internal node id missing mapping")
                })?;
            out.insert(*pyid, val);
        }
        Ok(out)
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
}

/// Compute in-degree centrality for nodes.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
///
/// Returns
/// -------
/// dict
///     Dictionary mapping node IDs to in-degree centrality scores.
///     For undirected graphs, this corresponds to degree centrality.
#[pyfunction]
pub fn in_degree(graph: &Bound<'_, PyAny>) -> PyResult<HashMap<usize, f64>> {
    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        // Undirected: in_degree = degree
        let res = degree_centrality(&py_graph.graph)
            .map_err(|e| crate::GraphinaError::new_err(e.to_string()))?;
        let mut out = HashMap::new();
        for (nid, val) in res.into_iter() {
            let pyid =
                py_graph.mapper.internal_to_py.get(&nid).ok_or_else(|| {
                    crate::GraphinaError::new_err("Internal node id missing mapping")
                })?;
            out.insert(*pyid, val);
        }
        Ok(out)
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        let res = in_degree_centrality(&py_graph.graph)
            .map_err(|e| crate::GraphinaError::new_err(e.to_string()))?;
        let mut out = HashMap::new();
        for (nid, val) in res.into_iter() {
            let pyid =
                py_graph.mapper.internal_to_py.get(&nid).ok_or_else(|| {
                    crate::GraphinaError::new_err("Internal node id missing mapping")
                })?;
            out.insert(*pyid, val);
        }
        Ok(out)
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
}

/// Compute out-degree centrality for nodes.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
///
/// Returns
/// -------
/// dict
///     Dictionary mapping node IDs to out-degree centrality scores.
///     For undirected graphs, this corresponds to degree centrality.
#[pyfunction]
pub fn out_degree(graph: &Bound<'_, PyAny>) -> PyResult<HashMap<usize, f64>> {
    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        // Undirected: out_degree = degree
        let res = degree_centrality(&py_graph.graph)
            .map_err(|e| crate::GraphinaError::new_err(e.to_string()))?;
        let mut out = HashMap::new();
        for (nid, val) in res.into_iter() {
            let pyid =
                py_graph.mapper.internal_to_py.get(&nid).ok_or_else(|| {
                    crate::GraphinaError::new_err("Internal node id missing mapping")
                })?;
            out.insert(*pyid, val);
        }
        Ok(out)
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        let res = out_degree_centrality(&py_graph.graph)
            .map_err(|e| crate::GraphinaError::new_err(e.to_string()))?;
        let mut out = HashMap::new();
        for (nid, val) in res.into_iter() {
            let pyid =
                py_graph.mapper.internal_to_py.get(&nid).ok_or_else(|| {
                    crate::GraphinaError::new_err("Internal node id missing mapping")
                })?;
            out.insert(*pyid, val);
        }
        Ok(out)
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
}

pub fn register_degree(m: &pyo3::prelude::Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(degree, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(in_degree, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(out_degree, m)?)?;
    Ok(())
}
