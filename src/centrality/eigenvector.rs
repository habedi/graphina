//! Eigenvector centrality algorithms.
//!
//! This module provides eigenvector centrality measures.

use crate::core::exceptions::GraphinaException;
use crate::core::types::{BaseGraph, GraphConstructor, NodeId, NodeMap};
use nalgebra::{DMatrix, DVector};

/// Eigenvector centrality: computes the eigenvector corresponding to the largest eigenvalue
/// of the adjacency matrix.
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
pub fn eigenvector_centrality<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    max_iter: usize,
    tolerance: f64,
) -> Result<NodeMap<f64>, GraphinaException>
where
    W: Copy + PartialOrd + Into<f64>,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    if n == 0 {
        return Ok(NodeMap::new());
    }

    // Build adjacency matrix
    let mut adj = DMatrix::<f64>::zeros(n, n);
    for (u, v, w) in graph.edges() {
        let ui = u.index();
        let vi = v.index();
        let weight: f64 = (*w).into();
        adj[(ui, vi)] = weight;
        // Assuming undirected, but for directed it's fine
        adj[(vi, ui)] = weight;
    }

    // Power iteration
    let mut x = DVector::<f64>::from_element(n, 1.0 / (n as f64));
    for _ in 0..max_iter {
        let x_new = &adj * &x;
        let norm = x_new.norm();
        if norm == 0.0 {
            return Err(GraphinaException::new(
                "Eigenvector centrality failed: zero vector",
            ));
        }
        let x_new = x_new / norm;
        if (&x_new - &x).norm() < tolerance {
            x = x_new;
            break;
        }
        x = x_new;
    }

    let mut centrality = NodeMap::new();
    for (i, &val) in x.iter().enumerate() {
        let node = NodeId::new(petgraph::graph::NodeIndex::new(i));
        centrality.insert(node, val);
    }
    Ok(centrality)
}
