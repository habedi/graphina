/*!
# Graphina Graph Types

This module defines the core graph types supported by Graphina.
The `BaseGraph` struct is a wrapper around petgraph’s `StableGraph` that provides a uniform API
for both directed and undirected graphs. Graphina provides two sets of APIs:
- The **standard API**, which returns simple values (such as booleans or `Option`s).
- The **`try_…` API**, which returns `Result` types with custom errors defined in `graphina::core::exceptions`.

# Examples

Basic usage:

```rust
use graphina::core::types::{Graph, NodeId};

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(10);

// Using the standard API:
let success = g.update_node(n1, 20);
assert!(success);

// Using the try_… API:
g.try_update_node(n1, 30).expect("Node update should succeed");
*/

use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::prelude::EdgeRef;
use petgraph::stable_graph::StableGraph as PetGraph;
use petgraph::visit::{IntoEdgeReferences, IntoNodeReferences};
use sprs::{CsMat, TriMat};
use std::collections::HashMap;

// Import exceptions from the core exceptions module.
use crate::core::exceptions::{GraphinaException, NodeNotFound};

pub use petgraph::EdgeType;
/// Marker type for directed graphs.
pub use petgraph::{Directed, Undirected};

/// Alias for `NodeIndex` that provides additional functionality.
pub type NodeId = NodeIndex;

/// Alias for `EdgeIndex` that provides additional functionality.
pub type EdgeId = EdgeIndex;

/// Base graph structure that wraps around a petgraph instance.
///
/// Generic parameters:
/// - `A`: Node attribute type.
/// - `W`: Edge weight type.
/// - `Ty`: Graph type (directed/undirected) implementing `GraphConstructor` and `EdgeType`.
#[derive(Debug, Clone)]
pub struct BaseGraph<A, W, Ty: EdgeType> {
    inner: PetGraph<A, W, Ty>,
}

impl<A, W, Ty: EdgeType> Default for BaseGraph<A, W, Ty> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A, W, Ty: EdgeType> BaseGraph<A, W, Ty> {
    /// Creates a new `BaseGraph`.
    pub fn new() -> Self {
        Self {
            inner: PetGraph::with_capacity(0, 0),
        }
    }

    /// Adds a node with the specified attribute to the graph.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use graphina::core::types::Graph;
    /// let mut g = Graph::<i32, f64>::new();
    /// let n = g.add_node(42);
    /// ```
    pub fn add_node(&mut self, attr: A) -> NodeId {
        self.inner.add_node(attr)
    }

    /// Updates the attribute of an existing node.
    ///
    /// Returns `true` if the update was successful, or `false` if the node was not found.
    pub fn update_node(&mut self, node: NodeId, new_attr: A) -> bool {
        match self.inner.node_weight_mut(node) {
            Some(attr) => {
                *attr = new_attr;
                true
            }
            None => false,
        }
    }

    /// Attempts to update the attribute of an existing node.
    ///
    /// Returns `Ok(())` if the update was successful; otherwise, returns a `NodeNotFound` error.
    ///
    /// # Errors
    ///
    /// Returns an error if the node does not exist (has been removed).
    pub fn try_update_node(&mut self, node: NodeId, new_attr: A) -> Result<(), NodeNotFound> {
        if let Some(attr) = self.inner.node_weight_mut(node) {
            *attr = new_attr;
            Ok(())
        } else {
            Err(NodeNotFound::new("Node not found during update"))
        }
    }

    /// Adds an edge with the given weight between two nodes.
    pub fn add_edge(&mut self, source: NodeId, target: NodeId, weight: W) -> EdgeId {
        self.inner.add_edge(source, target, weight)
    }

    /// Removes a node from the graph, returning its attribute if it existed.
    /// All incident edges will be removed.
    pub fn remove_node(&mut self, node: NodeId) -> Option<A> {
        self.inner.remove_node(node)
    }

    /// Attempts to remove a node from the graph.
    ///
    /// Returns the node's attribute if successful, or a `NodeNotFound` error if the node did not exist.
    pub fn try_remove_node(&mut self, node: NodeId) -> Result<A, NodeNotFound> {
        self.inner
            .remove_node(node)
            .ok_or_else(|| NodeNotFound::new("Node not found during removal"))
    }

