//! Katz centrality algorithms.
//!
//! This module provides Katz centrality measures.
//!
//! Convention: returns `Result<_, crate::core::error::GraphinaError>` to handle
//! convergence/parameter validation with clear error propagation.

use crate::core::error::{GraphinaError, Result};
use crate::core::types::{BaseGraph, GraphConstructor, NodeId, NodeMap};
use nalgebra::{DMatrix, DVector};

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
        return Ok(NodeMap::new());
    }

    // Build proper node index mapping
    let node_list: Vec<NodeId> = graph.nodes().map(|(node, _)| node).collect();
    let mut node_to_idx = std::collections::HashMap::new();
    for (idx, &node) in node_list.iter().enumerate() {
        node_to_idx.insert(node, idx);
    }

    // Build adjacency matrix with proper indexing
    let mut adj = DMatrix::<f64>::zeros(n, n);
    for (u, v, w) in graph.edges() {
        let ui = node_to_idx[&u];
        let vi = node_to_idx[&v];
        let weight: f64 = (*w).into();
        adj[(ui, vi)] += weight;
    }

    // Initial vector
    let mut x = DVector::<f64>::from_element(n, 0.0);
    let beta_vec = if let Some(b) = beta {
        DVector::from_fn(n, |idx, _| b(node_list[idx]))
    } else {
        DVector::from_element(n, 1.0)
    };

    let mut converged = false;
    for _ in 0..max_iter {
        let x_new = alpha * &adj * &x + &beta_vec;
        if (&x_new - &x).norm() < tolerance {
            x = x_new;
            converged = true;
            break;
        }
        x = x_new;
    }

    if !converged {
        return Err(GraphinaError::convergence_failed(
            max_iter,
            "Katz centrality failed to converge within maximum iterations",
        ));
    }

    let mut centrality = NodeMap::new();
    for (idx, &val) in x.iter().enumerate() {
        centrality.insert(node_list[idx], val);
    }
    Ok(centrality)
}

#[cfg(test)]
mod tests {
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
}
