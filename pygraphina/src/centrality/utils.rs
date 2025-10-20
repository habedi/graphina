use crate::PyGraph;
use graphina::core::types::{Graph as CoreGraph, NodeId};
use ordered_float::OrderedFloat;

/// Convert PyGraph's internal graph (Graph<i64, f64>) into a Graph<i64, OrderedFloat<f64>>
/// Returns (converted_graph, mapping_old_to_new)
pub fn to_ordered_graph(
    py_graph: &PyGraph,
) -> (
    CoreGraph<i64, OrderedFloat<f64>>,
    std::collections::HashMap<NodeId, NodeId>,
) {
    let mut g: CoreGraph<i64, OrderedFloat<f64>> = CoreGraph::new();
    let mut old_to_new: std::collections::HashMap<NodeId, NodeId> =
        std::collections::HashMap::new();

    // Add nodes preserving attributes
    for (old_nid, &attr) in py_graph.graph.nodes() {
        let new_nid = g.add_node(attr);
        old_to_new.insert(old_nid, new_nid);
    }

    // Add edges converting weights to OrderedFloat
    for (u, v, w) in py_graph.graph.edges() {
        let nu = old_to_new[&u];
        let nv = old_to_new[&v];
        g.add_edge(nu, nv, OrderedFloat(*w));
    }

    (g, old_to_new)
}
