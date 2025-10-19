/*!
# Graph Traits

This module defines trait-based abstractions for graph operations, allowing multiple backend implementations
while maintaining a consistent API. This is a key architectural improvement that enables:
- Pluggable graph backends (petgraph, custom implementations, etc.)
- Better testability through mock implementations
- Future extensibility without breaking changes

## Design Philosophy

These traits follow the "interface segregation principle" - smaller, focused traits that can be
composed together rather than one monolithic trait. This allows implementors to support only
the operations that make sense for their graph type.
*/

use crate::core::error::Result;
use crate::core::types::{EdgeId, NodeId};

/// Core read-only graph operations.
///
/// This trait defines the minimal interface for querying graph structure.
pub trait GraphQuery<A, W> {
    /// Returns true if the graph is directed.
    fn is_directed(&self) -> bool;

    /// Returns true if the graph contains no nodes.
    fn is_empty(&self) -> bool;

    /// Returns the number of nodes in the graph.
    fn node_count(&self) -> usize;

    /// Returns the number of edges in the graph.
    fn edge_count(&self) -> usize;

    /// Returns true if the node exists in the graph.
    fn contains_node(&self, node: NodeId) -> bool;

    /// Returns true if there is an edge from source to target.
    fn contains_edge(&self, source: NodeId, target: NodeId) -> bool;

    /// Returns a reference to the attribute of a node.
    fn node_attr(&self, node: NodeId) -> Option<&A>;

    /// Returns the weight of an edge between two nodes.
    fn edge_weight(&self, source: NodeId, target: NodeId) -> Option<&W>;

    /// Returns the density of the graph (ratio of actual edges to possible edges).
    fn density(&self) -> f64 {
        let n = self.node_count();
        if n < 2 {
            return 0.0;
        }
        let m = self.edge_count() as f64;
        let max_edges = (n * (n - 1)) as f64;

        if self.is_directed() {
            m / max_edges
        } else {
            (2.0 * m) / max_edges
        }
    }
}

/// Graph mutation operations.
///
/// This trait extends read operations with modification capabilities.
pub trait GraphMutate<A, W>: GraphQuery<A, W> {
    /// Adds a node with the specified attribute to the graph.
    fn add_node(&mut self, attr: A) -> NodeId;

    /// Adds an edge with the given weight between two nodes.
    fn add_edge(&mut self, source: NodeId, target: NodeId, weight: W) -> Result<EdgeId>;

    /// Updates the attribute of an existing node.
    fn update_node(&mut self, node: NodeId, new_attr: A) -> Result<()>;

    /// Removes a node from the graph, returning its attribute if it existed.
    fn remove_node(&mut self, node: NodeId) -> Result<A>;

    /// Removes an edge from the graph, returning its weight if it existed.
    fn remove_edge(&mut self, edge: EdgeId) -> Result<W>;

    /// Clears all nodes and edges from the graph.
    fn clear(&mut self);
}

/// Graph traversal operations.
///
/// This trait provides methods for iterating over graph elements.
pub trait GraphTraversal<A, W>: GraphQuery<A, W> {
    /// Iterator type for nodes.
    type NodeIter<'a>: Iterator<Item = NodeId>
    where
        Self: 'a,
        A: 'a,
        W: 'a;

    /// Iterator type for neighbors.
    type NeighborIter<'a>: Iterator<Item = NodeId>
    where
        Self: 'a,
        A: 'a,
        W: 'a;

    /// Returns an iterator over all node IDs.
    fn node_ids(&self) -> Self::NodeIter<'_>;

    /// Returns an iterator over the neighbors of a node.
    fn neighbors(&self, node: NodeId) -> Self::NeighborIter<'_>;

    /// Returns the degree of a node (number of incident edges).
    fn degree(&self, node: NodeId) -> Option<usize>;

    /// Returns the in-degree of a node (number of incoming edges).
    fn in_degree(&self, node: NodeId) -> Option<usize>;

    /// Returns the out-degree of a node (number of outgoing edges).
    fn out_degree(&self, node: NodeId) -> Option<usize>;
}

/// Bulk operations for performance-critical scenarios.
///
/// This trait provides optimized methods for adding multiple elements at once.
pub trait GraphBulkOps<A, W>: GraphMutate<A, W> {
    /// Adds multiple nodes at once from a slice of attributes.
    fn add_nodes_bulk(&mut self, attributes: &[A]) -> Vec<NodeId>
    where
        A: Clone;

