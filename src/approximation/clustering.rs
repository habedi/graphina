//! Approximation algorithms for clustering problems.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::{HashMap, HashSet};

/// Estimate the average clustering coefficient using cached neighbor sets.
pub fn average_clustering<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> f64
where
    Ty: GraphConstructor<A, f64>,
{
    let mut total = 0.0;
    let mut count = 0;
    let neighbor_cache: HashMap<NodeId, HashSet<NodeId>> = graph
        .nodes()
        .map(|(u, _)| (u, graph.neighbors(u).filter(|&v| v != u).collect()))
        .collect();
    for (u, _) in graph.nodes() {
        if let Some(neighbors) = neighbor_cache.get(&u) {
            let k = neighbors.len();
            // A node with fewer than two neighbors has coefficient 0 and still
            // counts toward the average, matching NetworkX and
            // `metrics::average_clustering_coefficient`.
            if k < 2 {
                count += 1;
                continue;
            }
            let mut links = 0;
            let neighbor_vec: Vec<&NodeId> = neighbors.iter().collect();
            for i in 0..neighbor_vec.len() {
                for j in (i + 1)..neighbor_vec.len() {
                    if let Some(set_i) = neighbor_cache.get(neighbor_vec[i]) {
                        if set_i.contains(neighbor_vec[j]) {
                            links += 1;
                        }
                    }
                }
            }
            let possible = k * (k - 1) / 2;
            total += links as f64 / possible as f64;
            count += 1;
        }
    }
    if count > 0 { total / count as f64 } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::average_clustering;
    use crate::core::types::Graph;

    #[test]
    fn average_clustering_ignores_self_loops() {
        let mut g = Graph::<i32, f64>::new();
        let hub = g.add_node(0);
        let x = g.add_node(1);
        let y = g.add_node(2);
        g.add_edge(hub, hub, 1.0);
        g.add_edge(hub, x, 1.0);
        g.add_edge(hub, y, 1.0);
        assert_eq!(average_clustering(&g), 0.0);
    }

    #[test]
    fn average_clustering_counts_nodes_with_fewer_than_two_neighbors() {
        // Triangle a-b-c with a pendant node d on a. Coefficients are 1/3, 1, 1,
        // and 0, so the average over all four nodes is 7/12. Averaging only over
        // nodes with two or more neighbors would give 7/9.
        let mut g = Graph::<i32, f64>::new();
        let a = g.add_node(0);
        let b = g.add_node(1);
        let c = g.add_node(2);
        let d = g.add_node(3);
        g.add_edge(a, b, 1.0);
        g.add_edge(b, c, 1.0);
        g.add_edge(a, c, 1.0);
        g.add_edge(a, d, 1.0);
        assert!((average_clustering(&g) - 7.0 / 12.0).abs() < 1e-12);
    }
}
