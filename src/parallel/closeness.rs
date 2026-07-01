/*!
Parallel closeness centrality
*/

use rayon::prelude::*;
use std::fmt::Debug;

use crate::core::error::{GraphinaError, Result};
use crate::core::paths::dijkstra_path_f64;
use crate::core::types::{BaseGraph, GraphConstructor, GraphinaGraph, NodeMap};

/// Parallel closeness centrality.
///
/// Computes the same Wasserman-Faust closeness as the sequential
/// [`crate::centrality::closeness::closeness_centrality`], but runs the
/// independent single-source shortest path searches across nodes in parallel.
/// The single-source searches do not share state, so the result is identical to
/// the sequential version and independent of the thread count.
///
/// Reimplemented over `core` (Dijkstra) rather than calling the `centrality`
/// extension, so `parallel` stays dependent on `core` alone.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::Graph;
/// use graphina::parallel::closeness_centrality_parallel;
///
/// let mut g = Graph::<i32, f64>::new();
/// let n0 = g.add_node(0);
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// g.add_edge(n0, n1, 1.0);
/// g.add_edge(n1, n2, 1.0);
///
/// let cc = closeness_centrality_parallel(&g).unwrap();
/// assert!((cc[&n1] - 1.0).abs() < 1e-9);
/// ```
pub fn closeness_centrality_parallel<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> Result<NodeMap<f64>>
where
    A: Debug + Sync,
    Ty: GraphConstructor<A, f64> + Sync,
    BaseGraph<A, f64, Ty>: GraphinaGraph<A, f64> + Sync,
{
    if graph.node_count() == 0 {
        return Err(GraphinaError::invalid_graph("Empty graph"));
    }

    let n = graph.node_count();
    let nodes: Vec<_> = graph.node_ids().collect();

    nodes
        .par_iter()
        .map(|&node| {
            let (dist_map, _) = dijkstra_path_f64(graph, node, None)?;
            // Sum of shortest path distances to reachable nodes, and how many are
            // reachable. Summation is order-independent, so iterating the distance
            // map directly is fine.
            let mut sum_dist = 0.0;
            let mut reachable = 0usize;
            for (&other_node, dist_opt) in &dist_map {
                if node != other_node {
                    if let Some(dist_f64) = dist_opt {
                        if *dist_f64 > 0.0 && dist_f64.is_finite() {
                            sum_dist += *dist_f64;
                            reachable += 1;
                        }
                    }
                }
            }
            // Wasserman-Faust improved closeness, matching the sequential version.
            let closeness = if sum_dist > 0.0 && n > 1 {
                (reachable as f64 / sum_dist) * (reachable as f64 / (n as f64 - 1.0))
            } else {
                0.0
            };
            Ok((node, closeness))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_parallel_closeness_path_values() {
        // On the unit-weight path 0-1-2 the Wasserman-Faust closeness is 0.6667 at
        // the endpoints ((2 / 3) * (2 / 2)) and 1.0 at the center ((2 / 2) * (2 /
        // 2)). These are the same values the sequential implementation pins, so the
        // parallel port must reproduce them exactly. The test stays independent of
        // the `centrality` extension so `parallel` depends on `core` alone.
        let mut g = Graph::<i32, f64>::new();
        let n0 = g.add_node(0);
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n0, n1, 1.0);
        g.add_edge(n1, n2, 1.0);

        let cc = closeness_centrality_parallel(&g).expect("parallel closeness");
        assert!(
            (cc[&n0] - 2.0 / 3.0).abs() < 1e-12,
            "endpoint got {}",
            cc[&n0]
        );
        assert!((cc[&n1] - 1.0).abs() < 1e-12, "center got {}", cc[&n1]);
        assert!(
            (cc[&n2] - 2.0 / 3.0).abs() < 1e-12,
            "endpoint got {}",
            cc[&n2]
        );
    }

    #[test]
    fn test_parallel_closeness_empty_graph_errors() {
        let g = Graph::<i32, f64>::new();
        assert!(closeness_centrality_parallel(&g).is_err());
    }
}
