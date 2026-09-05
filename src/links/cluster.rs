//! Within-inter cluster link prediction algorithms.
//!
//! Cluster-based link prediction algorithms.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::HashSet;

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

/// Within-Inter Cluster ratio (Valverde-Rebaza and de Andrade Lopes), as in NetworkX.
///
/// For a pair (u, v) in the same community, the score is
/// `within / (inter + delta)`, where `within` counts the common neighbors in that
/// community and `inter` the common neighbors outside it. A pair whose endpoints
/// are in different communities scores `0.0`. `delta` must be positive; it keeps
/// the score finite when there are no inter-cluster common neighbors.
pub fn within_inter_cluster<A, Ty, F, C>(
    graph: &BaseGraph<A, f64, Ty>,
    ebunch: Option<&[(NodeId, NodeId)]>,
    community: F,
    delta: f64,
) -> Vec<((NodeId, NodeId), f64)>
where
    Ty: GraphConstructor<A, f64>,
    F: Fn(NodeId) -> C,
    C: Eq,
{
    let pairs = match ebunch {
        Some(p) => p.to_vec(),
        None => default_ebunch(graph),
    };
    let mut results = Vec::with_capacity(pairs.len());
    for (u, v) in pairs {
        let cu = community(u);
        if cu != community(v) {
            results.push(((u, v), 0.0));
            continue;
        }
        let set_v: HashSet<NodeId> = graph.neighbors(v).collect();
        let mut within = 0usize;
        let mut inter = 0usize;
        for w in graph.neighbors(u).filter(|w| set_v.contains(w)) {
            if community(w) == cu {
                within += 1;
            } else {
                inter += 1;
            }
        }
        results.push(((u, v), within as f64 / (inter as f64 + delta)));
    }
    results
}

#[cfg(test)]
mod tests {
    use super::within_inter_cluster;
    use crate::core::types::Graph;

    #[test]
    fn within_inter_cluster_matches_networkx() {
        // Triangle 0-1-2 plus the path 2-3-4. Nodes 0..3 are community 0 and
        // nodes 3 and 4 community 1. NetworkX with delta = 0.5 gives
        // (0, 2) -> 1 / (0 + 0.5) = 2 and 0 for every pair that spans communities.
        let mut g = Graph::<i32, f64>::new();
        let n: Vec<_> = (0..5).map(|i| g.add_node(i)).collect();
        for (u, v) in [(0, 1), (1, 2), (0, 2), (2, 3), (3, 4)] {
            g.add_edge(n[u], n[v], 1.0);
        }
        let community = |node: crate::core::types::NodeId| if node.index() < 3 { 0 } else { 1 };
        let pairs = [(n[0], n[2]), (n[1], n[3]), (n[2], n[4]), (n[0], n[3])];
        let scores = within_inter_cluster(&g, Some(&pairs), community, 0.5);
        assert_eq!(scores[0].1, 2.0);
        assert_eq!(scores[1].1, 0.0);
        assert_eq!(scores[2].1, 0.0);
        assert_eq!(scores[3].1, 0.0);
    }

    #[test]
    fn within_inter_cluster_counts_inter_neighbors_in_denominator_only() {
        // a and b share three neighbors: w0 in their community and w1, w2 outside
        // it, so the score is 1 / (2 + delta).
        let mut g = Graph::<i32, f64>::new();
        let a = g.add_node(0);
        let b = g.add_node(1);
        let w0 = g.add_node(2);
        let w1 = g.add_node(3);
        let w2 = g.add_node(4);
        for w in [w0, w1, w2] {
            g.add_edge(a, w, 1.0);
            g.add_edge(b, w, 1.0);
        }
        let community = |node: crate::core::types::NodeId| if node.index() < 3 { 0 } else { 1 };
        let scores = within_inter_cluster(&g, Some(&[(a, b)]), community, 0.5);
        assert!((scores[0].1 - 0.4).abs() < 1e-12, "got {}", scores[0].1);
    }
}
