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
        let Reverse((deg, u)) = heap.pop().unwrap();
        if !remaining.contains(&u) {
            continue;
        }
        if deg > treewidth {
            treewidth = deg;
        }
        order.push(u);
        remaining.remove(&u);
        let neighbors = neighbor_cache.get(&u).unwrap().clone();
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
    let mut treewidth = 0;
    while !remaining.is_empty() {
        let &u = remaining
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
            .unwrap();
        let deg = graph.neighbors(u).filter(|v| remaining.contains(v)).count();
        if deg > treewidth {
            treewidth = deg;
        }
        order.push(u);
        remaining.remove(&u);
    }
    (treewidth, order)
}
