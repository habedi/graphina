use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::PyGraph;
use graphina::centrality::pagerank::pagerank as pagerank_core;
use graphina::centrality::personalized::personalized_pagerank as personalized_pagerank_core;

#[pyfunction]
pub fn pagerank(
    py_graph: &PyGraph,
    damping: f64,
    max_iter: usize,
    tolerance: f64,
) -> PyResult<HashMap<usize, f64>> {
    let res = pagerank_core(&py_graph.graph, damping, max_iter, tolerance)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
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

/// Compute personalized PageRank with optional personalization vector.
///
/// Args:
///     graph: Input graph
///     personalization: Optional list of personalization weights (one per node, in node order).
///                      If None, uses uniform personalization (standard PageRank).
///     damping: Damping factor, typically 0.85
///     tolerance: Convergence tolerance
///     max_iter: Maximum iterations
///
/// Returns:
///     dict: Mapping of node ID to personalized PageRank score
///
/// Example:
///     >>> g = pygraphina.PyGraph()
///     >>> n0 = g.add_node(0)
///     >>> n1 = g.add_node(1)
///     >>> g.add_edge(n0, n1, 1.0)
///     >>> # Without personalization (like regular PageRank)
///     >>> scores = pygraphina.centrality.personalized_pagerank(g, None, 0.85, 1e-6, 100)
///     >>> # With personalization (bias towards first node)
///     >>> scores = pygraphina.centrality.personalized_pagerank(g, [2.0, 1.0], 0.85, 1e-6, 100)
#[pyfunction]
#[pyo3(signature = (graph, personalization, damping, tolerance, max_iter))]
pub fn personalized_pagerank(
    graph: &PyGraph,
    personalization: Option<Vec<f64>>,
    damping: f64,
    tolerance: f64,
    max_iter: usize,
) -> PyResult<HashMap<usize, f64>> {
    let res =
        personalized_pagerank_core(&graph.graph, personalization, damping, tolerance, max_iter)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

    let mut out = HashMap::new();
    for (nid, val) in res.into_iter() {
        let pyid = graph
            .internal_to_py
            .get(&nid)
            .ok_or_else(|| PyValueError::new_err("Internal node id missing mapping"))?;
        out.insert(*pyid, val);
    }
    Ok(out)
}

pub fn register_pagerank(m: &pyo3::prelude::Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(pagerank, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(personalized_pagerank, m)?)?;
    Ok(())
}
