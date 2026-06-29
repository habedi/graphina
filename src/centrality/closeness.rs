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
    let mut centralities = NodeMap::default();

    for (node, _) in graph.nodes() {
        let dist_map = dijkstra(graph, node)?;
        // Sum of shortest path distances to reachable nodes, and how many are
        // reachable. Closeness is the reciprocal of the mean distance.
        let mut sum_dist = 0.0;
        let mut reachable = 0usize;
        // Iterate the distance map directly rather than doing one hash lookup per
        // other node. Summation is order-independent, so iteration order is fine.
        for (&other_node, dist_opt) in &dist_map {
            if node != other_node {
                if let Some(d) = dist_opt {
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

#[cfg(test)]
mod tests {
    // Regression: closeness centrality summed reciprocal distances (the harmonic
    // centrality formula) instead of computing closeness. On the unit-weight path
    // 0-1-2 the endpoint's closeness is (reachable / sum_dist) * (reachable / (n-1))
    // = (2/3) * (2/2) = 0.6667, not the harmonic value 1/1 + 1/2 = 1.5.
    #[test]
    fn test_closeness_centrality_is_not_harmonic() {
        use crate::centrality::closeness::closeness_centrality;
        use crate::core::types::Graph;
        use ordered_float::OrderedFloat;

        let mut g = Graph::<i32, OrderedFloat<f64>>::new();
        let n0 = g.add_node(0);
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n0, n1, OrderedFloat(1.0));
        g.add_edge(n1, n2, OrderedFloat(1.0));

        let cc = closeness_centrality(&g).expect("closeness should succeed");
        assert!(
            (cc[&n0] - 2.0 / 3.0).abs() < 1e-9,
            "expected 0.6667, got {}",
            cc[&n0]
        );
        assert!(
            (cc[&n1] - 1.0).abs() < 1e-9,
            "expected 1.0, got {}",
            cc[&n1]
        );
        assert!(
            (cc[&n2] - 2.0 / 3.0).abs() < 1e-9,
            "expected 0.6667, got {}",
            cc[&n2]
        );
    }
}
