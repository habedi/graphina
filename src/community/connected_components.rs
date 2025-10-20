//! Connected components algorithms.
//!
//! This module provides connected components for community detection.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::{HashSet, VecDeque};

/// Compute connected components of an undirected graph using BFS.
///
/// **Time Complexity:** O(n + m)
///
/// # Returns
/// A vector of components, where each component is a vector of `NodeId`s.
///
/// # Correctness Fix
/// Previous implementation assumed contiguous node indices and had O(n*m) complexity
/// due to iterating over all edges for each node. This version uses proper neighbor
/// iteration and handles deleted nodes correctly.
pub fn connected_components<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Vec<Vec<NodeId>>
where
    W: Copy,
    Ty: GraphConstructor<A, W>,
{
    let mut visited: HashSet<NodeId> = HashSet::new();
    let mut components = Vec::new();

    for (start_node, _) in graph.nodes() {
        if visited.contains(&start_node) {
            continue;
        }

        let mut component = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(start_node);
        visited.insert(start_node);

        while let Some(node) = queue.pop_front() {
            component.push(node);

            // Use proper neighbor iterator instead of scanning all edges
            for neighbor in graph.neighbors(node) {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        components.push(component);
    }

    components
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_connected_components_simple() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n3, n4, 1.0);

        let components = connected_components(&g);
        assert_eq!(components.len(), 2);
    }

    #[test]
    fn test_connected_components_with_removed_nodes() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);
        let n5 = g.add_node(5);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n4, n5, 1.0);

        // Remove node in the middle
        g.remove_node(n2);

        let components = connected_components(&g);
        // Should have 3 components now: {n1}, {n3}, {n4, n5}
        assert_eq!(components.len(), 3);
    }

    #[test]
    fn test_connected_components_empty() {
        let g = Graph::<i32, f64>::new();
        let components = connected_components(&g);
        assert_eq!(components.len(), 0);
    }

    #[test]
    fn test_connected_components_single_node() {
        let mut g = Graph::<i32, f64>::new();
        g.add_node(1);
        let components = connected_components(&g);
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].len(), 1);
    }
}
