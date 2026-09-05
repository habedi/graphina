//! Personalized PageRank algorithm.
//!
//! This module provides the raw personalized PageRank ranking vector. The NodeMap facade lives in
//! [`super::personalized`].

use crate::core::error::{GraphinaError, Result};
use crate::core::types::{BaseGraph, GraphConstructor, NodeId};

/// Production-level Personalized PageRank.
///
/// Computes a ranking vector for nodes using a damping factor, convergence tolerance, and a maximum
/// number of iterations. An optional personalization vector can be supplied; if not, a uniform vector is used.
///
/// Update rule:
///
/// rank_new\[j\] = (1 - damping) * p\[j\] + damping * Σ_i (rank\[i\] * (w_ij / outdegree\[i\]))
///
/// Dangling nodes (zero outdegree) redistribute their rank uniformly.
///
/// **Time Complexity:** O(max_iter * (n + m))
///
/// # Returns
/// A vector of f64 scores (one per node).
pub fn personalized_page_rank<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    personalization: Option<Vec<f64>>,
    damping: f64,
    tol: f64,
    max_iter: usize,
) -> Result<Vec<f64>>
where
    W: Copy + PartialOrd + Into<f64> + From<u8>,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    if n == 0 {
        return Err(GraphinaError::invalid_graph(
            "PersonalizedPageRank: empty graph",
        ));
    }
    if damping <= 0.0 || damping >= 1.0 {
        return Err(GraphinaError::invalid_graph(
            "PersonalizedPageRank: damping out of (0,1) range",
        ));
    }
    if max_iter == 0 {
        return Err(GraphinaError::invalid_graph(
            "PersonalizedPageRank: max_iter=0",
        ));
    }
    if let Some(vec) = &personalization {
        if vec.len() != n {
            return Err(GraphinaError::invalid_argument(format!(
                "PersonalizedPageRank: personalization has {} entries but the graph has {} nodes",
                vec.len(),
                n
            )));
        }
    }
    let p = if let Some(mut vec) = personalization {
        let sum: f64 = vec.iter().sum();
        if sum > 0.0 {
            for val in vec.iter_mut() {
                *val /= sum;
            }
            vec
        } else {
            vec![1.0 / n as f64; n]
        }
    } else {
        vec![1.0 / n as f64; n]
    };

    // Stable node list mapping to contiguous indices
    let node_list: Vec<NodeId> = graph.nodes().map(|(nid, _)| nid).collect();
    let mut node_to_idx = std::collections::HashMap::new();
    for (i, &nid) in node_list.iter().enumerate() {
        node_to_idx.insert(nid, i);
    }
    let mut outdegree = vec![0.0; n];
    let mut neighbors: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n];
    let undirected = !graph.is_directed();
    for (u, v, &w) in graph.edges() {
        let ui = node_to_idx[&u];
        let vi = node_to_idx[&v];
        let weight: f64 = w.into();
        outdegree[ui] += weight;
        neighbors[ui].push((vi, weight));
        // A self-loop is a single edge in both directions, so it is added once.
        if undirected && ui != vi {
            outdegree[vi] += weight;
            neighbors[vi].push((ui, weight));
        }
    }

    let mut rank = p.clone();
    for _ in 0..max_iter {
        let mut new_rank = vec![0.0; n];
        for (j, nr) in new_rank.iter_mut().enumerate() {
            *nr += (1.0 - damping) * p[j];
        }
        for (i, &deg_i) in outdegree.iter().enumerate() {
            if deg_i > 0.0 {
                let contribution = damping * rank[i] / deg_i;
                for &(j, weight) in &neighbors[i] {
                    new_rank[j] += contribution * weight;
                }
            } else {
                // Dangling nodes redistribute their rank according to the
                // personalization vector (which is uniform for plain PageRank),
                // matching the standard personalized PageRank convention.
                let dangling = damping * rank[i];
                for (j, nr) in new_rank.iter_mut().enumerate() {
                    *nr += dangling * p[j];
                }
            }
        }
        let diff: f64 = rank
            .iter()
            .zip(new_rank.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        rank = new_rank;
        if diff < tol {
            break;
        }
    }
    // rank vector aligned with node_list order
    Ok(rank)
}

#[cfg(test)]
mod tests {
    use super::personalized_page_rank;
    use crate::core::types::Graph;

    fn path_graph() -> Graph<i32, f64> {
        let mut g = Graph::<i32, f64>::new();
        let a = g.add_node(0);
        let b = g.add_node(1);
        let c = g.add_node(2);
        g.add_edge(a, b, 1.0);
        g.add_edge(b, c, 1.0);
        g
    }

    #[test]
    fn personalization_length_mismatch_is_an_error() {
        let g = path_graph();
        assert!(personalized_page_rank(&g, Some(vec![1.0]), 0.85, 1e-9, 100).is_err());
        assert!(personalized_page_rank(&g, Some(vec![1.0; 4]), 0.85, 1e-9, 100).is_err());
        assert!(personalized_page_rank(&g, Some(vec![1.0; 3]), 0.85, 1e-9, 100).is_ok());
    }

    #[test]
    fn undirected_self_loop_counts_once() {
        // Node a has a self-loop and an edge to b. With the self-loop counted once
        // the walk leaves a with probability 1/2 in each direction, so the
        // stationary ranks are 0.925/1.425 for a and 0.5/1.425 for b.
        let mut g = Graph::<i32, f64>::new();
        let a = g.add_node(0);
        let b = g.add_node(1);
        g.add_edge(a, a, 1.0);
        g.add_edge(a, b, 1.0);
        let ranks = personalized_page_rank(&g, None, 0.85, 1e-12, 1000).unwrap();
        assert!((ranks[0] - 0.925 / 1.425).abs() < 1e-6, "a = {}", ranks[0]);
        assert!((ranks[1] - 0.5 / 1.425).abs() < 1e-6, "b = {}", ranks[1]);
    }
}
