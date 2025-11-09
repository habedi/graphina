//! Centrality-facing wrappers for community algorithms that produce node->label maps.
//! These provide a NodeMap<usize> interface for label propagation and infomap.

use crate::community::{infomap::infomap, label_propagation::label_propagation};
use crate::core::error::Result;
use crate::core::types::{BaseGraph, GraphConstructor, NodeId, NodeMap};
use petgraph::EdgeType;

/// Run label propagation and return a NodeMap<NodeId, usize> mapping nodes to labels.
pub fn label_propagation_map<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    max_iter: usize,
    seed: Option<u64>,
) -> Result<NodeMap<usize>>
where
    W: Copy + PartialOrd + Into<f64>,
    Ty: GraphConstructor<A, W> + EdgeType,
{
    let labels_vec = label_propagation(graph, max_iter, seed)?;
    let nodes: Vec<NodeId> = graph.nodes().map(|(nid, _)| nid).collect();
    // Map vector indices to NodeIds
    let mut map = NodeMap::new();
    for (i, nid) in nodes.iter().enumerate() {
        if let Some(label) = labels_vec.get(i) {
            map.insert(*nid, *label);
        }
    }
    Ok(map)
}

/// Run infomap and return a NodeMap<NodeId, usize> mapping nodes to module IDs.
pub fn infomap_map<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    max_iter: usize,
    seed: Option<u64>,
) -> Result<NodeMap<usize>>
where
    W: Copy + PartialOrd + Into<f64> + From<u8>,
    Ty: GraphConstructor<A, W> + EdgeType,
{
    let modules_vec = infomap(graph, max_iter, seed)?;
    let nodes: Vec<NodeId> = graph.nodes().map(|(nid, _)| nid).collect();
    let mut map = NodeMap::new();
    for (i, nid) in nodes.iter().enumerate() {
        if let Some(module) = modules_vec.get(i) {
            map.insert(*nid, *module);
        }
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_label_propagation_map_shapes() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n1, n2, 1.0);
        let map = label_propagation_map(&g, 10, Some(42)).unwrap();
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_infomap_map_shapes() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n1, n2, 1.0);
        let map = infomap_map(&g, 10, Some(42)).unwrap();
        assert_eq!(map.len(), 2);
    }
}
