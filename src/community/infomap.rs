//! Infomap algorithms.
//!
//! This module provides Infomap for community detection.

use crate::core::error::{GraphinaError, Result};
use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
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
/// Returns `GraphinaError::InvalidGraph` on empty graph or invalid parameters.
pub fn infomap<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    max_iter: usize,
    seed: Option<u64>,
) -> Result<Vec<usize>>
where
    W: Copy + PartialOrd + Into<f64> + From<u8>,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    if n == 0 {
        return Err(GraphinaError::invalid_graph("Infomap: empty graph"));
    }
    if max_iter == 0 {
        return Err(GraphinaError::invalid_graph("Infomap: max_iter=0"));
    }
    // Map nodes to contiguous indices and build a weighted adjacency list once
    // (O(E)). The previous version scanned every edge for every node on every
    // iteration (O(max_iter * n * E)) and indexed by `NodeId::index()`, which
    // also broke after a node removal. Mirror the original both-endpoints
    // accumulation: edge (src, tgt) contributes to src's view of tgt and tgt's
    // view of src.
    let node_list: Vec<NodeId> = graph.nodes().map(|(node, _)| node).collect();
    let mut index_of: HashMap<NodeId, usize> = HashMap::with_capacity(n);
    for (i, &node) in node_list.iter().enumerate() {
        index_of.insert(node, i);
    }
    let mut adjacency: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n];
    for (src, tgt, &w) in graph.edges() {
        let s = index_of[&src];
        let t = index_of[&tgt];
        let weight: f64 = w.into();
        adjacency[s].push((t, weight));
        adjacency[t].push((s, weight));
    }

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
            for &(nbr, weight) in &adjacency[i] {
                let module = modules[nbr];
                *flow.entry(module).or_insert(0.0) += weight;
                total_flow += weight;
            }
            if total_flow > 0.0 {
                for val in flow.values_mut() {
                    *val /= total_flow;
                }
            }
            if let Some((&best_module, _)) = flow
                .iter()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
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
    Ok(modules)
}
