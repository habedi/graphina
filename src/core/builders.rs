/*!
# Advanced Graph Builders

This module provides enhanced builder patterns for constructing graphs with complex configurations.
These builders support fluent APIs, validation, and preset configurations for common graph types.
*/

use crate::core::error::{GraphinaError, Result};
use crate::core::types::{BaseGraph, Directed, GraphConstructor, NodeId, Undirected};
use petgraph::EdgeType;
use std::marker::PhantomData;

/// Ergonomic aliases for common builder types.
pub type DirectedGraphBuilder<A, W> = AdvancedGraphBuilder<A, W, Directed>;
pub type UndirectedGraphBuilder<A, W> = AdvancedGraphBuilder<A, W, Undirected>;

/// Advanced builder for graphs with validation and configuration options.
///
/// # Example
///
/// ```rust
/// use graphina::core::builders::AdvancedGraphBuilder;
/// use graphina::core::types::Directed;
///
/// let graph = AdvancedGraphBuilder::<i32, f64, Directed>::directed()
///     .with_capacity(100, 200)
///     .allow_self_loops(false)
///     .allow_parallel_edges(false)
///     .build();
/// ```
pub struct AdvancedGraphBuilder<A, W, Ty: GraphConstructor<A, W> + EdgeType> {
    capacity_nodes: usize,
    capacity_edges: usize,
    allow_self_loops: bool,
    allow_parallel_edges: bool,
    nodes: Vec<A>,
    edges: Vec<(usize, usize, W)>,
    _marker: PhantomData<Ty>,
}

impl<A, W> AdvancedGraphBuilder<A, W, Directed> {
    /// Creates a new builder for a directed graph.
    pub fn directed() -> Self {
        Self::new()
    }
}

impl<A, W> AdvancedGraphBuilder<A, W, Undirected> {
    /// Creates a new builder for an undirected graph.
    pub fn undirected() -> Self {
        Self::new()
    }
}

impl<A, W, Ty: GraphConstructor<A, W> + EdgeType> Default for AdvancedGraphBuilder<A, W, Ty> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A, W, Ty: GraphConstructor<A, W> + EdgeType> AdvancedGraphBuilder<A, W, Ty> {
    /// Creates a new advanced graph builder.
    pub fn new() -> Self {
        Self {
            capacity_nodes: 0,
            capacity_edges: 0,
            allow_self_loops: true,
            allow_parallel_edges: true,
            nodes: Vec::new(),
            edges: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// Sets the pre-allocated capacity for nodes and edges.
    pub fn with_capacity(mut self, nodes: usize, edges: usize) -> Self {
        self.capacity_nodes = nodes;
        self.capacity_edges = edges;
        self
    }

    /// Sets whether self-loops (edges from a node to itself) are allowed.
    pub fn allow_self_loops(mut self, allow: bool) -> Self {
        self.allow_self_loops = allow;
        self
    }

    /// Sets whether parallel edges (multiple edges between the same pair of nodes) are allowed.
    pub fn allow_parallel_edges(mut self, allow: bool) -> Self {
        self.allow_parallel_edges = allow;
        self
    }

    /// Adds a node to the builder.
    pub fn add_node(mut self, attr: A) -> Self {
        self.nodes.push(attr);
        self
    }

    /// Adds multiple nodes from an iterator.
    pub fn add_nodes<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        self.nodes.extend(iter);
        self
    }

    /// Adds an edge to the builder.
    pub fn add_edge(mut self, source: usize, target: usize, weight: W) -> Self {
        self.edges.push((source, target, weight));
        self
    }

    /// Adds multiple edges from an iterator.
    pub fn add_edges<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = (usize, usize, W)>,
    {
        self.edges.extend(iter);
        self
    }

    /// Validates the configuration and constructs the graph.
    pub fn build(self) -> Result<BaseGraph<A, W, Ty>> {
        self.validate()?;

        let mut graph = BaseGraph::with_capacity(
            self.capacity_nodes.max(self.nodes.len()),
            self.capacity_edges.max(self.edges.len()),
        );

        // Add nodes
        let node_ids: Vec<NodeId> = self
            .nodes
            .into_iter()
            .map(|attr| graph.add_node(attr))
            .collect();

        // Add edges with validation
        for (source, target, weight) in self.edges {
            if source >= node_ids.len() || target >= node_ids.len() {
                return Err(GraphinaError::invalid_argument(format!(
                    "Edge references invalid node index: ({}, {})",
                    source, target
                )));
            }

            if !self.allow_self_loops && source == target {
                return Err(GraphinaError::invalid_argument(
                    "Self-loops are not allowed in this graph configuration",
                ));
            }

            if !self.allow_parallel_edges && graph.contains_edge(node_ids[source], node_ids[target])
            {
                return Err(GraphinaError::invalid_argument(format!(
                    "Parallel edges are not allowed: edge ({}, {}) already exists",
                    source, target
                )));
            }

            graph.add_edge(node_ids[source], node_ids[target], weight);
        }

        Ok(graph)
    }

    /// Validates the builder configuration without constructing the graph.
    pub fn validate(&self) -> Result<()> {
        // Check for node index validity in edges
        for (source, target, _) in &self.edges {
            if *source >= self.nodes.len() {
                let max_info = if self.nodes.is_empty() {
                    "none".to_string()
                } else {
                    (self.nodes.len() - 1).to_string()
                };
                return Err(GraphinaError::invalid_argument(format!(
                    "Edge source index {} is out of bounds (max: {})",
                    source, max_info
                )));
            }
            if *target >= self.nodes.len() {
                let max_info = if self.nodes.is_empty() {
                    "none".to_string()
                } else {
                    (self.nodes.len() - 1).to_string()
                };
                return Err(GraphinaError::invalid_argument(format!(
                    "Edge target index {} is out of bounds (max: {})",
                    target, max_info
                )));
            }
        }

        Ok(())
    }
}

/// Builder for common graph topologies.
pub struct TopologyBuilder;

impl TopologyBuilder {
    /// Creates a complete graph with n nodes.
    pub fn complete<A, W>(n: usize, node_attr: A, edge_weight: W) -> BaseGraph<A, W, Undirected>
    where
        A: Clone,
        W: Clone,
    {
        let mut builder = AdvancedGraphBuilder::undirected().with_capacity(n, n * (n - 1) / 2);

        // Add nodes
        for _ in 0..n {
            builder = builder.add_node(node_attr.clone());
        }

        // Add all possible edges
        for i in 0..n {
            for j in (i + 1)..n {
                builder = builder.add_edge(i, j, edge_weight.clone());
            }
        }

        builder
            .build()
            .expect("Complete graph should always be valid")
    }

