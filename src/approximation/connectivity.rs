//! Connectivity approximation algorithms.
//!
//! Approximation algorithms for connectivity problems.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::{HashSet, VecDeque};

/// -------------------------------
/// Helper: Find a path from `source` to `target` avoiding nodes in `blocked` using BFS.
fn find_path<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    source: NodeId,
    target: NodeId,
    blocked: &HashSet<NodeId>,
    forbid_direct: bool,
) -> Option<Vec<NodeId>>
where
    Ty: crate::core::types::GraphConstructor<A, f64>,
{
    // Size the BFS buffers by the index bound, not `node_count`: `BaseGraph` wraps
    // a `StableGraph`, so after a node removal a remaining node's index can exceed
    // the count, and these buffers are indexed by `NodeId::index()`.
    let bound = graph
        .node_ids()
        .map(|node| node.index())
        .max()
        .map_or(0, |m| m + 1);
    let mut prev: Vec<Option<NodeId>> = vec![None; bound];
    let mut visited = vec![false; bound];
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
            // The single-hop source-target path has no intermediate node to
            // block, so the disjoint-path search skips it and counts it apart.
            if forbid_direct && u == source && v == target {
                continue;
            }
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
    graph: &BaseGraph<A, f64, Ty>,
    source: NodeId,
    target: NodeId,
) -> usize
where
    Ty: GraphConstructor<A, f64>,
{
    // Quick check: if source equals target, return 0
    if source == target {
        return 0;
    }

    let mut connectivity = 0;

    // The direct edge is a vertex-disjoint path with no intermediate node, so
    // it can never be excluded by blocking; count it once here and forbid the
    // single-hop path in the searches below.
    if graph.neighbors(source).any(|v| v == target) {
        connectivity += 1;
    }

    let mut blocked = HashSet::new();

    // Limit iterations to prevent infinite loops
    let max_iterations = graph.node_count();
    let mut iterations = 0;

    while let Some(path) = find_path(graph, source, target, &blocked, true) {
        iterations += 1;
        if iterations > max_iterations {
            // Safety check: prevent infinite loops
            break;
        }

        // Every found path has at least one intermediate node (the direct hop
        // is forbidden); block them all so the next path is vertex-disjoint.
        for &node in path.iter().skip(1).take(path.len().saturating_sub(2)) {
            blocked.insert(node);
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
        g.add_edge(n1, n2, 1.0);

        // Direct edge should return 1 without hanging
        let conn = local_node_connectivity(&g, n1, n2);
        assert_eq!(conn, 1);
    }

    #[test]
    fn test_local_node_connectivity_direct_edge_plus_indirect_path() {
        // A triangle has two vertex-disjoint s-t paths: the direct edge and the
        // path through the third node. Stopping at the direct edge undercounts.
        let mut g = Graph::new();
        let s = g.add_node(0);
        let t = g.add_node(1);
        let a = g.add_node(2);
        g.add_edge(s, t, 1.0);
        g.add_edge(s, a, 1.0);
        g.add_edge(a, t, 1.0);

        let conn = local_node_connectivity(&g, s, t);
        assert_eq!(conn, 2);
    }

    #[test]
    fn test_local_node_connectivity_complete_graph() {
        // In K5 any two nodes are joined by the direct edge plus three paths
        // through the remaining nodes, so the connectivity is 4.
        let mut g = Graph::new();
        let nodes: Vec<_> = (0..5).map(|i| g.add_node(i)).collect();
        for i in 0..5 {
            for j in (i + 1)..5 {
                g.add_edge(nodes[i], nodes[j], 1.0);
            }
        }

        let conn = local_node_connectivity(&g, nodes[0], nodes[1]);
        assert_eq!(conn, 4);
    }

    #[test]
    fn test_local_node_connectivity_with_intermediate() {
        let mut g = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);

        let conn = local_node_connectivity(&g, n1, n3);
        assert!(conn >= 1);
    }

    #[test]
    fn test_local_node_connectivity_sparse_indices_after_removal() {
        // Removing a node leaves stable but non-contiguous indices, so a remaining
        // node's index can exceed `node_count()`. Sizing the BFS buffers by
        // `node_count()` and indexing them by `NodeId::index()` panics out of
        // range. There is one path between the surviving endpoints.
        let mut g = Graph::new();
        let nodes: Vec<_> = (0..4).map(|i| g.add_node(i)).collect();
        g.remove_node(nodes[1]);
        // Remaining nodes 0, 2, 3 (indices 0, 2, 3; node_count is now 3).
        g.add_edge(nodes[0], nodes[2], 1.0);
        g.add_edge(nodes[2], nodes[3], 1.0);

        let conn = local_node_connectivity(&g, nodes[0], nodes[3]);
        assert_eq!(conn, 1);
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
