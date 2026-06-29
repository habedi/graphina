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
    let mut centrality = NodeMap::default();
    for (node, _) in graph.nodes() {
        let distances = dijkstra(graph, node)?;
        // Exclude the node itself: its distance is 0, and 1 / 0 is infinite.
        // Harmonic centrality sums reciprocal distances over the other nodes.
        let sum: f64 = distances
            .into_iter()
            .filter(|(other, _)| *other != node)
            .filter_map(|(_, d)| d.map(|od| 1.0 / od.0))
            .sum();
        centrality.insert(node, sum);
    }
    Ok(centrality)
}

#[cfg(test)]
mod tests {
    // Regression: harmonic centrality summed reciprocal distances over all nodes
    // including the source itself, whose distance is 0, yielding 1/0 = infinity for
    // every node. In a unit-weight triangle each node's harmonic centrality is
    // 1/1 + 1/1 = 2.
    #[test]
    fn test_harmonic_centrality_excludes_source() {
        use crate::centrality::harmonic::harmonic_centrality;
        use crate::core::types::Graph;
        use ordered_float::OrderedFloat;

        let mut g = Graph::<i32, OrderedFloat<f64>>::new();
        let n0 = g.add_node(0);
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n0, n1, OrderedFloat(1.0));
        g.add_edge(n1, n2, OrderedFloat(1.0));
        g.add_edge(n2, n0, OrderedFloat(1.0));

        let hc = harmonic_centrality(&g).expect("harmonic should succeed");
        for n in [n0, n1, n2] {
            assert!(hc[&n].is_finite(), "harmonic centrality must be finite");
            assert!((hc[&n] - 2.0).abs() < 1e-9, "expected 2.0, got {}", hc[&n]);
        }
    }
}
