//! Betweenness centrality algorithms.
//!
//! This module provides betweenness centrality measures.
//!
//! Convention: returns `Result<_, crate::core::error::GraphinaError>` to surface
//! invalid inputs and improve observability and error propagation.

use crate::core::error::{GraphinaError, Result};
use crate::core::types::{BaseGraph, GraphConstructor, NodeId, NodeMap};
use std::collections::{HashMap, VecDeque};

/// Returns an upper bound on node indices, for sizing dense `Vec`s indexed by
/// `NodeId::index()`. Indices are stable but not contiguous after removals, so
/// this bound (not `node_count`) keeps `vec[id.index()]` in range.
fn dist_bound<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> usize
where
    Ty: GraphConstructor<A, W>,
{
    graph
        .node_ids()
        .map(|n| n.index())
        .max()
        .map_or(0, |m| m + 1)
}

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
    graph: &BaseGraph<A, f64, Ty>,
    normalized: bool,
) -> Result<NodeMap<f64>>
where
    Ty: GraphConstructor<A, f64>,
{
    let n = graph.node_count();
    if n == 0 {
        return Err(GraphinaError::invalid_graph(
            "Cannot compute betweenness centrality on an empty graph.",
        ));
    }

    // Dense, index-keyed buffers reused across all sources. `vec[id.index()]` is
    // hash-free in the inner loops; we convert to the `NodeMap` return type once
    // at the end. See `dist_bound` below for why the bound, not `node_count`.
    let bound = dist_bound(graph);
    let mut centrality_vec = vec![0.0f64; bound];
    let mut preds: Vec<Vec<NodeId>> = vec![Vec::new(); bound];
    let mut sigma = vec![0.0f64; bound];
    let mut dist = vec![-1.0f64; bound];
    let mut delta = vec![0.0f64; bound];
    let mut stack: Vec<NodeId> = Vec::new();
    let mut queue: VecDeque<NodeId> = VecDeque::new();

    for (s, _) in graph.nodes() {
        // Reset per-source state, reusing the buffers' allocations.
        stack.clear();
        for i in 0..bound {
            preds[i].clear();
            sigma[i] = 0.0;
            dist[i] = -1.0;
            delta[i] = 0.0;
        }
        let si = s.index();
        sigma[si] = 1.0;
        dist[si] = 0.0;
        queue.push_back(s);

        // BFS to find shortest paths
        while let Some(v) = queue.pop_front() {
            let vi = v.index();
            stack.push(v);
            let v_dist = dist[vi];

            for w in graph.neighbors(v) {
                let wi = w.index();
                // w found for the first time?
                if dist[wi] < 0.0 {
                    dist[wi] = v_dist + 1.0;
                    queue.push_back(w);
                }
                // shortest path to w via v?
                if dist[wi] == v_dist + 1.0 {
                    sigma[wi] += sigma[vi];
                    preds[wi].push(v);
                }
            }
        }

        // Accumulation
        while let Some(w) = stack.pop() {
            let wi = w.index();
            let delta_w = delta[wi];
            let sigma_w = sigma[wi];

            for &v in &preds[wi] {
                let contribution = (sigma[v.index()] / sigma_w) * (1.0 + delta_w);
                delta[v.index()] += contribution;
            }

            if w != s {
                centrality_vec[wi] += delta_w;
            }
        }
    }

    let mut centrality = NodeMap::with_capacity_and_hasher(n, rustc_hash::FxBuildHasher);
    for node in graph.node_ids() {
        centrality.insert(node, centrality_vec[node.index()]);
    }

    if normalized {
        // Brandes accumulates each shortest path from both endpoints on an
        // undirected graph, so the normalization constant is the same for
        // directed and undirected graphs: the doubled undirected count cancels
        // the halved pair count.
        if n > 2 {
            let norm = 1.0 / ((n - 1) * (n - 2)) as f64;
            for val in centrality.values_mut() {
                *val *= norm;
            }
        }
    } else if !graph.is_directed() {
        // Unnormalized undirected betweenness halves the raw count, since each
        // shortest path is accumulated once from each of its two endpoints.
        for val in centrality.values_mut() {
            *val *= 0.5;
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
    graph: &BaseGraph<A, f64, Ty>,
    normalized: bool,
) -> Result<HashMap<(NodeId, NodeId), f64>>
where
    Ty: GraphConstructor<A, f64>,
{
    let n = graph.node_count();
    if n == 0 {
        return Err(GraphinaError::invalid_graph(
            "Cannot compute edge betweenness centrality on an empty graph.",
        ));
    }

    // Accumulate into an Fx-hashed edge map for the hot inner updates; convert to
    // the std `HashMap` return type once at the end.
    let mut centrality: rustc_hash::FxHashMap<(NodeId, NodeId), f64> =
        rustc_hash::FxHashMap::default();
    for (u, v, _) in graph.edges() {
        centrality.insert((u, v), 0.0);
        if !graph.is_directed() {
            centrality.insert((v, u), 0.0);
        }
    }

    // Dense, index-keyed buffers reused across all sources (see
    // `betweenness_centrality`).
    let bound = dist_bound(graph);
    let mut preds: Vec<Vec<NodeId>> = vec![Vec::new(); bound];
    let mut sigma = vec![0.0f64; bound];
    let mut dist = vec![-1.0f64; bound];
    let mut delta = vec![0.0f64; bound];
    let mut stack: Vec<NodeId> = Vec::new();
    let mut queue: VecDeque<NodeId> = VecDeque::new();

    for (s, _) in graph.nodes() {
        stack.clear();
        for i in 0..bound {
            preds[i].clear();
            sigma[i] = 0.0;
            dist[i] = -1.0;
            delta[i] = 0.0;
        }
        let si = s.index();
        sigma[si] = 1.0;
        dist[si] = 0.0;
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            let vi = v.index();
            stack.push(v);
            let v_dist = dist[vi];

            for w in graph.neighbors(v) {
                let wi = w.index();
                if dist[wi] < 0.0 {
                    dist[wi] = v_dist + 1.0;
                    queue.push_back(w);
                }
                if dist[wi] == v_dist + 1.0 {
                    sigma[wi] += sigma[vi];
                    preds[wi].push(v);
                }
            }
        }

        while let Some(w) = stack.pop() {
            let wi = w.index();
            let delta_w = delta[wi];
            let sigma_w = sigma[wi];

            for &v in &preds[wi] {
                let contribution = (sigma[v.index()] / sigma_w) * (1.0 + delta_w);
                delta[v.index()] += contribution;

                // Update edge centrality
                if let Some(edge_cent) = centrality.get_mut(&(v, w)) {
                    *edge_cent += contribution;
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

    Ok(centrality.into_iter().collect())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_betweenness_centrality_two_nodes_division_by_zero_fix() {
        use crate::centrality::betweenness::betweenness_centrality;
        use crate::core::types::Graph;

        let mut graph = Graph::<i32, f64>::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);

        graph.add_edge(n1, n2, 1.0);

        let result = betweenness_centrality(&graph, true);
        assert!(result.is_ok());

        let centrality = result.unwrap();
        assert_eq!(centrality.len(), 2);
        assert_eq!(*centrality.get(&n1).unwrap(), 0.0);
        assert_eq!(*centrality.get(&n2).unwrap(), 0.0);
    }

    // Regression: unnormalized undirected betweenness did not halve the raw Brandes
    // count (which accumulates each shortest path from both endpoints), so values
    // were double the standard definition. On the unit-weight path 0-1-2-3 the
    // middle nodes have unnormalized betweenness 2.0, not 4.0.
    #[test]
    fn test_betweenness_undirected_halving() {
        use crate::centrality::betweenness::betweenness_centrality;
        use crate::core::types::Graph;

        let mut g = Graph::<i32, f64>::new();
        let nodes: Vec<_> = (0..4).map(|i| g.add_node(i)).collect();
        g.add_edge(nodes[0], nodes[1], 1.0);
        g.add_edge(nodes[1], nodes[2], 1.0);
        g.add_edge(nodes[2], nodes[3], 1.0);

        let bc = betweenness_centrality(&g, false).expect("betweenness should succeed");
        assert!((bc[&nodes[0]] - 0.0).abs() < 1e-9);
        assert!(
            (bc[&nodes[1]] - 2.0).abs() < 1e-9,
            "expected 2.0, got {}",
            bc[&nodes[1]]
        );
        assert!(
            (bc[&nodes[2]] - 2.0).abs() < 1e-9,
            "expected 2.0, got {}",
            bc[&nodes[2]]
        );
        assert!((bc[&nodes[3]] - 0.0).abs() < 1e-9);
    }
    use super::{betweenness_centrality, edge_betweenness_centrality};
    use crate::core::types::Graph;

    #[test]
    fn test_betweenness_centrality_simple() {
        let mut graph = Graph::<i32, f64>::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);

        let result = betweenness_centrality(&graph, false);
        assert!(result.is_ok());

        let centrality = result.unwrap();
        assert_eq!(centrality.len(), 3);
        // Node 2 is on the path between 1 and 3
        assert!(centrality[&n2] > 0.0);
    }

    #[test]
    fn test_betweenness_centrality_empty_graph() {
        let graph: Graph<i32, f64> = Graph::new();
        let result = betweenness_centrality(&graph, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_edge_betweenness_centrality() {
        let mut graph = Graph::<i32, f64>::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);

        let result = edge_betweenness_centrality(&graph, false);
        assert!(result.is_ok());

        let centrality = result.unwrap();
        assert!(!centrality.is_empty());
    }
}
