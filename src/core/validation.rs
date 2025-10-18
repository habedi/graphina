/*!
# Graph Validation Utilities

This module provides common validation functions for graphs, such as checking if a graph is empty,
connected, or has negative weights. These utilities help centralize precondition checks across
algorithms, reducing duplication and improving maintainability.
*/

use std::collections::{HashSet, VecDeque};

use crate::core::exceptions::GraphinaException;
use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use petgraph::EdgeType;

/// Returns true if the graph contains no nodes.
pub fn is_empty<A, W, Ty: GraphConstructor<A, W> + EdgeType>(graph: &BaseGraph<A, W, Ty>) -> bool {
    graph.is_empty()
}

/// Returns true if the graph is connected.
///
/// For undirected graphs, this checks if the graph is connected (has one component).
/// For directed graphs, this checks if the graph is weakly connected.
pub fn is_connected<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
) -> bool {
    if graph.is_empty() {
        return false; // Conventionally, empty graphs are not considered connected
    }

    let mut visited = HashSet::new();
    let start = graph.inner.node_indices().next().unwrap();
    let mut stack = vec![start];
    visited.insert(start);

    while let Some(node) = stack.pop() {
        for neighbor in graph.inner.neighbors_undirected(node) {
            if visited.insert(neighbor) {
                stack.push(neighbor);
            }
        }
    }

    visited.len() == graph.inner.node_count()
}

/// Returns true if the graph has any negative edge weights.
///
/// This function assumes edge weights can be converted to f64 for comparison.
/// It is specialized for weight types that implement `Into<f64>`.
pub fn has_negative_weights<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
) -> bool
where
    W: Copy + Into<f64>,
{
    graph.edges().any(|(_, _, w)| (*w).into() < 0.0)
}

/// Returns true if the graph contains any self-loops (edges from a node to itself).
pub fn has_self_loops<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
) -> bool {
    graph.edges().any(|(src, tgt, _)| src == tgt)
}

/// Returns true if the directed graph is acyclic (is a DAG).
///
/// Uses depth-first search with cycle detection.
/// For undirected graphs, this always returns false (undirected graphs with edges always have cycles).
pub fn is_dag<A, W, Ty: GraphConstructor<A, W> + EdgeType>(graph: &BaseGraph<A, W, Ty>) -> bool {
    if !graph.is_directed() {
        // Undirected graphs with any edges have cycles
        return graph.edge_count() == 0;
    }

    let mut white = HashSet::new(); // Not visited
    let mut gray = HashSet::new(); // Currently exploring
    let mut black = HashSet::new(); // Finished exploring

    for node in graph.node_ids() {
        white.insert(node);
    }

    fn dfs_has_cycle<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
        graph: &BaseGraph<A, W, Ty>,
        node: NodeId,
        white: &mut HashSet<NodeId>,
        gray: &mut HashSet<NodeId>,
        black: &mut HashSet<NodeId>,
    ) -> bool {
        white.remove(&node);
        gray.insert(node);

        for neighbor in graph.neighbors(node) {
            if black.contains(&neighbor) {
                continue;
            }
            if gray.contains(&neighbor) {
                return true; // Back edge found - cycle detected
            }
            if dfs_has_cycle(graph, neighbor, white, gray, black) {
                return true;
            }
        }

        gray.remove(&node);
        black.insert(node);
        false
    }

    while let Some(&node) = white.iter().next() {
        if dfs_has_cycle(graph, node, &mut white, &mut gray, &mut black) {
            return false; // Cycle found
        }
    }

    true // No cycles found
}

