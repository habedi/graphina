use pyo3::prelude::*;
use pyo3::types::PyDict;

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
    py: Python<'_>,
    graph: &Bound<'_, PyAny>,
    max_iter: usize,
    tolerance: f64,
) -> PyResult<Py<PyDict>> {
    if let Ok(py_graph) = graph.extract::<PyRef<PyGraph>>() {
        match eigenvector_centrality(&py_graph.graph, max_iter, tolerance) {
            Ok(map) => crate::nodemap_to_pydict(py, map, &py_graph.mapper),
            Err(e) => Err(crate::GraphinaError::new_err(format!(
                "eigenvector failed: {}",
                e
            ))),
        }
    } else if let Ok(py_graph) = graph.extract::<PyRef<PyDiGraph>>() {
        match eigenvector_centrality(&py_graph.graph, max_iter, tolerance) {
            Ok(map) => crate::nodemap_to_pydict(py, map, &py_graph.mapper),
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
