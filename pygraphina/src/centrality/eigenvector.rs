use pyo3::prelude::*;
use std::collections::HashMap;

use crate::{PyDiGraph, PyGraph};
use graphina::centrality::eigenvector::eigenvector_centrality;

/// Compute eigenvector centrality for nodes.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
/// max_iter : int
///     Maximum number of iterations.
/// tolerance : float
///     Error tolerance for convergence.
///
/// Returns
/// -------
/// dict
///     Dictionary mapping node IDs to eigenvector centrality scores.
///
/// Raises
/// ------
/// GraphinaError
///     If the algorithm fails.
/// TypeError
///     If graph is not PyGraph or PyDiGraph.
#[pyfunction]
pub fn eigenvector(
    graph: &Bound<'_, PyAny>,
    max_iter: usize,
    tolerance: f64,
) -> PyResult<HashMap<usize, f64>> {
    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        match eigenvector_centrality(&py_graph.graph, max_iter, tolerance) {
            Ok(map) => {
                let mut out = HashMap::new();
                for (nid, val) in map.into_iter() {
                    let pyid = py_graph.mapper.internal_to_py.get(&nid).ok_or_else(|| {
                        crate::GraphinaError::new_err("Internal node id missing mapping")
                    })?;
                    out.insert(*pyid, val);
                }
                Ok(out)
            }
            Err(e) => Err(crate::GraphinaError::new_err(format!(
                "eigenvector failed: {}",
                e
            ))),
        }
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        match eigenvector_centrality(&py_graph.graph, max_iter, tolerance) {
            Ok(map) => {
                let mut out = HashMap::new();
                for (nid, val) in map.into_iter() {
                    let pyid = py_graph.mapper.internal_to_py.get(&nid).ok_or_else(|| {
                        crate::GraphinaError::new_err("Internal node id missing mapping")
                    })?;
                    out.insert(*pyid, val);
                }
                Ok(out)
            }
            Err(e) => Err(crate::GraphinaError::new_err(format!(
                "eigenvector failed: {}",
                e
            ))),
        }
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
}

pub fn register_eigenvector(m: &pyo3::prelude::Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(eigenvector, m)?)?;
    Ok(())
}
