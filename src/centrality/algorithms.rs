//! Centrality algorithms module.
//!
//! This module provides implementations for a selection of centrality measures.
//! Measures included are:
//! - Degree centrality (total, in–, out–)
//! - Eigenvector centrality (wrapper: takes graph and max_iter)
//! - Katz centrality (wrapper: takes graph, alpha, beta, max_iter)
//! - Closeness centrality (using Dijkstra’s algorithm)
//! - PageRank (wrapper: takes graph, damping, max_iter)
//! - Betweenness centrality (node and edge)
//! - Harmonic centrality
//! - Local and global reaching centrality
//! - VoteRank
//! - Laplacian centrality

use petgraph::graph::NodeIndex;

use crate::core::exceptions::GraphinaException;
use crate::core::paths::dijkstra;
use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::{HashMap, VecDeque};

//
// -----------------------------
// Degree Centralities
// -----------------------------
//

/// Degree centrality: sum of a node’s in–degree and out–degree.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
///
/// # Returns
///
/// a vector of `f64` representing out degree centralities of each node in the graph.
///
/// # Example
/// ```rust
/// use graphina::core::types::Graph;
///
/// use graphina::centrality::algorithms::degree_centrality;
///
/// let mut g: Graph<i32, ()> = Graph::new();
/// let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
/// g.add_edge(nodes[0], nodes[1], ());
/// g.add_edge(nodes[0], nodes[2], ());
///
/// let centrality = degree_centrality(&g);
/// println!("{:?}", centrality); // [2.0, 1.0, 1.0]
/// ```
pub fn degree_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Vec<f64>
where
    W: Copy,
    Ty: GraphConstructor<A, W>,
{
    if !<Ty as GraphConstructor<A, W>>::is_directed() {
        return out_degree_centrality(graph);
    }
    let n = graph.node_count();
    let mut degree = vec![0; n];
    for (node, _) in graph.nodes() {
        degree[node.index()] += graph.neighbors(node).count();
    }
    for (_u, v, _w) in graph.edges() {
        degree[v.index()] += 1;
    }
    degree.into_iter().map(|d| d as f64).collect()
}

/// In–degree centrality.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
///
/// # Returns
///
/// a vector of `f64` representing out degree centralities of each node in the graph.
///
/// # Example
/// ```rust
/// use graphina::core::types::Digraph;
///
/// use graphina::centrality::algorithms::in_degree_centrality;
///
/// let mut g: Digraph<i32, ()> = Digraph::new();
/// let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
/// g.add_edge(nodes[0], nodes[1], ());
/// g.add_edge(nodes[0], nodes[2], ());
///
/// let centrality = in_degree_centrality(&g);
/// println!("{:?}", centrality); // [0.0, 1.0, 1.0]
/// ```
pub fn in_degree_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Vec<f64>
where
    W: Copy,
    Ty: GraphConstructor<A, W>,
{
    if !<Ty as GraphConstructor<A, W>>::is_directed() {
        return out_degree_centrality(graph);
    }
    let n = graph.node_count();
    let mut cent = vec![0.0; n];
    for (_u, v, _w) in graph.edges() {
        cent[v.index()] += 1.0;
    }
    cent
}

/// Out–degree centrality.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
///
/// # Returns
///
/// a vector of `f64` representing out degree centralities of each node in the graph.
///
/// # Example
/// ```rust
/// use graphina::core::types::Digraph;
///
/// use graphina::centrality::algorithms::out_degree_centrality;
///
/// let mut g: Digraph<i32, ()> = Digraph::new();
/// let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
/// g.add_edge(nodes[0], nodes[1], ());
/// g.add_edge(nodes[0], nodes[2], ());
///
/// let centrality = out_degree_centrality(&g);
/// println!("{:?}", centrality); // [2.0, 0.0, 0.0]
/// ```
pub fn out_degree_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Vec<f64>
where
    W: Copy,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    let mut cent = vec![0.0; n];
    for (u, _) in graph.nodes() {
        cent[u.index()] = graph.neighbors(u).count() as f64;
    }
    cent
}

//
// -----------------------------
// Eigenvector Centrality
// -----------------------------
//

