//! Closeness centrality algorithms.
//!
//! This module provides closeness centrality measures.

use crate::core::exceptions::GraphinaException;
use crate::core::paths::dijkstra;
use crate::core::types::{BaseGraph, GraphConstructor, NodeMap};
use ordered_float::OrderedFloat;

/// Closeness centrality: measures how close a node is to all other nodes in the graph.
/// It is the reciprocal of the sum of the shortest path distances from the node to all other nodes.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing closeness centralities of each node in the graph.
pub fn closeness_centrality<A, Ty>(
    graph: &BaseGraph<A, OrderedFloat<f64>, Ty>,
) -> Result<NodeMap<f64>, GraphinaException>
where
    Ty: GraphConstructor<A, OrderedFloat<f64>>,
{
    let n = graph.node_count() as f64;
    let mut centrality = NodeMap::new();
    for (node, _) in graph.nodes() {
        let distances = dijkstra(graph, node)?;
        let sum: f64 = distances.into_iter().filter_map(|d| d.map(|od| od.0)).sum();
        if sum > 0.0 {
            centrality.insert(node, (n - 1.0) / sum);
        } else {
            centrality.insert(node, 0.0);
        }
    }
    Ok(centrality)
}
