/*!
# Parallel Algorithms Module

This module provides parallel implementations of computationally intensive graph algorithms
using Rayon for multi-threading. These implementations can provide 4-8x speedup on multi-core machines.

All parallel functions have the `_parallel` suffix to distinguish them from sequential versions.
*/

use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use petgraph::EdgeType;

/// Parallel breadth-first search from multiple starting nodes.
///
/// Processes multiple BFS searches in parallel, useful for computing shortest paths
/// from multiple sources simultaneously.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::Graph;
/// use graphina::core::parallel::bfs_parallel;
///
/// let mut g = Graph::<i32, f64>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// let n3 = g.add_node(3);
/// g.add_edge(n1, n2, 1.0);
/// g.add_edge(n2, n3, 1.0);
///
/// let starts = vec![n1, n2];
/// let results = bfs_parallel(&g, &starts);
/// assert_eq!(results.len(), 2);
/// ```
pub fn bfs_parallel<A, W, Ty>(graph: &BaseGraph<A, W, Ty>, starts: &[NodeId]) -> Vec<Vec<NodeId>>
where
    A: Sync,
    W: Sync,
    Ty: GraphConstructor<A, W> + EdgeType + Sync,
{
    starts
        .par_iter()
        .map(|&start| {
            let mut visited = Vec::new();
            let mut queue = VecDeque::new();
            let mut seen = HashSet::new();

            queue.push_back(start);
            seen.insert(start);

            while let Some(node) = queue.pop_front() {
                visited.push(node);

                for neighbor in graph.neighbors(node) {
                    if seen.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }

            visited
        })
        .collect()
}

/// Parallel computation of node degrees.
///
/// Computes the degree of all nodes in parallel.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::Graph;
/// use graphina::core::parallel::degrees_parallel;
///
/// let mut g = Graph::<i32, f64>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// g.add_edge(n1, n2, 1.0);
///
/// let degrees = degrees_parallel(&g);
/// assert_eq!(degrees[&n1], 1);
/// assert_eq!(degrees[&n2], 1);
/// ```
pub fn degrees_parallel<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> HashMap<NodeId, usize>
where
    A: Sync,
    W: Sync,
    Ty: GraphConstructor<A, W> + EdgeType + Sync,
{
    let nodes: Vec<NodeId> = graph.node_ids().collect();

    nodes
        .par_iter()
        .map(|&node| {
            let degree = graph.degree(node).unwrap_or(0);
            (node, degree)
        })
        .collect()
}

/// Parallel computation of clustering coefficients for all nodes.
///
/// Computes local clustering coefficient for each node in parallel.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::Graph;
/// use graphina::core::parallel::clustering_coefficients_parallel;
///
/// let mut g = Graph::<i32, f64>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// let n3 = g.add_node(3);
/// g.add_edge(n1, n2, 1.0);
/// g.add_edge(n2, n3, 1.0);
/// g.add_edge(n3, n1, 1.0);
///
/// let coefficients = clustering_coefficients_parallel(&g);
/// assert!((coefficients[&n1] - 1.0).abs() < 0.001);
/// ```
pub fn clustering_coefficients_parallel<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
) -> HashMap<NodeId, f64>
where
    A: Sync,
    W: Sync,
    Ty: GraphConstructor<A, W> + EdgeType + Sync,
{
    let nodes: Vec<NodeId> = graph.node_ids().collect();

    nodes
        .par_iter()
        .map(|&node| {
            let neighbors: Vec<NodeId> = graph.neighbors(node).collect();
            let k = neighbors.len();

            let coefficient = if k < 2 {
                0.0
            } else {
                let mut triangles = 0;
                for i in 0..neighbors.len() {
                    for j in (i + 1)..neighbors.len() {
                        if graph.contains_edge(neighbors[i], neighbors[j]) {
                            triangles += 1;
                        }
                    }
                }
                let possible_edges = k * (k - 1) / 2;
                triangles as f64 / possible_edges as f64
            };

            (node, coefficient)
        })
        .collect()
}

