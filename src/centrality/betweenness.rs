//! Betweenness centrality algorithms.
//!
//! This module provides betweenness centrality measures.

use crate::core::exceptions::GraphinaException;
use crate::core::types::{BaseGraph, GraphConstructor, NodeId, NodeMap};
use ordered_float::OrderedFloat;
use std::collections::HashMap;

/// Betweenness centrality: measures the extent to which a node lies on paths between other nodes.
/// It is the sum of the fraction of all-pairs shortest paths that pass through the node.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `normalized`: whether to normalize the centrality values.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing betweenness centralities of each node in the graph.
pub fn betweenness_centrality<A, Ty>(
    graph: &BaseGraph<A, OrderedFloat<f64>, Ty>,
    normalized: bool,
) -> Result<NodeMap<f64>, GraphinaException>
where
    Ty: GraphConstructor<A, OrderedFloat<f64>>,
{
    let n = graph.node_count();
    let mut centrality = NodeMap::new();
    for (node, _) in graph.nodes() {
        centrality.insert(node, 0.0);
    }

    for (s, _) in graph.nodes() {
        let mut stack = Vec::new();
        let mut preds: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        let mut sigma = NodeMap::new();
        let mut dist = NodeMap::new();
        let mut delta = NodeMap::new();

        for (node, _) in graph.nodes() {
            preds.insert(node, Vec::new());
            sigma.insert(node, 0.0);
            dist.insert(node, -1.0);
            delta.insert(node, 0.0);
        }
        sigma.insert(s, 1.0);
        dist.insert(s, 0.0);

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for (u, w, _) in graph.edges() {
                if u == v {
                    let d = dist[&v] + 1.0;
                    if dist[&w] < 0.0 {
                        dist.insert(w, d);
                        queue.push_back(w);
                    }
                    if dist[&w] == d {
                        sigma.insert(w, sigma[&w] + sigma[&v]);
                        preds.get_mut(&w).unwrap().push(v);
                    }
                }
            }
        }

        while let Some(w) = stack.pop() {
            for &v in &preds[&w] {
                delta.insert(v, delta[&v] + (sigma[&v] / sigma[&w]) * (1.0 + delta[&w]));
            }
            if w != s {
                *centrality.get_mut(&w).unwrap() += delta[&w];
            }
        }
    }

    if normalized {
        let norm = if n > 1 {
            1.0 / ((n - 1) * (n - 2)) as f64
        } else {
            1.0
        };
        for val in centrality.values_mut() {
            *val *= norm;
        }
    }

    Ok(centrality)
}

/// Edge betweenness centrality: measures the extent to which an edge lies on paths between other nodes.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `normalized`: whether to normalize the centrality values.
///
/// # Returns
///
/// [`HashMap`] of `(NodeId, NodeId)` to `f64` representing edge betweenness centralities.
pub fn edge_betweenness_centrality<A, Ty>(
    graph: &BaseGraph<A, OrderedFloat<f64>, Ty>,
    normalized: bool,
) -> Result<HashMap<(NodeId, NodeId), f64>, GraphinaException>
where
    Ty: GraphConstructor<A, OrderedFloat<f64>>,
{
    let n = graph.node_count();
    let mut centrality = HashMap::new();
    for (u, v, _) in graph.edges() {
        centrality.insert((u, v), 0.0);
    }

    for (s, _) in graph.nodes() {
        let mut stack = Vec::new();
        let mut preds: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        let mut sigma = NodeMap::new();
        let mut dist = NodeMap::new();
        let mut delta = NodeMap::new();

        for (node, _) in graph.nodes() {
            preds.insert(node, Vec::new());
            sigma.insert(node, 0.0);
            dist.insert(node, -1.0);
            delta.insert(node, 0.0);
        }
        sigma.insert(s, 1.0);
        dist.insert(s, 0.0);

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for (u, w, _) in graph.edges() {
                if u == v {
                    let d = dist[&v] + 1.0;
                    if dist[&w] < 0.0 {
                        dist.insert(w, d);
                        queue.push_back(w);
                    }
                    if dist[&w] == d {
                        sigma.insert(w, sigma[&w] + sigma[&v]);
                        preds.get_mut(&w).unwrap().push(v);
                    }
                }
            }
        }

        while let Some(w) = stack.pop() {
            for &v in &preds[&w] {
                delta.insert(v, delta[&v] + (sigma[&v] / sigma[&w]) * (1.0 + delta[&w]));
                if let Some(edge) = centrality.get_mut(&(v, w)) {
                    *edge += (sigma[&v] / sigma[&w]) * (1.0 + delta[&w]);
                }
            }
        }
    }

    if normalized {
        let norm = if n > 1 {
            1.0 / ((n - 1) * (n - 2)) as f64
        } else {
            1.0
        };
        for val in centrality.values_mut() {
            *val *= norm;
        }
    }

    Ok(centrality)
}
