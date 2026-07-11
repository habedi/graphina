//! Spectral clustering algorithms.
//!
//! This module provides spectral clustering for community detection.

use crate::core::error::{GraphinaError, Result};
use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use nalgebra::DMatrix;
use rand::prelude::*;
use rand::{SeedableRng, rngs::StdRng};
use std::collections::HashMap;

/// Private helper: Create a seeded RNG from an optional seed.
fn create_rng(seed: Option<u64>) -> StdRng {
    match seed {
        Some(s) => StdRng::seed_from_u64(s),
        None => StdRng::seed_from_u64(crate::core::rng::entropy_seed()),
    }
}

/// Production-level Spectral embeddings.
///
/// Constructs the unnormalized Laplacian from the weighted adjacency matrix,
/// computes the smallest `k` eigenvectors via nalgebra’s symmetric eigen-decomposition,
/// and returns, for each node _n_, an embedding vector of dimensionality `k` consisting
/// of the nth eigenvector value for the `k`th eigenvector.
/// If k > the number of nodes in the graph, this routine will panic.
///
/// **Time Complexity:** Dominated by the eigen-decomposition (≈ O(n³) worst-case).
///
/// # Parameters
/// - `k`: embedding dimensionality (number of eigenvectors). Must be <= |nodes|, or panic.
///
/// # Returns
/// A vector of weight vectors, where each weight vector is the embedding for the nth node.
/// Returns `GraphinaError::InvalidGraph` if k==0, k>n or graph empty.
pub fn spectral_embeddings<A, W, Ty>(graph: &BaseGraph<A, W, Ty>, k: usize) -> Result<Vec<Vec<f64>>>
where
    W: Copy + PartialOrd + Into<f64> + From<u8>,
    Ty: GraphConstructor<A, W>,
{
    // Build explicit node index mapping to avoid relying on StableGraph raw indices
    let node_list: Vec<NodeId> = graph.nodes().map(|(node, _)| node).collect();
    let n = node_list.len();
    if n == 0 {
        return Err(GraphinaError::invalid_graph(
            "SpectralEmbeddings: empty graph",
        ));
    }
    if k == 0 {
        return Err(GraphinaError::invalid_graph("SpectralEmbeddings: k=0"));
    }
    if k > n {
        return Err(GraphinaError::invalid_graph(
            "SpectralEmbeddings: k > node count",
        ));
    }
    let mut node_to_idx: HashMap<NodeId, usize> = HashMap::new();
    for (idx, &node) in node_list.iter().enumerate() {
        node_to_idx.insert(node, idx);
    }
    // Build the unnormalized Laplacian L = D - A directly in a single dense
    // matrix. Each edge subtracts its weight from the symmetric off-diagonal
    // entries and adds it to both endpoints' diagonal (degree) entries. This
    // accumulates degrees in O(E) and avoids the separate degree matrix, the
    // O(n^2) row-sum scan, and the full O(n^2) matrix subtraction the previous
    // version used. The O(n^3) symmetric eigendecomposition below is inherent to
    // the dense spectral method.
    let mut lap = DMatrix::<f64>::zeros(n, n);
    for (u, v, &w) in graph.edges() {
        let ui = node_to_idx[&u];
        let vi = node_to_idx[&v];
        let weight: f64 = w.into();
        lap[(ui, vi)] -= weight;
        lap[(vi, ui)] -= weight;
        lap[(ui, ui)] += weight;
        lap[(vi, vi)] += weight;
    }
    let eig = lap.symmetric_eigen();
    // nalgebra's symmetric eigendecomposition returns eigenpairs in no
    // particular order, so sort column indices by ascending eigenvalue to make
    // the first k columns the smallest-eigenvalue eigenvectors.
    let mut cols: Vec<usize> = (0..n).collect();
    cols.sort_by(|&a, &b| {
        eig.eigenvalues[a]
            .partial_cmp(&eig.eigenvalues[b])
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut embedding = vec![vec![0.0; k]; n];
    for (i, row) in embedding.iter_mut().enumerate() {
        for (j, val) in row.iter_mut().enumerate().take(k) {
            *val = eig.eigenvectors[(i, cols[j])];
        }
    }
    Ok(embedding)
}

/// Production-level Spectral Clustering.
///
/// Constructs the unnormalized Laplacian from the weighted adjacency matrix,
/// computes the smallest `k` eigenvectors via nalgebra’s symmetric eigen-decomposition,
/// and clusters the rows of the eigenvector matrix using a k-means routine.
///
/// **Time Complexity:** Dominated by the eigen-decomposition (≈ O(n³) worst-case).
///
/// # Parameters
/// - `k`: Number of clusters.
/// - `seed`: Optional seed for the RNG used in k-means.
///
/// # Returns
/// A vector of communities, where each community is a vector of `NodeId`s.
/// Returns `GraphinaError::InvalidGraph` if k==0, k>n or graph empty.
pub fn spectral_clustering<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    k: usize,
    seed: Option<u64>,
) -> Result<Vec<Vec<NodeId>>>
where
    W: Copy + PartialOrd + Into<f64> + From<u8>,
    Ty: GraphConstructor<A, W>,
{
    // Build mapping for safe NodeId reconstruction
    let node_list: Vec<NodeId> = graph.nodes().map(|(node, _)| node).collect();
    let embedding = spectral_embeddings(graph, k)?;
    Ok(k_means(&embedding, k, seed, &node_list))
}

/// A simple k-means routine on rows of a data matrix.
///
/// **Time Complexity:** O(max_iter * n * k * d)
///
/// # Parameters
/// - `seed`: Optional RNG seed for initialization.
///
/// # Returns
/// Clusters as a vector of `NodeId`s grouped by cluster.
fn k_means(
    data: &[Vec<f64>],
    k: usize,
    seed: Option<u64>,
    node_list: &[NodeId],
) -> Vec<Vec<NodeId>> {
    let n = data.len();
    let d = if n > 0 { data[0].len() } else { 0 };
    let mut rng = create_rng(seed);
    let mut centroids: Vec<Vec<f64>> = data.choose_multiple(&mut rng, k).cloned().collect();
    let mut assignments = vec![0; n];
    let mut changed = true;
    let max_iter = 100;
    let mut iter = 0;

    while changed && iter < max_iter {
        changed = false;
        for (i, point) in data.iter().enumerate() {
            let (best_j, _) = centroids
                .iter()
                .enumerate()
                .map(|(j, centroid)| (j, euclidean_distance(point, centroid)))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or((0, 0.0));
            if assignments[i] != best_j {
                assignments[i] = best_j;
                changed = true;
            }
        }
        let mut new_centroids = vec![vec![0.0; d]; k];
        let mut counts = vec![0; k];
        for (i, &cluster) in assignments.iter().enumerate() {
            counts[cluster] += 1;
            for (j, &val) in data[i].iter().enumerate() {
                new_centroids[cluster][j] += val;
            }
        }
        for j in 0..k {
            if counts[j] > 0 {
                for l in 0..d {
                    new_centroids[j][l] /= counts[j] as f64;
                }
            } else {
                new_centroids[j] = data[rng.random_range(0..n)].clone();
            }
        }
        centroids = new_centroids;
        iter += 1;
    }
    let mut clusters: Vec<Vec<NodeId>> = vec![Vec::new(); k];
    for (i, &cluster) in assignments.iter().enumerate() {
        clusters[cluster].push(node_list[i]);
    }
    clusters
}

/// Compute Euclidean distance between two points.
fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_spectral_embeddings_first_column_is_laplacian_null_space() {
        // For a connected graph the smallest Laplacian eigenvalue is 0 and its
        // eigenvector is constant, so a k=1 embedding must give every node the
        // same value. Unsorted eigenpairs put an arbitrary eigenvector first.
        let mut g: Graph<i32, f64> = Graph::new();
        let nodes: Vec<_> = (0..5).map(|i| g.add_node(i)).collect();
        g.add_edge(nodes[0], nodes[1], 1.0);
        g.add_edge(nodes[1], nodes[2], 1.0);
        g.add_edge(nodes[2], nodes[3], 1.0);
        g.add_edge(nodes[3], nodes[4], 1.0);
        g.add_edge(nodes[0], nodes[2], 1.0);

        let emb = spectral_embeddings(&g, 1).expect("embedding should succeed");
        let first = emb[0][0];
        for row in &emb {
            assert!(
                (row[0] - first).abs() < 1e-8,
                "k=1 embedding must be constant, got {:?}",
                emb
            );
        }
    }

    #[test]
    fn test_spectral_clustering_separates_two_triangles() {
        // Two triangles joined by a single bridge: the Fiedler vector separates
        // them, so k=2 must recover the triangles as the two clusters.
        let mut g: Graph<i32, f64> = Graph::new();
        let nodes: Vec<_> = (0..6).map(|i| g.add_node(i)).collect();
        for &(a, b) in &[(0, 1), (1, 2), (0, 2), (3, 4), (4, 5), (3, 5)] {
            g.add_edge(nodes[a], nodes[b], 1.0);
        }
        g.add_edge(nodes[2], nodes[3], 1.0);

        let clusters = spectral_clustering(&g, 2, Some(42)).expect("clustering should succeed");
        let mut sorted: Vec<Vec<usize>> = clusters
            .iter()
            .map(|c| {
                let mut ids: Vec<usize> = c.iter().map(|n| n.index()).collect();
                ids.sort_unstable();
                ids
            })
            .collect();
        sorted.sort();
        assert_eq!(
            sorted,
            vec![vec![0, 1, 2], vec![3, 4, 5]],
            "clusters must match the two triangles"
        );
    }
}
