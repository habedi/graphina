//! Connectivity approximation algorithms.
//!
//! This module provides approximations for connectivity-related problems.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use ordered_float::OrderedFloat;
use std::collections::{HashSet, VecDeque};

/// -------------------------------
/// Helper: Find a path from `source` to `target` avoiding nodes in `blocked` using BFS.
fn find_path<A, Ty>(
    graph: &BaseGraph<A, OrderedFloat<f64>, Ty>,
    source: NodeId,
    target: NodeId,
    blocked: &HashSet<NodeId>,
) -> Option<Vec<NodeId>>
where
    Ty: crate::core::types::GraphConstructor<A, OrderedFloat<f64>>,
{
    let n = graph.node_count();
    let mut prev: Vec<Option<NodeId>> = vec![None; n];
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    visited[source.index()] = true;
    queue.push_back(source);
    while let Some(u) = queue.pop_front() {
        if u == target {
            let mut path = Vec::new();
            let mut cur = u;
            path.push(cur);
            while let Some(p) = prev[cur.index()] {
                cur = p;
                path.push(cur);
            }
            path.reverse();
            return Some(path);
        }
        for v in graph.neighbors(u) {
            if !visited[v.index()] && !blocked.contains(&v) {
                visited[v.index()] = true;
                prev[v.index()] = Some(u);
                queue.push_back(v);
            }
        }
    }
    None
}

/// Compute an approximate local node connectivity between source and target by
/// repeatedly finding vertex-disjoint paths using BFS.
pub fn local_node_connectivity<A, Ty>(
    graph: &BaseGraph<A, OrderedFloat<f64>, Ty>,
    source: NodeId,
    target: NodeId,
) -> usize
where
    Ty: GraphConstructor<A, OrderedFloat<f64>>,
{
    // Quick check: if source equals target, return 0
    if source == target {
        return 0;
    }

    let mut connectivity = 0;
    let mut blocked = HashSet::new();

    // Limit iterations to prevent infinite loops
    let max_iterations = graph.node_count();
    let mut iterations = 0;

    while let Some(path) = find_path(graph, source, target, &blocked) {
        iterations += 1;
        if iterations > max_iterations {
            // Safety check: prevent infinite loops
            break;
        }

        // Block all intermediate nodes (exclude source and target)
        // For a path [s, n1, n2, ..., nk, t], we want to block n1, n2, ..., nk
        if path.len() > 2 {
            for &node in path.iter().skip(1).take(path.len() - 2) {
                blocked.insert(node);
            }
        } else if path.len() == 2 {
            // Direct edge from source to target
            // No intermediate nodes to block, but we can't find more disjoint paths
            connectivity += 1;
            break;
        }

        connectivity += 1;
    }
    connectivity
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_local_node_connectivity_direct_edge() {
        let mut g = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n1, n2, OrderedFloat(1.0));

        // Direct edge should return 1 without hanging
        let conn = local_node_connectivity(&g, n1, n2);
        assert_eq!(conn, 1);
    }

    #[test]
    fn test_local_node_connectivity_with_intermediate() {
        let mut g = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);

        g.add_edge(n1, n2, OrderedFloat(1.0));
        g.add_edge(n2, n3, OrderedFloat(1.0));

        let conn = local_node_connectivity(&g, n1, n3);
        assert!(conn >= 1);
    }

    #[test]
    fn test_local_node_connectivity_same_node() {
        let mut g = Graph::new();
        let n1 = g.add_node(1);

        // Same node should return 0
        let conn = local_node_connectivity(&g, n1, n1);
        assert_eq!(conn, 0);
    }
}
