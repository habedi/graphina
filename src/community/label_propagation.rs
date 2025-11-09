//! Label propagation algorithms.
//!
//! This module provides label propagation for community detection.

use crate::core::error::{GraphinaError, Result};
use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use rand::prelude::*;
use rand::{SeedableRng, rngs::StdRng};
use std::collections::HashMap as StdHashMap;

/// Private helper: Create a seeded RNG from an optional seed.
fn create_rng(seed: Option<u64>) -> StdRng {
    match seed {
        Some(s) => StdRng::seed_from_u64(s),
        None => StdRng::seed_from_u64(rand::random::<u64>()),
    }
}

/// Production-level Label Propagation.
///
/// Each node is initially assigned its own community. In randomized order,
/// each node updates its label to the most frequent label among its neighbors.
/// The process stops when no changes occur or when `max_iter` iterations are reached.
///
/// **Time Complexity:** O(max_iter * (n + m))
///
/// # Parameters
/// - `max_iter`: Maximum number of iterations.
/// - `seed`: Optional seed for the RNG (for reproducibility).
///
/// # Returns
/// A vector (length n) of final community labels (usize) for each node.
/// Returns `GraphinaError::InvalidGraph` on empty graph or invalid parameters.
pub fn label_propagation<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    max_iter: usize,
    seed: Option<u64>,
) -> Result<Vec<usize>>
where
    W: Copy + PartialOrd + Into<f64>,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    if n == 0 {
        return Err(GraphinaError::invalid_graph(
            "LabelPropagation: empty graph",
        ));
    }
    if max_iter == 0 {
        return Err(GraphinaError::invalid_graph("LabelPropagation: max_iter=0"));
    }
    // Build stable node list and mapping to contiguous indices
    let node_list: Vec<NodeId> = graph.nodes().map(|(nid, _)| nid).collect();
    let mut node_to_idx: StdHashMap<NodeId, usize> = StdHashMap::new();
    for (i, &nid) in node_list.iter().enumerate() {
        node_to_idx.insert(nid, i);
    }
    let mut labels: Vec<usize> = (0..n).collect();
    let mut rng = create_rng(seed);
    let mut iter = 0;

    loop {
        let mut changed = false;
        let mut nodes: Vec<usize> = (0..n).collect();
        nodes.shuffle(&mut rng);
        for &i in &nodes {
            let mut freq: StdHashMap<usize, usize> = StdHashMap::new();
            // Count labels among neighbors (treated as undirected).
            for (src, tgt, _w) in graph.edges() {
                let si = node_to_idx[&src];
                let ti = node_to_idx[&tgt];
                if si == i {
                    *freq.entry(labels[ti]).or_insert(0) += 1;
                }
                if ti == i {
                    *freq.entry(labels[si]).or_insert(0) += 1;
                }
            }
            if let Some((&best_label, _)) = freq.iter().max_by_key(|&(_, count)| count) {
                if best_label != labels[i] {
                    labels[i] = best_label;
                    changed = true;
                }
            }
        }
        iter += 1;
        if !changed || iter >= max_iter {
            break;
        }
    }
    Ok(labels)
}