/// Full implementation of eigenvector centrality with convergence tolerance,
/// calculates eigenvector centrality for nodes in a graph iteratively.
///
/// this function designed for generic edge type,
/// see [`eigenvector_centrality`] and [`eigenvector_centrality_numpy`]
/// for cleaner interaction with `f64` edge graph.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `max_iter`: The maximum number of iterations that the algorithm will run for.
/// * `tol`: the average tolerance for convergence.
/// * `eval_weight`: callback to evaluate the weight of edges in the graph.
///
/// # Returns
///
/// a vector of `f64` representing eigenvector centralities of each node in the graph.
///
/// # Example
/// ```rust
/// use graphina::core::types::Graph;
/// use graphina::centrality::algorithms::eigenvector_centrality_impl;
///
/// let mut g: Graph<i32, (f64, f64)> = Graph::new();
/// //                    ^^^^^^^^^^
/// //                             L arbitrary type as edge
/// let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
/// g.add_edge(nodes[0], nodes[1], (0.0, 1.0));
/// g.add_edge(nodes[0], nodes[2], (1.0, 0.0));
/// let centrality = eigenvector_centrality_impl(
///     &g,
///     1000,
///     1e-6_f64,
///     |w| w.0 * 10.0 + w.1 // <-- custom evaluation for edge weight
/// );
/// println!("{:.5?}", centrality); // [0.70711, 0.07036, 0.70360]
/// ```
pub fn eigenvector_centrality_impl<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    max_iter: usize,
    tol: f64,
    eval_weight: impl Fn(&W) -> f64,
) -> Vec<f64>
where
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    let mut centrality = vec![1.0; n];
    let mut next = centrality.clone();
    for _ in 0..max_iter {
        for (src, dst, w) in graph.edges() {
            let w = eval_weight(w);
            next[dst.index()] += w * centrality[src.index()];
            if !<Ty as GraphConstructor<A, W>>::is_directed() {
                next[src.index()] += w * centrality[dst.index()];
            }
        }
        let norm = next.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 0.0 {
            for x in &mut next {
                *x /= norm;
            }
        }
        let diff: f64 = centrality
            .iter_mut()
            .zip(next.iter())
            .map(|(a, b)| {
                let d = (*a - b).abs();
                *a = *b;
                d
            })
            .sum();
        if diff < tol * n as f64 {
            break;
        }
    }
    centrality
}

/// Wrapper for eigenvector centrality with default tolerance (1e-6).
/// calculates eigenvector centrality for nodes in a graph iteratively.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `max_iter`: The maximum number of iterations that the algorithm will run for.
/// * `weighted`: whether or not the calculated centrality will be weighed.
///
/// # Returns
///
/// a vector of `f64` representing eigenvector centralities of each node in the graph.
///
/// # Examples
/// ```rust
/// use graphina::core::types::Graph;
///
/// use graphina::centrality::algorithms::eigenvector_centrality;
///
/// let mut g = Graph::new();
/// let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
/// g.add_edge(nodes[0], nodes[1], 1.0);
/// g.add_edge(nodes[0], nodes[2], 2.0);
///
/// let centrality = eigenvector_centrality(&g, 1000, false);
/// println!("{:.5?}", centrality); // [0.70711, 0.50000, 0.50000]
/// let centrality = eigenvector_centrality(&g, 1000, true);
/// println!("{:.5?}", centrality); // [0.70711, 0.31623, 0.63246]
/// ```
pub fn eigenvector_centrality<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    max_iter: usize,
    weighted: bool,
) -> Vec<f64>
where
    Ty: GraphConstructor<A, f64>,
{
    let eval_weight = if weighted {
        |f: &f64| *f
    } else {
        |_f: &f64| 1.0
    };
    eigenvector_centrality_impl(graph, max_iter, 1e-6_f64, eval_weight)
}

/// NumPy–style eigenvector centrality (alias to [`eigenvector_centrality`]).
/// calculates eigenvector centrality for nodes in a graph iteratively.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `max_iter`: The maximum number of iterations that the algorithm will run for.
/// * `weighted`: whether or not the calculated centrality will be weighed.
///
/// # Returns
///
/// a vector of `f64` representing eigenvector centralities of each node in the graph.
///
/// # Examples
/// ```rust
/// use graphina::core::types::Graph;
///
/// use graphina::centrality::algorithms::eigenvector_centrality_numpy;
/// let mut g = Graph::new();
/// let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
/// g.add_edge(nodes[0], nodes[1], 1.0);
/// g.add_edge(nodes[0], nodes[2], 2.0);
///
/// let centrality = eigenvector_centrality_numpy(&g, 1000, 1e-6_f64, false);
/// println!("{:.5?}", centrality); // [0.70711, 0.50000, 0.50000]
/// let centrality = eigenvector_centrality_numpy(&g, 1000, 1e-6_f64, true);
/// println!("{:.5?}", centrality); // [0.70711, 0.31623, 0.63246]
/// ```
pub fn eigenvector_centrality_numpy<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    max_iter: usize,
    tol: f64,
    weighted: bool,
) -> Vec<f64>
where
    Ty: GraphConstructor<A, f64>,
{
    let eval_weight = if weighted {
        |f: &f64| *f
    } else {
        |_f: &f64| 1.0
    };
    eigenvector_centrality_impl(graph, max_iter, tol, eval_weight)
}

