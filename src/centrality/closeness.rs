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

    let mut centralities = NodeMap::new();

    for (node, _) in graph.nodes() {
        let dist_map = dijkstra(graph, node)?;
        let mut sum = 0.0;
        let mut count = 0;
        for (other_node, _) in graph.nodes() {
            if node != other_node {
                if let Some(Some(d)) = dist_map.get(&other_node) {
                    let dist_f64: f64 = (*d).into();
                    if dist_f64 > 0.0 && dist_f64.is_finite() {
                        sum += 1.0 / dist_f64;
                        count += 1;
                    }
                }
            }
        }
        if count > 0 {
            centralities.insert(node, sum);
        } else {
            centralities.insert(node, 0.0);
        }
    }

    Ok(centralities)
}
