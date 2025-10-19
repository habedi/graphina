//! Approximation algorithms for graph diameter.

use crate::core::error::Result;
use crate::core::paths::dijkstra;
use crate::core::types::{BaseGraph, GraphConstructor};
use ordered_float::OrderedFloat;

/// Approximates the graph diameter using sampling.
pub fn approximate_diameter<A, Ty>(graph: &BaseGraph<A, OrderedFloat<f64>, Ty>) -> Result<f64>
where
    Ty: GraphConstructor<A, OrderedFloat<f64>>,
{
    let mut max_distance = 0.0_f64;
    for (node, _) in graph.nodes() {
        if let Ok(distances) = dijkstra(graph, node) {
            for (_, dist_opt) in distances.iter() {
                if let Some(dist) = dist_opt {
                    max_distance = max_distance.max(dist.into_inner());
                }
            }
        }
    }
    Ok(max_distance)
}
