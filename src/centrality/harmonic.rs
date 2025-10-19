//! Harmonic centrality algorithms.
//!
//! This module provides harmonic centrality measures.
//!
//! Convention: returns `Result<_, crate::core::error::GraphinaError>` to propagate
//! path-computation errors and improve observability.

use crate::core::error::Result;
use crate::core::paths::dijkstra;
use crate::core::types::{BaseGraph, GraphConstructor, NodeMap};
use ordered_float::OrderedFloat;

/// Harmonic centrality: a variant of closeness centrality, summing the reciprocals of distances.
/// It is more robust for disconnected graphs.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing harmonic centralities of each node in the graph.
pub fn harmonic_centrality<A, Ty>(
    graph: &BaseGraph<A, OrderedFloat<f64>, Ty>,
) -> Result<NodeMap<f64>>
where
    Ty: GraphConstructor<A, OrderedFloat<f64>>,
{
    let mut centrality = NodeMap::new();
    for (node, _) in graph.nodes() {
        let distances = dijkstra(graph, node)?;
        let sum: f64 = distances
            .into_values()
            .filter_map(|d| d.map(|od| 1.0 / od.0))
            .sum();
        centrality.insert(node, sum);
    }
    Ok(centrality)
}
