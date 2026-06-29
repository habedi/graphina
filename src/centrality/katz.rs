//! Katz centrality algorithms.
//!
//! This module provides Katz centrality measures.
//!
//! Convention: returns `Result<_, crate::core::error::GraphinaError>` to handle
//! convergence/parameter validation with clear error propagation.

use crate::core::error::{GraphinaError, Result};
use crate::core::types::{BaseGraph, GraphConstructor, NodeId, NodeMap};

/// Katz centrality: computes the relative influence of a node within a network
/// by measuring the number of walks of length k between a pair of nodes.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `alpha`: attenuation factor (must be less than the reciprocal of the largest eigenvalue).
/// * `beta`: optional weight function for each node.
/// * `max_iter`: maximum number of iterations.
/// * `tolerance`: convergence tolerance.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing Katz centralities of each node in the graph.
///
/// # Errors
///
/// Returns an error if the graph is empty or if convergence fails.
pub fn katz_centrality<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    alpha: f64,
    beta: Option<&dyn Fn(NodeId) -> f64>,
    max_iter: usize,
    tolerance: f64,
) -> Result<NodeMap<f64>>
where
    W: Copy + PartialOrd + Into<f64>,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    if n == 0 {
        return Ok(NodeMap::default());
    }

    // Build proper node index mapping
    let node_list: Vec<NodeId> = graph.nodes().map(|(node, _)| node).collect();
    let mut node_to_idx = std::collections::HashMap::new();
    for (idx, &node) in node_list.iter().enumerate() {
        node_to_idx.insert(node, idx);
    }

    // Store the adjacency as a sparse edge list rather than a dense n x n matrix.
    // The Katz iteration only needs the matrix-vector product `adj * x`, which
    // costs O(E) over this representation instead of O(n^2) over a dense matrix,
    // and uses O(E) memory instead of O(n^2). For undirected graphs each edge is
    // stored once, so the reverse contribution is added explicitly to keep the
    // operator symmetric; otherwise Katz centrality would not respect the graph's
    // symmetry.
    let directed = graph.is_directed();
    let mut edges: Vec<(usize, usize, f64)> = Vec::with_capacity(graph.edge_count());
    for (u, v, w) in graph.edges() {
        let ui = node_to_idx[&u];
        let vi = node_to_idx[&v];
        let weight: f64 = (*w).into();
        edges.push((ui, vi, weight));
        if !directed && ui != vi {
            edges.push((vi, ui, weight));
        }
    }

    // Initial vector
    let mut x = vec![0.0_f64; n];
    let beta_vec: Vec<f64> = if let Some(b) = beta {
        node_list.iter().map(|&node| b(node)).collect()
    } else {
        vec![1.0; n]
    };

    let mut converged = false;
    for _ in 0..max_iter {
        // x_new = alpha * (adj * x) + beta
        let mut x_new = beta_vec.clone();
        for &(ui, vi, weight) in &edges {
            x_new[ui] += alpha * weight * x[vi];
        }
        let diff_sq: f64 = x_new.iter().zip(&x).map(|(a, b)| (a - b) * (a - b)).sum();
        x = x_new;
        if diff_sq.sqrt() < tolerance {
            converged = true;
            break;
        }
    }

    if !converged {
        return Err(GraphinaError::convergence_failed(
            max_iter,
            "Katz centrality failed to converge within maximum iterations",
        ));
    }

    let mut centrality = NodeMap::default();
    for (idx, &val) in x.iter().enumerate() {
        centrality.insert(node_list[idx], val);
    }
    Ok(centrality)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_katz_with_deleted_nodes() {
        use crate::centrality::katz::katz_centrality;
        use crate::core::types::Digraph;

        let mut graph: Digraph<i32, f64> = Digraph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);

        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n4, 1.0);

        graph.remove_node(n2);

        let katz = katz_centrality(&graph, 0.1, None, 100, 1e-6).unwrap();

        assert!(katz.contains_key(&n1));
        assert!(!katz.contains_key(&n2));
        assert!(katz.contains_key(&n3));
        assert!(katz.contains_key(&n4));
    }

    // Regression: Katz centrality built a directed-only adjacency matrix, so on an
    // undirected graph it was asymmetric and broke the graph's symmetry. Here nodes
    // 1 and 3 are symmetric, as are 0 and 4, so their Katz centralities must be
    // equal.
    #[test]
    fn test_katz_centrality_symmetric_on_undirected() {
        use crate::centrality::katz::katz_centrality;
        use crate::core::types::Graph;
        use ordered_float::OrderedFloat;

        let mut g = Graph::<i32, OrderedFloat<f64>>::new();
        let ids: Vec<_> = (0..5).map(|i| g.add_node(i)).collect();
        for (u, v, w) in [
            (0, 1, 1.0),
            (1, 2, 1.0),
            (2, 3, 1.0),
            (1, 3, 2.0),
            (3, 4, 1.0),
        ] {
            g.add_edge(ids[u], ids[v], OrderedFloat(w));
        }

        let kc = katz_centrality(&g, 0.1, None, 2000, 1e-9).expect("katz should succeed");
        assert!(
            (kc[&ids[1]] - kc[&ids[3]]).abs() < 1e-9,
            "symmetric nodes 1 and 3 must be equal: {} vs {}",
            kc[&ids[1]],
            kc[&ids[3]]
        );
        assert!(
            (kc[&ids[0]] - kc[&ids[4]]).abs() < 1e-9,
            "symmetric nodes 0 and 4 must be equal: {} vs {}",
            kc[&ids[0]],
            kc[&ids[4]]
        );
    }
    use super::*;
    use crate::core::types::{Digraph, Graph};

    #[test]
    fn test_katz_simple_directed() {
        let mut graph: Digraph<i32, f64> = Digraph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n1, 1.0);

        let katz = katz_centrality(&graph, 0.1, None, 100, 1e-6).unwrap();

        // All nodes in a cycle should have similar Katz centrality
        let k1 = katz[&n1];
        let k2 = katz[&n2];
        let k3 = katz[&n3];

        assert!((k1 - k2).abs() < 1e-3);
        assert!((k2 - k3).abs() < 1e-3);
    }

    #[test]
    fn test_katz_empty_graph() {
        let graph: Graph<i32, f64> = Graph::new();
        let result = katz_centrality(&graph, 0.1, None, 100, 1e-6).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_katz_with_beta() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);

        graph.add_edge(n1, n2, 1.0);

        let beta_fn = |node: NodeId| if node == n1 { 2.0 } else { 1.0 };
        let katz = katz_centrality(&graph, 0.1, Some(&beta_fn), 100, 1e-6).unwrap();

        // Node with higher beta should have higher centrality
        assert!(katz[&n1] > katz[&n2]);
    }

    #[test]
    fn test_katz_undirected_path_symmetry() {
        // On an undirected path 0 - 1 - 2 the sparse operator must stay symmetric,
        // so the two endpoints get equal Katz centrality and the middle node, with
        // two neighbors, scores strictly higher.
        let mut graph: Graph<i32, f64> = Graph::new();
        let n0 = graph.add_node(0);
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        graph.add_edge(n0, n1, 1.0);
        graph.add_edge(n1, n2, 1.0);

        let katz = katz_centrality(&graph, 0.1, None, 1000, 1e-9).unwrap();
        assert!((katz[&n0] - katz[&n2]).abs() < 1e-9);
        assert!(katz[&n1] > katz[&n0]);
    }
}
