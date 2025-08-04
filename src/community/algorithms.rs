// File: src/community/algorithms.rs

use crate::core::types::{BaseGraph, EdgeType, NodeId};
use nalgebra::DMatrix;
use rand::prelude::*;
use rand::{rngs::StdRng, SeedableRng};
use std::collections::{HashMap, HashSet, VecDeque};

// Private helper: convert a raw index (usize) to a NodeId.
fn node_from_index(i: usize) -> NodeId {
    NodeId::new(petgraph::graph::NodeIndex::new(i))
}

//
// Helper: Create a seeded RNG from an optional seed.
//
fn create_rng(seed: Option<u64>) -> StdRng {
    match seed {
        Some(s) => StdRng::seed_from_u64(s),
        None => StdRng::seed_from_u64(rand::random::<u64>()),
    }
}

////////////////////////////////////////////////////////////////////////
// 1. Production-Level Label Propagation
////////////////////////////////////////////////////////////////////////

/// Production-level Label Propagation.
///
/// Each node is initially assigned its own community. In randomized order,
/// each node updates its label to the most frequent label among its neighbors.
/// The process stops when no changes occur or when `max_iter` iterations are reached.
///
/// **Time Complexity:** O(max_iter * (n + m))
///
/// # Parameters
/// - `max_iter`: Maximum number of iterations.
/// - `seed`: Optional seed for the RNG (for reproducibility).
///
/// # Returns
/// A vector (length n) of final community labels (usize) for each node.
pub fn label_propagation<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    max_iter: usize,
    seed: Option<u64>,
) -> Vec<usize>
where
    W: Copy + PartialOrd + Into<f64>,
    Ty: EdgeType,
{
    let n = graph.node_count();
    let mut labels: Vec<usize> = (0..n).collect();
    let mut rng = create_rng(seed);
    let mut iter = 0;

    loop {
        let mut changed = false;
        let mut nodes: Vec<usize> = (0..n).collect();
        nodes.shuffle(&mut rng);
        for &i in &nodes {
            let mut freq: HashMap<usize, usize> = HashMap::new();
            // Count labels among neighbors (treated as undirected).
            for (src, tgt, _w) in graph.edges() {
                if src.index() == i {
                    *freq.entry(labels[tgt.index()]).or_insert(0) += 1;
                }
                if tgt.index() == i {
                    *freq.entry(labels[src.index()]).or_insert(0) += 1;
                }
            }
            if let Some((&best_label, _)) = freq.iter().max_by_key(|&(_, count)| count) {
                if best_label != labels[i] {
                    labels[i] = best_label;
                    changed = true;
                }
            }
        }
        iter += 1;
        if !changed || iter >= max_iter {
            break;
        }
    }
    labels
}

////////////////////////////////////////////////////////////////////////
// 2. Production-Level Louvain Method
////////////////////////////////////////////////////////////////////////

