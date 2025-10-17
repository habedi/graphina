//! Infomap algorithms.
//!
//! This module provides Infomap for community detection.

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

/// Production-level Infomap (simplified) for community detection.
///
/// Inspired by the map equation framework, each node is initially in its own module.
/// In randomized order, each node is re-assigned to a neighbor's module if that move increases flow (reduces description length).
///
/// **Time Complexity:** Approximately O(max_iter * (n + m))
///
/// # Parameters
/// - `max_iter`: Maximum number of iterations.
/// - `seed`: Optional seed for RNG used for shuffling nodes.
///
/// # Returns
/// A vector (length n) of module assignments (usize) for each node.
pub fn infomap<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    max_iter: usize,
    seed: Option<u64>,
) -> Vec<usize>
where
    W: Copy + PartialOrd + Into<f64> + From<u8>,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    let mut modules: Vec<usize> = (0..n).collect();
    let mut rng = create_rng(seed);
    let mut iter = 0;

    loop {
        let mut changed = false;
        let mut nodes: Vec<usize> = (0..n).collect();
        nodes.shuffle(&mut rng);
        for &i in &nodes {
            let mut flow: HashMap<usize, f64> = HashMap::new();
            let mut total_flow = 0.0;
            for (src, tgt, &w) in graph.edges() {
                let weight: f64 = w.into();
                if src.index() == i {
                    let module = modules[tgt.index()];
                    *flow.entry(module).or_insert(0.0) += weight;
                    total_flow += weight;
                }
                if tgt.index() == i {
                    let module = modules[src.index()];
                    *flow.entry(module).or_insert(0.0) += weight;
                    total_flow += weight;
                }
            }
            if total_flow > 0.0 {
                for val in flow.values_mut() {
                    *val /= total_flow;
                }
            }
            if let Some((&best_module, _)) =
                flow.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            {
                if best_module != modules[i] {
                    modules[i] = best_module;
                    changed = true;
                }
            }
        }
        iter += 1;
        if !changed || iter >= max_iter {
            break;
        }
    }
    modules
}