    /// Creates a cycle graph with n nodes.
    pub fn cycle<A, W>(n: usize, node_attr: A, edge_weight: W) -> BaseGraph<A, W, Undirected>
    where
        A: Clone,
        W: Clone,
    {
        let mut builder = AdvancedGraphBuilder::undirected().with_capacity(n, n);

        // Add nodes
        for _ in 0..n {
            builder = builder.add_node(node_attr.clone());
        }

        // Add cycle edges
        for i in 0..n {
            let next = (i + 1) % n;
            builder = builder.add_edge(i, next, edge_weight.clone());
        }

        builder.build().expect("Cycle graph should always be valid")
    }

    /// Creates a path graph with n nodes.
    pub fn path<A, W>(n: usize, node_attr: A, edge_weight: W) -> BaseGraph<A, W, Undirected>
    where
        A: Clone,
        W: Clone,
    {
        if n == 0 {
            return AdvancedGraphBuilder::undirected().build().unwrap();
        }

        let mut builder = AdvancedGraphBuilder::undirected().with_capacity(n, n.saturating_sub(1));

        // Add nodes
        for _ in 0..n {
            builder = builder.add_node(node_attr.clone());
        }

        // Add path edges
        for i in 0..(n - 1) {
            builder = builder.add_edge(i, i + 1, edge_weight.clone());
        }

        builder.build().expect("Path graph should always be valid")
    }

    /// Creates a star graph with n nodes (1 central node + (n-1) peripheral nodes).
    pub fn star<A, W>(n: usize, node_attr: A, edge_weight: W) -> BaseGraph<A, W, Undirected>
    where
        A: Clone,
        W: Clone,
    {
        if n == 0 {
            return AdvancedGraphBuilder::undirected().build().unwrap();
        }

        let mut builder = AdvancedGraphBuilder::undirected().with_capacity(n, n - 1);

        // Add nodes
        for _ in 0..n {
            builder = builder.add_node(node_attr.clone());
        }

        // Add edges from center (node 0) to all others
        for i in 1..n {
            builder = builder.add_edge(0, i, edge_weight.clone());
        }

        builder.build().expect("Star graph should always be valid")
    }