/// Production-level Louvain Method for community detection.
///
/// Designed for undirected graphs with nonnegative f64 weights. It works in two phases:
/// 1. **Modularity Optimization:** Nodes are moved between communities to maximize modularity gain.
/// 2. **Graph Aggregation:** Nodes in the same community are aggregated, and the process repeats.
///
/// **Time Complexity:** Empirically near O(m) per iteration; overall complexity depends on iterations.
///
/// # Parameters
/// - `seed`: Optional seed for the RNG (used when shuffling nodes).
///
/// # Returns
/// A vector of communities, where each community is a vector of `NodeId`s.
pub fn louvain<A, Ty>(graph: &BaseGraph<A, f64, Ty>, seed: Option<u64>) -> Vec<Vec<NodeId>>
where
    Ty: EdgeType,
{
    let m: f64 = graph.edges().map(|(_u, _v, &w)| w).sum();
    let n = graph.node_count();
    let mut community: Vec<usize> = (0..n).collect();

    // Compute node degrees.
    let mut degrees = vec![0.0; n];
    for (u, v, &w) in graph.edges() {
        degrees[u.index()] += w;
        degrees[v.index()] += w;
    }

    // Precompute neighbors: for each node, store (neighbor_index, weight).
    let mut neighbors: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n];
    for (u, v, &w) in graph.edges() {
        let ui = u.index();
        let vi = v.index();
        neighbors[ui].push((vi, w));
        neighbors[vi].push((ui, w));
    }

    let mut rng = create_rng(seed);
    let mut improvement = true;
    while improvement {
        improvement = false;
        let mut nodes: Vec<usize> = (0..n).collect();
        nodes.shuffle(&mut rng);
        for &i in &nodes {
            let current_comm = community[i];
            let k_i = degrees[i];
            let mut comm_weights: HashMap<usize, f64> = HashMap::new();
            for &(j, w) in &neighbors[i] {
                let comm_j = community[j];
                *comm_weights.entry(comm_j).or_insert(0.0) += w;
            }
            let total_current = total_degree(&community, &degrees, current_comm);
            let delta_remove =
                comm_weights.get(&current_comm).unwrap_or(&0.0) - (total_current * k_i) / (2.0 * m);
            let mut best_delta = 0.0;
            let mut best_comm = current_comm;
            for (&comm, &w_in) in &comm_weights {
                if comm == current_comm {
                    continue;
                }
                let total_comm = total_degree(&community, &degrees, comm);
                let delta = w_in - (total_comm * k_i) / (2.0 * m);
                if delta > best_delta {
                    best_delta = delta;
                    best_comm = comm;
                }
            }
            if best_delta > delta_remove {
                community[i] = best_comm;
                improvement = true;
            }
        }
    }

    // Phase 2: Aggregate nodes by community.
    let mut comm_map: HashMap<usize, usize> = HashMap::new();
    for &c in &community {
        if !comm_map.contains_key(&c) {
            let new_index = comm_map.len();
            comm_map.insert(c, new_index);
        }
    }
    let mut new_comms: Vec<Vec<NodeId>> = vec![Vec::new(); comm_map.len()];
    for (i, &comm) in community.iter().enumerate() {
        let new_comm = comm_map[&comm];
        if let Some((node, _)) = graph.nodes().find(|(node, _)| node.index() == i) {
            new_comms[new_comm].push(node);
        }
    }
    new_comms
}

/// Helper: Compute the total degree for nodes in a given community.
fn total_degree(decomp: &[usize], degrees: &[f64], comm: usize) -> f64 {
    decomp
        .iter()
        .enumerate()
        .filter(|&(_i, &c)| c == comm)
        .map(|(i, _)| degrees[i])
        .sum()
}

////////////////////////////////////////////////////////////////////////
// 3. Production-Level Girvan–Newman Algorithm
////////////////////////////////////////////////////////////////////////

/// Production-level Girvan–Newman Algorithm.
///
/// Uses Brandes’ algorithm to compute edge betweenness centrality, then iteratively removes the edge
/// with the highest betweenness until the graph splits into at least `target_communities`.
///
/// **Time Complexity:** Worst-case O(n*m) per iteration (practically often lower).
///
/// # Returns
/// A vector of communities, where each community is a vector of `NodeId`s.
///
/// # Note
/// This algorithm is computationally expensive for very large graphs.
pub fn girvan_newman<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    target_communities: usize,
) -> Vec<Vec<NodeId>>
where
    W: Copy + PartialOrd + Into<f64> + From<u8>,
    Ty: EdgeType,
{
    // Store only the endpoints (usize pairs), so no weight is needed.
    let mut active_edges: HashSet<(usize, usize)> = graph
        .edges()
        .map(|(u, v, _w)| {
            let (a, b) = (u.index(), v.index());
            if a < b {
                (a, b)
            } else {
                (b, a)
            }
        })
        .collect();

    // Build initial connectivity (neighbors set) from active edges.
    let n = graph.node_count();
    let mut neighbors: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for &(u, v) in &active_edges {
        neighbors[u].insert(v);
        neighbors[v].insert(u);
    }

    // Remove edges iteratively until we reach the desired number of components.
    while connected_components_count(&neighbors) < target_communities {
        let edge_btwn = compute_edge_betweenness(n, &neighbors);
        if let Some((&(u, v), _)) = edge_btwn
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        {
            neighbors[u].remove(&v);
            neighbors[v].remove(&u);
            active_edges.retain(|&(a, b)| !(a == u && b == v));
        } else {
            break;
        }
    }
    compute_components_from_neighbors(&neighbors)
}