//
// -----------------------------
// Katz Centrality
// -----------------------------
//

/// Full implementation of Katz centrality with convergence tolerance.
///
/// Formula: x = alpha * A * x + beta.
///
/// calculates katz centrality for nodes in a graph iteratively.
///
/// this function designed for generic node and edge type,
/// see [`katz_centrality`] and [`katz_centrality_numpy`]
/// for cleaner interaction with `f64` edge graph.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `alpha`: callback to evaluate the alphas of nodes in the graph.
/// * `beta`: callback to evaluate the betas of nodes in the graph.
/// * `max_iter`: The maximum number of iterations that the algorithm will run for.
/// * `tol`: the average tolerance for convergence.
/// * `normalized`: whether the returned result will be normalized.
/// * `eval_weight`: callback to evaluate the weight of edges in the graph.
///
/// # Returns
///
/// a vector of `f64` representing eigenvector centralities of each node in the graph.
///
/// # Example
/// ```rust
/// use graphina::centrality::algorithms::katz_centrality_impl;
/// use graphina::core::types::Graph;
///
/// let mut g: Graph<(i32, f64), (f64, f64)> = Graph::new();
/// //               ^^^^^^^^^^  ^^^^^^^^^^
/// //                        |           L arbitrary type as edge
/// //                        L arbitrary type as node
/// let nodes = [
///     g.add_node((1, 2.0)),
///     g.add_node((2, 3.0)),
///     g.add_node((3, 2.0)),
/// ];
/// g.add_edge(nodes[0], nodes[1], (0.0, 1.0));
/// g.add_edge(nodes[0], nodes[2], (1.0, 0.0));
///
/// let centrality = katz_centrality_impl(
///     &g,
///     |_n| 0.01,                           // <-- custom alpha depend on node attribute
///     |(i, f): &(i32, f64)| f.powi(*i),    // <-- custom beta depend on node attribute
///     1_000,
///     1e-6_f64,
///     true,
///     |w| w.0 * 10.0 + w.1,                // <-- custom evaluation for edge weight
/// );
/// println!("{:.5?}", centrality); // [0.23167, 0.71650, 0.65800]
/// ```
pub fn katz_centrality_impl<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    alpha: impl Fn(&A) -> f64,
    beta: impl Fn(&A) -> f64,
    max_iter: usize,
    tol: f64,
    normalized: bool,
    eval_weight: impl Fn(&W) -> f64,
) -> Vec<f64>
where
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    let betas = (0..n)
        .map(|i| beta(graph.node_attr(NodeId(NodeIndex::new(i))).unwrap()))
        .collect::<Vec<_>>();
    let alphas = (0..n)
        .map(|i| alpha(graph.node_attr(NodeId(NodeIndex::new(i))).unwrap()))
        .collect::<Vec<_>>();

    let mut centrality = vec![0.0; n];
    let mut next = vec![0.0; n];

    for _ in 0..max_iter {
        for (src, dst, w) in graph.edges() {
            let w = eval_weight(w);
            next[dst.index()] += w * centrality[src.index()];
            if !<Ty as GraphConstructor<A, W>>::is_directed() {
                next[src.index()] += w * centrality[dst.index()];
            }
        }

        for (i, n) in next.iter_mut().enumerate() {
            *n = alphas[i] * *n + betas[i];
        }

        let diff: f64 = centrality
            .iter_mut()
            .zip(next.iter_mut())
            .map(|(a, b)| {
                let d = (*a - *b).abs();
                *a = *b;
                *b = 0.0;
                d
            })
            .sum();

        if diff < tol * n as f64 {
            if normalized {
                let norm = centrality.iter().map(|x| x * x).sum::<f64>().sqrt();
                if norm > 0.0 {
                    for x in &mut centrality {
                        *x /= norm;
                    }
                }
            }
            break;
        }
    }
    centrality
}

