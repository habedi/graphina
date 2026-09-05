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
/// Rank is distributed in proportion to edge weight, matching the sequential
/// `pagerank` in the `centrality` module.
///
/// # Arguments
/// * `graph` - The graph to analyze
/// * `damping` - Damping factor (typically 0.85)
/// * `max_iterations` - Maximum number of iterations
/// * `tolerance` - Convergence threshold
/// * `nstart` - Optional starting value for each node
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
/// let ranks = pagerank_parallel(&g, 0.85, 100, 1e-6, None);
/// assert!(ranks[&n1] > 0.0);
/// ```
pub fn pagerank_parallel<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    damping: f64,
    max_iterations: usize,
    tolerance: f64,
    nstart: Option<&HashMap<NodeId, f64>>,
) -> HashMap<NodeId, f64>
where
    A: Sync,
    W: Copy + Into<f64> + Sync,
    Ty: GraphConstructor<A, W> + EdgeType + Sync,
{
    let n = graph.node_count();
    if n == 0 {
        return HashMap::new();
    }

    let nodes: Vec<NodeId> = graph.node_ids().collect();

    // Initialize ranks
    let mut ranks: HashMap<NodeId, f64> = if let Some(start_map) = nstart {
        let mut sum = 0.0;
        let mut temp_ranks = HashMap::with_capacity(n);

        // Collect values and calculate sum
        for &node in &nodes {
            let val = start_map.get(&node).copied().unwrap_or(0.0);
            temp_ranks.insert(node, val);
            sum += val;
        }

        // Normalize
        if sum.abs() > 1e-9 {
            for val in temp_ranks.values_mut() {
                *val /= sum;
            }
            temp_ranks
        } else {
            // Fallback to uniform if sum is zero (or could return empty/panic)
            // Sticking to uniform fallback to avoid error handling change in parallelism for now
            // or we just allow it to fail silently/produce 0s?
            // Better to respect nstart logic: if 0 sum, maybe fallback to uniform is safest for stability
            nodes.iter().map(|&node| (node, 1.0 / n as f64)).collect()
        }
    } else {
        nodes.iter().map(|&node| (node, 1.0 / n as f64)).collect()
    };

    // Precompute weighted incoming edge lists and weighted out-degrees so rank
    // is distributed in proportion to edge weight, mirroring the sequential
    // pagerank in the centrality module.
    let mut incoming: HashMap<NodeId, Vec<(NodeId, f64)>> = HashMap::with_capacity(n);
    let mut weighted_out_degree: HashMap<NodeId, f64> = HashMap::with_capacity(n);
    for &node in &nodes {
        incoming.insert(node, Vec::new());
        weighted_out_degree.insert(node, 0.0);
    }

    let is_directed = graph.is_directed();
    for (src, tgt, w) in graph.edges() {
        let weight: f64 = (*w).into();
        if let Some(list) = incoming.get_mut(&tgt) {
            list.push((src, weight));
        }
        if let Some(deg) = weighted_out_degree.get_mut(&src) {
            *deg += weight;
        }
        // A self-loop is a single edge in both directions, so it is added once.
        if !is_directed && src != tgt {
            if let Some(list) = incoming.get_mut(&src) {
                list.push((tgt, weight));
            }
            if let Some(deg) = weighted_out_degree.get_mut(&tgt) {
                *deg += weight;
            }
        }
    }

    for _iteration in 0..max_iterations {
        // Snapshot previous ranks for this iteration (immutable view for parallelism)
        let prev = ranks.clone();

        // Compute sum of ranks of dangling nodes (weighted out-degree == 0) for redistribution
        let dangling_sum: f64 = nodes
            .par_iter()
            .map(|&node| {
                if weighted_out_degree[&node] == 0.0 {
                    prev[&node]
                } else {
                    0.0
                }
            })
            .sum();

        let base = (1.0 - damping) / n as f64 + damping * dangling_sum / n as f64;

        // Parallel computation of new ranks
        let new_ranks_vec: Vec<(NodeId, f64)> = nodes
            .par_iter()
            .map(|&node| {
                let rank_sum: f64 = incoming[&node]
                    .iter()
                    .map(|&(src, weight)| {
                        let out_weight = weighted_out_degree[&src];
                        if out_weight > 0.0 {
                            prev[&src] * weight / out_weight
                        } else {
                            0.0
                        }
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
    use crate::core::types::{Digraph, Graph};

    #[test]
    fn test_pagerank_parallel_respects_edge_weights() {
        let mut g = Digraph::<i32, f64>::new();
        let source = g.add_node(0);
        let heavy = g.add_node(1);
        let light = g.add_node(2);
        g.add_edge(source, heavy, 10.0);
        g.add_edge(source, light, 1.0);
        g.add_edge(heavy, source, 1.0);
        g.add_edge(light, source, 1.0);

        let ranks = pagerank_parallel(&g, 0.85, 100, 1e-9, None);

        assert!(
            ranks[&heavy] > ranks[&light],
            "target of the heavier edge must rank higher: heavy={}, light={}",
            ranks[&heavy],
            ranks[&light]
        );
    }

    #[test]
    fn test_pagerank_parallel() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n1, 1.0);

        let ranks = pagerank_parallel(&g, 0.85, 100, 1e-6, None);

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

    #[test]
    fn test_pagerank_parallel_undirected_self_loop_counts_once() {
        let mut g = Graph::<i32, f64>::new();
        let a = g.add_node(0);
        let b = g.add_node(1);
        g.add_edge(a, a, 1.0);
        g.add_edge(a, b, 1.0);
        let pr = pagerank_parallel(&g, 0.85, 1000, 1e-12, None);
        assert!((pr[&a] - 0.925 / 1.425).abs() < 1e-6, "a = {}", pr[&a]);
        assert!((pr[&b] - 0.5 / 1.425).abs() < 1e-6, "b = {}", pr[&b]);
    }
}
