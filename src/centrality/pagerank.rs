//! PageRank algorithms.
//!
//! This module provides PageRank centrality measures.
//!
//! Convention: functions in this module return `Result<_, crate::core::exceptions::GraphinaException>`
//! for better observability and error propagation.

use crate::core::exceptions::GraphinaException;
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
/// Returns an error only in exceptional cases.
pub fn pagerank<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    damping: f64,
    max_iter: usize,
    tolerance: f64,
) -> Result<NodeMap<f64>, GraphinaException>
where
    W: Copy + PartialOrd + Into<f64>,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    if n == 0 {
        return Ok(NodeMap::new());
    }

    // Build proper node index mapping to handle non-contiguous indices
    let node_list: Vec<NodeId> = graph.nodes().map(|(node, _)| node).collect();
    let mut node_to_idx = std::collections::HashMap::new();
    for (idx, &node) in node_list.iter().enumerate() {
        node_to_idx.insert(node, idx);
    }

    // Build adjacency structure: for each node, store (target_idx, weight)
    let mut out_degrees = vec![0.0; n];
    let mut out_edges: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n];

    for (u, v, w) in graph.edges() {
        let ui = node_to_idx[&u];
        let vi = node_to_idx[&v];
        let weight: f64 = (*w).into();
        out_degrees[ui] += weight;
        out_edges[ui].push((vi, weight));
    }

    let mut pr = vec![1.0 / n as f64; n];
    let mut pr_new = vec![0.0; n];

    for _ in 0..max_iter {
        // Handle dangling nodes (nodes with no outgoing edges)
        let mut dangling_sum = 0.0;
        for (i, &deg) in out_degrees.iter().enumerate() {
            if deg == 0.0 {
                dangling_sum += pr[i];
            }
        }
        dangling_sum *= damping / n as f64;

        // Initialize with teleportation probability and dangling contribution
        for pr_new_item in pr_new.iter_mut() {
            *pr_new_item = (1.0 - damping) / n as f64 + dangling_sum;
        }

        // Distribute rank from each node to its neighbors
        for (i, edges) in out_edges.iter().enumerate() {
            if out_degrees[i] > 0.0 {
                let contribution = damping * pr[i] / out_degrees[i];
                for &(j, weight) in edges {
                    pr_new[j] += contribution * weight;
                }
            }
        }

        // Check convergence
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

    // Convert to NodeMap using the node list
    let mut centrality = NodeMap::new();
    for (idx, &node) in node_list.iter().enumerate() {
        centrality.insert(node, pr[idx]);
    }
    Ok(centrality)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{Digraph, Graph};

    #[test]
    fn test_pagerank_simple_directed() {
        let mut graph: Digraph<i32, f64> = Digraph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n1, 1.0);

        let pr = pagerank(&graph, 0.85, 100, 1e-6).unwrap();

        // In a cycle, all nodes should have equal PageRank
        let pr1 = pr[&n1];
        let pr2 = pr[&n2];
        let pr3 = pr[&n3];

        assert!((pr1 - pr2).abs() < 1e-5);
        assert!((pr2 - pr3).abs() < 1e-5);

        // Sum should be approximately 1.0
        let sum = pr1 + pr2 + pr3;
        assert!((sum - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_pagerank_dangling_node() {
        let mut graph: Digraph<i32, f64> = Digraph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n1, n3, 1.0);
        // n2 and n3 are dangling nodes

        let pr = pagerank(&graph, 0.85, 100, 1e-6).unwrap();

        // n1 should have lower rank than dangling nodes
        assert!(pr[&n1] < pr[&n2]);
        assert!(pr[&n1] < pr[&n3]);
    }

    #[test]
    fn test_pagerank_empty_graph() {
        let graph: Graph<i32, f64> = Graph::new();
        let pr = pagerank(&graph, 0.85, 100, 1e-6).unwrap();
        assert!(pr.is_empty());
    }

    #[test]
    fn test_pagerank_single_node() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let n1 = graph.add_node(1);

        let pr = pagerank(&graph, 0.85, 100, 1e-6).unwrap();
        assert!((pr[&n1] - 1.0).abs() < 1e-5);
    }
}