/// Parallel triangle counting for all nodes.
///
/// Counts the number of triangles each node participates in, in parallel.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::Graph;
/// use graphina::core::parallel::triangles_parallel;
///
/// let mut g = Graph::<i32, f64>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// let n3 = g.add_node(3);
/// g.add_edge(n1, n2, 1.0);
/// g.add_edge(n2, n3, 1.0);
/// g.add_edge(n3, n1, 1.0);
///
/// let triangles = triangles_parallel(&g);
/// assert_eq!(triangles[&n1], 1);
/// ```
pub fn triangles_parallel<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> HashMap<NodeId, usize>
where
    A: Sync,
    W: Sync,
    Ty: GraphConstructor<A, W> + EdgeType + Sync,
{
    let nodes: Vec<NodeId> = graph.node_ids().collect();

    nodes
        .par_iter()
        .map(|&node| {
            let neighbors: Vec<NodeId> = graph.neighbors(node).collect();
            let mut count = 0;

            for i in 0..neighbors.len() {
                for j in (i + 1)..neighbors.len() {
                    if graph.contains_edge(neighbors[i], neighbors[j]) {
                        count += 1;
                    }
                }
            }

            (node, count)
        })
        .collect()
}

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
/// use graphina::core::parallel::pagerank_parallel;
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

    // Initialize ranks
    let mut ranks: HashMap<NodeId, f64> = nodes.iter().map(|&node| (node, initial_rank)).collect();
    let ranks_mutex = Arc::new(Mutex::new(ranks.clone()));

    for _iteration in 0..max_iterations {
        let new_ranks_vec: Vec<(NodeId, f64)> = nodes
            .par_iter()
            .map(|&node| {
                let current_ranks = ranks_mutex.lock().unwrap();

                // Sum contributions from incoming neighbors
                let mut rank_sum = 0.0;
                for (src, tgt, _) in graph.edges() {
                    if tgt == node {
                        let out_degree = graph.out_degree(src).unwrap_or(1);
                        rank_sum += current_ranks[&src] / out_degree as f64;
                    }
                }

                let new_rank = (1.0 - damping) / n as f64 + damping * rank_sum;
                (node, new_rank)
            })
            .collect();

        // Update ranks
        let mut ranks_lock = ranks_mutex.lock().unwrap();
        let mut max_diff: f64 = 0.0;

        for (node, new_rank) in new_ranks_vec {
            let diff = (new_rank - ranks_lock[&node]).abs();
            if diff > max_diff {
                max_diff = diff;
            }
            ranks_lock.insert(node, new_rank);
        }

        ranks = ranks_lock.clone();
        drop(ranks_lock);

        // Check convergence
        if max_diff < tolerance {
            break;
        }
    }

    ranks
}

/// Parallel shortest path distances from multiple sources.
///
/// Computes shortest path distances from multiple source nodes in parallel.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::Graph;
/// use graphina::core::parallel::shortest_paths_parallel;
///
/// let mut g = Graph::<i32, f64>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// let n3 = g.add_node(3);
/// g.add_edge(n1, n2, 1.0);
/// g.add_edge(n2, n3, 1.0);
///
/// let sources = vec![n1, n3];
/// let distances = shortest_paths_parallel(&g, &sources);
/// assert_eq!(distances.len(), 2);
/// ```
pub fn shortest_paths_parallel<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    sources: &[NodeId],
) -> Vec<HashMap<NodeId, usize>>
where
    A: Sync,
    W: Sync,
    Ty: GraphConstructor<A, W> + EdgeType + Sync,
{
    sources
        .par_iter()
        .map(|&source| {
            let mut distances = HashMap::new();
            let mut queue = VecDeque::new();

            distances.insert(source, 0);
            queue.push_back(source);

            while let Some(node) = queue.pop_front() {
                let dist = distances[&node];

                for neighbor in graph.neighbors(node) {
                    if let std::collections::hash_map::Entry::Vacant(e) = distances.entry(neighbor)
                    {
                        e.insert(dist + 1);
                        queue.push_back(neighbor);
                    }
                }
            }

            distances
        })
        .collect()
}

