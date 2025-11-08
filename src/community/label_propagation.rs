//! Label propagation algorithms.
//!
//! This module provides label propagation for community detection.

use crate::core::types::{BaseGraph, GraphConstructor};
use rand::prelude::*;
use rand::{SeedableRng, rngs::StdRng};
use std::collections::HashMap;

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
pub fn label_propagation<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    max_iter: usize,
    seed: Option<u64>,
) -> Vec<usize>
where
    W: Copy + PartialOrd + Into<f64>,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    let mut labels: Vec<usize> = (0..n).collect();
    let mut rng = create_rng(seed);
    let mut iter = 0;

    loop {
        let mut changed = false;
        let mut nodes: Vec<usize> = (0..n).collect();
        nodes.shuffle(&mut rng);
        for &i in &nodes {
            let mut freq: HashMap<usize, usize> = HashMap::new();
            // Count labels among neighbors (treated as undirected).
            for (src, tgt, _w) in graph.edges() {
                if src.index() == i {
                    *freq.entry(labels[tgt.index()]).or_insert(0) += 1;
                }
                if tgt.index() == i {
                    *freq.entry(labels[src.index()]).or_insert(0) += 1;
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
    labels
}
