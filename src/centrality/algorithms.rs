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

use crate::core::exceptions::GraphinaException;
use crate::core::paths::{dijkstra, dijkstra_path_impl};
use crate::core::types::{BaseGraph, GraphConstructor, GraphinaGraph, NodeId, NodeMap};
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

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
/// [`NodeMap`] of `f64` representing out degree centralities of each node in the graph.
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
/// let expected = [2.0, 1.0, 1.0];
/// for (i, f) in expected.into_iter().enumerate() {
///     assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
/// }
/// ```
pub fn degree_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> NodeMap<f64>
where
    Ty: GraphConstructor<A, W>,
    BaseGraph<A, W, Ty>: GraphinaGraph<A, W>,
{
    let mut cent = graph.to_nodemap_default();

    if graph.is_directed() {
        // For directed graphs, count both in-degree and out-degree
        for (src, dst, _) in graph.edges() {
            *cent.get_mut(&src).unwrap() += 1.0;
            *cent.get_mut(&dst).unwrap() += 1.0;
        }
    } else {
        // For undirected graphs, count each edge once per node
        for (src, dst, _) in graph.edges() {
            *cent.get_mut(&src).unwrap() += 1.0;
            *cent.get_mut(&dst).unwrap() += 1.0;
        }
    }
    cent
}

