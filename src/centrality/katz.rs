//! Katz centrality algorithms.
//!
//! This module provides Katz centrality measures.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId, NodeMap};
use nalgebra::{DMatrix, DVector};

/// Katz centrality: computes the relative influence of a node within a network
/// by measuring the number of walks of length k between a pair of nodes.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `alpha`: attenuation factor.
/// * `beta`: weight for each node.
/// * `max_iter`: maximum number of iterations.
/// * `tolerance`: convergence tolerance.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing Katz centralities of each node in the graph.
pub fn katz_centrality<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    alpha: f64,
    beta: Option<&dyn Fn(NodeId) -> f64>,
    max_iter: usize,
    tolerance: f64,
) -> NodeMap<f64>
where
    W: Copy + PartialOrd + Into<f64>,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    if n == 0 {
        return NodeMap::new();
    }

    // Build adjacency matrix
    let mut adj = DMatrix::<f64>::zeros(n, n);
    for (u, v, w) in graph.edges() {
        let ui = u.index();
        let vi = v.index();
        let weight: f64 = (*w).into();
        adj[(ui, vi)] = weight;
    }

    // Initial vector
    let mut x = DVector::<f64>::from_element(n, 0.0);
    let beta_vec = if let Some(b) = beta {
        DVector::from_fn(n, |idx, _| {
            b(NodeId::new(petgraph::graph::NodeIndex::new(idx)))
        })
    } else {
        DVector::from_element(n, 1.0)
    };

    for _ in 0..max_iter {
        let x_new = alpha * &adj * &x + &beta_vec;
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
    centrality
}