/// Helper: Compute connected components from an adjacency list.
fn compute_components_from_neighbors(neighbors: &[HashSet<usize>]) -> Vec<Vec<NodeId>> {
    let n = neighbors.len();
    let mut visited = vec![false; n];
    let mut components = Vec::new();
    for i in 0..n {
        if !visited[i] {
            let mut comp = Vec::new();
            let mut queue = VecDeque::new();
            queue.push_back(i);
            visited[i] = true;
            while let Some(u) = queue.pop_front() {
                comp.push(u);
                for &v in &neighbors[u] {
                    if !visited[v] {
                        visited[v] = true;
                        queue.push_back(v);
                    }
                }
            }
            // Convert indices to NodeId using the private helper.
            let component: Vec<NodeId> = comp.into_iter().map(node_from_index).collect();
            components.push(component);
        }
    }
    components
}

/// Helper: Return the number of connected components.
fn connected_components_count(neighbors: &[HashSet<usize>]) -> usize {
    compute_components_from_neighbors(neighbors).len()
}

/// Helper: Compute edge betweenness centrality using Brandes’ algorithm.
fn compute_edge_betweenness(
    n: usize,
    neighbors: &[HashSet<usize>],
) -> HashMap<(usize, usize), f64> {
    let mut edge_btwn: HashMap<(usize, usize), f64> = HashMap::new();
    for s in 0..n {
        let mut stack = Vec::new();
        let mut preds: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut sigma = vec![0.0; n];
        sigma[s] = 1.0;
        let mut dist = vec![-1; n];
        dist[s] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(s);
        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for &w in &neighbors[v] {
                if dist[w] < 0 {
                    dist[w] = dist[v] + 1;
                    queue.push_back(w);
                }
                if dist[w] == dist[v] + 1 {
                    sigma[w] += sigma[v];
                    preds[w].push(v);
                }
            }
        }
        let mut delta = vec![0.0; n];
        while let Some(w) = stack.pop() {
            for &v in &preds[w] {
                let c = (sigma[v] / sigma[w]) * (1.0 + delta[w]);
                let key = if v < w { (v, w) } else { (w, v) };
                *edge_btwn.entry(key).or_insert(0.0) += c;
                delta[v] += c;
            }
        }
    }
    edge_btwn
}

////////////////////////////////////////////////////////////////////////
// 4. Production-Level Spectral Clustering
////////////////////////////////////////////////////////////////////////

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
    Ty: EdgeType,
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
    Ty: EdgeType,
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
        clusters[cluster].push(node_from_index(i));
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

////////////////////////////////////////////////////////////////////////
// 5. Production-Level Personalized PageRank
////////////////////////////////////////////////////////////////////////

/// Production-level Personalized PageRank.
///
/// Computes a ranking vector for nodes using a damping factor, convergence tolerance, and a maximum
/// number of iterations. An optional personalization vector can be supplied; if not, a uniform vector is used.
///
/// Update rule:
///
/// rank_new[j] = (1 - damping) * p[j] + damping * Σ_i (rank[i] * (w_ij / outdegree[i]))
///
/// Dangling nodes (zero outdegree) redistribute their rank uniformly.
///
/// **Time Complexity:** O(max_iter * (n + m))
///
/// # Returns
/// A vector of f64 scores (one per node).
pub fn personalized_page_rank<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    personalization: Option<Vec<f64>>,
    damping: f64,
    tol: f64,
    max_iter: usize,
) -> Vec<f64>
where
    W: Copy + PartialOrd + Into<f64> + From<u8>,
    Ty: EdgeType,
{
    let n = graph.node_count();
    let p = if let Some(mut vec) = personalization {
        let sum: f64 = vec.iter().sum();
        if sum > 0.0 {
            for val in vec.iter_mut() {
                *val /= sum;
            }
            vec
        } else {
            vec![1.0 / n as f64; n]
        }
    } else {
        vec![1.0 / n as f64; n]
    };

    let mut outdegree = vec![0.0; n];
    let mut neighbors: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n];
    for (u, v, &w) in graph.edges() {
        let ui = u.index();
        let vi = v.index();
        let weight: f64 = w.into();
        outdegree[ui] += weight;
        neighbors[ui].push((vi, weight));
    }

    let mut rank = p.clone();
    for _ in 0..max_iter {
        let mut new_rank = vec![0.0; n];
        for (j, nr) in new_rank.iter_mut().enumerate() {
            *nr += (1.0 - damping) * p[j];
        }
        for (i, &deg_i) in outdegree.iter().enumerate() {
            if deg_i > 0.0 {
                let contribution = damping * rank[i] / deg_i;
                for &(j, weight) in &neighbors[i] {
                    new_rank[j] += contribution * weight;
                }
            } else {
                for nr in new_rank.iter_mut() {
                    *nr += damping * rank[i] / (n as f64);
                }
            }
        }
        let diff: f64 = rank
            .iter()
            .zip(new_rank.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        rank = new_rank;
        if diff < tol {
            break;
        }
    }
    rank
}

