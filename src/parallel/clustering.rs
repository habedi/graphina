/*!
Parallel clustering coefficient computation
*/

use rayon::prelude::*;
use std::collections::HashMap;

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use petgraph::EdgeType;

/// Parallel computation of clustering coefficients for all nodes.
///
/// Computes local clustering coefficient for each node in parallel.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::Graph;
/// use graphina::parallel::clustering_coefficients_parallel;
///
/// let mut g = Graph::<i32, f64>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// let n3 = g.add_node(3);
/// g.add_edge(n1, n2, 1.0);
/// g.add_edge(n2, n3, 1.0);
/// g.add_edge(n3, n1, 1.0);
///
/// let coefficients = clustering_coefficients_parallel(&g);
/// assert!((coefficients[&n1] - 1.0).abs() < 0.001);
/// ```
pub fn clustering_coefficients_parallel<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
) -> HashMap<NodeId, f64>
where
    A: Sync,
    W: Sync,
    Ty: GraphConstructor<A, W> + EdgeType + Sync,
{
    let nodes: Vec<NodeId> = graph.node_ids().collect();

    nodes
        .par_iter()
        .map(|&node| {
            let neighbors: Vec<NodeId> = graph.neighbors(node).collect();
            let k = neighbors.len();

            let coefficient = if k < 2 {
                0.0
            } else {
                let mut triangles = 0;
                for i in 0..neighbors.len() {
                    for j in (i + 1)..neighbors.len() {
                        if graph.contains_edge(neighbors[i], neighbors[j]) {
                            triangles += 1;
                        }
                    }
                }
                let possible_edges = k * (k - 1) / 2;
                triangles as f64 / possible_edges as f64
            };

            (node, coefficient)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_clustering_coefficients_parallel() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n1, 1.0);

        let coefficients = clustering_coefficients_parallel(&g);
        assert!((coefficients[&n1] - 1.0).abs() < 0.001);
        assert!((coefficients[&n2] - 1.0).abs() < 0.001);
        assert!((coefficients[&n3] - 1.0).abs() < 0.001);
    }
}
