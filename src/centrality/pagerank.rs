//! PageRank algorithms.
//!
//! This module provides PageRank centrality measures.
//!
//! Convention: functions in this module return `Result<_, crate::core::error::GraphinaError>`
//! for better observability and error propagation.

use crate::core::error::{GraphinaError, Result};
use crate::core::types::{BaseGraph, GraphConstructor, NodeId, NodeMap};

/// PageRank: a link analysis algorithm that assigns a numerical weighting to each element
/// of a hyperlinked set of documents.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `damping`: damping factor (usually 0.85).
/// * `max_iter`: maximum number of iterations.
/// * `tolerance`: per-node convergence tolerance; iteration stops when the L1 change of the
///   rank vector drops below `tolerance * n`, as in NetworkX.
/// * `nstart`: optional starting value for each node.
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
    nstart: Option<&NodeMap<f64>>,
) -> Result<NodeMap<f64>>
where
    W: Copy + PartialOrd + Into<f64>,
    Ty: GraphConstructor<A, W>,
{
    let n = graph.node_count();
    if n == 0 {
        return Ok(NodeMap::default());
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

    let is_directed = graph.is_directed();
    for (u, v, w) in graph.edges() {
        let ui = node_to_idx[&u];
        let vi = node_to_idx[&v];
        let weight: f64 = (*w).into();
        out_degrees[ui] += weight;
        out_edges[ui].push((vi, weight));

        // A self-loop is a single edge in both directions, so it is added once.
        if !is_directed && ui != vi {
            out_degrees[vi] += weight;
            out_edges[vi].push((ui, weight));
        }
    }

    let mut pr = if let Some(start_map) = nstart {
        let mut p = vec![0.0; n];
        let mut sum = 0.0;
        for (idx, &node) in node_list.iter().enumerate() {
            if let Some(&val) = start_map.get(&node) {
                p[idx] = val;
                sum += val;
            }
        }
        if sum.abs() < 1e-9 {
            // If sum is zero, fallback to uniform or error? NetworkX raises error.
            // But to be safe let's raise error if nstart was provided but useless.
            return Err(GraphinaError::invalid_argument("nstart sum is zero"));
        }
        // Normalize
        for x in p.iter_mut() {
            *x /= sum;
        }
        p
    } else {
        vec![1.0 / n as f64; n]
    };

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

        // NetworkX rule: stop when the L1 change is below tolerance * n.
        if diff < tolerance * n as f64 {
            break;
        }
    }

    // Convert to NodeMap using the node list
    let mut centrality = NodeMap::default();
    for (idx, &node) in node_list.iter().enumerate() {
        centrality.insert(node, pr[idx]);
    }
    Ok(centrality)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_pagerank_with_deleted_nodes() {
        use crate::centrality::pagerank::pagerank;
        use crate::core::types::Digraph;

        let mut graph: Digraph<i32, f64> = Digraph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);

        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n4, 1.0);
        graph.add_edge(n4, n1, 1.0);

        graph.remove_node(n2);

        let pr = pagerank(&graph, 0.85, 100, 1e-6, None).unwrap();

        assert!(!pr.contains_key(&n2));
        assert!(pr.contains_key(&n1));
        assert!(pr.contains_key(&n3));
        assert!(pr.contains_key(&n4));

        let sum: f64 = pr.values().sum();
        assert!((sum - 1.0).abs() < 1e-4);
    }
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

        let pr = pagerank(&graph, 0.85, 100, 1e-6, None).unwrap();

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

        let pr = pagerank(&graph, 0.85, 100, 1e-6, None).unwrap();

        // n1 should have lower rank than dangling nodes
        assert!(pr[&n1] < pr[&n2]);
        assert!(pr[&n1] < pr[&n3]);
    }

    #[test]
    fn test_pagerank_empty_graph() {
        let graph: Graph<i32, f64> = Graph::new();
        let pr = pagerank(&graph, 0.85, 100, 1e-6, None).unwrap();
        assert!(pr.is_empty());
    }

    #[test]
    fn test_pagerank_single_node() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let n1 = graph.add_node(1);

        let pr = pagerank(&graph, 0.85, 100, 1e-6, None).unwrap();
        assert!((pr[&n1] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_pagerank_with_nstart() {
        let mut graph: Digraph<i32, f64> = Digraph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);

        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n1, 1.0);

        // Bias start towards node 1
        let mut nstart = NodeMap::default();
        nstart.insert(n1, 0.8);
        nstart.insert(n2, 0.2);

        // With sufficient iterations, it should still converge to 0.5/0.5 for symmetric graph
        let pr = pagerank(&graph, 0.85, 100, 1e-6, Some(&nstart)).unwrap();
        assert!((pr[&n1] - 0.5).abs() < 1e-3);
        assert!((pr[&n2] - 0.5).abs() < 1e-3);

        // With 0 iterations using small max_iter (if we could control validation better),
        // effectively we check it doesn't crash and converges normally.
        // Let's verify it accepts partial nstart (rest zero assumption)
        let mut partial_start = NodeMap::default();
        partial_start.insert(n1, 1.0);
        // n2 missing -> assumed 0.0

        let pr_partial = pagerank(&graph, 0.85, 100, 1e-6, Some(&partial_start)).unwrap();
        assert!((pr_partial[&n1] - 0.5).abs() < 1e-3);
    }

    #[test]
    fn test_pagerank_undirected_self_loop_counts_once() {
        use crate::centrality::pagerank::pagerank;
        use crate::core::types::Graph;

        // Node a has a self-loop and an edge to b. The self-loop is a single
        // edge, so the walk leaves a with probability 1/2 in each direction and
        // the stationary ranks are 0.925/1.425 for a and 0.5/1.425 for b.
        let mut g = Graph::<i32, f64>::new();
        let a = g.add_node(0);
        let b = g.add_node(1);
        g.add_edge(a, a, 1.0);
        g.add_edge(a, b, 1.0);
        let pr = pagerank(&g, 0.85, 1000, 1e-12, None).unwrap();
        assert!((pr[&a] - 0.925 / 1.425).abs() < 1e-6, "a = {}", pr[&a]);
        assert!((pr[&b] - 0.5 / 1.425).abs() < 1e-6, "b = {}", pr[&b]);
    }

    #[test]
    fn test_pagerank_tolerance_is_scaled_by_node_count_like_networkx() {
        use crate::centrality::pagerank::pagerank;
        use crate::core::types::Graph;

        // One power step from the uniform start on the path a-b-c changes the
        // vector by 0.5667 in L1. NetworkX stops when the change is below
        // tol * n, so tolerance 0.2 (0.6 total) stops after that single step.
        let mut g = Graph::<i32, f64>::new();
        let a = g.add_node(0);
        let b = g.add_node(1);
        let c = g.add_node(2);
        g.add_edge(a, b, 1.0);
        g.add_edge(b, c, 1.0);
        let end = 0.05 + 0.85 / 6.0;
        let mid = 0.05 + 0.85 * 2.0 / 3.0;
        let pr = pagerank(&g, 0.85, 100, 0.2, None).unwrap();
        assert!((pr[&a] - end).abs() < 1e-12, "a = {}", pr[&a]);
        assert!((pr[&b] - mid).abs() < 1e-12, "b = {}", pr[&b]);
        assert!((pr[&c] - end).abs() < 1e-12, "c = {}", pr[&c]);
    }
}
