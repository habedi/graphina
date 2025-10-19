/*!
# Graph Metrics Module

This module provides common graph metrics and statistics used in network analysis.
Includes global metrics (entire graph) and local metrics (per node).
*/

use std::collections::{HashMap, VecDeque};

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use petgraph::EdgeType;

/// Computes the diameter of the graph (longest shortest path).
///
/// For disconnected graphs, returns None.
///
/// # Time Complexity
/// O(V * (V + E)) - Runs BFS from each node
///
/// # Example
///
/// ```rust
/// use graphina::core::{types::Graph, metrics::diameter};
///
/// let mut g = Graph::<i32, f64>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// let n3 = g.add_node(3);
/// g.add_edge(n1, n2, 1.0);
/// g.add_edge(n2, n3, 1.0);
///
/// assert_eq!(diameter(&g), Some(2));
/// ```
pub fn diameter<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
) -> Option<usize> {
    if graph.is_empty() {
        return None;
    }

    let mut max_distance = 0;

    for start_node in graph.node_ids() {
        let distances = bfs_distances(graph, start_node);

        // If any node is unreachable, graph is disconnected
        if distances.len() != graph.node_count() {
            return None;
        }

        if let Some(&max_dist) = distances.values().max() {
            if max_dist > max_distance {
                max_distance = max_dist;
            }
        }
    }

    Some(max_distance)
}

/// Computes the radius of the graph (minimum eccentricity).
///
/// The radius is the minimum over all nodes of their eccentricity
/// (maximum distance to any other node).
///
/// # Time Complexity
/// O(V * (V + E))
pub fn radius<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
) -> Option<usize> {
    if graph.is_empty() {
        return None;
    }

    let mut min_eccentricity = usize::MAX;

    for start_node in graph.node_ids() {
        let distances = bfs_distances(graph, start_node);

        // If any node is unreachable, graph is disconnected
        if distances.len() != graph.node_count() {
            return None;
        }

        if let Some(&max_dist) = distances.values().max() {
            if max_dist < min_eccentricity {
                min_eccentricity = max_dist;
            }
        }
    }

    Some(min_eccentricity)
}

/// Computes the average clustering coefficient of the graph.
///
/// The clustering coefficient measures the degree to which nodes tend to
/// cluster together. Returns the average of all local clustering coefficients.
///
/// # Time Complexity
/// O(V * d²) where d is average degree
///
/// # Example
///
/// ```rust
/// use graphina::core::{types::Graph, metrics::average_clustering_coefficient};
///
/// let mut g = Graph::<i32, f64>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// let n3 = g.add_node(3);
/// g.add_edge(n1, n2, 1.0);
/// g.add_edge(n2, n3, 1.0);
/// g.add_edge(n3, n1, 1.0); // Triangle
///
/// assert!((average_clustering_coefficient(&g) - 1.0).abs() < 0.001);
/// ```
pub fn average_clustering_coefficient<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
) -> f64 {
    if graph.is_empty() {
        return 0.0;
    }

    let coefficients: Vec<f64> = graph
        .node_ids()
        .map(|node| clustering_coefficient(graph, node))
        .collect();

    coefficients.iter().sum::<f64>() / coefficients.len() as f64
}

/// Computes the local clustering coefficient for a specific node.
///
/// Measures the probability that two neighbors of a node are also connected.
///
/// # Time Complexity
/// O(d²) where d is the node's degree
pub fn clustering_coefficient<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
    node: NodeId,
) -> f64 {
    let neighbors: Vec<NodeId> = graph.neighbors(node).collect();
    let k = neighbors.len();

    if k < 2 {
        return 0.0;
    }

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
}

/// Computes the transitivity (global clustering coefficient) of the graph.
///
/// Measures the ratio of triangles to connected triples in the graph.
///
/// # Time Complexity
/// O(V * d²)
pub fn transitivity<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
) -> f64 {
    let mut triangles = 0;
    let mut triples = 0;

    for node in graph.node_ids() {
        let neighbors: Vec<NodeId> = graph.neighbors(node).collect();
        let k = neighbors.len();

        if k < 2 {
            continue;
        }

        triples += k * (k - 1) / 2;

        for i in 0..neighbors.len() {
            for j in (i + 1)..neighbors.len() {
                if graph.contains_edge(neighbors[i], neighbors[j]) {
                    triangles += 1;
                }
            }
        }
    }

    if triples == 0 {
        return 0.0;
    }

    triangles as f64 / triples as f64
}

