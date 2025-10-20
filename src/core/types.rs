/*!
# Graphina Graph Types
This module defines the core graph types supported by Graphina.
The `BaseGraph` struct is a wrapper around petgraph's `StableGraph` that provides a uniform API
for both directed and undirected graphs. Graphina provides two sets of APIs:
- The **standard API**, which returns simple values (such as booleans or `Option`s).
- The **`try_…` API**, which returns `Result` types with custom errors defined in `graphina::core::error::GraphinaError`.
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
```
*/
use petgraph::EdgeType;
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::prelude::EdgeRef;
use petgraph::stable_graph::StableGraph as PetGraph;
use petgraph::visit::{IntoEdgeReferences, IntoNodeReferences};
use sprs::{CsMat, TriMat};
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
// Import the new unified error type
use crate::core::error::{GraphinaError, Result};
/// Marker type for directed graphs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Directed;
/// Marker type for undirected graphs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Undirected;
/// Implements `Default` for directed graphs.
impl Default for Directed {
    fn default() -> Self {
        Directed
    }
}
/// Implements `Default` for undirected graphs.
impl Default for Undirected {
    fn default() -> Self {
        Undirected
    }
}
/// Implements petgraph's `EdgeType` for directed graphs.
impl EdgeType for Directed {
    fn is_directed() -> bool {
        true
    }
}
/// Implements petgraph's `EdgeType` for undirected graphs.
impl EdgeType for Undirected {
    fn is_directed() -> bool {
        false
    }
}
/// Trait for constructing graphs with specific edge types.
/// Types implementing `GraphConstructor` must also implement petgraph's `EdgeType`.
pub trait GraphConstructor<A, W>: EdgeType + Sized {
    /// Creates a new graph.
    fn new_graph() -> PetGraph<A, W, Self>;
    /// Returns true if the graph is directed.
    fn is_directed() -> bool;
}
impl<A, W> GraphConstructor<A, W> for Directed {
    fn new_graph() -> PetGraph<A, W, Self> {
        // Using StableGraph prevents node index recycling.
        PetGraph::<A, W, Directed>::with_capacity(0, 0)
    }
    fn is_directed() -> bool {
        true
    }
}
impl<A, W> GraphConstructor<A, W> for Undirected {
    fn new_graph() -> PetGraph<A, W, Self> {
        PetGraph::<A, W, Undirected>::with_capacity(0, 0)
    }
    fn is_directed() -> bool {
        false
    }
}
/// Wrapper for `NodeIndex` that provides additional functionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub(crate) NodeIndex);
impl NodeId {
    /// Returns the numeric part of the node's index.
    pub fn index(&self) -> usize {
        self.0.index()
    }
    /// Creates a new `NodeId` from a `NodeIndex`.
    pub fn new(index: NodeIndex) -> Self {
        Self(index)
    }
}
/// Wrapper for `EdgeIndex` that provides additional functionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeId(pub(crate) EdgeIndex);
impl EdgeId {
    /// Returns the numeric part of the edge's index.
    pub fn index(&self) -> usize {
        self.0.index()
    }
    /// Creates a new `EdgeId` from an `EdgeIndex`.
    pub(crate) fn new(index: EdgeIndex) -> Self {
        Self(index)
    }
}
/// Type alias for a directed graph using the `Directed` marker type.
pub type Digraph<A, W> = BaseGraph<A, W, Directed>;
/// Marker alias for directed graphs.
pub type DigraphMarker = Directed;
/// Type alias for an undirected graph using the `Undirected` marker type.
pub type Graph<A, W> = BaseGraph<A, W, Undirected>;
/// Marker alias for undirected graphs.
pub type GraphMarker = Undirected;
/// type alias for HashMap mapping NodeId to T
pub type NodeMap<T> = HashMap<NodeId, T>;
/// type alias for HashMap mapping EdgeId to T
pub type EdgeMap<T> = HashMap<EdgeId, T>;
/// Base graph structure that wraps around a petgraph instance.
///
/// Generic parameters:
/// - `A`: Node attribute type.
/// - `W`: Edge weight type.
/// - `Ty`: Graph type (directed/undirected) implementing `GraphConstructor` and `EdgeType`.
#[derive(Debug, Clone)]
pub struct BaseGraph<A, W, Ty: GraphConstructor<A, W> + EdgeType> {
    pub(crate) inner: PetGraph<A, W, Ty>,
}
impl<A, W, Ty: GraphConstructor<A, W> + EdgeType> Default for BaseGraph<A, W, Ty> {
    fn default() -> Self {
        Self::new()
    }
}
impl<A, W, Ty: GraphConstructor<A, W> + EdgeType> BaseGraph<A, W, Ty> {
    /// Creates a new `BaseGraph`.
    pub fn new() -> Self {
        Self {
            inner: Ty::new_graph(),
        }
    }
    /// Creates a new graph with pre-allocated capacity for nodes and edges.
    ///
    /// This is useful when you know the approximate size of your graph upfront.
    pub fn with_capacity(nodes: usize, edges: usize) -> Self {
        Self {
            inner: PetGraph::<A, W, Ty>::with_capacity(nodes, edges),
        }
    }
    /// Returns a builder for constructing a graph with a fluent API.
    pub fn builder() -> GraphBuilder<A, W, Ty> {
        GraphBuilder::new()
    }
    pub fn is_directed(&self) -> bool {
        self.inner.is_directed()
    }
    /// Returns true if the graph contains no nodes.
    pub fn is_empty(&self) -> bool {
        self.inner.node_count() == 0
    }
    /// Returns the density of the graph.
    ///
    /// Density is the ratio of actual edges to possible edges:
    /// - For directed graphs: edges / (nodes * (nodes - 1))
    /// - For undirected graphs: (2 * edges) / (nodes * (nodes - 1))
    ///
    /// Returns 0.0 for empty graphs or graphs with fewer than 2 nodes.
    pub fn density(&self) -> f64 {
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
    /// Returns true if the node exists in the graph.
    pub fn contains_node(&self, node: NodeId) -> bool {
        self.inner.node_weight(node.0).is_some()
    }
    /// Returns true if there is an edge from source to target.
    pub fn contains_edge(&self, source: NodeId, target: NodeId) -> bool {
        self.find_edge(source, target).is_some()
    }
    /// Returns the degree of a node (number of incident edges).
    ///
    /// For directed graphs, this returns the sum of in-degree and out-degree.
    /// Returns None if the node doesn't exist.
    pub fn degree(&self, node: NodeId) -> Option<usize> {
        if !self.contains_node(node) {
            return None;
        }
        if self.is_directed() {
            Some(self.in_degree(node).unwrap() + self.out_degree(node).unwrap())
        } else {
            Some(self.inner.edges(node.0).count())
        }
    }
    /// Returns the in-degree of a node (number of incoming edges).
    ///
    /// For undirected graphs, this is equivalent to degree.
    /// Returns None if the node doesn't exist.
    pub fn in_degree(&self, node: NodeId) -> Option<usize> {
        if !self.contains_node(node) {
            return None;
        }
        if self.is_directed() {
            Some(
                self.inner
                    .edge_references()
                    .filter(|edge| edge.target() == node.0)
                    .count(),
            )
        } else {
            self.degree(node)
        }
    }
    /// Returns the out-degree of a node (number of outgoing edges).
    ///
    /// For undirected graphs, this is equivalent to degree.
    /// Returns None if the node doesn't exist.
    pub fn out_degree(&self, node: NodeId) -> Option<usize> {
        if !self.contains_node(node) {
            return None;
        }
        Some(self.inner.edges(node.0).count())
    }
    /// Adds a node with the specified attribute to the graph.
    pub fn add_node(&mut self, attr: A) -> NodeId {
        NodeId::new(self.inner.add_node(attr))
    }
    /// Updates the attribute of an existing node.
    pub fn update_node(&mut self, node: NodeId, new_attr: A) -> bool {
        match self.inner.node_weight_mut(node.0) {
            Some(attr) => {
                *attr = new_attr;
                true
            }
            None => false,
        }
    }
    /// Attempts to update the attribute of an existing node.
    pub fn try_update_node(&mut self, node: NodeId, new_attr: A) -> Result<()> {
        if let Some(attr) = self.inner.node_weight_mut(node.0) {
            *attr = new_attr;
            Ok(())
        } else {
            Err(GraphinaError::node_not_found(
                "Node not found during update",
            ))
        }
    }
    /// Adds an edge with the given weight between two nodes.
    pub fn add_edge(&mut self, source: NodeId, target: NodeId, weight: W) -> EdgeId {
        EdgeId::new(self.inner.add_edge(source.0, target.0, weight))
    }
    /// Adds multiple nodes at once from a slice of attributes.
    pub fn add_nodes_bulk(&mut self, attributes: &[A]) -> Vec<NodeId>
    where
        A: Clone,
    {
        let mut node_ids = Vec::with_capacity(attributes.len());
        for attr in attributes {
            node_ids.push(self.add_node(attr.clone()));
        }
        node_ids
    }
    /// Adds multiple edges at once from a slice of (source, target, weight) tuples.
    pub fn add_edges_bulk(&mut self, edges: &[(NodeId, NodeId, W)]) -> Vec<EdgeId>
    where
        W: Clone,
    {
        let mut edge_ids = Vec::with_capacity(edges.len());
        for (src, tgt, weight) in edges {
            edge_ids.push(self.add_edge(*src, *tgt, weight.clone()));
        }
        edge_ids
    }
    /// Extends the graph with nodes from an iterator.
    pub fn extend_nodes<I>(&mut self, iter: I) -> Vec<NodeId>
    where
        I: IntoIterator<Item = A>,
    {
        let iter = iter.into_iter();
        let (lower_bound, _) = iter.size_hint();
        let mut node_ids = Vec::with_capacity(lower_bound);
        for attr in iter {
            node_ids.push(self.add_node(attr));
        }
        node_ids
    }
    /// Extends the graph with edges from an iterator.
    pub fn extend_edges<I>(&mut self, iter: I) -> Vec<EdgeId>
    where
        I: IntoIterator<Item = (NodeId, NodeId, W)>,
    {
        let iter = iter.into_iter();
        let (lower_bound, _) = iter.size_hint();
        let mut edge_ids = Vec::with_capacity(lower_bound);
        for (src, tgt, weight) in iter {
            edge_ids.push(self.add_edge(src, tgt, weight));
        }
        edge_ids
    }
    /// Removes a node from the graph, returning its attribute if it existed.
    pub fn remove_node(&mut self, node: NodeId) -> Option<A> {
        self.inner.remove_node(node.0)
    }
    /// Attempts to remove a node from the graph.
    pub fn try_remove_node(&mut self, node: NodeId) -> Result<A> {
        self.inner
            .remove_node(node.0)
            .ok_or_else(|| GraphinaError::node_not_found("Node not found during removal"))
    }
    /// Removes an edge from the graph, returning its weight if it existed.
    pub fn remove_edge(&mut self, edge: EdgeId) -> Option<W> {
        self.inner.remove_edge(edge.0)
    }
    /// Attempts to remove an edge from the graph.
    pub fn try_remove_edge(&mut self, edge: EdgeId) -> Result<W> {
        self.inner
            .remove_edge(edge.0)
            .ok_or_else(|| GraphinaError::edge_not_found("Edge not found during removal"))
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
        self.inner.neighbors(node.0).map(NodeId::new)
    }
    /// Returns an iterator over outgoing neighbors of a node.
    pub fn outgoing_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.inner.edges(node.0).map(|e| NodeId::new(e.target()))
    }
    /// Returns an iterator over incoming neighbors of a node.
    pub fn incoming_neighbors(&self, node: NodeId) -> Box<dyn Iterator<Item = NodeId> + '_> {
        if self.is_directed() {
            Box::new(
                self.inner
                    .edge_references()
                    .filter(move |e| e.target() == node.0)
                    .map(|e| NodeId::new(e.source())),
            )
        } else {
            Box::new(self.neighbors(node))
        }
    }
    /// Returns a reference to the attribute of a node.
    pub fn node_attr(&self, node: NodeId) -> Option<&A> {
        self.inner.node_weight(node.0)
    }
    /// Returns a mutable reference to the attribute of a node.
    pub fn node_attr_mut(&mut self, node: NodeId) -> Option<&mut A> {
        self.inner.node_weight_mut(node.0)
    }
    /// Returns a reference to the weight of an edge.
    pub fn edge_weight(&self, edge: EdgeId) -> Option<&W> {
        self.inner.edge_weight(edge.0)
    }
    /// Returns a mutable reference to the weight of an edge.
    pub fn edge_weight_mut(&mut self, edge: EdgeId) -> Option<&mut W> {
        self.inner.edge_weight_mut(edge.0)
    }
    /// Alias for `edge_weight` to maintain backward compatibility.
    #[deprecated(
        since = "0.4.0",
        note = "Use `edge_weight` instead for consistent naming"
    )]
    pub fn edge_attr(&self, edge: EdgeId) -> Option<&W> {
        self.edge_weight(edge)
    }
    /// Alias for `edge_weight_mut` to maintain backward compatibility.
    #[deprecated(
        since = "0.4.0",
        note = "Use `edge_weight_mut` instead for consistent naming"
    )]
    pub fn edge_attr_mut(&mut self, edge: EdgeId) -> Option<&mut W> {
        self.edge_weight_mut(edge)
    }
    /// Returns an iterator over all nodes and their attributes.
    pub fn nodes(&self) -> impl Iterator<Item = (NodeId, &A)> + '_ {
        self.inner
            .node_references()
            .map(|(idx, attr)| (NodeId::new(idx), attr))
    }
    /// Returns an iterator over all edges and their weights.
    pub fn edges(&self) -> impl Iterator<Item = (NodeId, NodeId, &W)> + '_ {
        self.inner.edge_references().map(|edge| {
            (
                NodeId::new(edge.source()),
                NodeId::new(edge.target()),
                edge.weight(),
            )
        })
    }
    pub fn outgoing_edges(&self, source: NodeId) -> impl Iterator<Item = (NodeId, &W)> + '_ {
        self.inner
            .edges(source.0)
            .map(|edge| (NodeId(edge.target()), edge.weight()))
    }
    /// Returns a reference to the inner petgraph instance.
    fn inner(&self) -> &PetGraph<A, W, Ty> {
        &self.inner
    }
    /// Exposes a read-only reference to the underlying petgraph StableGraph.
    pub fn as_petgraph(&self) -> &PetGraph<A, W, Ty> {
        &self.inner
    }
    pub fn to_nodemap<T>(&self, mut eval: impl FnMut(NodeId, &A) -> T) -> NodeMap<T> {
        self.nodes()
            .map(|(nodeid, a)| (nodeid, eval(nodeid, a)))
            .collect()
    }
    pub fn to_nodemap_default<T: Default>(&self) -> NodeMap<T> {
        self.to_nodemap(|_, _| Default::default())
    }
    /// Finds and returns the first edge from `source` to `target`.
    pub fn find_edge(&self, source: NodeId, target: NodeId) -> Option<EdgeId> {
        if <Ty as GraphConstructor<A, W>>::is_directed() {
            self.inner
                .edge_references()
                .find(|edge| edge.source() == source.0 && edge.target() == target.0)
                .map(|edge| EdgeId::new(edge.id()))
        } else {
            self.inner
                .edge_references()
                .find(|edge| {
                    (edge.source() == source.0 && edge.target() == target.0)
                        || (edge.source() == target.0 && edge.target() == source.0)
                })
                .map(|edge| EdgeId::new(edge.id()))
        }
    }
    /// Clears all nodes and edges from the graph.
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    /// Returns an iterator over all node IDs.
    pub fn node_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.inner.node_indices().map(NodeId::new)
    }
    /// Returns an iterator over all edge IDs.
    pub fn edge_ids(&self) -> impl Iterator<Item = EdgeId> + '_ {
        self.inner.edge_indices().map(EdgeId::new)
    }
    /// Retains only the nodes that satisfy the predicate.
    pub fn retain_nodes<F>(&mut self, mut predicate: F)
    where
        F: FnMut(NodeId, &A) -> bool,
    {
        let to_remove: Vec<NodeId> = self
            .nodes()
            .filter(|(id, attr)| !predicate(*id, attr))
            .map(|(id, _)| id)
            .collect();
        for node in to_remove {
            self.remove_node(node);
        }
    }
    /// Retains only the edges that satisfy the predicate.
    pub fn retain_edges<F>(&mut self, mut predicate: F)
    where
        F: FnMut(NodeId, NodeId, &W) -> bool,
    {
        let to_remove: Vec<EdgeId> = self
            .edges()
            .filter_map(|(src, dst, w)| {
                let edge_id = self.find_edge(src, dst)?;
                if !predicate(src, dst, w) {
                    Some(edge_id)
                } else {
                    None
                }
            })
            .collect();
        for edge in to_remove {
            self.remove_edge(edge);
        }
    }
    /// Returns an iterator over all edges with their EdgeId.
    pub fn edges_with_ids(&self) -> impl Iterator<Item = (EdgeId, NodeId, NodeId, &W)> + '_ {
        self.inner.edge_references().map(|edge| {
            (
                EdgeId::new(edge.id()),
                NodeId::new(edge.source()),
                NodeId::new(edge.target()),
                edge.weight(),
            )
        })
    }
    /// Adds an edge if it doesn't already exist. Returns (edge_id, inserted?).
    pub fn add_edge_if_absent(
        &mut self,
        source: NodeId,
        target: NodeId,
        weight: W,
    ) -> (EdgeId, bool)
    where
        W: Clone,
    {
        if let Some(eid) = self.find_edge(source, target) {
            (eid, false)
        } else {
            if !<Ty as GraphConstructor<A, W>>::is_directed() {
                if let Some(eid) = self.find_edge(target, source) {
                    return (eid, false);
                }
            }
            let eid = self.add_edge(source, target, weight);
            (eid, true)
        }
    }
}
/// Dense matrix API using owned values.
impl<A, W, Ty: GraphConstructor<A, W> + EdgeType> BaseGraph<A, W, Ty>
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
            let source = NodeId::new(edge.source());
            let target = NodeId::new(edge.target());
            if let (Some(&i), Some(&j)) = (mapping.get(&source), mapping.get(&target)) {
                matrix[i][j] = Some(edge.weight().clone());
                if !<Ty as GraphConstructor<A, W>>::is_directed() {
                    matrix[j][i] = Some(edge.weight().clone());
                }
            }
        }
        matrix
    }
    /// Constructs a new graph from an adjacency matrix.
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
                    if <Ty as GraphConstructor<A, W>>::is_directed() || i <= j {
                        graph.add_edge(nodes[i], nodes[j], weight.clone());
                    }
                }
            }
        }
        graph
    }
}
/// Sparse matrix API using sprs for efficient representation on large graphs.
impl<A, W, Ty: GraphConstructor<A, W> + EdgeType> BaseGraph<A, W, Ty>
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
            let source = NodeId::new(edge.source());
            let target = NodeId::new(edge.target());
            if let (Some(&i), Some(&j)) = (mapping.get(&source), mapping.get(&target)) {
                triplet.add_triplet(i, j, edge.weight().clone());
                if !<Ty as GraphConstructor<A, W>>::is_directed() && i != j {
                    triplet.add_triplet(j, i, edge.weight().clone());
                }
            }
        }
        triplet.to_csr()
    }
    /// Constructs a new graph from a sparse adjacency matrix.
    pub fn from_sparse_adjacency_matrix(sparse: &CsMat<W>) -> Self
    where
        A: Default,
    {
        let n = sparse.rows();
        let mut graph = Self::new();
        let nodes: Vec<NodeId> = (0..n).map(|_| graph.add_node(A::default())).collect();
        for (i, row) in sparse.outer_iterator().enumerate() {
            for (&j, weight) in row.indices().iter().zip(row.data().iter()) {
                if <Ty as GraphConstructor<A, W>>::is_directed() || i <= j {
                    graph.add_edge(nodes[i], nodes[j], weight.clone());
                }
            }
        }
        graph
    }
}
/// Conversion method for graphs with f64 weights to a new weight type U.
impl<A, Ty> BaseGraph<A, f64, Ty>
where
    Ty: GraphConstructor<A, f64> + EdgeType,
{
    /// Converts a graph with `f64` weights to one with weight type `U`.
    pub fn convert<U>(&self) -> BaseGraph<A, U, Ty>
    where
        U: From<f64> + Clone,
        A: Clone,
        Ty: GraphConstructor<A, U>,
    {
        let mut new_graph = BaseGraph::<A, U, Ty> {
            inner: <Ty as GraphConstructor<A, U>>::new_graph(),
        };
        let mut mapping: HashMap<NodeId, NodeId> = HashMap::new();
        for (node, attr) in self.nodes() {
            let new_node = new_graph.add_node(attr.clone());
            mapping.insert(node, new_node);
        }
        for (u, v, weight) in self.edges() {
            let new_weight = U::from(*weight);
            let new_source = mapping.get(&u).expect("Missing mapping for source node");
            let new_target = mapping.get(&v).expect("Missing mapping for target node");
            new_graph.add_edge(*new_source, *new_target, new_weight);
        }
        new_graph
    }
}
/// Mapping utilities for transforming node attributes and edge weights.
impl<A, W, Ty> BaseGraph<A, W, Ty>
where
    Ty: GraphConstructor<A, W> + EdgeType,
{
    /// Map node attributes to a new type, producing a new graph with cloned structure.
    pub fn map_node_attrs<B>(&self, mut f: impl FnMut(NodeId, &A) -> B) -> BaseGraph<B, W, Ty>
    where
        W: Clone,
        Ty: GraphConstructor<B, W>,
    {
        let mut new_graph = BaseGraph::<B, W, Ty> {
            inner: <Ty as GraphConstructor<B, W>>::new_graph(),
        };
        let mut id_map = Vec::new();
        for (_nid, a) in self.nodes() {
            let nb = f(_nid, a);
            id_map.push(new_graph.add_node(nb));
        }
        for (u, v, w) in self.edges() {
            new_graph.add_edge(id_map[u.index()], id_map[v.index()], w.clone());
        }
        new_graph
    }
    /// Map edge weights to a new type, producing a new graph with cloned node attributes.
    pub fn map_edge_weights<U>(&self, mut f: impl FnMut(EdgeId, &W) -> U) -> BaseGraph<A, U, Ty>
    where
        A: Clone,
        Ty: GraphConstructor<A, U>,
    {
        let mut new_graph = BaseGraph::<A, U, Ty> {
            inner: <Ty as GraphConstructor<A, U>>::new_graph(),
        };
        let mut id_map = Vec::new();
        for (_nid, a) in self.nodes() {
            id_map.push(new_graph.add_node(a.clone()));
        }
        for (eid, u, v, w) in self.edges_with_ids() {
            new_graph.add_edge(id_map[u.index()], id_map[v.index()], f(eid, w));
        }
        new_graph
    }
}
/// Indexing support for node attributes using NodeId.
impl<A, W, Ty> Index<NodeId> for BaseGraph<A, W, Ty>
where
    Ty: GraphConstructor<A, W> + EdgeType,
{
    type Output = A;
    fn index(&self, index: NodeId) -> &Self::Output {
        self.node_attr(index).expect("invalid NodeId")
    }
}
impl<A, W, Ty> IndexMut<NodeId> for BaseGraph<A, W, Ty>
where
    Ty: GraphConstructor<A, W> + EdgeType,
{
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        self.node_attr_mut(index).expect("invalid NodeId")
    }
}
/// Builder for constructing graphs with a fluent API.
pub struct GraphBuilder<A, W, Ty: GraphConstructor<A, W> + EdgeType> {
    nodes: Vec<A>,
    edges: Vec<(usize, usize, W)>,
    _marker: std::marker::PhantomData<Ty>,
}
impl<A, W, Ty: GraphConstructor<A, W> + EdgeType> Default for GraphBuilder<A, W, Ty> {
    fn default() -> Self {
        Self::new()
    }
}
impl<A, W, Ty: GraphConstructor<A, W> + EdgeType> GraphBuilder<A, W, Ty> {
    /// Creates a new `GraphBuilder`.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }
    /// Adds a node to the builder.
    pub fn add_node(mut self, attr: A) -> Self {
        self.nodes.push(attr);
        self
    }
    /// Adds an edge to the builder.
    pub fn add_edge(mut self, source: usize, target: usize, weight: W) -> Self {
        self.edges.push((source, target, weight));
        self
    }
    /// Consumes the builder and constructs the graph.
    pub fn build(self) -> BaseGraph<A, W, Ty> {
        let mut graph = BaseGraph::with_capacity(self.nodes.len(), self.edges.len());
        let node_ids: Vec<NodeId> = self
            .nodes
            .into_iter()
            .map(|attr| graph.add_node(attr))
            .collect();
        for (source, target, weight) in self.edges {
            graph.add_edge(node_ids[source], node_ids[target], weight);
        }
        graph
    }
}
/// Extra util trait for graph-specific operations.
pub trait GraphinaGraph<A, W> {
    /// Returns edges where backward edges are included for undirected graphs.
    fn flow_edges<'a>(&'a self) -> impl Iterator<Item = (NodeId, NodeId, &'a W)> + 'a
    where
        W: 'a;
    /// Returns an iterator over outgoing edges from a node as (target, weight).
    fn outgoing_edges<'a>(&'a self, node: NodeId) -> impl Iterator<Item = (NodeId, &'a W)> + 'a
    where
        W: 'a;
    /// Returns default value map initialized with default for all nodes.
    fn to_nodemap_default<T: Default>(&self) -> NodeMap<T>;
}
impl<A, W> GraphinaGraph<A, W> for BaseGraph<A, W, Undirected> {
    fn flow_edges<'a>(&'a self) -> impl Iterator<Item = (NodeId, NodeId, &'a W)> + 'a
    where
        W: 'a,
    {
        self.edges()
            .flat_map(|(src, dst, w)| [(src, dst, w), (dst, src, w)].into_iter())
    }
    fn outgoing_edges<'a>(&'a self, node: NodeId) -> impl Iterator<Item = (NodeId, &'a W)> + 'a
    where
        W: 'a,
    {
        self.inner
            .edges(node.0)
            .map(|e| (NodeId::new(e.target()), e.weight()))
    }
    fn to_nodemap_default<T: Default>(&self) -> NodeMap<T> {
        self.nodes().map(|(node, _)| (node, T::default())).collect()
    }
}
impl<A, W> GraphinaGraph<A, W> for BaseGraph<A, W, Directed> {
    fn flow_edges<'a>(&'a self) -> impl Iterator<Item = (NodeId, NodeId, &'a W)> + 'a
    where
        W: 'a,
    {
        self.edges()
    }
    fn outgoing_edges<'a>(&'a self, node: NodeId) -> impl Iterator<Item = (NodeId, &'a W)> + 'a
    where
        W: 'a,
    {
        self.inner
            .edges(node.0)
            .map(|e| (NodeId::new(e.target()), e.weight()))
    }
    fn to_nodemap_default<T: Default>(&self) -> NodeMap<T> {
        self.nodes().map(|(node, _)| (node, T::default())).collect()
    }
}
/// A deterministic, strongly-typed wrapper over a NodeId-keyed map.
#[derive(Debug, Clone, Default)]
pub struct OrderedNodeMap<T>(std::collections::BTreeMap<NodeId, T>);
impl<T> OrderedNodeMap<T> {
    pub fn new() -> Self {
        Self(Default::default())
    }
    pub fn insert(&mut self, k: NodeId, v: T) -> Option<T> {
        self.0.insert(k, v)
    }
    pub fn get(&self, k: &NodeId) -> Option<&T> {
        self.0.get(k)
    }
    pub fn get_mut(&mut self, k: &NodeId) -> Option<&mut T> {
        self.0.get_mut(k)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&NodeId, &T)> {
        self.0.iter()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl<T> From<NodeMap<T>> for OrderedNodeMap<T> {
    fn from(h: NodeMap<T>) -> Self {
        let mut bt = std::collections::BTreeMap::new();
        for (k, v) in h {
            bt.insert(k, v);
        }
        OrderedNodeMap(bt)
    }
}
impl<T> From<OrderedNodeMap<T>> for NodeMap<T> {
    fn from(ordered: OrderedNodeMap<T>) -> Self {
        let mut h = HashMap::with_capacity(ordered.0.len());
        for (k, v) in ordered.0 {
            h.insert(k, v);
        }
        h
    }
}
impl<T> IntoIterator for OrderedNodeMap<T> {
    type Item = (NodeId, T);
    type IntoIter = std::collections::btree_map::IntoIter<NodeId, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_digraph() {
        let mut dgraph = Digraph::<i32, f32>::new();
        let n1 = dgraph.add_node(1);
        let n2 = dgraph.add_node(2);
        let n3 = dgraph.add_node(3);
        let _e1 = dgraph.add_edge(n1, n2, 1.0);
        let _e2 = dgraph.add_edge(n2, n3, 2.0);
        let _e3 = dgraph.add_edge(n3, n1, 3.0);
        assert_eq!(dgraph.node_count(), 3);
        assert_eq!(dgraph.edge_count(), 3);
        let neighbors_n1: Vec<NodeId> = dgraph.neighbors(n1).collect();
        assert!(neighbors_n1.contains(&n2));
        assert!(!neighbors_n1.contains(&n3));
        assert_eq!(*dgraph.node_attr(n1).unwrap(), 1);
        assert_eq!(*dgraph.node_attr(n2).unwrap(), 2);
        assert_eq!(*dgraph.node_attr(n3).unwrap(), 3);
    }
    #[test]
    fn test_undirected_graph() {
        let mut graph = Graph::<i32, f32>::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let _e1 = graph.add_edge(n1, n2, 1.0);
        let _e2 = graph.add_edge(n2, n3, 2.0);
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
        let neighbors_n2: Vec<NodeId> = graph.neighbors(n2).collect();
        assert!(neighbors_n2.contains(&n1));
        assert!(neighbors_n2.contains(&n3));
    }
}
