//! Personalized PageRank centrality facade.
//!
//! Provides a NodeMap-based personalized PageRank interface consistent with other centrality
//! routines, wrapping the community implementation. Use `personalized_pagerank_vec` if you need
//! the raw contiguous vector aligned to internal node ordering.

use crate::community::personalized_pagerank::personalized_page_rank;
use crate::core::error::Result;
use crate::core::types::{BaseGraph, GraphConstructor, NodeId, NodeMap};
use petgraph::EdgeType;

/// Compute personalized PageRank returning a NodeMap<NodeId, f64> for consistency.
///
/// Errors on invalid parameters or empty graph.
pub fn personalized_pagerank<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    personalization: Option<Vec<f64>>,
    damping: f64,
    tol: f64,
    max_iter: usize,
) -> Result<NodeMap<f64>>
where
    W: Copy + PartialOrd + Into<f64> + From<u8>,
    Ty: GraphConstructor<A, W> + EdgeType,
{
    // Build stable node list to map ranks
    let node_list: Vec<NodeId> = graph.nodes().map(|(nid, _)| nid).collect();
    let ranks_vec = personalized_page_rank(graph, personalization, damping, tol, max_iter)?;
    // Safety: personalized_page_rank returns rank vector sized to node_list.len()
    let mut map = NodeMap::new();
    for (i, nid) in node_list.iter().enumerate() {
        if let Some(val) = ranks_vec.get(i) {
            map.insert(*nid, *val);
        }
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_personalized_pagerank_map_basic() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n1, n2, 1.0);
        let pr = personalized_pagerank(&g, None, 0.85, 1e-6, 50).unwrap();
        assert_eq!(pr.len(), 2);
        assert!(pr[&n1] > 0.0);
        assert!(pr[&n2] > 0.0);
    }

    #[test]
    fn test_personalized_pagerank_map_with_personalization() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n1, n2, 1.0);
        let pr = personalized_pagerank(&g, Some(vec![2.0, 1.0]), 0.85, 1e-6, 50).unwrap();
        // Node with higher personalization weight should have higher rank.
        assert!(pr[&n1] > pr[&n2]);
    }
}