/// Returns true if the undirected graph is bipartite.
///
/// A bipartite graph is one whose nodes can be divided into two disjoint sets
/// such that every edge connects a node in one set to a node in the other set.
/// Uses BFS coloring algorithm.
pub fn is_bipartite<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
) -> bool {
    if graph.is_empty() {
        return true;
    }

    let mut color = std::collections::HashMap::new();

    for start_node in graph.node_ids() {
        if color.contains_key(&start_node) {
            continue; // Already colored in a previous component
        }

        let mut queue = VecDeque::new();
        queue.push_back(start_node);
        color.insert(start_node, 0);

        while let Some(node) = queue.pop_front() {
            let current_color = color[&node];
            let next_color = 1 - current_color;

            for neighbor in graph.neighbors(node) {
                if let Some(&neighbor_color) = color.get(&neighbor) {
                    if neighbor_color == current_color {
                        return false; // Same color - not bipartite
                    }
                } else {
                    color.insert(neighbor, next_color);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    true
}

/// Returns the number of connected components in the graph.
///
/// For directed graphs, this counts weakly connected components.
pub fn count_components<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
) -> usize {
    let mut visited = HashSet::new();
    let mut component_count = 0;

    for node in graph.node_ids() {
        if visited.contains(&node.0) {
            continue;
        }

        component_count += 1;
        let mut stack = vec![node.0];
        visited.insert(node.0);

        while let Some(current) = stack.pop() {
            for neighbor in graph.inner.neighbors_undirected(current) {
                if visited.insert(neighbor) {
                    stack.push(neighbor);
                }
            }
        }
    }

    component_count
}

/// Validates that the graph is non-empty.
///
/// Returns `Ok(())` if the graph has at least one node, otherwise returns an error.
pub fn require_non_empty<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
    algo_name: &str,
) -> Result<(), GraphinaException> {
    if is_empty(graph) {
        Err(GraphinaException::new(&format!(
            "{} requires a non-empty graph",
            algo_name
        )))
    } else {
        Ok(())
    }
}

/// Validates that the graph is connected.
///
/// Returns `Ok(())` if the graph is connected, otherwise returns an error.
pub fn require_connected<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
    algo_name: &str,
) -> Result<(), GraphinaException> {
    if !is_connected(graph) {
        Err(GraphinaException::new(&format!(
            "{} requires a connected graph",
            algo_name
        )))
    } else {
        Ok(())
    }
}

/// Validates that all edge weights are non-negative.
///
/// Returns `Ok(())` if all weights are >= 0, otherwise returns an error.
pub fn require_non_negative_weights<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
    algo_name: &str,
) -> Result<(), GraphinaException>
where
    W: Copy + Into<f64>,
{
    if has_negative_weights(graph) {
        Err(GraphinaException::new(&format!(
            "{} requires non-negative edge weights",
            algo_name
        )))
    } else {
        Ok(())
    }
}

/// Validates that the graph has no self-loops.
///
/// Returns `Ok(())` if there are no self-loops, otherwise returns an error.
pub fn require_no_self_loops<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
    algo_name: &str,
) -> Result<(), GraphinaException> {
    if has_self_loops(graph) {
        Err(GraphinaException::new(&format!(
            "{} does not support graphs with self-loops",
            algo_name
        )))
    } else {
        Ok(())
    }
}

/// Validates that the directed graph is acyclic (DAG).
///
/// Returns `Ok(())` if the graph is a DAG, otherwise returns an error.
pub fn require_dag<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
    algo_name: &str,
) -> Result<(), GraphinaException> {
    if !graph.is_directed() {
        return Err(GraphinaException::new(&format!(
            "{} requires a directed graph",
            algo_name
        )));
    }

    if !is_dag(graph) {
        Err(GraphinaException::new(&format!(
            "{} requires a directed acyclic graph (DAG)",
            algo_name
        )))
    } else {
        Ok(())
    }
}