    /// Removes an edge from the graph, returning its weight if it existed.
    pub fn remove_edge(&mut self, edge: EdgeId) -> Option<W> {
        self.inner.remove_edge(edge)
    }

    /// Attempts to remove an edge from the graph.
    ///
    /// Returns the edge's weight if successful, or a `GraphinaException` if the edge was not found.
    pub fn try_remove_edge(&mut self, edge: EdgeId) -> Result<W, GraphinaException> {
        self.inner
            .remove_edge(edge)
            .ok_or_else(|| GraphinaException::new("Edge not found during removal"))
    }

    /// Returns the number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    /// Returns the number of edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.inner.edge_count()
    }

    /// Returns an iterator over the neighbors of a node.
    pub fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.inner.neighbors(node)
    }

    /// Returns a reference to the attribute of a node.
    pub fn node_attr(&self, node: NodeId) -> Option<&A> {
        self.inner.node_weight(node)
    }

    /// Returns a mutable reference to the attribute of a node.
    pub fn node_attr_mut(&mut self, node: NodeId) -> Option<&mut A> {
        self.inner.node_weight_mut(node)
    }

    /// Returns a reference to the weight of an edge.
    pub fn edge_attr(&self, edge: EdgeId) -> Option<&W> {
        self.inner.edge_weight(edge)
    }

    /// Returns a mutable reference to the weight of an edge.
    pub fn edge_attr_mut(&mut self, edge: EdgeId) -> Option<&mut W> {
        self.inner.edge_weight_mut(edge)
    }

    /// Returns an iterator over all nodes and their attributes.
    pub fn nodes(&self) -> impl Iterator<Item = (NodeId, &A)> + '_ {
        self.inner.node_references()
    }

    /// Returns an iterator over all edges and their weights.
    pub fn edges(&self) -> impl Iterator<Item = (NodeId, NodeId, &W)> + '_ {
        self.inner
            .edge_references()
            .map(|edge| (edge.source(), edge.target(), edge.weight()))
    }

    /// Returns a reference to the inner petgraph instance.
    fn inner(&self) -> &PetGraph<A, W, Ty> {
        &self.inner
    }

    /// Finds and returns the first edge from `source` to `target`.
    ///
    /// # Returns
    ///
    /// * `Option<EdgeId>` - The edge identifier if the edge exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use graphina::core::types::{Graph, NodeId, EdgeId};
    /// let mut g = Graph::<i32, f64>::new();
    /// let n1 = g.add_node(1);
    /// let n2 = g.add_node(2);
    /// let e = g.add_edge(n1, n2, 3.0);
    /// assert_eq!(g.find_edge(n1, n2).map(|eid| eid.index()), Some(e.index()));
    /// ```
    pub fn find_edge(&self, source: NodeId, target: NodeId) -> Option<EdgeId> {
        self.inner
            .edge_references()
            .find(|edge| edge.source() == source && edge.target() == target)
            .map(|edge| edge.id())
    }
}

/// Dense matrix API using owned values.
///
/// The adjacency matrix is built using a contiguous mapping of the current nodes.
/// For undirected graphs, the matrix is symmetric.
impl<A, W, Ty: EdgeType> BaseGraph<A, W, Ty>
where
    W: Clone,
{
    /// Returns the adjacency matrix of the graph as a 2D vector.
    pub fn to_adjacency_matrix(&self) -> Vec<Vec<Option<W>>> {
        let nodes: Vec<NodeId> = self.nodes().map(|(node, _)| node).collect();
        let n = nodes.len();
        let mut mapping: HashMap<NodeId, usize> = HashMap::new();
        for (i, node) in nodes.iter().enumerate() {
            mapping.insert(*node, i);
        }
        let mut matrix = vec![vec![None; n]; n];
        for edge in self.inner().edge_references() {
            let source = edge.source();
            let target = edge.target();
            if let (Some(&i), Some(&j)) = (mapping.get(&source), mapping.get(&target)) {
                matrix[i][j] = Some(edge.weight().clone());
                if !Ty::is_directed() {
                    matrix[j][i] = Some(edge.weight().clone());
                }
            }
        }
        matrix
    }

    /// Constructs a new graph from an adjacency matrix.
    ///
    /// Node attributes are initialized using `A::default()`.
    pub fn from_adjacency_matrix(matrix: &[Vec<Option<W>>]) -> Self
    where
        A: Default,
    {
        let n = matrix.len();
        let mut graph = Self::new();
        let nodes: Vec<NodeId> = (0..n).map(|_| graph.add_node(A::default())).collect();
        for i in 0..n {
            for j in 0..matrix[i].len() {
                if let Some(weight) = &matrix[i][j] {
                    if Ty::is_directed() || i <= j {
                        graph.add_edge(nodes[i], nodes[j], weight.clone());
                    }
                }
            }
        }
        graph
    }
}

