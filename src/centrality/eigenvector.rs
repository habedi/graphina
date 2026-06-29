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
        return Ok(NodeMap::default());
    }

    // Fast path: graphs with no edges yield uniform centrality
    if graph.edge_count() == 0 {
        let mut centrality = NodeMap::default();
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

    // Store the adjacency as a sparse edge list rather than a dense n x n matrix.
    // Each entry is (row, col, weight), and the operator product accumulates
    // `out[row] += weight * x[col]`. This costs O(E) per iteration and O(E)
    // memory instead of O(n^2). For directed graphs the entry orients so an
    // incoming edge influences the target (v influences u); for undirected
    // graphs both orientations are stored to keep the operator symmetric.
    let directed = graph.is_directed();
    let mut adj: Vec<(usize, usize, f64)> = Vec::with_capacity(graph.edge_count());
    for (u, v, w) in graph.edges() {
        let ui = node_to_idx[&u];
        let vi = node_to_idx[&v];
        let weight: f64 = (*w).into();

        if directed {
            // For directed graphs: v influences u (incoming edges)
            adj.push((vi, ui, weight));
        } else {
            // For undirected graphs the operator is symmetric.
            adj.push((ui, vi, weight));
            adj.push((vi, ui, weight));
        }
    }

    // Sparse power iteration. For undirected graphs iterate on the shifted
    // operator (A + I): shifting by the identity moves every eigenvalue up by one
    // without changing the eigenvectors, which makes the dominant eigenvalue
    // strictly largest in magnitude and removes the |lambda_max| == |lambda_min|
    // oscillation that bipartite graphs cause. This replaces the dense symmetric
    // eigendecomposition the undirected path used before. Directed graphs iterate
    // on A itself and keep the zero-norm and sign-oscillation guards, since the
    // shift would make a defective directed operator converge only linearly.
    let shift = if directed { 0.0 } else { 1.0 };
    let mut x = vec![1.0 / (n as f64).sqrt(); n];
    let mut converged = false;

    for iter in 0..max_iter {
        // y = (A + shift * I) x
        let mut y: Vec<f64> = x.iter().map(|&xi| shift * xi).collect();
        for &(row, col, weight) in &adj {
            y[row] += weight * x[col];
        }

        let norm: f64 = y.iter().map(|v| v * v).sum::<f64>().sqrt();
        if norm < 1e-10 {
            // Degenerate operator (disconnected, all-zero weights, or a defective
            // directed structure): fall back to a uniform distribution.
            let mut centrality = NodeMap::default();
            let uniform_value = 1.0 / n as f64;
            for &node in &node_list {
                centrality.insert(node, uniform_value);
            }
            return Ok(centrality);
        }

        let mut diff_sq = 0.0;
        let mut diff_neg_sq = 0.0;
        for (xi, yi) in x.iter().zip(&y) {
            let normalized = yi / norm;
            let d = normalized - xi;
            diff_sq += d * d;
            let dn = normalized + xi;
            diff_neg_sq += dn * dn;
        }
        for (xi, yi) in x.iter_mut().zip(&y) {
            *xi = yi / norm;
        }

        if diff_sq.sqrt() < tolerance {
            converged = true;
            break;
        }

        // Directed graphs can oscillate between x and -x on a negative dominant
        // eigenvalue; detect the sign flip and converge on the magnitudes.
        if directed && iter > 10 && diff_neg_sq.sqrt() < tolerance {
            converged = true;
            break;
        }
    }

    if !converged {
        return Err(GraphinaError::convergence_failed(
            max_iter,
            "Eigenvector centrality failed to converge within maximum iterations",
        ));
    }

    // Normalize so values sum to the number of nodes, matching the prior
    // convention, and report magnitudes (eigenvector orientation is arbitrary).
    let sum: f64 = x.iter().map(|v| v.abs()).sum();
    if sum > 0.0 {
        for v in x.iter_mut() {
            *v = v.abs() * (n as f64) / sum;
        }
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
}