/// Wrapper for Katz centrality with default tolerance (1e-6).
///
/// Formula: x = alpha * A * x + beta.
///
/// calculates katz centrality for nodes in a graph iteratively.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `alpha`: value of alpha
/// * `beta`: value of alpha
/// * `max_iter`: The maximum number of iterations that the algorithm will run for.
/// * `normalized`: whether the returned result will be normalized.
/// * `weighted`: whether or not the calculated centrality will be weighed.
///
/// # Returns
///
/// a vector of `f64` representing eigenvector centralities of each node in the graph.
///
/// # Example
/// ```rust
/// use graphina::centrality::algorithms::katz_centrality;
/// use graphina::core::types::Graph;
///
/// let mut g = Graph::new();
/// let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
/// g.add_edge(nodes[0], nodes[1], 1.0);
/// g.add_edge(nodes[0], nodes[2], 2.0);
///
/// let centrality = katz_centrality(&g, 0.1, 1.0, 1000, false, true);
/// println!("{:.5?}", centrality); // [0.61078, 0.55989, 0.55989]
/// let centrality = katz_centrality(&g, 0.01, 0.5, 1000, true, true);
/// println!("{:.5?}", centrality); // [0.58301, 0.57158, 0.57741]
/// ```
pub fn katz_centrality<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    alpha: f64,
    beta: f64,
    max_iter: usize,
    weighted: bool,
    normalized: bool,
) -> Vec<f64>
where
    Ty: crate::core::types::GraphConstructor<A, f64>,
{
    let alpha = |_a: &A| alpha;
    let beta = |_a: &A| beta;
    let eval_weight = if weighted {
        |f: &f64| *f
    } else {
        |_f: &f64| 1.0
    };
    katz_centrality_impl(
        graph,
        alpha,
        beta,
        max_iter,
        1e-6_f64,
        normalized,
        eval_weight,
    )
}

/// NumPy–style Katz centrality, with default tolerance (1e-6) and max iteration of 100.
///
/// Formula: x = alpha * A * x + beta.
///
/// calculates katz centrality for nodes in a graph iteratively.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `alpha`: value of alpha
/// * `beta`: value of alpha
/// * `normalized`: whether the returned result will be normalized.
/// * `weighted`: whether or not the calculated centrality will be weighed.
///
/// # Returns
///
/// a vector of `f64` representing eigenvector centralities of each node in the graph.
///
/// # Example
/// ```rust
/// use graphina::centrality::algorithms::katz_centrality_numpy;
/// use graphina::core::types::Graph;
///
/// let mut g = Graph::new();
/// let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
/// g.add_edge(nodes[0], nodes[1], 1.0);
/// g.add_edge(nodes[0], nodes[2], 2.0);
///
/// let centrality = katz_centrality_numpy(&g, 0.1, 1.0, false, true);
/// println!("{:.5?}", centrality); // [0.61078, 0.55989, 0.55989]
/// let centrality = katz_centrality_numpy(&g, 0.01, 0.5, true, true);
/// println!("{:.5?}", centrality); // [0.58301, 0.57158, 0.57741]
/// ```
pub fn katz_centrality_numpy<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    alpha: f64,
    beta: f64,
    weighted: bool,
    normalized: bool,
) -> Vec<f64>
where
    Ty: crate::core::types::GraphConstructor<A, f64>,
{
    let alpha = |_a: &A| alpha;
    let beta = |_a: &A| beta;
    let eval_weight = if weighted {
        |f: &f64| *f
    } else {
        |_f: &f64| 1.0
    };
    katz_centrality_impl(graph, alpha, beta, 100, 1e-6_f64, normalized, eval_weight)
}

//
// -----------------------------
// Closeness Centrality
// -----------------------------
//

/// Compute closeness centrality using Dijkstra’s algorithm.
/// Closeness = (n - 1) / (sum of shortest-path distances).
pub fn closeness_centrality<A, Ty>(
    graph: &BaseGraph<A, ordered_float::OrderedFloat<f64>, Ty>,
) -> Result<Vec<f64>, GraphinaException>
where
    Ty: GraphConstructor<A, ordered_float::OrderedFloat<f64>>,
{
    let n = graph.node_count();
    let mut closeness = vec![0.0; n];
    for (node, _) in graph.nodes() {
        let distances = dijkstra(graph, node)?;
        let sum: f64 = distances.iter().filter_map(|d| d.map(|od| od.0)).sum();
        if sum > 0.0 {
            closeness[node.index()] = (n as f64 - 1.0) / sum;
        }
    }
    Ok(closeness)
}

//
// -----------------------------
// PageRank
// -----------------------------
//

