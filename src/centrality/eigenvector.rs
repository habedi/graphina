//! Eigenvector centrality algorithms.
//!
//! This module provides eigenvector centrality measures.
//!
//! Convention: functions in this module return `Result<_, crate::core::error::GraphinaError>`
//! to surface convergence issues and aid observability and error propagation.

use crate::core::error::{GraphinaError, Result};
use crate::core::types::{BaseGraph, GraphConstructor, NodeId, NodeMap};

/// Eigenvector centrality: computes the eigenvector corresponding to the largest eigenvalue
/// of the adjacency matrix.
///
/// For directed graphs, computes the left eigenvector (based on incoming edges).
/// For undirected graphs, computes the standard eigenvector centrality. Both use
/// power iteration on `A + I`, which converges even when the adjacency matrix has
/// an eigenvalue of maximum modulus that is negative or complex.
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
/// Returns an error if the power iteration fails to converge. The returned vector
/// has unit L2 norm and nonnegative entries, as in NetworkX. An empty graph yields
/// an empty map, and a graph with no edges yields the uniform unit vector (every
/// entry `1/sqrt(n)`).
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
        return Ok(NodeMap::default());
    }

    let node_list: Vec<NodeId> = graph.nodes().map(|(node, _)| node).collect();
    // Every node is equivalent when there are no edges, and the uniform unit
    // vector is the fixed point of the (A + I) iteration, matching NetworkX.
    let uniform = || -> NodeMap<f64> {
        let value = 1.0 / (n as f64).sqrt();
        node_list.iter().map(|&node| (node, value)).collect()
    };
    if graph.edge_count() == 0 {
        return Ok(uniform());
    }

    let mut node_to_idx = std::collections::HashMap::new();
    for (idx, &node) in node_list.iter().enumerate() {
        node_to_idx.insert(node, idx);
    }

    // Entries (row, col, weight) of the operator applied each iteration:
    // y[row] += weight * x[col]. For a directed edge u -> v the centrality of v
    // accumulates that of its predecessor u (the left eigenvector, as in
    // NetworkX). An undirected edge contributes in both directions, and a
    // self-loop is a single diagonal entry in either case.
    let directed = graph.is_directed();
    let mut adj: Vec<(usize, usize, f64)> = Vec::with_capacity(2 * graph.edge_count());
    for (u, v, w) in graph.edges() {
        let ui = node_to_idx[&u];
        let vi = node_to_idx[&v];
        let weight: f64 = (*w).into();
        adj.push((vi, ui, weight));
        if !directed && ui != vi {
            adj.push((ui, vi, weight));
        }
    }

    // Power iteration on (A + I). The shift keeps the eigenvectors but moves the
    // spectrum, so the iteration converges even when an eigenvalue of maximum
    // modulus is negative or complex (bipartite or periodic graphs).
    let mut x = vec![1.0 / (n as f64).sqrt(); n];
    let mut y = vec![0.0; n];
    for _ in 0..max_iter {
        y.copy_from_slice(&x);
        for &(row, col, weight) in &adj {
            y[row] += weight * x[col];
        }

        let norm: f64 = y.iter().map(|v| v * v).sum::<f64>().sqrt();
        if norm < 1e-10 {
            return Ok(uniform());
        }

        let mut diff_sq = 0.0;
        for (xi, yi) in x.iter_mut().zip(&y) {
            let next = yi / norm;
            let d = next - *xi;
            diff_sq += d * d;
            *xi = next;
        }

        if diff_sq.sqrt() < tolerance {
            // `x` has unit L2 norm and nonnegative entries, the normalization
            // NetworkX returns.
            return Ok(node_list.iter().copied().zip(x).collect());
        }
    }

    Err(GraphinaError::convergence_failed(
        max_iter,
        "Eigenvector centrality failed to converge within maximum iterations",
    ))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_eigenvector_with_deleted_nodes() {
        use crate::centrality::eigenvector::eigenvector_centrality;
        use crate::core::types::Graph;

        let mut graph: Graph<i32, f64> = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);
        let n5 = graph.add_node(5);

        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n1, n3, 1.0);
        graph.add_edge(n1, n4, 1.0);
        graph.add_edge(n1, n5, 1.0);

        graph.remove_node(n5);

        let eig = eigenvector_centrality(&graph, 100, 1e-6).unwrap();

        assert!(eig[&n1] > eig[&n2]);
        assert!(eig[&n1] > eig[&n3]);
        assert!(eig[&n1] > eig[&n4]);
        assert!(!eig.contains_key(&n5));
    }

    #[test]
    fn test_eigenvector_issue_21_regression() {
        use crate::core::types::Graph;
        // Regression test for Issue #21: "Using Vec to return centrality might cause error for graph with removed node"
        // Ensures that creating a "gap" in NodeIds by removing an intermediate node doesn't cause out-of-bounds access.
        use crate::centrality::eigenvector::eigenvector_centrality;

        let mut g = Graph::<i32, f64>::new();

        let n0 = g.add_node(0);
        let n1 = g.add_node(1);
        let n2 = g.add_node(4);
        let n3 = g.add_node(9);

        g.add_edge(n0, n3, 1.0);
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n3, n1, 1.0);

        // Remove an intermediate node (n1) to create a gap if IDs were treated as dense indices
        g.remove_node(n1);

        // This should not panic
        let result = eigenvector_centrality(&g, 1000, 1e-6);
        assert!(result.is_ok());

        let centrality = result.unwrap();
        // n1 should not be in the result
        assert!(!centrality.contains_key(&n1));
        // Remaining nodes should be present
        assert!(centrality.contains_key(&n0));
        assert!(centrality.contains_key(&n2));
        assert!(centrality.contains_key(&n3));
    }
    use super::eigenvector_centrality;
    use crate::core::types::{Digraph, Graph};

    #[test]
    fn eigenvector_directed_vs_undirected_basic() {
        // Directed two-cycle 0 <-> 1. (A single directed edge 0 -> 1 has no
        // positive dominant eigenvector, and NetworkX fails to converge on it
        // too, so it is not a meaningful input here.)
        let mut dg: Digraph<i32, f64> = Digraph::new();
        let n0 = dg.add_node(0);
        let n1 = dg.add_node(1);
        dg.add_edge(n0, n1, 1.0);
        dg.add_edge(n1, n0, 1.0);
        let c_dir = eigenvector_centrality(&dg, 100, 1e-9).unwrap();
        assert!(c_dir[&n0] > 0.0);
        assert!((c_dir[&n0] - c_dir[&n1]).abs() < 1e-6);

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

    #[test]
    fn test_eigenvector_bipartite_converges() {
        // A 4-cycle is bipartite, so the adjacency matrix has eigenvalues +2 and
        // -2 of equal magnitude. A plain power iteration on A oscillates and never
        // settles; the sparse iteration on (A + I) breaks the tie and converges.
        // By symmetry every node on the cycle has equal centrality.
        let mut g: Graph<i32, f64> = Graph::new();
        let nodes: Vec<_> = (0..4).map(|i| g.add_node(i)).collect();
        for i in 0..4 {
            g.add_edge(nodes[i], nodes[(i + 1) % 4], 1.0);
        }

        let c = eigenvector_centrality(&g, 1000, 1e-9).unwrap();
        for &node in &nodes {
            assert!(c[&node] > 0.0);
            assert!((c[&node] - c[&nodes[0]]).abs() < 1e-6);
        }
    }

    #[test]
    fn test_eigenvector_undirected_self_loop_counts_once() {
        use crate::centrality::eigenvector::eigenvector_centrality;
        use crate::core::types::Graph;

        // Adjacency [[1, 1], [1, 0]] has leading eigenvector (phi, 1) with
        // phi the golden ratio. Doubling the self-loop would give (1 + sqrt 2, 1).
        let mut g = Graph::<i32, f64>::new();
        let a = g.add_node(0);
        let b = g.add_node(1);
        g.add_edge(a, a, 1.0);
        g.add_edge(a, b, 1.0);
        let c = eigenvector_centrality(&g, 10_000, 1e-12).unwrap();
        let phi = (1.0 + 5f64.sqrt()) / 2.0;
        assert!(
            (c[&a] / c[&b] - phi).abs() < 1e-6,
            "ratio = {}",
            c[&a] / c[&b]
        );
    }

    #[test]
    fn test_eigenvector_has_unit_l2_norm_like_networkx() {
        use crate::centrality::eigenvector::eigenvector_centrality;
        use crate::core::types::Graph;

        // NetworkX documents the path graph P4 as (0.37, 0.60, 0.60, 0.37).
        let mut g = Graph::<i32, f64>::new();
        let n: Vec<_> = (0..4).map(|i| g.add_node(i)).collect();
        for i in 0..3 {
            g.add_edge(n[i], n[i + 1], 1.0);
        }
        let c = eigenvector_centrality(&g, 10_000, 1e-12).unwrap();
        let norm: f64 = c.values().map(|v| v * v).sum::<f64>().sqrt();
        assert!((norm - 1.0).abs() < 1e-9, "norm = {norm}");
        assert!((c[&n[0]] - 0.3717).abs() < 1e-3, "end = {}", c[&n[0]]);
        assert!((c[&n[1]] - 0.6015).abs() < 1e-3, "inner = {}", c[&n[1]]);
    }

    #[test]
    fn test_eigenvector_no_edges_is_uniform_unit_vector() {
        use crate::centrality::eigenvector::eigenvector_centrality;
        use crate::core::types::Graph;

        let mut g = Graph::<i32, f64>::new();
        let n: Vec<_> = (0..3).map(|i| g.add_node(i)).collect();
        let c = eigenvector_centrality(&g, 100, 1e-9).unwrap();
        for node in &n {
            assert!((c[node] - 1.0 / 3f64.sqrt()).abs() < 1e-12);
        }
    }

    #[test]
    fn test_eigenvector_directed_matches_networkx() {
        use crate::centrality::eigenvector::eigenvector_centrality;
        use crate::core::types::Digraph;

        // Cycle 0 -> 1 -> 2 -> 0 with a source node 3 -> 0. NetworkX gives the
        // three cycle nodes 1/sqrt(3) each and node 3 zero: nothing points to it.
        let mut g = Digraph::<i32, f64>::new();
        let n: Vec<_> = (0..4).map(|i| g.add_node(i)).collect();
        g.add_edge(n[0], n[1], 1.0);
        g.add_edge(n[1], n[2], 1.0);
        g.add_edge(n[2], n[0], 1.0);
        g.add_edge(n[3], n[0], 1.0);
        let c = eigenvector_centrality(&g, 10_000, 1e-12).unwrap();
        for i in 0..3 {
            assert!(
                (c[&n[i]] - 1.0 / 3f64.sqrt()).abs() < 1e-6,
                "node {i} = {}",
                c[&n[i]]
            );
        }
        assert!(c[&n[3]].abs() < 1e-6, "source = {}", c[&n[3]]);
    }
}
