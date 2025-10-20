//! Betweenness centrality algorithms.
//!
//! This module provides betweenness centrality measures.
//!
//! Convention: returns `Result<_, crate::core::error::GraphinaError>` to surface
//! invalid inputs and improve observability and error propagation.

use crate::core::error::{GraphinaError, Result};
use crate::core::types::{BaseGraph, GraphConstructor, NodeId, NodeMap};
use ordered_float::OrderedFloat;
use std::collections::{HashMap, VecDeque};

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
///
/// # Errors
///
/// Returns an error if the graph is empty.
pub fn betweenness_centrality<A, Ty>(
    graph: &BaseGraph<A, OrderedFloat<f64>, Ty>,
    normalized: bool,
) -> Result<NodeMap<f64>>
where
    Ty: GraphConstructor<A, OrderedFloat<f64>>,
{
    let n = graph.node_count();
    if n == 0 {
        return Err(GraphinaError::invalid_graph(
            "Cannot compute betweenness centrality on an empty graph.",
        ));
    }

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

        let mut queue = VecDeque::new();
        queue.push_back(s);

        // BFS to find shortest paths
        while let Some(v) = queue.pop_front() {
            stack.push(v);
            let v_dist = dist.get(&v).copied().unwrap_or(-1.0);

            // Fixed: iterate over neighbors of v, not all edges
            for w in graph.neighbors(v) {
                let w_dist = dist.get(&w).copied().unwrap_or(-1.0);
                // w found for the first time?
                if w_dist < 0.0 {
                    let new_dist = v_dist + 1.0;
                    dist.insert(w, new_dist);
                    queue.push_back(w);
                }
                // shortest path to w via v?
                // Need to re-read w_dist after potential update
                let w_dist = dist.get(&w).copied().unwrap_or(-1.0);
                if w_dist == v_dist + 1.0 {
                    let sigma_w = sigma.get(&w).copied().unwrap_or(0.0);
                    let sigma_v = sigma.get(&v).copied().unwrap_or(0.0);
                    sigma.insert(w, sigma_w + sigma_v);
                    if let Some(pred_list) = preds.get_mut(&w) {
                        pred_list.push(v);
                    }
                }
            }
        }

        // Accumulation
        while let Some(w) = stack.pop() {
            let delta_w = delta.get(&w).copied().unwrap_or(0.0);
            let sigma_w = sigma.get(&w).copied().unwrap_or(1.0);

            if let Some(pred_list) = preds.get(&w) {
                for &v in pred_list {
                    let sigma_v = sigma.get(&v).copied().unwrap_or(0.0);
                    let delta_v = delta.get(&v).copied().unwrap_or(0.0);
                    let contribution = (sigma_v / sigma_w) * (1.0 + delta_w);
                    delta.insert(v, delta_v + contribution);
                }
            }

            if w != s {
                if let Some(cent) = centrality.get_mut(&w) {
                    *cent += delta_w;
                }
            }
        }
    }

    if normalized && n > 2 {
        let norm = if graph.is_directed() {
            1.0 / ((n - 1) * (n - 2)) as f64
        } else {
            2.0 / ((n - 1) * (n - 2)) as f64
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
///
/// # Errors
///
/// Returns an error if the graph is empty.
pub fn edge_betweenness_centrality<A, Ty>(
    graph: &BaseGraph<A, OrderedFloat<f64>, Ty>,
    normalized: bool,
) -> Result<HashMap<(NodeId, NodeId), f64>>
where
    Ty: GraphConstructor<A, OrderedFloat<f64>>,
{
    let n = graph.node_count();
    if n == 0 {
        return Err(GraphinaError::invalid_graph(
            "Cannot compute edge betweenness centrality on an empty graph.",
        ));
    }

    let mut centrality = HashMap::new();
    for (u, v, _) in graph.edges() {
        centrality.insert((u, v), 0.0);
        if !graph.is_directed() {
            centrality.insert((v, u), 0.0);
        }
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

        let mut queue = VecDeque::new();
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            let v_dist = dist.get(&v).copied().unwrap_or(-1.0);

            // Fixed: iterate over neighbors of v
            for w in graph.neighbors(v) {
                let w_dist = dist.get(&w).copied().unwrap_or(-1.0);
                if w_dist < 0.0 {
                    let new_dist = v_dist + 1.0;
                    dist.insert(w, new_dist);
                    queue.push_back(w);
                }
                // Re-read w_dist after potential update
                let w_dist = dist.get(&w).copied().unwrap_or(-1.0);
                if w_dist == v_dist + 1.0 {
                    let sigma_w = sigma.get(&w).copied().unwrap_or(0.0);
                    let sigma_v = sigma.get(&v).copied().unwrap_or(0.0);
                    sigma.insert(w, sigma_w + sigma_v);
                    if let Some(pred_list) = preds.get_mut(&w) {
                        pred_list.push(v);
                    }
                }
            }
        }

        while let Some(w) = stack.pop() {
            let delta_w = delta.get(&w).copied().unwrap_or(0.0);
            let sigma_w = sigma.get(&w).copied().unwrap_or(1.0);

            if let Some(pred_list) = preds.get(&w) {
                for &v in pred_list {
                    let sigma_v = sigma.get(&v).copied().unwrap_or(0.0);
                    let delta_v = delta.get(&v).copied().unwrap_or(0.0);
                    let contribution = (sigma_v / sigma_w) * (1.0 + delta_w);
                    delta.insert(v, delta_v + contribution);

                    // Update edge centrality
                    if let Some(edge_cent) = centrality.get_mut(&(v, w)) {
                        *edge_cent += contribution;
                    }
                }
            }
        }
    }

    if normalized && n > 2 {
        let norm = if graph.is_directed() {
            1.0 / ((n - 1) * (n - 2)) as f64
        } else {
            2.0 / ((n - 1) * (n - 2)) as f64
        };
        for val in centrality.values_mut() {
            *val *= norm;
        }
    }

    Ok(centrality)
}

#[cfg(test)]
mod tests {
    use super::{betweenness_centrality, edge_betweenness_centrality};
    use crate::core::types::Graph;
    use ordered_float::OrderedFloat;

    #[test]
    fn test_betweenness_centrality_simple() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        graph.add_edge(n1, n2, OrderedFloat(1.0));
        graph.add_edge(n2, n3, OrderedFloat(1.0));

        let result = betweenness_centrality(&graph, false);
        assert!(result.is_ok());

        let centrality = result.unwrap();
        assert_eq!(centrality.len(), 3);
        // Node 2 is on the path between 1 and 3
        assert!(centrality[&n2] > 0.0);
    }

    #[test]
    fn test_betweenness_centrality_empty_graph() {
        let graph: Graph<i32, OrderedFloat<f64>> = Graph::new();
        let result = betweenness_centrality(&graph, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_edge_betweenness_centrality() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        graph.add_edge(n1, n2, OrderedFloat(1.0));
        graph.add_edge(n2, n3, OrderedFloat(1.0));

        let result = edge_betweenness_centrality(&graph, false);
        assert!(result.is_ok());

        let centrality = result.unwrap();
        assert!(!centrality.is_empty());
    }
}
