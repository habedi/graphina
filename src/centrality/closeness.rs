//! Closeness centrality algorithms.
//!
//! This module provides closeness centrality measures.
//!
//! Convention: returns `Result<_, crate::core::error::GraphinaError>` to propagate
//!

use crate::core::error::{GraphinaError, Result};
use crate::core::paths::dijkstra;
use crate::core::types::{BaseGraph, GraphConstructor, NodeMap};

/// Compute closeness centrality for all nodes.
pub fn closeness_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Result<NodeMap<f64>>
where
    A: Clone,
    W: Copy
        + std::cmp::PartialOrd
        + std::ops::Add<Output = W>
        + std::ops::Sub<Output = W>
        + From<u8>
        + Ord
        + std::fmt::Debug
        + Into<f64>,
    Ty: GraphConstructor<A, W>,
{
    if graph.node_count() == 0 {
        return Err(GraphinaError::invalid_graph("Empty graph"));
    }

    let n = graph.node_count();
    let mut centralities = NodeMap::new();

    for (node, _) in graph.nodes() {
        let dist_map = dijkstra(graph, node)?;
        // Sum of shortest path distances to reachable nodes, and how many are
        // reachable. Closeness is the reciprocal of the mean distance.
        let mut sum_dist = 0.0;
        let mut reachable = 0usize;
        for (other_node, _) in graph.nodes() {
            if node != other_node {
                if let Some(Some(d)) = dist_map.get(&other_node) {
                    let dist_f64: f64 = (*d).into();
                    if dist_f64 > 0.0 && dist_f64.is_finite() {
                        sum_dist += dist_f64;
                        reachable += 1;
                    }
                }
            }
        }
        // Wasserman-Faust improved closeness: (reachable / sum_dist) scaled by
        // the fraction of the graph that is reachable, which is well defined on
        // disconnected graphs and reduces to (n - 1) / sum_dist when connected.
        let closeness = if sum_dist > 0.0 && n > 1 {
            (reachable as f64 / sum_dist) * (reachable as f64 / (n as f64 - 1.0))
        } else {
            0.0
        };
        centralities.insert(node, closeness);
    }

    Ok(centralities)
}
