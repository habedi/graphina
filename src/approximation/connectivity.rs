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
    let mut connectivity = 0;
    let mut blocked = HashSet::new();
    while let Some(path) = find_path(graph, source, target, &blocked) {
        // Block all intermediate nodes (exclude source and target)
        for &node in path.iter().skip(1).take(path.len() - 2) {
            blocked.insert(node);
        }
        connectivity += 1;
    }
    connectivity
}
