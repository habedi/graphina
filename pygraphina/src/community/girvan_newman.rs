use crate::PyGraph;
use graphina::community::girvan_newman::girvan_newman as girvan_newman_core;
use pyo3::prelude::*;

/// Find communities using the Girvan-Newman algorithm.
///
/// Parameters
/// ----------
/// graph : PyGraph
///     The input graph.
/// target_communities : int
///     Number of communities to stop at.
///
/// Returns
/// -------
/// list of list of int
///     List of communities.
///
/// Raises
/// ------
/// GraphinaError
///     If the algorithm fails.
/// TypeError
///     If graph is not PyGraph.
#[pyfunction]
pub fn girvan_newman(py_graph: &PyGraph, target_communities: usize) -> PyResult<Vec<Vec<usize>>> {
    match girvan_newman_core(&py_graph.graph, target_communities) {
        Ok(communities) => Ok(communities
            .into_iter()
            .map(|community| {
                community
                    .into_iter()
                    .filter_map(|node_id| py_graph.mapper.internal_to_py.get(&node_id).copied())
                    .collect()
            })
            .collect()),
        Err(e) => Err(crate::GraphinaError::new_err(e.to_string())),
    }
}

pub fn register_girvan_newman(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(girvan_newman, m)?)?;
    Ok(())
}
