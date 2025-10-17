//! Spectral clustering algorithms.
//!
//! This module provides spectral clustering for community detection.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use nalgebra::DMatrix;
use rand::prelude::*;
use rand::{SeedableRng, rngs::StdRng};

/// Private helper: Create a seeded RNG from an optional seed.
fn create_rng(seed: Option<u64>) -> StdRng {
    match seed {
        Some(s) => StdRng::seed_from_u64(s),
        None => StdRng::seed_from_u64(rand::random::<u64>()),
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
pub fn spectral_embeddings<A, W, Ty>(graph: &BaseGraph<A, W, Ty>, k: usize) -> Vec<Vec<f64>>
where
    W: Copy + PartialOrd + Into<f64> + From<u8>,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    let mut adj = DMatrix::<f64>::zeros(n, n);
    for (u, v, &w) in graph.edges() {
        let ui = u.index();
        let vi = v.index();
        let weight: f64 = w.into();
        adj[(ui, vi)] = weight;
        adj[(vi, ui)] = weight;
    }
    let mut deg = DMatrix::<f64>::zeros(n, n);
    for i in 0..n {
        let d: f64 = (0..n).map(|j| adj[(i, j)]).sum();
        deg[(i, i)] = d;
    }
    let lap = &deg - &adj;
    let eig = lap.symmetric_eigen();
    let mut embedding = vec![vec![0.0; k]; n];
    for (i, row) in embedding.iter_mut().enumerate() {
        for (j, val) in row.iter_mut().enumerate().take(k) {
            *val = eig.eigenvectors[(i, j)];
        }
    }
    embedding
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
pub fn spectral_clustering<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    k: usize,
    seed: Option<u64>,
) -> Vec<Vec<NodeId>>
where
    W: Copy + PartialOrd + Into<f64> + From<u8>,
    Ty: GraphConstructor<A, W>,
{
    let embedding = spectral_embeddings(graph, k);
    k_means(&embedding, k, seed)
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
fn k_means(data: &[Vec<f64>], k: usize, seed: Option<u64>) -> Vec<Vec<NodeId>> {
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
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();
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
        clusters[cluster].push(NodeId::new(petgraph::graph::NodeIndex::new(i)));
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
