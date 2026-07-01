use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::centrality::utils::{to_f64_digraph, to_f64_graph};
use crate::{PyDiGraph, PyGraph};
use graphina::centrality::harmonic::harmonic_centrality;
use graphina::core::types::NodeId;

/// Compute harmonic centrality for nodes.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
///
/// Returns
/// -------
/// dict
///     Dictionary mapping node IDs to harmonic centrality scores.
///
/// Raises
/// ------
/// GraphinaError
///     If the algorithm fails.
/// TypeError
///     If graph is not PyGraph or PyDiGraph.
#[pyfunction]
pub fn harmonic(py: Python<'_>, graph: &Bound<'_, PyAny>) -> PyResult<Py<PyDict>> {
    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        let (og, old_to_new) = to_f64_graph(&py_graph);
        let mut new_to_old: std::collections::HashMap<NodeId, NodeId> =
            std::collections::HashMap::new();
        for (old, new) in old_to_new.iter() {
            new_to_old.insert(*new, *old);
        }

        match harmonic_centrality(&og) {
            Ok(map) => crate::f64_entries_to_pydict(py, map, |new_nid| {
                let old_nid = new_to_old.get(&new_nid).ok_or_else(|| {
                    crate::GraphinaError::new_err("missing mapping back to original node")
                })?;
                py_graph
                    .mapper
                    .internal_to_py
                    .get(old_nid)
                    .copied()
                    .ok_or_else(|| {
                        crate::GraphinaError::new_err("Internal node id missing mapping")
                    })
            }),
            Err(e) => Err(crate::GraphinaError::new_err(format!(
                "harmonic failed: {}",
                e
            ))),
        }
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        let (og, old_to_new) = to_f64_digraph(&py_graph);
        let mut new_to_old: std::collections::HashMap<NodeId, NodeId> =
            std::collections::HashMap::new();
        for (old, new) in old_to_new.iter() {
            new_to_old.insert(*new, *old);
        }

        match harmonic_centrality(&og) {
            Ok(map) => crate::f64_entries_to_pydict(py, map, |new_nid| {
                let old_nid = new_to_old.get(&new_nid).ok_or_else(|| {
                    crate::GraphinaError::new_err("missing mapping back to original node")
                })?;
                py_graph
                    .mapper
                    .internal_to_py
                    .get(old_nid)
                    .copied()
                    .ok_or_else(|| {
                        crate::GraphinaError::new_err("Internal node id missing mapping")
                    })
            }),
            Err(e) => Err(crate::GraphinaError::new_err(format!(
                "harmonic failed: {}",
                e
            ))),
        }
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
}

pub fn register_harmonic(m: &pyo3::prelude::Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(harmonic, m)?)?;
    Ok(())
}
