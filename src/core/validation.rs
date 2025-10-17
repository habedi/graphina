/*!
# Graph Validation Utilities

This module provides common validation functions for graphs, such as checking if a graph is empty,
connected, or has negative weights. These utilities help centralize precondition checks across
algorithms, reducing duplication and improving maintainability.
*/

use std::collections::HashSet;

use crate::core::exceptions::GraphinaException;
use crate::core::types::{BaseGraph, GraphConstructor};
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
    if is_empty(graph) {
        return Err(GraphinaException::new(&format!(
            "Graph is empty, cannot run {}",
            algo_name
        )));
    }

    if !is_connected(graph) {
        return Err(GraphinaException::new(&format!(
            "{} requires a connected graph",
            algo_name
        )));
    }

    if has_negative_weights(graph) {
        return Err(GraphinaException::new(&format!(
            "{} does not support negative weights",
            algo_name
        )));
    }

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
