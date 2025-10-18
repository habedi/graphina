//! Degree centrality algorithms.
//!
//! This module provides degree centrality measures.
//!
//! Convention: returns `Result<_, crate::core::exceptions::GraphinaException>` for consistency
//! and better observability.

use crate::core::exceptions::GraphinaException;
use crate::core::types::{BaseGraph, GraphConstructor, NodeMap};

/// Degree centrality: number of incident edges (for directed, in + out).
///
/// Behavior and conventions:
/// - Returns raw counts (not normalized).
/// - Directed graphs: degree = in-degree + out-degree.
/// - Undirected graphs: counts each incident edge once; a self-loop counts as 2.
pub fn degree_centrality<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
) -> Result<NodeMap<f64>, GraphinaException>
where
    Ty: GraphConstructor<A, W>,
{
    let mut centrality = NodeMap::new();
    for (node, _) in graph.nodes() {
        let count = if graph.is_directed() {
            let indeg = graph.in_degree(node).unwrap_or(0);
            let outdeg = graph.out_degree(node).unwrap_or(0);
            indeg + outdeg
        } else {
            // Undirected: count incident edges, self-loop counts as 2
            let mut c = 0usize;
            for (u, v, _w) in graph.edges() {
                if u == node && v == node {
                    c += 2; // self-loop contributes twice
                } else if u == node || v == node {
                    c += 1;
                }
            }
            c
        };
        centrality.insert(node, count as f64);
    }
    Ok(centrality)
}

/// In-degree centrality: number of incoming edges (raw count).
///
/// Behavior and conventions:
/// - Directed graphs: counts only incoming edges.
/// - Undirected graphs: equal to total degree; a self-loop counts as 2.
pub fn in_degree_centrality<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
) -> Result<NodeMap<f64>, GraphinaException>
where
    Ty: GraphConstructor<A, W>,
{
    let mut centrality = NodeMap::new();
    for (node, _) in graph.nodes() {
        let count = if graph.is_directed() {
            graph.edges().filter(|(_src, tgt, _)| *tgt == node).count()
        } else {
            // Undirected: treat as total degree (self-loop counts as 2)
            let mut c = 0usize;
            for (u, v, _w) in graph.edges() {
                if u == node && v == node {
                    c += 2;
                } else if u == node || v == node {
                    c += 1;
                }
            }
            c
        };
        centrality.insert(node, count as f64);
    }
    Ok(centrality)
}

/// Out-degree centrality: number of outgoing edges (raw count).
///
/// Behavior and conventions:
/// - Directed graphs: counts only outgoing edges.
/// - Undirected graphs: equal to total degree; a self-loop counts as 2.
pub fn out_degree_centrality<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
) -> Result<NodeMap<f64>, GraphinaException>
where
    Ty: GraphConstructor<A, W>,
{
    let mut centrality = NodeMap::new();
    for (node, _) in graph.nodes() {
        let count = if graph.is_directed() {
            graph.edges().filter(|(src, _tgt, _)| *src == node).count()
        } else {
            // Undirected: treat as total degree (self-loop counts as 2)
            let mut c = 0usize;
            for (u, v, _w) in graph.edges() {
                if u == node && v == node {
                    c += 2;
                } else if u == node || v == node {
                    c += 1;
                }
            }
            c
        };
        centrality.insert(node, count as f64);
    }
    Ok(centrality)
}
