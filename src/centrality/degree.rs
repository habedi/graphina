//! Degree centrality algorithms.
//!
//! This module provides degree centrality measures.
//!
//! Convention: returns `Result<_, crate::core::error::GraphinaError>` for consistency
//! and better observability.

use crate::core::error::Result;
use crate::core::types::{BaseGraph, GraphConstructor, NodeId, NodeMap};
use std::collections::HashMap;

/// Degree centrality: number of incident edges (for directed, in + out).
///
/// Behavior and conventions:
/// - Returns raw counts (not normalized).
/// - Directed graphs: degree = in-degree + out-degree.
/// - Undirected graphs: counts each incident edge once; a self-loop counts as 2.
pub fn degree_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Result<NodeMap<f64>>
where
    Ty: GraphConstructor<A, W>,
{
    let mut centrality = NodeMap::new();
    if graph.is_directed() {
        let mut in_counts: HashMap<NodeId, usize> = HashMap::new();
        let mut out_counts: HashMap<NodeId, usize> = HashMap::new();
        for (node, _) in graph.nodes() {
            in_counts.insert(node, 0);
            out_counts.insert(node, 0);
        }
        for (u, v, _w) in graph.edges() {
            *out_counts.get_mut(&u).unwrap() += 1;
            *in_counts.get_mut(&v).unwrap() += 1;
        }
        for (node, _) in graph.nodes() {
            let d = in_counts[&node] + out_counts[&node];
            centrality.insert(node, d as f64);
        }
    } else {
        let mut counts: HashMap<NodeId, usize> = HashMap::new();
        for (node, _) in graph.nodes() {
            counts.insert(node, 0);
        }
        for (u, v, _w) in graph.edges() {
            if u == v {
                *counts.get_mut(&u).unwrap() += 2;
            } else {
                *counts.get_mut(&u).unwrap() += 1;
                *counts.get_mut(&v).unwrap() += 1;
            }
        }
        for (node, _) in graph.nodes() {
            centrality.insert(node, counts[&node] as f64);
        }
    }
    Ok(centrality)
}

/// In-degree centrality: number of incoming edges (raw count).
///
/// Behavior and conventions:
/// - Directed graphs: counts only incoming edges.
/// - Undirected graphs: equal to total degree; a self-loop counts as 2.
pub fn in_degree_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Result<NodeMap<f64>>
where
    Ty: GraphConstructor<A, W>,
{
    let mut centrality = NodeMap::new();
    if graph.is_directed() {
        let mut in_counts: HashMap<NodeId, usize> = HashMap::new();
        for (node, _) in graph.nodes() {
            in_counts.insert(node, 0);
        }
        for (_u, v, _w) in graph.edges() {
            *in_counts.get_mut(&v).unwrap() += 1;
        }
        for (node, _) in graph.nodes() {
            centrality.insert(node, in_counts[&node] as f64);
        }
    } else {
        // Undirected: treat as total degree with self-loop as 2
        let mut counts: HashMap<NodeId, usize> = HashMap::new();
        for (node, _) in graph.nodes() {
            counts.insert(node, 0);
        }
        for (u, v, _w) in graph.edges() {
            if u == v {
                *counts.get_mut(&u).unwrap() += 2;
            } else {
                *counts.get_mut(&u).unwrap() += 1;
                *counts.get_mut(&v).unwrap() += 1;
            }
        }
        for (node, _) in graph.nodes() {
            centrality.insert(node, counts[&node] as f64);
        }
    }
    Ok(centrality)
}

/// Out-degree centrality: number of outgoing edges (raw count).
///
/// Behavior and conventions:
/// - Directed graphs: counts only outgoing edges.
/// - Undirected graphs: equal to total degree; a self-loop counts as 2.
pub fn out_degree_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Result<NodeMap<f64>>
where
    Ty: GraphConstructor<A, W>,
{
    let mut centrality = NodeMap::new();
    if graph.is_directed() {
        let mut out_counts: HashMap<NodeId, usize> = HashMap::new();
        for (node, _) in graph.nodes() {
            out_counts.insert(node, 0);
        }
        for (u, _v, _w) in graph.edges() {
            *out_counts.get_mut(&u).unwrap() += 1;
        }
        for (node, _) in graph.nodes() {
            centrality.insert(node, out_counts[&node] as f64);
        }
    } else {
        // Undirected: treat as total degree with self-loop as 2
        let mut counts: HashMap<NodeId, usize> = HashMap::new();
        for (node, _) in graph.nodes() {
            counts.insert(node, 0);
        }
        for (u, v, _w) in graph.edges() {
            if u == v {
                *counts.get_mut(&u).unwrap() += 2;
            } else {
                *counts.get_mut(&u).unwrap() += 1;
                *counts.get_mut(&v).unwrap() += 1;
            }
        }
        for (node, _) in graph.nodes() {
            centrality.insert(node, counts[&node] as f64);
        }
    }
    Ok(centrality)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{Digraph, Graph};
    #[test]
    fn test_self_loop_directed() {
        let mut g = Digraph::<i32, f64>::new();
        let n = g.add_node(1);
        g.add_edge(n, n, 1.0);
        let d = degree_centrality(&g).unwrap();
        let indeg = in_degree_centrality(&g).unwrap();
        let outdeg = out_degree_centrality(&g).unwrap();
        assert_eq!(d[&n], 2.0);
        assert_eq!(indeg[&n], 1.0);
        assert_eq!(outdeg[&n], 1.0);
    }
    #[test]
    fn test_self_loop_undirected() {
        let mut g = Graph::<i32, f64>::new();
        let n = g.add_node(1);
        g.add_edge(n, n, 1.0);
        let d = degree_centrality(&g).unwrap();
        let indeg = in_degree_centrality(&g).unwrap();
        let outdeg = out_degree_centrality(&g).unwrap();
        assert_eq!(d[&n], 2.0);
        assert_eq!(indeg[&n], 2.0);
        assert_eq!(outdeg[&n], 2.0);
    }
}