/// Counts the number of triangles containing a specific node.
///
/// # Time Complexity
/// O(d²) where d is the node's degree
pub fn triangles<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
    node: NodeId,
) -> usize {
    let neighbors: Vec<NodeId> = graph.neighbors(node).collect();
    let mut count = 0;

    for i in 0..neighbors.len() {
        for j in (i + 1)..neighbors.len() {
            if graph.contains_edge(neighbors[i], neighbors[j]) {
                count += 1;
            }
        }
    }

    count
}

/// Computes the average path length of the graph.
///
/// Returns the average shortest path length between all pairs of nodes.
/// For disconnected graphs, returns None.
///
/// # Time Complexity
/// O(V * (V + E))
pub fn average_path_length<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
) -> Option<f64> {
    if graph.is_empty() {
        return None;
    }

    let mut total_distance = 0.0;
    let mut pair_count = 0;

    for start_node in graph.node_ids() {
        let distances = bfs_distances(graph, start_node);

        // If any node is unreachable, graph is disconnected
        if distances.len() != graph.node_count() {
            return None;
        }

        for &dist in distances.values() {
            if dist > 0 {
                total_distance += dist as f64;
                pair_count += 1;
            }
        }
    }

    if pair_count == 0 {
        return Some(0.0);
    }

    Some(total_distance / pair_count as f64)
}

/// Computes the assortativity coefficient of the graph.
///
/// Measures the tendency of nodes to connect to others with similar degree.
/// Returns a value between -1 (disassortative) and 1 (assortative).
///
/// # Time Complexity
/// O(E)
pub fn assortativity<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
) -> f64 {
    if graph.edge_count() == 0 {
        return 0.0;
    }

    let mut sum_jk = 0.0;
    let mut sum_j = 0.0;
    let mut sum_k = 0.0;
    let mut sum_j2 = 0.0;
    let mut sum_k2 = 0.0;
    let m = graph.edge_count() as f64;

    for (u, v, _) in graph.edges() {
        let j = graph.degree(u).unwrap() as f64;
        let k = graph.degree(v).unwrap() as f64;

        sum_jk += j * k;
        sum_j += j;
        sum_k += k;
        sum_j2 += j * j;
        sum_k2 += k * k;
    }

    let numerator = sum_jk / m - (sum_j / m) * (sum_k / m);
    let denominator =
        ((sum_j2 / m - (sum_j / m).powi(2)) * (sum_k2 / m - (sum_k / m).powi(2))).sqrt();

    if denominator == 0.0 {
        return 0.0;
    }

    numerator / denominator
}

/// Helper function: Computes BFS distances from a start node.
fn bfs_distances<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
    start: NodeId,
) -> HashMap<NodeId, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    distances.insert(start, 0);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let dist = distances[&node];

        for neighbor in graph.neighbors(node) {
            if let std::collections::hash_map::Entry::Vacant(e) = distances.entry(neighbor) {
                e.insert(dist + 1);
                queue.push_back(neighbor);
            }
        }
    }

    distances
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_diameter() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);

        assert_eq!(diameter(&g), Some(2));
    }

    #[test]
    fn test_diameter_disconnected() {
        let g = Graph::<i32, f64>::new();
        assert_eq!(diameter(&g), None);
    }

    #[test]
    fn test_radius() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);

        assert_eq!(radius(&g), Some(1)); // Center node n2 has eccentricity 1
    }

    #[test]
    fn test_clustering_coefficient_triangle() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n1, 1.0);

        // All nodes in triangle have clustering coefficient 1.0
        assert!((clustering_coefficient(&g, n1) - 1.0).abs() < 0.001);
        assert!((clustering_coefficient(&g, n2) - 1.0).abs() < 0.001);
        assert!((clustering_coefficient(&g, n3) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_average_clustering_coefficient() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n1, 1.0);

        assert!((average_clustering_coefficient(&g) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_triangles() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n1, 1.0);
        g.add_edge(n1, n4, 1.0);

        assert_eq!(triangles(&g, n1), 1);
        assert_eq!(triangles(&g, n2), 1);
        assert_eq!(triangles(&g, n3), 1);
        assert_eq!(triangles(&g, n4), 0);
    }

    #[test]
    fn test_transitivity() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n1, 1.0);

        assert!((transitivity(&g) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_average_path_length() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);

        // Paths: 1->2 (1), 2->3 (1), 1->3 (2), avg = 4/3 ≈ 1.33
        let avg = average_path_length(&g).expect("Connected graph should have average path length");
        assert!((avg - 1.333).abs() < 0.01);
    }

    #[test]
    fn test_assortativity() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);

        // Create a simple graph
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n4, 1.0);

        // Just check it returns a value in valid range
        let assort = assortativity(&g);
        assert!(assort >= -1.0 && assort <= 1.0);
    }
}
