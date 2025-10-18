//! Approximation algorithms for diameter problems.

use crate::core::exceptions::GraphinaException;
use crate::core::paths::dijkstra;
use crate::core::types::{BaseGraph, GraphConstructor};
use ordered_float::OrderedFloat;

/// Compute a lower bound on the diameter using BFS from an arbitrary node.
pub fn diameter<A, Ty>(
    graph: &BaseGraph<A, OrderedFloat<f64>, Ty>,
) -> Result<f64, GraphinaException>
where
    Ty: GraphConstructor<A, OrderedFloat<f64>>,
{
    if let Some((start, _)) = graph.nodes().next() {
        let distances = dijkstra(graph, start)?;
        let max_dist = distances
            .into_iter()
            .filter_map(|d| d.map(|od| od.0))
            .fold(0.0, f64::max);
        Ok(max_dist)
    } else {
        Ok(0.0)
    }
}
