//! Approximation algorithms for treewidth problems.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

/// Compute a treewidth decomposition using the Minimum Degree heuristic with a min-heap.
pub fn treewidth_min_degree<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> (usize, Vec<NodeId>)
where
    Ty: GraphConstructor<A, f64>,
{
    let mut order = Vec::new();
    let mut remaining: HashSet<NodeId> = graph.nodes().map(|(u, _)| u).collect();

    // Early return for empty graph
    if remaining.is_empty() {
        return (0, order);
    }

    let mut neighbor_cache: HashMap<NodeId, HashSet<NodeId>> = graph
        .nodes()
        .map(|(u, _)| (u, graph.neighbors(u).collect()))
        .collect();
    let mut treewidth = 0;
    let mut heap: BinaryHeap<Reverse<(usize, NodeId)>> = BinaryHeap::new();
    for (&u, neighbors) in &neighbor_cache {
        heap.push(Reverse((neighbors.len(), u)));
    }

    while !remaining.is_empty() {
        // Find the next node to eliminate
        let u = loop {
            match heap.pop() {
                Some(Reverse((deg, node))) => {
                    if remaining.contains(&node) {
                        if deg > treewidth {
                            treewidth = deg;
                        }
                        break node;
                    }
                    // Node already processed, continue
                }
                None => {
                    // Heap is empty but we have remaining nodes - shouldn't happen
                    // This can occur if the graph has isolated nodes
                    // Pick any remaining node
                    if let Some(&node) = remaining.iter().next() {
                        break node;
                    } else {
                        // If no remaining nodes, we're done - this shouldn't happen
                        // because the while condition checks !remaining.is_empty()
                        unreachable!("remaining set should not be empty while in loop");
                    }
                }
            }
        };

        order.push(u);
        remaining.remove(&u);

        // Get neighbors of u - use defensive programming
        let neighbors = neighbor_cache.get(&u).cloned().unwrap_or_else(HashSet::new);

        for &v in &neighbors {
            if remaining.contains(&v) {
                if let Some(entry) = neighbor_cache.get_mut(&v) {
                    entry.remove(&u);
                    heap.push(Reverse((entry.len(), v)));
                }
            }
        }
    }
    (treewidth, order)
}

/// Compute a treewidth decomposition using the Minimum Fill-in heuristic.
pub fn treewidth_min_fill_in<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> (usize, Vec<NodeId>)
where
    Ty: GraphConstructor<A, f64>,
{
    let mut order = Vec::new();
    let mut remaining: HashSet<NodeId> = graph.nodes().map(|(u, _)| u).collect();

    // Early return for empty graph
    if remaining.is_empty() {
        return (0, order);
    }

    let mut treewidth = 0;
    while !remaining.is_empty() {
        let u = remaining
            .iter()
            .min_by_key(|&&u| {
                let neighbors: Vec<NodeId> = graph
                    .neighbors(u)
                    .filter(|v| remaining.contains(v))
                    .collect();
                let mut fill_in = 0;
                for i in 0..neighbors.len() {
                    for j in i + 1..neighbors.len() {
                        if !graph.neighbors(neighbors[i]).any(|x| x == neighbors[j]) {
                            fill_in += 1;
                        }
                    }
                }
                fill_in
            })
            .copied() // Safe: we know remaining is not empty
            .expect("remaining set should not be empty in this context");

        let deg = graph.neighbors(u).filter(|v| remaining.contains(v)).count();
        if deg > treewidth {
            treewidth = deg;
        }
        order.push(u);
        remaining.remove(&u);
    }
    (treewidth, order)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_treewidth_empty_graph() {
        let graph: Graph<i32, f64> = Graph::new();
        let (tw, order) = treewidth_min_degree(&graph);
        assert_eq!(tw, 0);
        assert!(order.is_empty());
    }

    #[test]
    fn test_treewidth_single_node() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let (tw, order) = treewidth_min_degree(&graph);
        assert_eq!(tw, 0);
        assert_eq!(order.len(), 1);
        assert_eq!(order[0], n1);
    }

    #[test]
    fn test_treewidth_path() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);

        let (tw, order) = treewidth_min_degree(&graph);
        assert!(tw <= 1); // Path has treewidth 1
        assert_eq!(order.len(), 3);
    }

    #[test]
    fn test_treewidth_min_fill_in_empty() {
        let graph: Graph<i32, f64> = Graph::new();
        let (tw, order) = treewidth_min_fill_in(&graph);
        assert_eq!(tw, 0);
        assert!(order.is_empty());
    }
}
