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

    // Build adjacency matrix; for directed graphs use incoming adjacency (A^T).
    let mut adj = DMatrix::<f64>::zeros(n, n);
    for (u, v, w) in graph.edges() {
        let ui = u.index();
        let vi = v.index();
        let weight: f64 = (*w).into();
        if graph.is_directed() {
            // A^T: accumulate inbound influence
            adj[(vi, ui)] = weight;
        } else {
            // Undirected: symmetric adjacency
            adj[(ui, vi)] = weight;
            adj[(vi, ui)] = weight;
        }
    }

    // Power iteration
    let mut x = DVector::<f64>::from_element(n, 1.0 / (n as f64));
    for _ in 0..max_iter {
        let x_new = &adj * &x;
        let norm = x_new.norm();
        if norm == 0.0 {
            // No new contribution; treat as converged to previous vector
            break;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{Digraph, Graph};

    #[test]
    fn eigenvector_directed_vs_undirected_basic() {
        // Directed: 0 -> 1
        let mut dg: Digraph<i32, f64> = Digraph::new();
        let n0 = dg.add_node(0);
        let n1 = dg.add_node(1);
        dg.add_edge(n0, n1, 1.0);
        let c_dir = eigenvector_centrality(&dg, 100, 1e-9).unwrap();
        // Node 1 should have higher centrality than node 0
        assert!(c_dir[&n1] > c_dir[&n0]);

        // Undirected: 0 -- 1
        let mut ug: Graph<i32, f64> = Graph::new();
        let m0 = ug.add_node(0);
        let m1 = ug.add_node(1);
        ug.add_edge(m0, m1, 1.0);
        let c_und = eigenvector_centrality(&ug, 100, 1e-9).unwrap();
        // Symmetric graph => equal centralities
        let diff = (c_und[&m0] - c_und[&m1]).abs();
        assert!(diff < 1e-9);
    }
}
