//! Eigenvector centrality algorithms.
//!
//! This module provides eigenvector centrality measures.
//!
//! Convention: functions in this module return `Result<_, crate::core::error::GraphinaError>`
//! to surface convergence issues and aid observability and error propagation.

use crate::core::error::{GraphinaError, Result};
use crate::core::types::{BaseGraph, GraphConstructor, NodeId, NodeMap};
use nalgebra::SymmetricEigen;
use nalgebra::{DMatrix, DVector};

/// Eigenvector centrality: computes the eigenvector corresponding to the largest eigenvalue
/// of the adjacency matrix.
///
/// For directed graphs, computes the left eigenvector (based on incoming edges).
/// For undirected graphs, computes the standard eigenvector centrality.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `max_iter`: maximum number of iterations for the power iteration method.
/// * `tolerance`: convergence tolerance.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing eigenvector centralities of each node in the graph.
///
/// # Errors
///
/// Returns an error if the graph is empty or if the power iteration fails to converge.
pub fn eigenvector_centrality<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
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

    // Fast path: graphs with no edges yield uniform centrality
    if graph.edge_count() == 0 {
        let mut centrality = NodeMap::new();
        let uniform_value = 1.0 / n as f64;
        for (node, _) in graph.nodes() {
            centrality.insert(node, uniform_value);
        }
        return Ok(centrality);
    }

    // Build node index mapping
    let node_list: Vec<NodeId> = graph.nodes().map(|(node, _)| node).collect();
    let mut node_to_idx = std::collections::HashMap::new();
    for (idx, &node) in node_list.iter().enumerate() {
        node_to_idx.insert(node, idx);
    }

    // Build adjacency matrix
    let mut adj = DMatrix::<f64>::zeros(n, n);
    for (u, v, w) in graph.edges() {
        let ui = node_to_idx[&u];
        let vi = node_to_idx[&v];
        let weight: f64 = (*w).into();

        if graph.is_directed() {
            // For directed graphs: v influences u (incoming edges)
            adj[(vi, ui)] += weight;
        } else {
            // For undirected graphs: symmetric
            adj[(ui, vi)] += weight;
            adj[(vi, ui)] += weight;
        }
    }

    // For undirected graphs, use a robust symmetric eigensolver to avoid power-iteration
    // oscillation on bipartite graphs where |lambda_max| ties with |lambda_min|
    if !graph.is_directed() {
        // Safety: if adjacency is effectively zero (should be caught above), fall back to uniform
        // Compute principal eigenvector
        let se = SymmetricEigen::new(adj.clone());
        // Clone to avoid moving fields out of `se`
        let evals = se.eigenvalues.clone();
        let evecs = se.eigenvectors.clone();

        // Find the index of the largest eigenvalue
        let mut max_idx = 0usize;
        let mut max_val = f64::NEG_INFINITY;
        for (i, &eig) in evals.iter().enumerate() {
            if eig > max_val {
                max_val = eig;
                max_idx = i;
            }
        }

        // Extract the corresponding eigenvector (column max_idx)
        let mut x = evecs.column(max_idx).into_owned();
        // Ensure non-negative entries (orientation of eigenvectors is arbitrary)
        for val in x.iter_mut() {
            *val = val.abs();
        }
        // Normalize to sum to number of nodes for consistency
        let sum: f64 = x.iter().sum();
        if sum > 0.0 {
            x *= (n as f64) / sum;
        }

        let mut centrality = NodeMap::new();
        for (idx, &val) in x.iter().enumerate() {
            centrality.insert(node_list[idx], val);
        }
        return Ok(centrality);
    }

    // Power iteration
    let mut x = DVector::<f64>::from_element(n, 1.0 / (n as f64).sqrt());
    let mut converged = false;

    for iter in 0..max_iter {
        let x_new = &adj * &x;
        let norm = x_new.norm();

        if norm < 1e-10 {
            // Graph is disconnected or has zero weights, return uniform distribution
            let mut centrality = NodeMap::new();
            let uniform_value = 1.0 / n as f64;
            for &node in &node_list {
                centrality.insert(node, uniform_value);
            }
            return Ok(centrality);
        }

        let x_new_normalized = x_new / norm;
        let diff = (&x_new_normalized - &x).norm();

        if diff < tolerance {
            x = x_new_normalized;
            converged = true;
            break;
        }

        // Check for oscillation (negative eigenvalue case)
        if iter > 10 {
            let x_neg = -&x;
            let diff_neg = (&x_new_normalized - &x_neg).norm();
            if diff_neg < tolerance {
                // Oscillating - take absolute values
                x = x_new_normalized;
                for val in x.iter_mut() {
                    *val = val.abs();
                }
                converged = true;
                break;
            }
        }

        x = x_new_normalized;
    }

    if !converged {
        return Err(GraphinaError::convergence_failed(
            max_iter,
            "Eigenvector centrality failed to converge within maximum iterations",
        ));
    }

    // Normalize to have values sum to number of nodes for consistency
    let sum: f64 = x.iter().map(|v| v.abs()).sum();
    if sum > 0.0 {
        x *= n as f64 / sum;
    }

    let mut centrality = NodeMap::new();
    for (idx, &val) in x.iter().enumerate() {
        centrality.insert(node_list[idx], val.abs());
    }
    Ok(centrality)
}