    /// Creates a grid graph with dimensions rows x cols.
    pub fn grid<A, W>(
        rows: usize,
        cols: usize,
        node_attr: A,
        edge_weight: W,
    ) -> BaseGraph<A, W, Undirected>
    where
        A: Clone,
        W: Clone,
    {
        let n = rows * cols;
        let mut builder = AdvancedGraphBuilder::undirected().with_capacity(n, 2 * n - rows - cols);

        // Add nodes
        for _ in 0..n {
            builder = builder.add_node(node_attr.clone());
        }

        // Add grid edges
        for i in 0..rows {
            for j in 0..cols {
                let current = i * cols + j;

                // Connect to right neighbor
                if j < cols - 1 {
                    builder = builder.add_edge(current, current + 1, edge_weight.clone());
                }

                // Connect to bottom neighbor
                if i < rows - 1 {
                    builder = builder.add_edge(current, current + cols, edge_weight.clone());
                }
            }
        }

        builder.build().expect("Grid graph should always be valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_builder_basic() {
        let graph = AdvancedGraphBuilder::directed()
            .add_node(1)
            .add_node(2)
            .add_edge(0, 1, 1.0)
            .build()
            .unwrap();

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_builder_with_capacity() {
        let graph: BaseGraph<i32, f64, Undirected> = AdvancedGraphBuilder::undirected()
            .with_capacity(100, 200)
            .build()
            .unwrap();

        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_self_loop_validation() {
        let result = AdvancedGraphBuilder::directed()
            .allow_self_loops(false)
            .add_node(1)
            .add_edge(0, 0, 1.0)
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Self-loops"));
    }

    #[test]
    fn test_parallel_edge_validation() {
        let result = AdvancedGraphBuilder::undirected()
            .allow_parallel_edges(false)
            .add_node(1)
            .add_node(2)
            .add_edge(0, 1, 1.0)
            .add_edge(0, 1, 2.0)
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Parallel edges"));
    }

    #[test]
    fn test_invalid_node_index() {
        let result = AdvancedGraphBuilder::directed()
            .add_node(1)
            .add_edge(0, 5, 1.0)
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_no_nodes_with_edge() {
        // Ensure that validate() does not panic when edges reference missing nodes and node list is empty.
        let result = AdvancedGraphBuilder::<i32, f64, Directed>::directed()
            .add_edge(0, 0, 1.0)
            .build();
        assert!(result.is_err());
        let err = result.err().unwrap().to_string();
        assert!(err.contains("out of bounds"));
    }

    #[test]
    fn test_topology_complete_graph() {
        let graph = TopologyBuilder::complete(5, (), 1.0);
        assert_eq!(graph.node_count(), 5);
        assert_eq!(graph.edge_count(), 10); // 5 * 4 / 2
    }

    #[test]
    fn test_topology_cycle_graph() {
        let graph = TopologyBuilder::cycle(6, (), 1.0);
        assert_eq!(graph.node_count(), 6);
        assert_eq!(graph.edge_count(), 6);
    }

    #[test]
    fn test_topology_path_graph() {
        let graph = TopologyBuilder::path(5, (), 1.0);
        assert_eq!(graph.node_count(), 5);
        assert_eq!(graph.edge_count(), 4);
    }

    #[test]
    fn test_topology_path_graph_zero_nodes() {
        let graph = TopologyBuilder::path::<(), f64>(0, (), 1.0);
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_topology_star_graph() {
        let graph = TopologyBuilder::star(6, (), 1.0);
        assert_eq!(graph.node_count(), 6);
        assert_eq!(graph.edge_count(), 5);
    }

    #[test]
    fn test_topology_grid_graph() {
        let graph = TopologyBuilder::grid(3, 4, (), 1.0);
        assert_eq!(graph.node_count(), 12);
        // (3-1)*4 + 3*(4-1) = 8 + 9 = 17
        assert_eq!(graph.edge_count(), 17);
    }

    #[test]
    fn test_add_multiple_nodes() {
        let graph: BaseGraph<i32, f64, Directed> = AdvancedGraphBuilder::directed()
            .add_nodes(vec![1, 2, 3, 4, 5])
            .build()
            .unwrap();

        assert_eq!(graph.node_count(), 5);
    }

    #[test]
    fn test_add_multiple_edges() {
        let graph = AdvancedGraphBuilder::directed()
            .add_nodes(vec![1, 2, 3])
            .add_edges(vec![(0, 1, 1.0), (1, 2, 2.0)])
            .build()
            .unwrap();

        assert_eq!(graph.edge_count(), 2);
    }
}