/// In–degree centrality.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing out degree centralities of each node in the graph.
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
/// let expected = [0.0, 1.0, 1.0];
/// for (i, f) in expected.into_iter().enumerate() {
///     assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
/// }
/// ```
pub fn in_degree_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> NodeMap<f64>
where
    Ty: GraphConstructor<A, W>,
    BaseGraph<A, W, Ty>: GraphinaGraph<A, W>,
{
    let mut cent = graph.to_nodemap_default();
    for (_, dst, _) in graph.flow_edges() {
        *cent.get_mut(&dst).unwrap() += 1.0;
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
/// [`NodeMap`] of `f64` representing out degree centralities of each node in the graph.
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
///
/// let expected = [2.0, 0.0, 0.0];
/// for (i, f) in expected.into_iter().enumerate() {
///     assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
/// }
/// ```
pub fn out_degree_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> NodeMap<f64>
where
    Ty: GraphConstructor<A, W>,
    BaseGraph<A, W, Ty>: GraphinaGraph<A, W>,
{
    let mut cent = graph.to_nodemap_default();
    for (src, _, _) in graph.flow_edges() {
        *cent.get_mut(&src).unwrap() += 1.0;
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
/// [`NodeMap`] of `f64` representing eigenvector centralities of each node in the graph.
///
/// # Example
/// ```rust
/// use graphina::centrality::algorithms::eigenvector_centrality_impl;
/// use graphina::core::types::Graph;
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
///     |w| w.0 * 10.0 + w.1, // <-- custom evaluation for edge weight
/// );
/// let expected = [0.70711, 0.07036, 0.70360];
/// for (i, f) in expected.into_iter().enumerate() {
///     assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
/// }
/// ```
pub fn eigenvector_centrality_impl<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    max_iter: usize,
    tol: f64,
    eval_weight: impl Fn(&W) -> f64,
) -> NodeMap<f64>
where
    Ty: GraphConstructor<A, W>,
    BaseGraph<A, W, Ty>: GraphinaGraph<A, W>,
{
    let n = graph.node_count();
    let mut centrality = graph.to_nodemap(|_, _| 1.0);
    let mut next = centrality.clone();
    for _ in 0..max_iter {
        for (src, dst, w) in graph.flow_edges() {
            let w = eval_weight(w);
            *next.get_mut(&dst).unwrap() += w * centrality[&src];
        }
        let norm = next.values().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 0.0 {
            for x in next.values_mut() {
                *x /= norm;
            }
        }
        let diff: f64 = centrality
            .iter_mut()
            .map(|(nodeid, a)| {
                let next = next.get(nodeid).unwrap();
                let d = (*a - *next).abs();
                *a = *next;
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
/// [`NodeMap`] of `f64` representing eigenvector centralities of each node in the graph.
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
/// let expected = [0.70711, 0.50000, 0.50000];
/// for (i, f) in expected.into_iter().enumerate() {
/// assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
/// }
///
/// let centrality = eigenvector_centrality(&g, 1000, true);
/// let expected = [0.70711, 0.31623, 0.63246];
/// for (i, f) in expected.into_iter().enumerate() {
/// assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
/// }
/// ```
pub fn eigenvector_centrality<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    max_iter: usize,
    weighted: bool,
) -> NodeMap<f64>
where
    Ty: GraphConstructor<A, f64>,
    BaseGraph<A, f64, Ty>: GraphinaGraph<A, f64>,
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
/// [`NodeMap`] of `f64` representing eigenvector centralities of each node in the graph.
///
/// # Examples
/// ```rust
/// use graphina::centrality::algorithms::eigenvector_centrality_numpy;
/// use graphina::core::types::Graph;
/// let mut g = Graph::new();
///
/// let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
/// g.add_edge(nodes[0], nodes[1], 1.0);
/// g.add_edge(nodes[0], nodes[2], 2.0);
///
/// let centrality = eigenvector_centrality_numpy(&g, 1000, 1e-6_f64, false);
/// let expected = [0.70711, 0.50000, 0.50000];
/// for (i, f) in expected.into_iter().enumerate() {
///     assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
/// }
///
/// let centrality = eigenvector_centrality_numpy(&g, 1000, 1e-6_f64, true);
/// let expected = [0.70711, 0.31623, 0.63246];
/// for (i, f) in expected.into_iter().enumerate() {
///     assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
/// }
/// ```
pub fn eigenvector_centrality_numpy<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    max_iter: usize,
    tol: f64,
    weighted: bool,
) -> NodeMap<f64>
where
    Ty: GraphConstructor<A, f64>,
    BaseGraph<A, f64, Ty>: GraphinaGraph<A, f64>,
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
/// [`NodeMap`] of `f64` representing eigenvector centralities of each node in the graph.
///
/// # Note
///
/// the katz centrality migh not converge if
/// alpha is larger than the recipocal of the larger eigen value of the network.
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
///     |_n| 0.01,                        // <-- custom alpha depend on node attribute
///     |(i, f): &(i32, f64)| f.powi(*i), // <-- custom beta depend on node attribute
///     1_000,
///     1e-6_f64,
///     true,
///     |w| w.0 * 10.0 + w.1, // <-- custom evaluation for edge weight
/// );
/// let expected = [0.23167, 0.71650, 0.65800];
/// for (i, f) in expected.into_iter().enumerate() {
///     assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
/// }
/// ```
pub fn katz_centrality_impl<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    alpha: impl Fn(&A) -> f64,
    beta: impl Fn(&A) -> f64,
    max_iter: usize,
    tol: f64,
    normalized: bool,
    eval_weight: impl Fn(&W) -> f64,
) -> NodeMap<f64>
where
    Ty: GraphConstructor<A, W>,
    BaseGraph<A, W, Ty>: GraphinaGraph<A, W>,
{
    let n = graph.node_count();
    let betas = graph.to_nodemap(|_, attr| beta(attr));
    let alphas = graph.to_nodemap(|_, attr| alpha(attr));

    let mut centrality = graph.to_nodemap_default();
    let mut next = graph.to_nodemap_default();

    for _ in 0..max_iter {
        for (src, dst, w) in graph.flow_edges() {
            let w = eval_weight(w);
            *next.get_mut(&dst).unwrap() += w * centrality[&src];
        }

        for (n, next) in next.iter_mut() {
            *next = alphas[n] * *next + betas[n];
        }

        let diff: f64 = centrality
            .iter_mut()
            .map(|(nodeid, a): (&NodeId, &mut f64)| {
                let next: &mut f64 = next.get_mut(nodeid).unwrap();
                let d = (*a - *next).abs();
                *a = *next;
                *next = 0.0;
                d
            })
            .sum();

        if diff < tol * n as f64 {
            if normalized {
                let norm = centrality.values().map(|x| x * x).sum::<f64>().sqrt();
                if norm > 0.0 {
                    for x in centrality.values_mut() {
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
/// [`NodeMap`] of `f64` representing eigenvector centralities of each node in the graph.
///
/// # Note
///
/// the katz centrality migh not converge if
/// alpha is larger than the recipocal of the larger eigen value of the network.
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
/// let expected = [0.61078, 0.55989, 0.55989];
/// for (i, f) in expected.into_iter().enumerate() {
///     assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
/// }
///
/// let centrality = katz_centrality(&g, 0.01, 0.5, 1000, true, true);
/// let expected = [0.58301, 0.57158, 0.57741];
/// for (i, f) in expected.into_iter().enumerate() {
///     assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
/// }
/// ```
pub fn katz_centrality<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    alpha: f64,
    beta: f64,
    max_iter: usize,
    weighted: bool,
    normalized: bool,
) -> NodeMap<f64>
where
    Ty: GraphConstructor<A, f64>,
    BaseGraph<A, f64, Ty>: GraphinaGraph<A, f64>,
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
/// [`NodeMap`] of `f64` representing eigenvector centralities of each node in the graph.
///
/// # Note
///
/// the katz centrality migh not converge if
/// alpha is larger than the recipocal of the larger eigen value of the network.
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
/// let expected = [0.61078, 0.55989, 0.55989];
/// for (i, f) in expected.into_iter().enumerate() {
///     assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
/// }
///
/// let centrality = katz_centrality_numpy(&g, 0.01, 0.5, true, true);
/// let expected = [0.58301, 0.57158, 0.57741];
/// for (i, f) in expected.into_iter().enumerate() {
///     assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
/// }
/// ```
pub fn katz_centrality_numpy<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    alpha: f64,
    beta: f64,
    weighted: bool,
    normalized: bool,
) -> NodeMap<f64>
where
    Ty: GraphConstructor<A, f64>,
    BaseGraph<A, f64, Ty>: GraphinaGraph<A, f64>,
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
///
/// Closeness = (n - 1) / (sum of shortest-path distances).
///
/// where n is number of reachable nodes.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `eval_cost`: callback to evaluate the cost of edges in the graph, returning
///   - `Some(f64)` for cost
///   - `None` for impassable
/// * `wf_improved`: whether or not to scale the result by reachability ratio.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing closeness centralities of each node in the graph.
///
/// # Example
/// ```rust
/// use graphina::core::types::Graph;
///
/// use graphina::centrality::algorithms::closeness_centrality_impl;
///
/// let mut graph: Graph<i32, (String, f64)> = Graph::new();
///
/// let nodes = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();
///
/// let edges = [
///     (0, 1, ("friend".to_string(), 0.9)),
///     (0, 2, ("family".to_string(), 0.8)),
///     (1, 3, ("friend".to_string(), 0.7)),
///     (2, 4, ("enemy".to_string(), 0.1)),
/// ];
/// for (s, d, w) in edges {
///     graph.add_edge(nodes[s], nodes[d], w);
/// }
///
/// let eval_cost = |(s, f): &(String, f64)| match s.as_str() {
///     "friend" => Some(1.0 / *f / 2.0),
///     "family" => Some(1.0 / *f / 4.0),
///     "enemy" => None,
///     _ => Some(1.0 / *f),
/// };
///
/// let centrality = closeness_centrality_impl(&graph, eval_cost, true).unwrap();
/// let expected = [1.05244, 1.05244, 0.81436, 0.63088, 0.00000];
/// for (i, f) in expected.into_iter().enumerate() {
///     assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
/// }
/// ```
pub fn closeness_centrality_impl<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    eval_cost: impl Fn(&W) -> Option<f64>,
    wf_improved: bool,
) -> Result<NodeMap<f64>, GraphinaException>
where
    A: Debug,
    W: Debug,
    Ty: GraphConstructor<A, W>,
    BaseGraph<A, W, Ty>: GraphinaGraph<A, W>,
{
    let n = graph.node_count();
    let mut closeness = graph.to_nodemap_default();
    for (node, _) in graph.nodes() {
        let (distances, _) = dijkstra_path_impl(graph, node, None, &eval_cost)?;
        let reachable = distances.values().filter(|d| d.is_some()).count() as f64;
        let sum: f64 = distances.values().filter_map(|d| d.to_owned()).sum();
        if sum > 0.0 {
            let _ = closeness.insert(node, (reachable - 1.0) / sum);
        }
        if wf_improved {
            *closeness.get_mut(&node).unwrap() *= (reachable - 1.0) / (n as f64 - 1.0);
        }
    }
    Ok(closeness)
}

/// Compute closeness centrality using Dijkstra’s algorithm.
///
/// Closeness = (n - 1) / (sum of shortest-path distances).
///
/// where n is number of reachable nodes.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `wf_improved`: whether or not to scale the result by reachability ratio.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing closeness centralities of each node in the graph.
///
/// # Example
/// ```rust
/// use graphina::core::types::Graph;
///
/// use graphina::centrality::algorithms::closeness_centrality;
///
/// let mut graph = Graph::new();
/// let nodes = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();
/// let edges = [(0, 1, 1.0), (0, 2, 1.0), (1, 3, 1.0)];
/// for (s, d, w) in edges {
///     graph.add_edge(nodes[s], nodes[d], w);
/// }
///
/// let centrality = closeness_centrality(&graph, false).unwrap();
/// let expected = [0.75000, 0.75000, 0.50000, 0.50000, 0.00000];
/// for (i, f) in expected.into_iter().enumerate() {
///     assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
/// }
/// ```
pub fn closeness_centrality<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    wf_improved: bool,
) -> Result<NodeMap<f64>, GraphinaException>
where
    A: Debug,
    Ty: GraphConstructor<A, f64>,
    BaseGraph<A, f64, Ty>: GraphinaGraph<A, f64>,
{
    let eval_cost = |f: &f64| Some(*f);
    closeness_centrality_impl(graph, eval_cost, wf_improved)
}

//
// -----------------------------
// PageRank
// -----------------------------
//

/// Full implementation of PageRank with convergence tolerance.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `damping`: the damping alpha parameter typically `0.85`.
/// * `max_iter`: The maximum number of iterations that the algorithm will run for.
/// * `tol`: the average tolerance for convergence.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing pagerank of each node in the graph.
///
/// see [`pagerank`] for example.
pub fn pagerank_impl<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    damping: f64,
    max_iter: usize,
    tol: f64,
) -> NodeMap<f64>
where
    Ty: GraphConstructor<A, W>,
    BaseGraph<A, W, Ty>: GraphinaGraph<A, W>,
{
    let n = graph.node_count() as f64;
    let mut rank = graph.to_nodemap(|_, _| 1.0 / n);
    let teleport = (1.0 - damping) / n;

    let out_deg = out_degree_centrality(graph);

    for _ in 0..max_iter {
        let mut new_rank = graph.to_nodemap(|_, _| teleport);
        for (u, _) in graph.nodes() {
            let r = rank[&u];
            if out_deg[&u] > 0.0 {
                let share = damping * r / out_deg[&u];
                for v in graph.neighbors(u) {
                    *new_rank.get_mut(&v).unwrap() += share;
                }
            } else {
                for x in new_rank.values_mut() {
                    *x += damping * r / n;
                }
            }
        }
        let diff: f64 = rank
            .iter()
            .zip(new_rank.iter())
            .map(|((_, a), (_, b))| (a - b).abs())
            .sum();
        rank = new_rank;
        if diff < tol * n {
            break;
        }
    }
    rank
}

/// Wrapper for PageRank with default tolerance (1e-6).
/// This wrapper takes 3 arguments: graph, damping, max_iter.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `damping`: the damping alpha parameter typically `0.85`.
/// * `max_iter`: The maximum number of iterations that the algorithm will run for.
/// * `tol`: the average tolerance for convergence.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing pagerank of each node in the graph.
///
/// # Example
/// ```rust
/// use graphina::core::types::Digraph;
///
/// use graphina::centrality::algorithms::pagerank;
///
/// let mut graph = Digraph::new();
/// let nodes = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();
/// let edges = [(0, 1, 1.0), (0, 2, 1.0), (1, 3, 1.0)];
/// for (s, d, w) in edges {
///     graph.add_edge(nodes[s], nodes[d], w);
/// }
///
/// let centrality = pagerank(&graph, 0.85, 1000);
/// let expected = [0.14161, 0.20180, 0.20180, 0.31315, 0.14161];
/// for (i, f) in expected.into_iter().enumerate() {
///     assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
/// }
/// ```
pub fn pagerank<A, Ty>(graph: &BaseGraph<A, f64, Ty>, damping: f64, max_iter: usize) -> NodeMap<f64>
where
    Ty: GraphConstructor<A, f64>,
    BaseGraph<A, f64, Ty>: GraphinaGraph<A, f64>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Digraph;
    use ordered_float::OrderedFloat;
    fn build_test_graph_f64() -> Digraph<i32, f64> {
        let mut graph: Digraph<i32, f64> = Digraph::default();
        let n0 = graph.add_node(0);
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n0, n1, 1.0);
        graph.add_edge(n0, n2, 1.0);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n1, n3, 1.0);
        graph.add_edge(n2, n0, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n0, 1.0);
        graph.add_edge(n3, n1, 1.0);
        graph
    }
    fn build_test_graph_ordered() -> Digraph<i32, OrderedFloat<f64>> {
        let mut graph: Digraph<i32, OrderedFloat<f64>> = Digraph::default();
        let n0 = graph.add_node(0);
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n0, n1, OrderedFloat(1.0));
        graph.add_edge(n0, n2, OrderedFloat(1.0));
        graph.add_edge(n1, n2, OrderedFloat(1.0));
        graph.add_edge(n1, n3, OrderedFloat(1.0));
        graph.add_edge(n2, n0, OrderedFloat(1.0));
        graph.add_edge(n2, n3, OrderedFloat(1.0));
        graph.add_edge(n3, n0, OrderedFloat(1.0));
        graph.add_edge(n3, n1, OrderedFloat(1.0));
        graph
    }
    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }
    #[test]
    fn test_degree_centrality() {
        let graph = build_test_graph_f64();
        let deg = degree_centrality(&graph);
        for d in deg {
            assert_eq!(d.1, 4.0);
        }
    }
    #[test]
    fn test_closeness_centrality() {
        let graph = build_test_graph_f64();
        let closeness = closeness_centrality(&graph, false).unwrap();
        for (_, c) in closeness {
            assert!(approx_eq(c, 0.75, 1e-6));
        }
    }
    #[test]
    fn test_betweenness_centrality() {
        let graph = build_test_graph_f64();
        let bc = betweenness_centrality(&graph);
        for score in bc {
            assert!(score >= 0.0);
        }
    }
    #[test]
    fn test_eigenvector_centrality() {
        let graph = build_test_graph_f64();
        let ev = eigenvector_centrality(&graph, 20, false);
        assert_eq!(ev.len(), 4);
        for (_, score) in ev.iter() {
            assert!(*score > 0.0);
        }
    }
    #[test]
    fn test_pagerank() {
        let graph = build_test_graph_f64();
        let pr = pagerank(&graph, 0.85, 50);
        let total: f64 = pr.values().sum();
        assert!(approx_eq(total, 1.0, 1e-6));
        for score in pr.into_values() {
            assert!(score > 0.0);
        }
    }
    #[test]
    fn test_katz_centrality() {
        let graph = build_test_graph_f64();
        let kc = katz_centrality(&graph, 0.1, 1.0, 50, false, true);
        assert_eq!(kc.len(), 4);
        for (_, score) in kc {
            assert!(score > 0.0);
        }
    }
    #[test]
    fn test_harmonic_centrality() {
        let graph = build_test_graph_ordered();
        let hc = harmonic_centrality(&graph).unwrap();
        for score in hc {
            assert!(approx_eq(score, 2.5, 1e-6));
        }
    }
}
