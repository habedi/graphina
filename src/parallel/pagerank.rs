/*!
Parallel PageRank computation
*/

use rayon::prelude::*;
use std::collections::HashMap;

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use petgraph::EdgeType;

/// Parallel PageRank computation.
///
/// Computes PageRank scores for all nodes using parallel iterations.
///
/// # Arguments
/// * `graph` - The graph to analyze
/// * `damping` - Damping factor (typically 0.85)
/// * `max_iterations` - Maximum number of iterations
/// * `tolerance` - Convergence threshold
///
/// # Example
///
/// ```rust
/// use graphina::core::types::Graph;
/// use graphina::parallel::pagerank_parallel;
///
/// let mut g = Graph::<i32, f64>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// let n3 = g.add_node(3);
/// g.add_edge(n1, n2, 1.0);
/// g.add_edge(n2, n3, 1.0);
/// g.add_edge(n3, n1, 1.0);
///
/// let ranks = pagerank_parallel(&g, 0.85, 100, 1e-6);
/// assert!(ranks[&n1] > 0.0);
/// ```
pub fn pagerank_parallel<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    damping: f64,
    max_iterations: usize,
    tolerance: f64,
) -> HashMap<NodeId, f64>
where
    A: Sync,
    W: Sync,
    Ty: GraphConstructor<A, W> + EdgeType + Sync,
{
    let n = graph.node_count();
    if n == 0 {
        return HashMap::new();
    }

    let nodes: Vec<NodeId> = graph.node_ids().collect();
    let initial_rank = 1.0 / n as f64;

    // Precompute incoming edges list to avoid scanning all edges per node.
    let mut incoming: HashMap<NodeId, Vec<NodeId>> = HashMap::with_capacity(n);
    for &node in &nodes {
        incoming.insert(node, Vec::new());
    }
    for (src, tgt, _) in graph.edges() {
        if incoming.contains_key(&tgt) {
            incoming
                .get_mut(&tgt)
                .expect("incoming map must contain target node")
                .push(src);
        }
    }

    let mut ranks: HashMap<NodeId, f64> = nodes.iter().map(|&node| (node, initial_rank)).collect();

    for _iteration in 0..max_iterations {
        // Snapshot previous ranks for this iteration (immutable view for parallelism)
        let prev = ranks.clone();

        // Compute sum of ranks of dangling nodes (out-degree == 0) for redistribution
        let dangling_sum: f64 = nodes
            .par_iter()
            .map(|&node| {
                let out_deg = graph.out_degree(node).unwrap_or(0);
                if out_deg == 0 { prev[&node] } else { 0.0 }
            })
            .sum();

        let base = (1.0 - damping) / n as f64 + damping * dangling_sum / n as f64;

        // Parallel computation of new ranks
        let new_ranks_vec: Vec<(NodeId, f64)> = nodes
            .par_iter()
            .map(|&node| {
                let rank_sum: f64 = incoming[&node]
                    .iter()
                    .map(|&src| {
                        let out_degree = graph.out_degree(src).unwrap_or(0);
                        let denom = if out_degree == 0 { 1 } else { out_degree }; // safeguard
                        prev[&src] / denom as f64
                    })
                    .sum();
                let new_rank = base + damping * rank_sum;
                (node, new_rank)
            })
            .collect();

        // Merge and check convergence
        let mut max_diff = 0.0;
        for (node, new_rank) in new_ranks_vec {
            let diff = (new_rank - prev[&node]).abs();
            if diff > max_diff {
                max_diff = diff;
            }
            ranks.insert(node, new_rank);
        }

        if max_diff < tolerance {
            break;
        }
    }

    ranks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_pagerank_parallel() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n1, 1.0);

        let ranks = pagerank_parallel(&g, 0.85, 100, 1e-6);

        // Verify all nodes have positive rank
        assert!(ranks[&n1] > 0.0);
        assert!(ranks[&n2] > 0.0);
        assert!(ranks[&n3] > 0.0);

        // All nodes should have similar rank in a symmetric cycle
        let avg = (ranks[&n1] + ranks[&n2] + ranks[&n3]) / 3.0;
        assert!((ranks[&n1] - avg).abs() < 0.1);
        assert!((ranks[&n2] - avg).abs() < 0.1);
        assert!((ranks[&n3] - avg).abs() < 0.1);
    }
}