/// Sparse matrix API using sprs for efficient representation on large graphs.
///
/// Duplicate entries are combined using the Add operation.
impl<A, W, Ty: EdgeType> BaseGraph<A, W, Ty>
where
    W: Clone + std::ops::Add<Output = W>,
{
    /// Returns the sparse adjacency matrix of the graph as a CsMat in CSR format.
    pub fn to_sparse_adjacency_matrix(&self) -> CsMat<W> {
        let nodes: Vec<NodeId> = self.nodes().map(|(node, _)| node).collect();
        let n = nodes.len();
        let mut mapping: HashMap<NodeId, usize> = HashMap::new();
        for (i, node) in nodes.iter().enumerate() {
            mapping.insert(*node, i);
        }
        let mut triplet = TriMat::new((n, n));
        for edge in self.inner().edge_references() {
            let source = edge.source();
            let target = edge.target();
            if let (Some(&i), Some(&j)) = (mapping.get(&source), mapping.get(&target)) {
                triplet.add_triplet(i, j, edge.weight().clone());
                if !Ty::is_directed() && i != j {
                    triplet.add_triplet(j, i, edge.weight().clone());
                }
            }
        }
        triplet.to_csr()
    }

    /// Constructs a new graph from a sparse adjacency matrix.
    ///
    /// Node attributes are initialized using `A::default()`.
    pub fn from_sparse_adjacency_matrix(sparse: &CsMat<W>) -> Self
    where
        A: Default,
    {
        let n = sparse.rows();
        let mut graph = Self::new();
        let nodes: Vec<NodeId> = (0..n).map(|_| graph.add_node(A::default())).collect();
        for (i, row) in sparse.outer_iterator().enumerate() {
            for (&j, weight) in row.indices().iter().zip(row.data().iter()) {
                if Ty::is_directed() || i <= j {
                    graph.add_edge(nodes[i], nodes[j], weight.clone());
                }
            }
        }
        graph
    }
}

/// Conversion method for graphs with f64 weights to a new weight type U.
///
/// Each edge weight is converted via `U::from`, and node attributes are cloned.
impl<A, Ty> BaseGraph<A, f64, Ty>
where
    Ty: EdgeType,
{
    /// Converts a graph with `f64` weights to one with weight type `U`.
    pub fn convert<U>(&self) -> BaseGraph<A, U, Ty>
    where
        U: From<f64> + Clone,
        A: Clone,
        Ty: EdgeType,
    {
        let mut new_graph = BaseGraph::<A, U, Ty>::new();

        // Build a mapping from old NodeId to new NodeId.
        let mut mapping: HashMap<NodeId, NodeId> = HashMap::new();
        for (node, attr) in self.nodes() {
            let new_node = new_graph.add_node(attr.clone());
            mapping.insert(node, new_node);
        }

        // Copy and convert edges.
        for (u, v, weight) in self.edges() {
            let new_weight = U::from(*weight);
            let new_source = mapping.get(&u).expect("Missing mapping for source node");
            let new_target = mapping.get(&v).expect("Missing mapping for target node");
            new_graph.add_edge(*new_source, *new_target, new_weight);
        }
        new_graph
    }
}

/// Type alias for a directed graph.
/// This creates a `BaseGraph` using the `Directed` edge type.
pub type Digraph<A, W> = BaseGraph<A, W, Directed>;

/// Marker type alias for directed graphs.
/// This refers to the `Directed` marker type.
pub type DigraphMarker = Directed;

/// Type alias for an undirected graph.
/// This creates a `BaseGraph` using the `Undirected` edge type.
pub type Graph<A, W> = BaseGraph<A, W, Undirected>;

/// Marker type alias for undirected graphs.
/// This refers to the `Undirected` marker type.
pub type GraphMarker = Undirected;