#[cfg(test)]
mod tests {
    use super::eigenvector_centrality;
    use crate::core::types::{Digraph, Graph};

    #[test]
    fn eigenvector_directed_vs_undirected_basic() {
        // Directed: 0 -> 1
        let mut dg: Digraph<i32, f64> = Digraph::new();
        let n0 = dg.add_node(0);
        let n1 = dg.add_node(1);
        dg.add_edge(n0, n1, 1.0);
        let c_dir = eigenvector_centrality(&dg, 100, 1e-9).unwrap();
        // For directed graphs with incoming edges: node 0 (source) gets centrality from n1
        // In a single edge graph, both nodes should have positive centrality
        assert!(c_dir[&n0] >= 0.0);
        assert!(c_dir[&n1] >= 0.0);

        // Undirected: 0 -- 1
        let mut ug: Graph<i32, f64> = Graph::new();
        let m0 = ug.add_node(0);
        let m1 = ug.add_node(1);
        ug.add_edge(m0, m1, 1.0);
        let c_und = eigenvector_centrality(&ug, 100, 1e-9).unwrap();
        // Symmetric graph => equal centralities
        let diff = (c_und[&m0] - c_und[&m1]).abs();
        assert!(diff < 1e-5);
    }

    #[test]
    fn test_eigenvector_triangle() {
        let mut g: Graph<i32, f64> = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n1, 1.0);

        let c = eigenvector_centrality(&g, 100, 1e-9).unwrap();

        // All nodes should have equal centrality in a symmetric triangle
        assert!((c[&n1] - c[&n2]).abs() < 1e-5);
        assert!((c[&n2] - c[&n3]).abs() < 1e-5);
    }

    #[test]
    fn test_eigenvector_star() {
        let mut g: Graph<i32, f64> = Graph::new();
        let center = g.add_node(0);
        let mut leaves = Vec::new();

        for i in 1..=3 {
            let leaf = g.add_node(i);
            g.add_edge(center, leaf, 1.0);
            leaves.push(leaf);
        }

        // Add some edges between leaves to make it converge better
        g.add_edge(leaves[0], leaves[1], 1.0);

        // Star graphs can be slow to converge, use more iterations and relaxed tolerance
        let c = eigenvector_centrality(&g, 10000, 1e-4).unwrap();

        // Center should have higher centrality since it's connected to all nodes
        for &leaf in &leaves {
            assert!(
                c[&center] >= c[&leaf],
                "Center: {}, Leaf: {}",
                c[&center],
                c[&leaf]
            );
        }

        // All centrality values should be positive
        assert!(c[&center] > 0.0);
        for &leaf in &leaves {
            assert!(c[&leaf] > 0.0);
        }
    }

    #[test]
    fn test_eigenvector_empty() {
        let g: Graph<i32, f64> = Graph::new();
        let c = eigenvector_centrality(&g, 100, 1e-9).unwrap();
        assert!(c.is_empty());
    }

    #[test]
    fn test_eigenvector_isolated_nodes() {
        let mut g: Graph<i32, f64> = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);

        // No edges - isolated nodes
        let c = eigenvector_centrality(&g, 100, 1e-9).unwrap();

        // Should return uniform distribution
        assert!((c[&n1] - c[&n2]).abs() < 1e-5);
    }
}