/// Validates common preconditions for running an algorithm on the graph.
///
/// This function checks that the graph is not empty, is connected, and has no negative weights.
/// If any precondition fails, it returns a `GraphinaException` with a descriptive message.
///
/// # Arguments
/// * `graph` - The graph to validate.
/// * `algo_name` - The name of the algorithm (for error messages).
///
/// # Returns
/// `Ok(())` if all preconditions pass, or `Err(GraphinaException)` otherwise.
pub fn validate_for_algorithm<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
    graph: &BaseGraph<A, W, Ty>,
    algo_name: &str,
) -> Result<(), GraphinaException>
where
    W: Copy + Into<f64>,
{
    require_non_empty(graph, algo_name)?;
    require_connected(graph, algo_name)?;
    require_non_negative_weights(graph, algo_name)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{Directed, Undirected};

    type Graph = BaseGraph<i32, f64, Undirected>;
    type DiGraph = BaseGraph<i32, f64, Directed>;

    #[test]
    fn test_is_empty() {
        let empty_graph: Graph = Graph::new();
        assert!(is_empty(&empty_graph));

        let mut g = Graph::new();
        g.add_node(1);
        assert!(!is_empty(&g));
    }

    #[test]
    fn test_is_connected() {
        let mut g = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n1, n2, 1.0);
        assert!(is_connected(&g));

        let mut disconnected = Graph::new();
        disconnected.add_node(1);
        disconnected.add_node(2);
        assert!(!is_connected(&disconnected));

        let empty: Graph = Graph::new();
        assert!(!is_connected(&empty));
    }

    #[test]
    fn test_has_negative_weights() {
        let mut g = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n1, n2, 1.0);
        assert!(!has_negative_weights(&g));

        g.add_edge(n1, n2, -1.0);
        assert!(has_negative_weights(&g));
    }

    #[test]
    fn test_has_self_loops() {
        let mut g = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n1, n2, 1.0);
        assert!(!has_self_loops(&g));

        g.add_edge(n1, n1, 1.0);
        assert!(has_self_loops(&g));
    }

    #[test]
    fn test_is_dag() {
        let mut dag = DiGraph::new();
        let n1 = dag.add_node(1);
        let n2 = dag.add_node(2);
        let n3 = dag.add_node(3);
        dag.add_edge(n1, n2, 1.0);
        dag.add_edge(n2, n3, 1.0);
        assert!(is_dag(&dag));

        // Add cycle
        dag.add_edge(n3, n1, 1.0);
        assert!(!is_dag(&dag));

        // Undirected graph with edges is not a DAG
        let mut undirected = Graph::new();
        let u1 = undirected.add_node(1);
        let u2 = undirected.add_node(2);
        undirected.add_edge(u1, u2, 1.0);
        assert!(!is_dag(&undirected));
    }

    #[test]
    fn test_is_bipartite() {
        let mut g = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);

        // Create bipartite graph: (1,2) - (3,4)
        g.add_edge(n1, n3, 1.0);
        g.add_edge(n1, n4, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n2, n4, 1.0);
        assert!(is_bipartite(&g));

        // Add edge within same partition
        g.add_edge(n1, n2, 1.0);
        assert!(!is_bipartite(&g));
    }

    #[test]
    fn test_count_components() {
        let mut g = Graph::new();
        assert_eq!(count_components(&g), 0);

        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);

        // Two components: {1,2} and {3,4}
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n3, n4, 1.0);
        assert_eq!(count_components(&g), 2);

        // Connect them
        g.add_edge(n2, n3, 1.0);
        assert_eq!(count_components(&g), 1);
    }

    #[test]
    fn test_require_functions() {
        let mut g = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n1, n2, 1.0);

        assert!(require_non_empty(&g, "test").is_ok());
        assert!(require_connected(&g, "test").is_ok());
        assert!(require_non_negative_weights(&g, "test").is_ok());
        assert!(require_no_self_loops(&g, "test").is_ok());

        let empty: Graph = Graph::new();
        assert!(require_non_empty(&empty, "test").is_err());
    }

    #[test]
    fn test_validate_for_algorithm() {
        let mut g = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n1, n2, 1.0);

        assert!(validate_for_algorithm(&g, "test_algo").is_ok());

        let empty: Graph = Graph::new();
        assert!(validate_for_algorithm(&empty, "test_algo").is_err());

        let mut disconnected = Graph::new();
        disconnected.add_node(1);
        disconnected.add_node(2);
        assert!(validate_for_algorithm(&disconnected, "test_algo").is_err());

        let mut negative = Graph::new();
        let n3 = negative.add_node(1);
        let n4 = negative.add_node(2);
        negative.add_edge(n3, n4, -1.0);
        assert!(validate_for_algorithm(&negative, "test_algo").is_err());
    }
}
