/*!
# Graph Views and Subgraphs Module

This module provides efficient ways to work with subsets of graphs:
- Filtered views (zero-copy)
- Subgraph extraction
- Ego networks
- Induced subgraphs
*/

use std::collections::{HashSet, VecDeque};

use crate::core::exceptions::GraphinaException;
use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use petgraph::EdgeType;

impl<A, W, Ty> BaseGraph<A, W, Ty>
where
    A: Clone,
    W: Clone,
    Ty: GraphConstructor<A, W> + EdgeType,
{
    /// Extracts a subgraph containing only the specified nodes.
    ///
    /// Creates a new graph with copies of the specified nodes and all edges between them.
    ///
    /// # Example
    ///
    /// ```rust
    /// use graphina::core::types::Graph;
    ///
    /// let mut g = Graph::<i32, f64>::new();
    /// let n1 = g.add_node(1);
    /// let n2 = g.add_node(2);
    /// let n3 = g.add_node(3);
    /// g.add_edge(n1, n2, 1.0);
    /// g.add_edge(n2, n3, 2.0);
    ///
    /// let subgraph = g.subgraph(&[n1, n2]).unwrap();
    /// assert_eq!(subgraph.node_count(), 2);
    /// assert_eq!(subgraph.edge_count(), 1);
    /// ```
    pub fn subgraph(&self, nodes: &[NodeId]) -> Result<Self, GraphinaException> {
        let node_set: HashSet<NodeId> = nodes.iter().copied().collect();

        // Verify all nodes exist
        for node in nodes {
            if !self.contains_node(*node) {
                return Err(GraphinaException::new(&format!(
                    "Node {} not found in graph",
                    node.index()
                )));
            }
        }

        let mut subgraph = Self::with_capacity(nodes.len(), self.edge_count());
        let mut node_mapping = std::collections::HashMap::new();

        // Add nodes
        for &node in nodes {
            if let Some(attr) = self.node_attr(node) {
                let new_id = subgraph.add_node(attr.clone());
                node_mapping.insert(node, new_id);
            }
        }

        // Add edges between included nodes
        for (src, tgt, weight) in self.edges() {
            if node_set.contains(&src) && node_set.contains(&tgt) {
                let new_src = node_mapping[&src];
                let new_tgt = node_mapping[&tgt];
                subgraph.add_edge(new_src, new_tgt, weight.clone());
            }
        }

        Ok(subgraph)
    }

    /// Creates an induced subgraph from a set of nodes.
    ///
    /// Same as `subgraph()` - includes all edges between the specified nodes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use graphina::core::types::Graph;
    /// use std::collections::HashSet;
    ///
    /// let mut g = Graph::<i32, f64>::new();
    /// let n1 = g.add_node(1);
    /// let n2 = g.add_node(2);
    /// let n3 = g.add_node(3);
    /// g.add_edge(n1, n2, 1.0);
    /// g.add_edge(n2, n3, 2.0);
    ///
    /// let nodes = vec![n1, n2].into_iter().collect();
    /// let induced = g.induced_subgraph(&nodes).unwrap();
    /// assert_eq!(induced.node_count(), 2);
    /// ```
    pub fn induced_subgraph(&self, nodes: &HashSet<NodeId>) -> Result<Self, GraphinaException> {
        let node_vec: Vec<NodeId> = nodes.iter().copied().collect();
        self.subgraph(&node_vec)
    }

    /// Extracts an ego network centered on a node with a given radius.
    ///
    /// An ego network includes the center node, all nodes within `radius` hops,
    /// and all edges among these nodes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use graphina::core::types::Graph;
    ///
    /// let mut g = Graph::<i32, f64>::new();
    /// let n1 = g.add_node(1);
    /// let n2 = g.add_node(2);
    /// let n3 = g.add_node(3);
    /// let n4 = g.add_node(4);
    /// g.add_edge(n1, n2, 1.0);
    /// g.add_edge(n2, n3, 1.0);
    /// g.add_edge(n3, n4, 1.0);
    ///
    /// // Get ego network of n2 with radius 1
    /// let ego = g.ego_graph(n2, 1).unwrap();
    /// assert_eq!(ego.node_count(), 3); // n1, n2, n3
    /// ```
    pub fn ego_graph(&self, center: NodeId, radius: usize) -> Result<Self, GraphinaException> {
        if !self.contains_node(center) {
            return Err(GraphinaException::new(&format!(
                "Center node {} not found",
                center.index()
            )));
        }

        let mut nodes_in_ego = HashSet::new();
        let mut queue = VecDeque::new();
        let mut distances = std::collections::HashMap::new();

        nodes_in_ego.insert(center);
        queue.push_back(center);
        distances.insert(center, 0);

        // BFS to find nodes within radius
        while let Some(node) = queue.pop_front() {
            let dist = distances[&node];

            if dist < radius {
                for neighbor in self.neighbors(node) {
                    if let std::collections::hash_map::Entry::Vacant(e) = distances.entry(neighbor)
                    {
                        e.insert(dist + 1);
                        nodes_in_ego.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        let nodes_vec: Vec<NodeId> = nodes_in_ego.into_iter().collect();
        self.subgraph(&nodes_vec)
    }

    /// Filters nodes based on a predicate and returns a new subgraph.
    ///
    /// # Example
    ///
    /// ```rust
    /// use graphina::core::types::Graph;
    ///
    /// let mut g = Graph::<i32, f64>::new();
    /// let n1 = g.add_node(1);
    /// let n2 = g.add_node(2);
    /// let n3 = g.add_node(3);
    /// let n4 = g.add_node(4);
    /// g.add_edge(n1, n2, 1.0);
    /// g.add_edge(n2, n3, 1.0);
    /// g.add_edge(n3, n4, 1.0);
    ///
    /// // Keep only even-valued nodes
    /// let filtered = g.filter_nodes(|_id, attr| *attr % 2 == 0);
    /// assert_eq!(filtered.node_count(), 2); // nodes 2 and 4
    /// ```
    pub fn filter_nodes<F>(&self, predicate: F) -> Self
    where
        F: Fn(NodeId, &A) -> bool,
    {
        let nodes: Vec<NodeId> = self
            .nodes()
            .filter(|(id, attr)| predicate(*id, attr))
            .map(|(id, _)| id)
            .collect();

        self.subgraph(&nodes).unwrap_or_else(|_| Self::new())
    }

    /// Filters edges based on a predicate and returns a new subgraph.
    ///
    /// Keeps all nodes but only edges that satisfy the predicate.
    ///
    /// # Example
    ///
    /// ```rust
    /// use graphina::core::types::Graph;
    ///
    /// let mut g = Graph::<i32, f64>::new();
    /// let n1 = g.add_node(1);
    /// let n2 = g.add_node(2);
    /// let n3 = g.add_node(3);
    /// g.add_edge(n1, n2, 0.5);
    /// g.add_edge(n2, n3, 1.5);
    /// g.add_edge(n3, n1, 2.5);
    ///
    /// // Keep only edges with weight > 1.0
    /// let filtered = g.filter_edges(|_src, _tgt, w| *w > 1.0);
    /// assert_eq!(filtered.edge_count(), 2);
    /// ```
    pub fn filter_edges<F>(&self, predicate: F) -> Self
    where
        F: Fn(NodeId, NodeId, &W) -> bool,
    {
        let mut result = Self::with_capacity(self.node_count(), self.edge_count());
        let mut node_mapping = std::collections::HashMap::new();

        // Copy all nodes
        for (node, attr) in self.nodes() {
            let new_id = result.add_node(attr.clone());
            node_mapping.insert(node, new_id);
        }

        // Copy edges that satisfy predicate
        for (src, tgt, weight) in self.edges() {
            if predicate(src, tgt, weight) {
                let new_src = node_mapping[&src];
                let new_tgt = node_mapping[&tgt];
                result.add_edge(new_src, new_tgt, weight.clone());
            }
        }

        result
    }

    /// Returns the k-hop neighborhood of a node.
    ///
    /// Returns all nodes reachable within k hops from the starting node.
    ///
    /// # Example
    ///
    /// ```rust
    /// use graphina::core::types::Graph;
    ///
    /// let mut g = Graph::<i32, f64>::new();
    /// let n1 = g.add_node(1);
    /// let n2 = g.add_node(2);
    /// let n3 = g.add_node(3);
    /// g.add_edge(n1, n2, 1.0);
    /// g.add_edge(n2, n3, 1.0);
    ///
    /// let neighborhood = g.k_hop_neighbors(n1, 2);
    /// assert_eq!(neighborhood.len(), 3); // n1, n2, n3
    /// ```
    pub fn k_hop_neighbors(&self, start: NodeId, k: usize) -> Vec<NodeId> {
        if !self.contains_node(start) {
            return vec![];
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut distances = std::collections::HashMap::new();
        let mut result = vec![];

        visited.insert(start);
        queue.push_back(start);
        distances.insert(start, 0);
        result.push(start);

        while let Some(node) = queue.pop_front() {
            let dist = distances[&node];

            if dist < k {
                for neighbor in self.neighbors(node) {
                    if visited.insert(neighbor) {
                        distances.insert(neighbor, dist + 1);
                        queue.push_back(neighbor);
                        result.push(neighbor);
                    }
                }
            }
        }

        result
    }

    /// Returns nodes connected to the given node (including itself).
    ///
    /// For undirected graphs, returns the connected component containing the node.
    /// For directed graphs, returns the weakly connected component.
    ///
    /// # Example
    ///
    /// ```rust
    /// use graphina::core::types::Graph;
    ///
    /// let mut g = Graph::<i32, f64>::new();
    /// let n1 = g.add_node(1);
    /// let n2 = g.add_node(2);
    /// let n3 = g.add_node(3);
    /// g.add_edge(n1, n2, 1.0);
    /// // n3 is isolated
    ///
    /// let component = g.connected_component(n1);
    /// assert_eq!(component.len(), 2); // n1, n2
    /// ```
    pub fn connected_component(&self, start: NodeId) -> Vec<NodeId> {
        if !self.contains_node(start) {
            return vec![];
        }

        let mut visited = HashSet::new();
        let mut stack = vec![start];
        let mut result = vec![];

        while let Some(node) = stack.pop() {
            if visited.insert(node) {
                result.push(node);

                for neighbor in self.neighbors(node) {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }

        result
    }

    /// Extracts the subgraph of a connected component.
    ///
    /// # Example
    ///
    /// ```rust
    /// use graphina::core::types::Graph;
    ///
    /// let mut g = Graph::<i32, f64>::new();
    /// let n1 = g.add_node(1);
    /// let n2 = g.add_node(2);
    /// let n3 = g.add_node(3);
    /// g.add_edge(n1, n2, 1.0);
    /// // n3 is isolated
    ///
    /// let component = g.component_subgraph(n1).unwrap();
    /// assert_eq!(component.node_count(), 2);
    /// ```
    pub fn component_subgraph(&self, start: NodeId) -> Result<Self, GraphinaException> {
        let nodes = self.connected_component(start);
        self.subgraph(&nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_subgraph() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 2.0);
        g.add_edge(n3, n4, 3.0);

        let sub = g.subgraph(&[n1, n2, n3]).unwrap();
        assert_eq!(sub.node_count(), 3);
        assert_eq!(sub.edge_count(), 2);
    }

    #[test]
    fn test_ego_graph() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n4, 1.0);

        let ego = g.ego_graph(n2, 1).unwrap();
        assert_eq!(ego.node_count(), 3); // n1, n2, n3
    }

    #[test]
    fn test_filter_nodes() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n4, 1.0);

        let filtered = g.filter_nodes(|_id, attr| *attr % 2 == 0);
        assert_eq!(filtered.node_count(), 2); // 2 and 4
    }

    #[test]
    fn test_filter_edges() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);

        g.add_edge(n1, n2, 0.5);
        g.add_edge(n2, n3, 1.5);
        g.add_edge(n3, n1, 2.5);

        let filtered = g.filter_edges(|_src, _tgt, w| *w > 1.0);
        assert_eq!(filtered.node_count(), 3);
        assert_eq!(filtered.edge_count(), 2);
    }

    #[test]
    fn test_k_hop_neighbors() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n4, 1.0);

        let neighbors = g.k_hop_neighbors(n1, 2);
        assert_eq!(neighbors.len(), 3); // n1, n2, n3
    }

    #[test]
    fn test_connected_component() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        // n4 isolated

        let component = g.connected_component(n1);
        assert_eq!(component.len(), 3); // n1, n2, n3
    }

    #[test]
    fn test_component_subgraph() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);

        g.add_edge(n1, n2, 1.0);
        // n3 is isolated

        let sub = g.component_subgraph(n1).unwrap();
        assert_eq!(sub.node_count(), 2);
        assert_eq!(sub.edge_count(), 1);
    }

    #[test]
    fn test_induced_subgraph() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 2.0);

        let nodes = vec![n1, n2].into_iter().collect();
        let induced = g.induced_subgraph(&nodes).unwrap();
        assert_eq!(induced.node_count(), 2);
        assert_eq!(induced.edge_count(), 1);
    }
}
