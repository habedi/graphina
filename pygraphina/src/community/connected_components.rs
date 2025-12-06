use crate::PyGraph;
use graphina::community::connected_components::connected_components as connected_components_core;
use pyo3::prelude::*;

/// Find connected components in an undirected graph.
///
/// Parameters
/// ----------
/// graph : PyGraph
///     The input graph.
///
/// Returns
/// -------
/// list of list of int
///     A list of sets of nodes, one for each component.
///     (Returns list of lists for compatibility with JSON serialization)
///
/// Raises
/// ------
/// TypeError
///     If graph is not PyGraph.
#[pyfunction]
pub fn connected_components(py_graph: &PyGraph) -> Vec<Vec<usize>> {
    let components = connected_components_core(&py_graph.graph);
    // Core returns Vec<Vec<NodeId>>. Infallible? It seems so in core wrapper?
    // If core returns Result, map it. If not, just map output.
    // The previous code didn't handle Result?
    // Let's check view of connected_components.rs (Step 938).
    // line 7: `let components = connected_components_core(&py_graph.graph);`
    // It seems it returns `Vec<Vec<NodeId>>` directly, no Result.
    // So no error handling needed for algorithm failure, but we assume it succeeds.
    components
        .into_iter()
        .map(|comp| {
            comp.into_iter()
                .filter_map(|node_id| py_graph.mapper.internal_to_py.get(&node_id).copied())
                .collect()
        })
        .collect()
}

pub fn register_connected_components(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(connected_components, m)?)?;
    Ok(())
}
