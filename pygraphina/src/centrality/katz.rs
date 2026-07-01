use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::{PyDiGraph, PyGraph};
use graphina::centrality::katz::katz_centrality;

/// Compute Katz centrality for nodes.
///
/// Parameters
/// ----------
/// graph : PyGraph or PyDiGraph
///     The input graph.
/// alpha : float
///     Attenuation factor.
/// max_iter : int
///     Maximum number of iterations.
/// tolerance : float
///     Error tolerance for convergence.
///
/// Returns
/// -------
/// dict
///     Dictionary mapping node IDs to Katz centrality scores.
///
/// Raises
/// ------
/// GraphinaError
///     If the algorithm fails.
/// TypeError
///     If graph is not PyGraph or PyDiGraph.
#[pyfunction]
pub fn katz(
    py: Python<'_>,
    graph: &Bound<'_, PyAny>,
    alpha: f64,
    max_iter: usize,
    tolerance: f64,
) -> PyResult<Py<PyDict>> {
    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        // We don't support a beta callback from Python; pass None
        let res = katz_centrality(&py_graph.graph, alpha, None, max_iter, tolerance)
            .map_err(|e| crate::GraphinaError::new_err(format!("Katz centrality failed: {}", e)))?;
        crate::nodemap_to_pydict(py, res, &py_graph.mapper)
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        let res = katz_centrality(&py_graph.graph, alpha, None, max_iter, tolerance)
            .map_err(|e| crate::GraphinaError::new_err(format!("Katz centrality failed: {}", e)))?;
        crate::nodemap_to_pydict(py, res, &py_graph.mapper)
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected PyGraph or PyDiGraph",
        ))
    }
}

pub fn register_katz(m: &pyo3::prelude::Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(katz, m)?)?;
    Ok(())
}
