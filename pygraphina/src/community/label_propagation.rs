use crate::PyGraph;
use graphina::community::label_propagation::label_propagation as label_propagation_core;
use pyo3::prelude::*;
use std::collections::HashMap;

/// Detect communities using Label Propagation Algorithm.
///
/// Parameters
/// ----------
/// graph : PyGraph
///     The input graph.
/// max_iter : int
///     Maximum number of iterations.
/// seed : int, optional
///     Random seed.
///
/// Returns
/// -------
/// dict
///     Dictionary mapping node IDs to community label IDs.
///
/// Raises
/// ------
/// GraphinaError
///     If the algorithm fails.
/// TypeError
///     If graph is not PyGraph.
#[pyfunction]
#[pyo3(signature = (py_graph, max_iter, seed=None))]
pub fn label_propagation(
    py_graph: &PyGraph,
    max_iter: usize,
    seed: Option<u64>,
) -> PyResult<HashMap<usize, usize>> {
    match label_propagation_core(&py_graph.graph, max_iter, seed) {
        Ok(labels) => {
            let mut result = HashMap::new();
            for (py_id, internal_id) in &py_graph.mapper.py_to_internal {
                let idx = internal_id.index();
                if idx < labels.len() {
                    result.insert(*py_id, labels[idx]);
                }
            }
            Ok(result)
        }
        Err(e) => Err(crate::GraphinaError::new_err(e.to_string())),
    }
}

pub fn register_label_propagation(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(label_propagation, m)?)?;
    Ok(())
}
