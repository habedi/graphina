//! Other centrality algorithms.
//!
//! This module provides additional centrality measures like local/global reaching, VoteRank, Laplacian centrality.
//!
//! Convention: most functions return `Result<_, crate::core::error::GraphinaError>` for
//! observability and error propagation. Selector-style routines that return node lists (e.g.,
//! `voterank`) may return plain values.

use crate::core::error::Result;
use crate::core::paths::dijkstra_path_impl;
use crate::core::types::{BaseGraph, GraphConstructor, GraphinaGraph, NodeId, NodeMap};
use std::collections::HashMap;
use std::collections::HashSet;

/// Local reaching centrality: measures the ability of a node to reach other nodes within a certain distance.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `distance`: the maximum distance to consider.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing local reaching centralities of each node in the graph.
pub fn local_reaching_centrality<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    distance: usize,
) -> Result<NodeMap<f64>>
where
    Ty: GraphConstructor<A, W>,
{
    let mut centrality = NodeMap::new();
    for (node, _) in graph.nodes() {
        let mut reached = HashSet::new();
        let mut current = HashSet::new();
        current.insert(node);
        reached.insert(node);

        for _ in 0..distance {
            let mut next = HashSet::new();
            for &n in &current {
                for neighbor in graph.neighbors(n) {
                    if !reached.contains(&neighbor) {
                        reached.insert(neighbor);
                        next.insert(neighbor);
                    }
                }
            }
            current = next;
        }

        centrality.insert(node, reached.len() as f64);
    }
    Ok(centrality)
}

/// Global reaching centrality: similar to local but considers the entire graph.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing global reaching centralities of each node in the graph.
pub fn global_reaching_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Result<NodeMap<f64>>
where
    Ty: GraphConstructor<A, W>,
{
    local_reaching_centrality(graph, graph.node_count())
}

/// VoteRank: a centrality measure based on voting.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `num_seeds`: number of seeds to select.
///
/// # Returns
///
/// A vector of `NodeId` representing the top nodes by VoteRank.
pub fn voterank<A, W, Ty>(graph: &BaseGraph<A, W, Ty>, num_seeds: usize) -> Vec<NodeId>
where
    Ty: GraphConstructor<A, W>,
{
    // Use a compact mapping to avoid relying on raw StableGraph indices
    let node_list: Vec<NodeId> = graph.nodes().map(|(u, _)| u).collect();
    let mut node_to_idx: HashMap<NodeId, usize> = HashMap::new();
    for (i, &nid) in node_list.iter().enumerate() {
        node_to_idx.insert(nid, i);
    }
    let mut votes = vec![0.0; node_list.len()];
    let mut selected = Vec::new();
    let mut remaining: HashSet<NodeId> = node_list.iter().copied().collect();

    for _ in 0..num_seeds.min(node_list.len()) {
        let mut max_vote = -1.0;
        let mut candidate = None;
        for &node in &remaining {
            let vote = graph
                .neighbors(node)
                .filter(|n| remaining.contains(n))
                .count() as f64;
            if vote > max_vote {
                max_vote = vote;
                candidate = Some(node);
            }
        }
        if let Some(node) = candidate {
            selected.push(node);
            remaining.remove(&node);
            for neighbor in graph.neighbors(node) {
                if remaining.contains(&neighbor) {
                    if let Some(&idx) = node_to_idx.get(&neighbor) {
                        votes[idx] -= 1.0;
                    }
                }
            }
        }
    }
    selected
}

/// Laplacian centrality: based on the Laplacian matrix of the graph.
///
/// # Arguments
///
/// * `graph`: the targeted graph.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing Laplacian centralities of each node in the graph.
pub fn laplacian_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Result<NodeMap<f64>>
where
    W: Copy + PartialOrd + Into<f64>,
    Ty: GraphConstructor<A, W>,
{
    let mut centrality = NodeMap::new();
    for (node, _) in graph.nodes() {
        let degree = graph.neighbors(node).count() as f64;
        let mut sum = degree * degree;
        for neighbor in graph.neighbors(node) {
            sum += graph.neighbors(neighbor).count() as f64;
        }
        centrality.insert(node, sum);
    }
    Ok(centrality)
}

/// Decay centrality: measures a node's importance by summing the discounted influence of all other nodes in a network,
/// where influence decreases ("decays") with distance
///
/// # Arguments
///
/// * `graph`: the targeted graph.
/// * `decay`: the decay rate.
/// * `influence`: the influence of the graph.
///
/// # Returns
///
/// [`NodeMap`] of `f64` representing Decay centralities of each node in the graph.
pub fn decay_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>, decay: f64) -> Result<NodeMap<f64>>
where
    A: std::fmt::Debug,
    W: Copy
        + PartialOrd
        + Into<f64>
        + std::ops::Add<Output = W>
        + std::ops::Sub<Output = W>
        + From<u8>
        + Ord
        + std::fmt::Debug,
    Ty: GraphConstructor<A, W>,
    BaseGraph<A, W, Ty>: GraphinaGraph<A, W>,
{
    let mut centrality = graph.to_nodemap_default();

    for (i_node, _) in graph.nodes() {
        let dij = dijkstra_path_impl(graph, i_node, None, |e| Some(<W as Into<f64>>::into(*e)))?.0;
        for (j_node, _) in graph.nodes() {
            if i_node == j_node {
                continue;
            }
            if let Some(dist) = dij.get(&j_node).unwrap() {
                let dist: f64 = (*dist).into();
                *centrality.get_mut(&j_node).unwrap() += decay.powf(dist);
            }
        }
    }
    Ok(centrality)
}

#[cfg(test)]
mod tests {
    use super::decay_centrality;
    use crate::core::types::Graph;
    use ordered_float::OrderedFloat;

    #[test]
    fn test_decay_centrality() {
        let mut g: Graph<i32, OrderedFloat<f64>> = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);
        let n5 = g.add_node(5);
        let n6 = g.add_node(6);
        let n7 = g.add_node(7);

        g.add_edge(n1, n2, OrderedFloat(1.0));
        g.add_edge(n2, n3, OrderedFloat(1.0));
        g.add_edge(n3, n1, OrderedFloat(1.0));

        g.add_edge(n3, n4, OrderedFloat(1.0));
        g.add_edge(n4, n5, OrderedFloat(1.0));

        g.add_edge(n5, n6, OrderedFloat(1.0));
        g.add_edge(n6, n7, OrderedFloat(1.0));
        g.add_edge(n7, n5, OrderedFloat(1.0));

        {
            let c = decay_centrality(&g, 0.5).unwrap();
            assert!((*c.get(&n1).unwrap() - 1.5).abs() < 0.1);
            assert!((*c.get(&n3).unwrap() - 2.0).abs() < 0.1);
            assert!((*c.get(&n4).unwrap() - 2.0).abs() < 0.1);
        }
        {
            let c = decay_centrality(&g, 0.75).unwrap();
            assert!((*c.get(&n1).unwrap() - 3.1).abs() < 0.1);
            assert!((*c.get(&n3).unwrap() - 3.7).abs() < 0.1);
            assert!((*c.get(&n4).unwrap() - 3.8).abs() < 0.1);
        }
        {
            let c = decay_centrality(&g, 0.25).unwrap();
            assert!((*c.get(&n1).unwrap() - 0.59).abs() < 0.01);
            assert!((*c.get(&n3).unwrap() - 0.84).abs() < 0.01);
            assert!((*c.get(&n4).unwrap() - 0.75).abs() < 0.01);
        }
    }
}