/// Full implementation of PageRank with convergence tolerance.
pub fn pagerank_impl<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    damping: f64,
    max_iter: usize,
    tol: f64,
) -> Vec<f64>
where
    Ty: GraphConstructor<A, f64>,
{
    let n = graph.node_count();
    let mut rank = vec![1.0 / n as f64; n];
    let teleport = (1.0 - damping) / n as f64;

    let mut out_deg = vec![0usize; n];
    for (node, _) in graph.nodes() {
        out_deg[node.index()] = graph.neighbors(node).count();
    }

    for _ in 0..max_iter {
        let mut new_rank = vec![teleport; n];
        for (u, _) in graph.nodes() {
            let r = rank[u.index()];
            if out_deg[u.index()] > 0 {
                let share = damping * r / out_deg[u.index()] as f64;
                for v in graph.neighbors(u) {
                    new_rank[v.index()] += share;
                }
            } else {
                for x in new_rank.iter_mut() {
                    *x += damping * r / n as f64;
                }
            }
        }
        let diff: f64 = rank
            .iter()
            .zip(new_rank.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        rank = new_rank;
        if diff < tol * n as f64 {
            break;
        }
    }
    rank
}

/// Wrapper for PageRank with default tolerance (1e-6).
/// This wrapper takes 3 arguments: graph, damping, max_iter.
pub fn pagerank<A, Ty>(graph: &BaseGraph<A, f64, Ty>, damping: f64, max_iter: usize) -> Vec<f64>
where
    Ty: GraphConstructor<A, f64>,
{
    pagerank_impl(graph, damping, max_iter, 1e-6_f64)
}

//
// -----------------------------
// Betweenness Centrality
// -----------------------------
//

/// Compute betweenness centrality (node version) using Brandes’ algorithm.
pub fn betweenness_centrality<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> Vec<f64>
where
    Ty: GraphConstructor<A, f64>,
{
    let n = graph.node_count();
    let mut bc = vec![0.0; n];

    for (s, _) in graph.nodes() {
        let mut stack = Vec::with_capacity(n);
        let mut pred = vec![Vec::new(); n];
        let mut sigma = vec![0.0; n];
        let mut dist = vec![-1.0_f64; n];
        sigma[s.index()] = 1.0;
        dist[s.index()] = 0.0;
        let mut queue = VecDeque::new();
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for w in graph.neighbors(v) {
                if dist[w.index()] < 0.0 {
                    dist[w.index()] = dist[v.index()] + 1.0_f64;
                    queue.push_back(w);
                }
                if (dist[w.index()] - (dist[v.index()] + 1.0_f64)).abs() < 1e-6_f64 {
                    sigma[w.index()] += sigma[v.index()];
                    pred[w.index()].push(v);
                }
            }
        }

        let mut delta = vec![0.0; n];
        while let Some(w) = stack.pop() {
            for &v in &pred[w.index()] {
                delta[v.index()] +=
                    (sigma[v.index()] / sigma[w.index()]) * (1.0 + delta[w.index()]);
            }
            if w != s {
                bc[w.index()] += delta[w.index()];
            }
        }
    }
    bc
}

/// Compute edge betweenness centrality using a modified Brandes’ algorithm.
/// Returns a map from (source_index, target_index) to betweenness score.
pub fn edge_betweenness_centrality<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
) -> HashMap<(usize, usize), f64>
where
    Ty: GraphConstructor<A, f64>,
{
    let n = graph.node_count();
    let mut eb: HashMap<(usize, usize), f64> = HashMap::new();

    for (u, v, _) in graph.edges() {
        eb.insert((u.index(), v.index()), 0.0);
    }

    for (s, _) in graph.nodes() {
        let mut stack = Vec::with_capacity(n);
        let mut pred = vec![Vec::new(); n];
        let mut sigma = vec![0.0; n];
        let mut dist = vec![-1.0_f64; n];
        sigma[s.index()] = 1.0;
        dist[s.index()] = 0.0;
        let mut queue = VecDeque::new();
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for w in graph.neighbors(v) {
                if dist[w.index()] < 0.0 {
                    dist[w.index()] = dist[v.index()] + 1.0_f64;
                    queue.push_back(w);
                }
                if (dist[w.index()] - (dist[v.index()] + 1.0_f64)).abs() < 1e-6_f64 {
                    sigma[w.index()] += sigma[v.index()];
                    pred[w.index()].push(v);
                }
            }
        }

        let mut delta = vec![0.0; n];
        while let Some(w) = stack.pop() {
            for &v in &pred[w.index()] {
                let c = (sigma[v.index()] / sigma[w.index()]) * (1.0 + delta[w.index()]);
                delta[v.index()] += c;
                if let Some(val) = eb.get_mut(&(v.index(), w.index())) {
                    *val += c;
                }
            }
        }
    }
    eb
}

