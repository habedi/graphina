//! Approximation algorithms for subgraph problems.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use ordered_float::OrderedFloat;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

/// Approximate the densest subgraph using a greedy peeling algorithm.
/// Uses a min-heap (with OrderedFloat) for efficiency.
pub fn densest_subgraph<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    _iterations: Option<usize>,
) -> HashSet<NodeId>
where
    Ty: GraphConstructor<A, f64>,
{
    let mut best_density = 0.0;
    let mut best_set = HashSet::new();
    let mut current_nodes: HashSet<NodeId> = graph.nodes().map(|(u, _)| u).collect();

    let mut degrees: HashMap<NodeId, usize> = current_nodes
        .iter()
        .map(|&u| {
            (
                u,
                graph
                    .neighbors(u)
                    .filter(|v| current_nodes.contains(v))
                    .count(),
            )
        })
        .collect();

    let mut heap: BinaryHeap<Reverse<(OrderedFloat<f64>, NodeId)>> = BinaryHeap::new();
    for (&u, &deg) in &degrees {
        heap.push(Reverse((OrderedFloat(deg as f64), u)));
    }

    while !current_nodes.is_empty() {
        let total_edges: usize = degrees.values().sum::<usize>() / 2;
        let density = total_edges as f64 / current_nodes.len() as f64;
        if density > best_density {
            best_density = density;
            best_set = current_nodes.clone();
        }
        if let Some(Reverse((_, u))) = heap.pop() {
            if !current_nodes.contains(&u) {
                continue;
            }
            current_nodes.remove(&u);
            let neighbors = graph.neighbors(u).collect::<HashSet<_>>();
            for v in neighbors {
                if current_nodes.contains(&v) {
                    if let Some(d) = degrees.get_mut(&v) {
                        *d = d.saturating_sub(1);
                        heap.push(Reverse((OrderedFloat(*d as f64), v)));
                    }
                }
            }
        } else {
            break;
        }
    }
    best_set
}
