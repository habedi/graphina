//! PageRank algorithms.
//!
//! This module provides PageRank centrality measures.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId, NodeMap};

/// PageRank: a link analysis algorithm that assigns a numerical weighting to each element
/// of a hyperlinked set of documents.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `damping`: damping factor (usually 0.85).
/// * `max_iter`: maximum number of iterations.
/// * `tolerance`: convergence tolerance.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing PageRank scores of each node in the graph.
pub fn pagerank<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    damping: f64,
    max_iter: usize,
    tolerance: f64,
) -> NodeMap<f64>
where
    W: Copy + PartialOrd + Into<f64>,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    if n == 0 {
        return NodeMap::new();
    }

    let mut out_degrees = vec![0; n];
    for (u, _, _) in graph.edges() {
        out_degrees[u.index()] += 1;
    }

    let mut pr = vec![1.0 / n as f64; n];
    let mut pr_new = vec![0.0; n];

    for _ in 0..max_iter {
        let mut dangling_sum = 0.0;
        for (i, &deg) in out_degrees.iter().enumerate() {
            if deg == 0 {
                dangling_sum += pr[i];
            }
        }
        dangling_sum *= damping / n as f64;

        for i in 0..n {
            pr_new[i] = (1.0 - damping) / n as f64 + dangling_sum;
            for (u, v, _) in graph.edges() {
                if v.index() == i && out_degrees[u.index()] > 0 {
                    pr_new[i] += damping * pr[u.index()] / out_degrees[u.index()] as f64;
                }
            }
        }

        let diff: f64 = pr
            .iter()
            .zip(pr_new.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        pr.copy_from_slice(&pr_new);

        if diff < tolerance {
            break;
        }
    }

    let mut centrality = NodeMap::new();
    for (i, &val) in pr.iter().enumerate() {
        let node = NodeId::new(petgraph::graph::NodeIndex::new(i));
        centrality.insert(node, val);
    }
    centrality
}