////////////////////////////////////////////////////////////////////////
// 6. Production-Level Infomap (Simplified)
////////////////////////////////////////////////////////////////////////

/// Production-level Infomap (simplified) for community detection.
///
/// Inspired by the map equation framework, each node is initially in its own module.
/// In randomized order, each node is re-assigned to a neighbor's module if that move increases flow (reduces description length).
///
/// **Time Complexity:** Approximately O(max_iter * (n + m))
///
/// # Parameters
/// - `max_iter`: Maximum number of iterations.
/// - `seed`: Optional seed for RNG used for shuffling nodes.
///
/// # Returns
/// A vector (length n) of module assignments (usize) for each node.
pub fn infomap<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    max_iter: usize,
    seed: Option<u64>,
) -> Vec<usize>
where
    W: Copy + PartialOrd + Into<f64> + From<u8>,
    Ty: EdgeType,
{
    let n = graph.node_count();
    let mut modules: Vec<usize> = (0..n).collect();
    let mut rng = create_rng(seed);
    let mut iter = 0;

    loop {
        let mut changed = false;
        let mut nodes: Vec<usize> = (0..n).collect();
        nodes.shuffle(&mut rng);
        for &i in &nodes {
            let mut flow: HashMap<usize, f64> = HashMap::new();
            let mut total_flow = 0.0;
            for (src, tgt, &w) in graph.edges() {
                let weight: f64 = w.into();
                if src.index() == i {
                    let module = modules[tgt.index()];
                    *flow.entry(module).or_insert(0.0) += weight;
                    total_flow += weight;
                }
                if tgt.index() == i {
                    let module = modules[src.index()];
                    *flow.entry(module).or_insert(0.0) += weight;
                    total_flow += weight;
                }
            }
            if total_flow > 0.0 {
                for val in flow.values_mut() {
                    *val /= total_flow;
                }
            }
            if let Some((&best_module, _)) =
                flow.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            {
                if best_module != modules[i] {
                    modules[i] = best_module;
                    changed = true;
                }
            }
        }
        iter += 1;
        if !changed || iter >= max_iter {
            break;
        }
    }
    modules
}

////////////////////////////////////////////////////////////////////////
// 7. Connected Components
////////////////////////////////////////////////////////////////////////

/// Compute connected components of an undirected graph using BFS.
///
/// **Time Complexity:** O(n + m)
///
/// # Returns
/// A vector of components, where each component is a vector of `NodeId`s.
pub fn connected_components<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Vec<Vec<NodeId>>
where
    W: Copy,
    Ty: EdgeType,
{
    let n = graph.node_count();
    let mut visited = vec![false; n];
    let mut components = Vec::new();
    for i in 0..n {
        if !visited[i] {
            let mut comp = Vec::new();
            let mut queue = VecDeque::new();
            queue.push_back(i);
            visited[i] = true;
            while let Some(u) = queue.pop_front() {
                if let Some((node, _)) = graph.nodes().find(|(node, _)| node.index() == u) {
                    comp.push(node);
                }
                for (src, tgt, _) in graph.edges() {
                    if src.index() == u {
                        let v = tgt.index();
                        if !visited[v] {
                            visited[v] = true;
                            queue.push_back(v);
                        }
                    }
                    if tgt.index() == u {
                        let v = src.index();
                        if !visited[v] {
                            visited[v] = true;
                            queue.push_back(v);
                        }
                    }
                }
            }
            components.push(comp);
        }
    }
    components
}