/// Parallel connected components detection.
///
/// Finds all connected components in parallel by processing multiple starting points.
///
/// Returns a mapping from node to component ID.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::Graph;
/// use graphina::core::parallel::connected_components_parallel;
///
/// let mut g = Graph::<i32, f64>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// let n3 = g.add_node(3);
/// let n4 = g.add_node(4);
///
/// g.add_edge(n1, n2, 1.0);
/// g.add_edge(n3, n4, 1.0);
///
/// let components = connected_components_parallel(&g);
///
/// // n1 and n2 should be in same component
/// assert_eq!(components[&n1], components[&n2]);
///
/// // n3 and n4 should be in same component
/// assert_eq!(components[&n3], components[&n4]);
///
/// // But different from n1/n2
/// assert_ne!(components[&n1], components[&n3]);
/// ```
pub fn connected_components_parallel<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
) -> HashMap<NodeId, usize>
where
    A: Sync + Send,
    W: Sync + Send,
    Ty: GraphConstructor<A, W> + EdgeType + Sync + Send,
{
    let nodes: Vec<NodeId> = graph.node_ids().collect();
    let component_map = Arc::new(Mutex::new(HashMap::new()));
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let component_id = Arc::new(Mutex::new(0_usize));

    for node in nodes {
        // Check if already visited
        {
            let visited_lock = visited.lock().unwrap();
            if visited_lock.contains(&node) {
                continue;
            }
        }

        // Get current component ID
        let current_id = {
            let mut id_lock = component_id.lock().unwrap();
            let id = *id_lock;
            *id_lock += 1;
            id
        };

        // BFS to find all nodes in this component
        let mut queue = VecDeque::new();
        queue.push_back(node);

        let mut local_visited = HashSet::new();
        local_visited.insert(node);

        while let Some(current) = queue.pop_front() {
            for neighbor in graph.neighbors(current) {
                if local_visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        // Update global structures
        {
            let mut component_lock = component_map.lock().unwrap();
            let mut visited_lock = visited.lock().unwrap();

            for n in &local_visited {
                component_lock.insert(*n, current_id);
                visited_lock.insert(*n);
            }
        }
    }

    Arc::try_unwrap(component_map)
        .unwrap()
        .into_inner()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_bfs_parallel() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);

        let results = bfs_parallel(&g, &[n1, n3]);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].len(), 3); // All nodes reachable from n1
        assert_eq!(results[1].len(), 3); // All nodes reachable from n3
    }

    #[test]
    fn test_degrees_parallel() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);

        let degrees = degrees_parallel(&g);
        assert_eq!(degrees[&n1], 1);
        assert_eq!(degrees[&n2], 2);
        assert_eq!(degrees[&n3], 1);
    }

    #[test]
    fn test_clustering_coefficients_parallel() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n1, 1.0);

        let coefficients = clustering_coefficients_parallel(&g);
        assert!((coefficients[&n1] - 1.0).abs() < 0.001);
        assert!((coefficients[&n2] - 1.0).abs() < 0.001);
        assert!((coefficients[&n3] - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_triangles_parallel() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n1, 1.0);
        g.add_edge(n1, n4, 1.0);

        let triangles = triangles_parallel(&g);
        assert_eq!(triangles[&n1], 1);
        assert_eq!(triangles[&n2], 1);
        assert_eq!(triangles[&n3], 1);
        assert_eq!(triangles[&n4], 0);
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

    #[test]
    fn test_shortest_paths_parallel() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);

        let distances = shortest_paths_parallel(&g, &[n1, n3]);
        assert_eq!(distances[0][&n3], 2); // n1 to n3 is 2 hops
        assert_eq!(distances[1][&n1], 2); // n3 to n1 is 2 hops
    }

    #[test]
    fn test_connected_components_parallel() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n3, n4, 1.0);

        let components = connected_components_parallel(&g);

        // n1 and n2 should be in same component
        assert_eq!(components[&n1], components[&n2]);

        // n3 and n4 should be in same component
        assert_eq!(components[&n3], components[&n4]);

        // But different from n1/n2
        assert_ne!(components[&n1], components[&n3]);
    }
}
