//! Connected components algorithms.
//!
//! This module provides connected components for community detection.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId, NodeMap, NodeSet};
use std::collections::VecDeque;

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
    let mut visited: NodeSet = NodeSet::default();
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

/// Compute connected components and return a NodeId -> component ID mapping.
///
/// Component IDs are assigned in the order components are discovered.
pub fn connected_components_map<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> NodeMap<usize>
where
    W: Copy,
    Ty: GraphConstructor<A, W>,
{
    let lists = connected_components(graph);
    let mut map: NodeMap<usize> = NodeMap::default();
    for (cid, comp) in lists.into_iter().enumerate() {
        for node in comp {
            map.insert(node, cid);
        }
    }
    map
}

/// Compute the weakly connected components of a graph using BFS.
///
/// Edges are followed in both directions, so a directed graph is treated as
/// undirected for the purpose of connectivity. On an undirected graph this is
/// equivalent to [`connected_components`].
///
/// **Time Complexity:** O(n + m)
///
/// # Returns
/// A vector of components, where each component is a vector of `NodeId`s.
pub fn weakly_connected_components<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Vec<Vec<NodeId>>
where
    W: Copy,
    Ty: GraphConstructor<A, W>,
{
    let mut visited: NodeSet = NodeSet::default();
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

            // Follow both outgoing and incoming edges so direction is ignored.
            for neighbor in graph.neighbors(node).chain(graph.incoming_neighbors(node)) {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        components.push(component);
    }

    components
}

/// Compute the strongly connected components of a directed graph.
///
/// A strongly connected component is a maximal set of nodes in which every node
/// is reachable from every other node following edge direction. This uses
/// Tarjan's algorithm. On an undirected graph each component is also strongly
/// connected, so the result matches [`connected_components`].
///
/// **Time Complexity:** O(n + m)
///
/// # Returns
/// A vector of components, where each component is a vector of `NodeId`s.
pub fn strongly_connected_components<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Vec<Vec<NodeId>>
where
    W: Copy,
    Ty: GraphConstructor<A, W>,
{
    petgraph::algo::tarjan_scc(graph.as_petgraph())
        .into_iter()
        .map(|component| component.into_iter().map(NodeId::new).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{Digraph, Graph};

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

    #[test]
    fn test_connected_components_map() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n3, n4, 1.0);

        let lists = connected_components(&g);
        let map = connected_components_map(&g);

        assert_eq!(lists.len(), 2);
        assert_eq!(map.len(), 4);
        assert_eq!(map[&n1], map[&n2]);
        assert_eq!(map[&n3], map[&n4]);
        assert_ne!(map[&n1], map[&n3]);
    }

    fn sorted_partition(components: Vec<Vec<NodeId>>) -> Vec<Vec<usize>> {
        let mut parts: Vec<Vec<usize>> = components
            .into_iter()
            .map(|c| {
                let mut v: Vec<usize> = c.iter().map(|n| n.index()).collect();
                v.sort_unstable();
                v
            })
            .collect();
        parts.sort();
        parts
    }

    #[test]
    fn test_weakly_connected_components_directed() {
        // A directed path 0 -> 1 -> 2 is one weakly connected component even
        // though no node reaches every other following direction.
        let mut g = Digraph::<i32, f64>::new();
        let n0 = g.add_node(0);
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let _n3 = g.add_node(3);
        g.add_edge(n0, n1, 1.0);
        g.add_edge(n1, n2, 1.0);

        let wcc = weakly_connected_components(&g);
        assert_eq!(
            sorted_partition(wcc),
            vec![vec![0, 1, 2], vec![3]],
            "the path 0->1->2 is one weak component; node 3 is isolated"
        );
    }

    #[test]
    fn test_strongly_connected_components_directed() {
        // A directed cycle 0 -> 1 -> 2 -> 0 is one strongly connected component;
        // the extra edge to node 3 is its own component (no path back).
        let mut g = Digraph::<i32, f64>::new();
        let n0 = g.add_node(0);
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(n0, n1, 1.0);
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n0, 1.0);
        g.add_edge(n2, n3, 1.0);

        let scc = strongly_connected_components(&g);
        assert_eq!(
            sorted_partition(scc),
            vec![vec![0, 1, 2], vec![3]],
            "the cycle 0->1->2->0 is one strong component; node 3 stands alone"
        );

        // The same graph is a single weakly connected component.
        let wcc = weakly_connected_components(&g);
        assert_eq!(sorted_partition(wcc), vec![vec![0, 1, 2, 3]]);
    }

    #[test]
    fn test_weak_and_strong_match_on_undirected() {
        // On an undirected graph WCC, SCC, and connected_components agree.
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n3, n4, 1.0);

        let cc = sorted_partition(connected_components(&g));
        assert_eq!(sorted_partition(weakly_connected_components(&g)), cc);
        assert_eq!(sorted_partition(strongly_connected_components(&g)), cc);
    }
}
