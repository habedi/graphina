use pyo3::prelude::*;
use std::collections::HashMap;

use crate::centrality::utils::{to_ordered_digraph, to_ordered_graph};
use crate::{PyDiGraph, PyGraph};
use graphina::centrality::betweenness::{betweenness_centrality, edge_betweenness_centrality};
use graphina::core::types::NodeId;

/// Compute the shortest-path betweenness centrality for nodes.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
/// normalized : bool
///     If True, betweenness values are normalized by 2/((n-1)(n-2)) for graphs,
///     and 1/((n-1)(n-2)) for directed graphs.
///
/// Returns
/// -------
/// dict
///     Dictionary mapping node IDs to betweenness scores.
///
/// Raises
/// ------
/// GraphinaError
///     If the algorithm fails.
/// TypeError
///     If graph is not PyGraph or PyDiGraph.
#[pyfunction]
pub fn betweenness(graph: &Bound<'_, PyAny>, normalized: bool) -> PyResult<HashMap<usize, f64>> {
    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        let (og, old_to_new) = to_ordered_graph(&py_graph);
        // ... (rest of logic for PyGraph)
        // Actually I should factor this out or duplicate?
        // Duplicating is cleaner for now to avoid generic mess in binding code.
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
                        crate::GraphinaError::new_err("missing mapping back to original node")
                    })?;
                    let pyid = py_graph.internal_to_py.get(old_nid).ok_or_else(|| {
                        crate::GraphinaError::new_err("Internal node id missing mapping")
                    })?;
                    out.insert(*pyid, val);
                }
                Ok(out)
            }
            Err(e) => Err(crate::GraphinaError::new_err(format!(
                "betweenness failed: {}",
                e
            ))),
        }
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        let (og, old_to_new) = to_ordered_digraph(&py_graph);
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
                        crate::GraphinaError::new_err("missing mapping back to original node")
                    })?;
                    let pyid = py_graph.internal_to_py.get(old_nid).ok_or_else(|| {
                        crate::GraphinaError::new_err("Internal node id missing mapping")
                    })?;
                    out.insert(*pyid, val);
                }
                Ok(out)
            }
            Err(e) => Err(crate::GraphinaError::new_err(format!(
                "betweenness failed: {}",
                e
            ))),
        }
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
}

/// Compute betweenness centrality for edges.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
/// normalized : bool
///     If True, betweenness values are normalized by 2/((n-1)(n-2)) for graphs,
///     and 1/((n-1)(n-2)) for directed graphs.
///
/// Returns
/// -------
/// dict
///     Dictionary mapping (u, v) tuples to edge betweenness scores.
///
/// Raises
/// ------
/// GraphinaError
///     If the algorithm fails.
/// TypeError
///     If graph is not PyGraph or PyDiGraph.
#[pyfunction]
pub fn edge_betweenness(
    graph: &Bound<'_, PyAny>,
    normalized: bool,
) -> PyResult<HashMap<(usize, usize), f64>> {
    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        let (og, old_to_new) = to_ordered_graph(&py_graph);
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
                        crate::GraphinaError::new_err("missing mapping back to original node u")
                    })?;
                    let ov = new_to_old.get(&nv).ok_or_else(|| {
                        crate::GraphinaError::new_err("missing mapping back to original node v")
                    })?;
                    let pu = py_graph.internal_to_py.get(ou).ok_or_else(|| {
                        crate::GraphinaError::new_err("missing node mapping for u")
                    })?;
                    let pv = py_graph.internal_to_py.get(ov).ok_or_else(|| {
                        crate::GraphinaError::new_err("missing node mapping for v")
                    })?;
                    out.insert((*pu, *pv), val);
                }
                Ok(out)
            }
            Err(e) => Err(crate::GraphinaError::new_err(format!(
                "edge_betweenness failed: {}",
                e
            ))),
        }
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        let (og, old_to_new) = to_ordered_digraph(&py_graph);
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
                        crate::GraphinaError::new_err("missing mapping back to original node u")
                    })?;
                    let ov = new_to_old.get(&nv).ok_or_else(|| {
                        crate::GraphinaError::new_err("missing mapping back to original node v")
                    })?;
                    let pu = py_graph.internal_to_py.get(ou).ok_or_else(|| {
                        crate::GraphinaError::new_err("missing node mapping for u")
                    })?;
                    let pv = py_graph.internal_to_py.get(ov).ok_or_else(|| {
                        crate::GraphinaError::new_err("missing node mapping for v")
                    })?;
                    out.insert((*pu, *pv), val);
                }
                Ok(out)
            }
            Err(e) => Err(crate::GraphinaError::new_err(format!(
                "edge_betweenness failed: {}",
                e
            ))),
        }
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
}

pub fn register_betweenness(m: &pyo3::prelude::Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(betweenness, m)?)?;
    m.add_function(pyo3::wrap_pyfunction!(edge_betweenness, m)?)?;
    Ok(())
}
