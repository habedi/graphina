use pyo3::prelude::*;
use std::collections::HashMap;

use crate::{PyDiGraph, PyGraph};
use graphina::centrality::pagerank::pagerank as pagerank_core;
use graphina::centrality::personalized::personalized_pagerank as personalized_pagerank_core;

/// Compute the PageRank of nodes in the graph.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
/// damping : float
///     Damping factor for PageRank, typically 0.85.
/// max_iter : int
///     Maximum number of iterations.
/// tolerance : float
///     Error tolerance for convergence.
/// nstart : dict, optional
///     Starting value for PageRank iteration for each node.
///
/// Returns
/// -------
/// dict
///     Dictionary mapping node IDs to PageRank scores.
///
/// Raises
/// ------
/// GraphinaError
///     If the algorithm fails to run.
/// TypeError
///     If graph is not a PyGraph or PyDiGraph.
#[pyfunction]
#[pyo3(signature = (graph, damping=0.85, max_iter=100, tolerance=1e-6, nstart=None))]
pub fn pagerank(
    graph: &Bound<'_, PyAny>,
    damping: f64,
    max_iter: usize,
    tolerance: f64,
    nstart: Option<HashMap<usize, f64>>,
) -> PyResult<HashMap<usize, f64>> {
    if let Ok(g) = graph.extract::<PyRef<PyGraph>>() {
        let nstart_map = if let Some(ns) = nstart {
            let mut map = HashMap::new();
            for (py_id, val) in ns {
                if let Some(&internal_id) = g.mapper.py_to_internal.get(&py_id) {
                    map.insert(internal_id, val);
                }
            }
            Some(map)
        } else {
            None
        };

        let res = pagerank_core(&g.graph, damping, max_iter, tolerance, nstart_map.as_ref())
            .map_err(|e| crate::GraphinaError::new_err(e.to_string()))?;
        let mut out = HashMap::new();
        for (nid, val) in res.into_iter() {
            let pyid =
                g.mapper.internal_to_py.get(&nid).ok_or_else(|| {
                    crate::GraphinaError::new_err("Internal node id missing mapping")
                })?;
            out.insert(*pyid, val);
        }
        Ok(out)
    } else if let Ok(g) = graph.extract::<PyRef<PyDiGraph>>() {
        let nstart_map = if let Some(ns) = nstart {
            let mut map = HashMap::new();
            for (py_id, val) in ns {
                if let Some(&internal_id) = g.mapper.py_to_internal.get(&py_id) {
                    map.insert(internal_id, val);
                }
            }
            Some(map)
        } else {
            None
        };

        let res = pagerank_core(&g.graph, damping, max_iter, tolerance, nstart_map.as_ref())
            .map_err(|e| crate::GraphinaError::new_err(e.to_string()))?;
        let mut out = HashMap::new();
        for (nid, val) in res.into_iter() {
            let pyid =
                g.mapper.internal_to_py.get(&nid).ok_or_else(|| {
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

/// Compute personalized PageRank with optional personalization vector.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     Input graph.
/// personalization : list of float, optional
///     The "teleportation" distribution. A list of weights for each node using the
///     internal node order. If None, uniform distribution is used.
/// damping : float
///     Damping factor, typically 0.85.
/// tolerance : float
///     Convergence tolerance.
/// max_iter : int
///     Maximum iterations.
/// nstart : dict, optional
///     Starting value for PageRank iteration for each node.
///
/// Returns
/// -------
/// dict
///     Mapping of node ID to personalized PageRank score.
///
/// Raises
/// ------
/// GraphinaError
///     If the algorithm fails.
/// TypeError
///     If graph is not a PyGraph or PyDiGraph.
#[pyfunction]
#[pyo3(signature = (graph, personalization=None, damping=0.85, tolerance=1e-6, max_iter=100, nstart=None)
)]
pub fn personalized_pagerank(
    graph: &Bound<'_, PyAny>,
    personalization: Option<Vec<f64>>,
    damping: f64,
    tolerance: f64,
    max_iter: usize,
    nstart: Option<HashMap<usize, f64>>,
) -> PyResult<HashMap<usize, f64>> {
    // personalized_pagerank_core does not support nstart, so it is accepted for API
    // compatibility but ignored here.
    let _ = nstart;

    if let Ok(g) = graph.extract::<PyRef<PyGraph>>() {
        let res =
            personalized_pagerank_core(&g.graph, personalization, damping, tolerance, max_iter)
                .map_err(|e| crate::GraphinaError::new_err(e.to_string()))?;

        let mut out = HashMap::new();
        for (nid, val) in res.into_iter() {
            let pyid =
                g.mapper.internal_to_py.get(&nid).ok_or_else(|| {
                    crate::GraphinaError::new_err("Internal node id missing mapping")
                })?;
            out.insert(*pyid, val);
        }
        Ok(out)
    } else if let Ok(g) = graph.extract::<PyRef<PyDiGraph>>() {
        let res =
            personalized_pagerank_core(&g.graph, personalization, damping, tolerance, max_iter)
                .map_err(|e| crate::GraphinaError::new_err(e.to_string()))?;

        let mut out = HashMap::new();
        for (nid, val) in res.into_iter() {
            let pyid =
                g.mapper.internal_to_py.get(&nid).ok_or_else(|| {
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

pub fn register_pagerank(m: &pyo3::prelude::Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(pagerank, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(personalized_pagerank, m)?)?;
    Ok(())
}