//
// -----------------------------
// Harmonic Centrality
// -----------------------------
//

/// Harmonic centrality: sum of reciprocals of shortest-path distances (ignoring unreachable nodes).
pub fn harmonic_centrality<A, Ty>(
    graph: &BaseGraph<A, ordered_float::OrderedFloat<f64>, Ty>,
) -> Result<Vec<f64>, GraphinaException>
where
    Ty: GraphConstructor<A, ordered_float::OrderedFloat<f64>>,
{
    let n = graph.node_count();
    let mut centrality = vec![0.0; n];
    for (node, _) in graph.nodes() {
        let distances = dijkstra(graph, node)?;
        let sum: f64 = distances
            .iter()
            .filter_map(|d| d.map(|od| od.0))
            .filter(|&d| d > 0.0)
            .map(|d| 1.0 / d)
            .sum();
        centrality[node.index()] = sum;
    }
    Ok(centrality)
}

//
// -----------------------------
// Reaching Centralities
// -----------------------------
//

/// Local reaching centrality: fraction of nodes reachable from a given node (via BFS).
pub fn local_reaching_centrality<A, Ty>(graph: &BaseGraph<A, f64, Ty>, v: NodeId) -> f64
where
    Ty: GraphConstructor<A, f64>,
{
    let n = graph.node_count();
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    visited[v.index()] = true;
    queue.push_back(v);
    let mut count = 0;
    while let Some(u) = queue.pop_front() {
        count += 1;
        for w in graph.neighbors(u) {
            if !visited[w.index()] {
                visited[w.index()] = true;
                queue.push_back(w);
            }
        }
    }
    if n > 1 {
        (count - 1) as f64 / (n as f64 - 1.0)
    } else {
        0.0
    }
}

/// Global reaching centrality: average difference between the maximum local reaching centrality and each node’s value.
pub fn global_reaching_centrality<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> f64
where
    Ty: GraphConstructor<A, f64>,
{
    let lrc: Vec<f64> = graph
        .nodes()
        .map(|(v, _)| local_reaching_centrality(graph, v))
        .collect();
    let max = lrc.iter().cloned().fold(f64::NEG_INFINITY, |a, b| a.max(b));
    let n = lrc.len() as f64;
    lrc.iter().map(|&x| max - x).sum::<f64>() / n
}

//
// -----------------------------
// VoteRank
// -----------------------------
//

/// VoteRank: iteratively select a set of influential nodes using a voting mechanism.
/// Initial scores are the nodes’ out–degrees; after selection, the scores of their neighbors are reduced.
pub fn voterank<A, Ty>(graph: &BaseGraph<A, f64, Ty>, number_of_nodes: usize) -> Vec<NodeId>
where
    Ty: GraphConstructor<A, f64>,
{
    let n = graph.node_count();
    let mut scores = vec![0.0; n];
    for (u, _) in graph.nodes() {
        scores[u.index()] = graph.neighbors(u).count() as f64;
    }
    let mut selected = Vec::new();
    let mut voted = vec![false; n];
    while selected.len() < number_of_nodes && selected.len() < n {
        let (max_idx, _) = scores
            .iter()
            .enumerate()
            .filter(|(i, _)| !voted[*i])
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap();
        selected.push(NodeId::new(petgraph::graph::NodeIndex::new(max_idx)));
        voted[max_idx] = true;
        for nb in graph.neighbors(NodeId::new(petgraph::graph::NodeIndex::new(max_idx))) {
            scores[nb.index()] *= 0.8;
        }
    }
    selected
}

//
// -----------------------------
// Laplacian Centrality
// -----------------------------
//

/// Laplacian centrality for nodes, computed from local degree information.
/// Here LC(v) = d(v)^2 + 2 * (sum of 1’s for each neighbor).
pub fn laplacian_centrality<A, Ty>(graph: &BaseGraph<A, f64, Ty>, _normalized: bool) -> Vec<f64>
where
    Ty: GraphConstructor<A, f64>,
{
    graph
        .nodes()
        .map(|(u, _)| {
            let deg = graph.neighbors(u).count() as f64;
            let sum: f64 = graph.neighbors(u).map(|_| 1.0).sum();
            deg * deg + 2.0 * sum
        })
        .collect()
}