    /// Adds multiple edges at once from a slice of (source, target, weight) tuples.
    fn add_edges_bulk(&mut self, edges: &[(NodeId, NodeId, W)]) -> Result<Vec<EdgeId>>
    where
        W: Clone;
}

/// Graph algorithms that require specific graph properties.
///
/// This trait is a marker for graphs that support common algorithmic operations.
pub trait GraphAlgorithms<A, W>: GraphTraversal<A, W> {
    /// Returns true if the graph is connected (or strongly connected for directed graphs).
    fn is_connected(&self) -> bool;

    /// Returns true if the graph is acyclic.
    fn is_acyclic(&self) -> bool;

    /// Checks if the graph is valid (e.g., no self-loops, consistent edge directions).
    fn validate(&self) -> Result<()>;
}

/// Weighted graph operations.
///
/// This trait provides methods specific to weighted graphs.
pub trait WeightedGraph<A, W>: GraphQuery<A, W>
where
    W: PartialOrd,
{
    /// Returns the minimum edge weight in the graph.
    fn min_edge_weight(&self) -> Option<&W>;

    /// Returns the maximum edge weight in the graph.
    fn max_edge_weight(&self) -> Option<&W>;

    /// Returns the total weight of all edges.
    fn total_weight(&self) -> W
    where
        W: Clone + std::ops::Add<Output = W> + Default;
}

/// Graph serialization operations.
///
/// This trait provides methods for saving and loading graphs.
pub trait GraphSerialization<A, W>: GraphQuery<A, W>
where
    A: serde::Serialize + serde::de::DeserializeOwned,
    W: serde::Serialize + serde::de::DeserializeOwned,
{
    /// Saves the graph to JSON format.
    fn save_json(&self, path: &str) -> Result<()>;

    /// Loads the graph from JSON format.
    fn load_json(path: &str) -> Result<Self>
    where
        Self: Sized;

    /// Saves the graph to binary format.
    fn save_binary(&self, path: &str) -> Result<()>;

    /// Loads the graph from binary format.
    fn load_binary(path: &str) -> Result<Self>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementation for testing trait design
    struct MockGraph {
        node_count: usize,
        edge_count: usize,
        is_directed: bool,
    }

    impl<A, W> GraphQuery<A, W> for MockGraph {
        fn is_directed(&self) -> bool {
            self.is_directed
        }

        fn is_empty(&self) -> bool {
            self.node_count == 0
        }

        fn node_count(&self) -> usize {
            self.node_count
        }

        fn edge_count(&self) -> usize {
            self.edge_count
        }

        fn contains_node(&self, _node: NodeId) -> bool {
            true
        }

        fn contains_edge(&self, _source: NodeId, _target: NodeId) -> bool {
            true
        }

        fn node_attr(&self, _node: NodeId) -> Option<&A> {
            None
        }

        fn edge_weight(&self, _source: NodeId, _target: NodeId) -> Option<&W> {
            None
        }
    }

    #[test]
    fn test_mock_graph_query() {
        let graph: MockGraph = MockGraph {
            node_count: 10,
            edge_count: 20,
            is_directed: true,
        };

        assert_eq!(GraphQuery::<i32, f64>::node_count(&graph), 10);
        assert_eq!(GraphQuery::<i32, f64>::edge_count(&graph), 20);
        assert!(GraphQuery::<i32, f64>::is_directed(&graph));
        assert!(!GraphQuery::<i32, f64>::is_empty(&graph));
    }

    #[test]
    fn test_density_calculation() {
        let directed_graph: MockGraph = MockGraph {
            node_count: 4,
            edge_count: 6,
            is_directed: true,
        };
        // For directed: 6 / (4 * 3) = 0.5
        assert_eq!(GraphQuery::<i32, f64>::density(&directed_graph), 0.5);

        let undirected_graph: MockGraph = MockGraph {
            node_count: 4,
            edge_count: 3,
            is_directed: false,
        };
        // For undirected: (2 * 3) / (4 * 3) = 0.5
        assert_eq!(GraphQuery::<i32, f64>::density(&undirected_graph), 0.5);
    }
}
