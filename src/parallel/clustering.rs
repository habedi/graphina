/*!
Parallel clustering coefficient computation
*/

use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

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

    // Precompute every node's neighbor set once (O(V + E)) and share it read-only
    // across the rayon tasks, so the inner adjacency test is an O(1) hash lookup
    // instead of an O(degree) `contains_edge` scan.
    let adjacency: HashMap<NodeId, HashSet<NodeId>> = nodes
        .iter()
        .map(|&node| (node, graph.neighbors(node).collect()))
        .collect();

    let directed = graph.is_directed();
    nodes
        .par_iter()
        .map(|&node| {
            let neighbors: Vec<NodeId> = graph.neighbors(node).filter(|&nb| nb != node).collect();
            let k = neighbors.len();

            let coefficient = if directed {
                directed_clustering_coefficient(graph, node)
            } else if k < 2 {
                0.0
            } else {
                let mut triangles = 0;
                for i in 0..neighbors.len() {
                    let si = &adjacency[&neighbors[i]];
                    for other in &neighbors[i + 1..] {
                        if si.contains(other) {
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

/// Fagiolo's directed clustering coefficient, matching NetworkX and the
/// sequential `metrics::clustering_coefficient` (reimplemented here because an
/// extension may depend only on `core`).
fn directed_clustering_coefficient<A, W, Ty: GraphConstructor<A, W>>(
    graph: &BaseGraph<A, W, Ty>,
    node: NodeId,
) -> f64 {
    let preds: HashSet<NodeId> = graph
        .incoming_neighbors(node)
        .filter(|&v| v != node)
        .collect();
    let succs: HashSet<NodeId> = graph.neighbors(node).filter(|&v| v != node).collect();
    let d_tot = preds.len() + succs.len();
    if d_tot < 2 {
        return 0.0;
    }
    let d_bi = preds.intersection(&succs).count();

    let mut triangles = 0usize;
    for &j in preds.iter().chain(succs.iter()) {
        let j_preds: HashSet<NodeId> = graph.incoming_neighbors(j).filter(|&v| v != j).collect();
        let j_succs: HashSet<NodeId> = graph.neighbors(j).filter(|&v| v != j).collect();
        triangles += preds.intersection(&j_preds).count()
            + preds.intersection(&j_succs).count()
            + succs.intersection(&j_preds).count()
            + succs.intersection(&j_succs).count();
    }
    if triangles == 0 {
        return 0.0;
    }
    let possible = 2 * (d_tot * (d_tot - 1) - 2 * d_bi);
    triangles as f64 / possible as f64
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

    #[test]
    fn test_clustering_coefficients_parallel_ignores_self_loops() {
        let mut g = Graph::<i32, f64>::new();
        let hub = g.add_node(0);
        let x = g.add_node(1);
        let y = g.add_node(2);
        g.add_edge(hub, hub, 1.0);
        g.add_edge(hub, x, 1.0);
        g.add_edge(hub, y, 1.0);
        let coeffs = clustering_coefficients_parallel(&g);
        assert_eq!(coeffs[&hub], 0.0);
    }

    #[test]
    fn test_clustering_coefficients_parallel_directed_matches_networkx() {
        use crate::core::types::Digraph;

        let mut g = Digraph::<i32, f64>::new();
        let n: Vec<_> = (0..4).map(|i| g.add_node(i)).collect();
        for (u, v) in [(0, 1), (1, 2), (2, 0), (0, 2), (3, 0)] {
            g.add_edge(n[u], n[v], 1.0);
        }
        let coeffs = clustering_coefficients_parallel(&g);
        let want = [0.2, 1.0, 0.5, 0.0];
        for (i, w) in want.iter().enumerate() {
            assert!(
                (coeffs[&n[i]] - w).abs() < 1e-12,
                "node {i}: got {}",
                coeffs[&n[i]]
            );
        }
    }
}
