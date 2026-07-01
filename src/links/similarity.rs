//! Similarity-based link prediction algorithms.
//!
//! Similarity-based link prediction algorithms.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::HashSet;

/// Neighbor set with `rustc_hash`'s fast hasher; `NodeId` keys do not need
/// SipHash's DoS resistance, and the default hasher dominated the per-pair cost.
type NodeSet = HashSet<NodeId, rustc_hash::FxBuildHasher>;

/// Helper: If no ebunch is provided, generate all unordered pairs of nodes.
fn default_ebunch<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Vec<(NodeId, NodeId)>
where
    Ty: crate::core::types::GraphConstructor<A, W>,
{
    let nodes: Vec<NodeId> = graph.nodes().map(|(u, _)| u).collect();
    let mut ebunch = Vec::new();
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            ebunch.push((nodes[i], nodes[j]));
        }
    }
    ebunch
}

/// Jaccard Coefficient
/// For each pair (u, v), Jaccard = |N(u) ∩ N(v)| / |N(u) ∪ N(v)|
pub fn jaccard_coefficient<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    ebunch: Option<&[(NodeId, NodeId)]>,
) -> Vec<((NodeId, NodeId), f64)>
where
    Ty: GraphConstructor<A, f64>,
{
    let pairs = match ebunch {
        Some(p) => p.to_vec(),
        None => default_ebunch(graph),
    };
    let mut results = Vec::with_capacity(pairs.len());
    for (u, v) in pairs {
        let set_u: NodeSet = graph.neighbors(u).collect();
        let set_v: NodeSet = graph.neighbors(v).collect();
        let intersection = set_u.intersection(&set_v).count();
        // |A ∪ B| = |A| + |B| - |A ∩ B|, so the union size needs no second pass.
        let union = set_u.len() + set_v.len() - intersection;
        let score = if union > 0 {
            intersection as f64 / union as f64
        } else {
            0.0
        };
        results.push(((u, v), score));
    }
    results
}

/// Adamic–Adar Index
/// For each pair (u, v), AA = sum_{w in N(u) ∩ N(v)} (1 / log(degree(w)))
pub fn adamic_adar_index<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    ebunch: Option<&[(NodeId, NodeId)]>,
) -> Vec<((NodeId, NodeId), f64)>
where
    Ty: GraphConstructor<A, f64>,
{
    let pairs = match ebunch {
        Some(p) => p.to_vec(),
        None => default_ebunch(graph),
    };
    let mut results = Vec::with_capacity(pairs.len());
    for (u, v) in pairs {
        let set_u: NodeSet = graph.neighbors(u).collect();
        let set_v: NodeSet = graph.neighbors(v).collect();
        // Sum over the common neighbors directly, without collecting them first.
        let score: f64 = set_u
            .intersection(&set_v)
            .filter_map(|w| {
                let deg = graph.neighbors(*w).count();
                if deg > 1 {
                    Some(1.0 / (deg as f64).ln())
                } else {
                    None
                }
            })
            .sum();
        results.push(((u, v), score));
    }
    results
}

/// Common Neighbors
/// For a pair (u, v), returns the number of common neighbors.
pub fn common_neighbors<A, W, Ty>(graph: &BaseGraph<A, W, Ty>, u: NodeId, v: NodeId) -> usize
where
    Ty: GraphConstructor<A, W>,
{
    let set_u: NodeSet = graph.neighbors(u).collect();
    let set_v: NodeSet = graph.neighbors(v).collect();
    set_u.intersection(&set_v).count()
}

#[cfg(test)]
mod tests {
    use super::{adamic_adar_index, common_neighbors, jaccard_coefficient};
    use crate::core::types::Graph;

    #[test]
    fn test_jaccard_coefficient() {
        let mut graph = Graph::<i32, f64>::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n1, n3, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n4, 1.0);
        let results = jaccard_coefficient(&graph, Some(&[(n1, n2)]));
        let score = results[0].1;
        assert!((0.0..=1.0).contains(&score));
    }

    #[test]
    fn test_jaccard_coefficient_exact() {
        // N(n1) = {n2, n3}, N(n2) = {n1, n3}: intersection {n3} (1), union
        // {n1, n2, n3} (3), so the coefficient is exactly 1/3.
        let mut graph = Graph::<i32, f64>::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n1, n3, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n4, 1.0);
        let results = jaccard_coefficient(&graph, Some(&[(n1, n2)]));
        assert!((results[0].1 - 1.0 / 3.0).abs() < 1e-12);
    }

    #[test]
    fn test_adamic_adar_exact() {
        // The only common neighbor of n1 and n2 is n3, whose degree is 3
        // (n1, n2, n4), so the Adamic-Adar index is exactly 1 / ln(3).
        let mut graph = Graph::<i32, f64>::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n1, n3, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n4, 1.0);
        let results = adamic_adar_index(&graph, Some(&[(n1, n2)]));
        assert!((results[0].1 - 1.0 / 3.0_f64.ln()).abs() < 1e-12);
    }

    #[test]
    fn test_common_neighbors() {
        let mut graph = Graph::<i32, ()>::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n3, ());
        graph.add_edge(n2, n3, ());
        let count = common_neighbors(&graph, n1, n2);
        assert_eq!(count, 1);
    }
}
