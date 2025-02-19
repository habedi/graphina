#![allow(dead_code)]
use petgraph::graph::{EdgeIndex, Graph as PetGraph, NodeIndex};
use petgraph::prelude::EdgeRef;
use petgraph::visit::IntoNodeReferences;
use petgraph::{Directed, Undirected};

pub trait GraphConstructor<A, W>: petgraph::EdgeType + Sized {
    fn new_graph() -> PetGraph<A, W, Self>;
    fn is_directed() -> bool;
}

impl<A, W, Ty: GraphConstructor<A, W>> Default for GraphWrapper<A, W, Ty> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A, W> GraphConstructor<A, W> for Directed {
    fn new_graph() -> PetGraph<A, W, Self> {
        PetGraph::new()
    }
    fn is_directed() -> bool {
        true
    }
}

impl<A, W> GraphConstructor<A, W> for Undirected {
    fn new_graph() -> PetGraph<A, W, Self> {
        PetGraph::new_undirected()
    }
    fn is_directed() -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(NodeIndex);

impl NodeId {
    pub fn index(&self) -> usize {
        self.0.index()
    }
    pub(crate) fn new(index: NodeIndex) -> Self {
        Self(index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeId(EdgeIndex);

impl EdgeId {
    pub fn index(&self) -> usize {
        self.0.index()
    }
    pub(crate) fn new(index: EdgeIndex) -> Self {
        Self(index)
    }
}

#[derive(Debug, Clone)]
pub struct GraphWrapper<A, W, Ty: GraphConstructor<A, W>> {
    inner: PetGraph<A, W, Ty>,
}

impl<A, W, Ty: GraphConstructor<A, W>> GraphWrapper<A, W, Ty> {
    pub fn new() -> Self {
        Self {
            inner: Ty::new_graph(),
        }
    }

    pub fn add_node(&mut self, attr: A) -> NodeId {
        NodeId::new(self.inner.add_node(attr))
    }

    pub fn update_node(&mut self, node: NodeId, new_attr: A) {
        let index = NodeIndex::new(node.index());
        if let Some(attr) = self.inner.node_weight_mut(index) {
            *attr = new_attr;
        }
    }

    pub fn add_edge(&mut self, source: NodeId, target: NodeId, weight: W) -> EdgeId {
        let source = NodeIndex::new(source.index());
        let target = NodeIndex::new(target.index());
        EdgeId::new(self.inner.add_edge(source, target, weight))
    }

    pub fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.inner.edge_count()
    }

    pub fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        let index = NodeIndex::new(node.index());
        self.inner.neighbors(index).map(NodeId::new)
    }

    pub fn node_attr(&self, node: NodeId) -> Option<&A> {
        let index = NodeIndex::new(node.index());
        self.inner.node_weight(index)
    }

    pub fn edge_attr(&self, edge: EdgeId) -> Option<&W> {
        let index = EdgeIndex::new(edge.index());
        self.inner.edge_weight(index)
    }

    pub fn nodes(&self) -> impl Iterator<Item = (NodeId, &A)> + '_ {
        self.inner
            .node_references()
            .map(|(idx, attr)| (NodeId::new(idx), attr))
    }

    pub fn edges(&self) -> impl Iterator<Item = (NodeId, NodeId, &W)> + '_ {
        self.inner.edge_references().map(|edge| {
            (
                NodeId::new(edge.source()),
                NodeId::new(edge.target()),
                edge.weight(),
            )
        })
    }

    pub fn inner(&self) -> &PetGraph<A, W, Ty> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut PetGraph<A, W, Ty> {
        &mut self.inner
    }
}

pub type Digraph<A, W> = GraphWrapper<A, W, Directed>;
pub type Graph<A, W> = GraphWrapper<A, W, Undirected>;
