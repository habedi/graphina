//! Connected components algorithms.
//!
//! This module provides connected components for community detection.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::VecDeque;

/// Compute connected components of an undirected graph using BFS.
///
/// **Time Complexity:** O(n + m)
///
/// # Returns
/// A vector of components, where each component is a vector of `NodeId`s.
pub fn connected_components<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Vec<Vec<NodeId>>
where
    W: Copy,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    let mut visited = vec![false; n];
    let mut components = Vec::new();
    for i in 0..n {
        if !visited[i] {
            let mut comp = Vec::new();
            let mut queue = VecDeque::new();
            queue.push_back(i);
            visited[i] = true;
            while let Some(u) = queue.pop_front() {
                if let Some((node, _)) = graph.nodes().find(|(node, _)| node.index() == u) {
                    comp.push(node);
                }
                for (src, tgt, _) in graph.edges() {
                    if src.index() == u {
                        let v = tgt.index();
                        if !visited[v] {
                            visited[v] = true;
                            queue.push_back(v);
                        }
                    }
                    if tgt.index() == u {
                        let v = src.index();
                        if !visited[v] {
                            visited[v] = true;
                            queue.push_back(v);
                        }
                    }
                }
            }
            components.push(comp);
        }
    }
    components
}
