//! Degree centrality algorithms.
//!
//! This module provides degree centrality measures.
//!
//! Convention: returns `Result<_, crate::core::error::GraphinaError>` for consistency
//! and better observability.

use crate::core::error::Result;
use crate::core::types::{BaseGraph, GraphConstructor, NodeId, NodeMap};

/// Builds a degree map by asking the graph for each node's degree directly, so the
/// work is a single pass over the nodes writing into the Fx-hashed result rather
/// than an intermediate `std` `HashMap` populated by scanning every edge.
///
/// petgraph counts an undirected self-loop as one incident edge, but the degree
/// convention here counts it as two, so the undirected paths add a correction of
/// one per self-loop. Directed degrees already count a self-loop as two (one in and
/// one out), so no correction is needed there.
fn degree_map<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    node_degree: impl Fn(NodeId) -> usize,
    correct_undirected_self_loops: bool,
) -> NodeMap<f64>
where
    Ty: GraphConstructor<A, W>,
{
    let mut centrality: NodeMap<f64> =
        NodeMap::with_capacity_and_hasher(graph.node_count(), Default::default());
    for node in graph.node_ids() {
        centrality.insert(node, node_degree(node) as f64);
    }
    if correct_undirected_self_loops && !graph.is_directed() {
        for (u, v, _w) in graph.edges() {
            if u == v {
                if let Some(d) = centrality.get_mut(&u) {
                    *d += 1.0;
                }
            }
        }
    }
    centrality
}

/// Degree centrality: number of incident edges (for directed, in + out).
///
/// Behavior and conventions:
/// - Returns raw counts (not normalized).
/// - Directed graphs: degree = in-degree + out-degree.
/// - Undirected graphs: counts each incident edge once; a self-loop counts as 2.
pub fn degree_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Result<NodeMap<f64>>
where
    Ty: GraphConstructor<A, W>,
{
    Ok(degree_map(
        graph,
        |node| graph.degree(node).unwrap_or(0),
        true,
    ))
}

/// In-degree centrality: number of incoming edges (raw count).
///
/// Behavior and conventions:
/// - Directed graphs: counts only incoming edges.
/// - Undirected graphs: equal to total degree; a self-loop counts as 2.
pub fn in_degree_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Result<NodeMap<f64>>
where
    Ty: GraphConstructor<A, W>,
{
    // On undirected graphs in-degree equals total degree, so the self-loop
    // correction applies; on directed graphs incoming self-loops already count once.
    Ok(degree_map(
        graph,
        |node| graph.in_degree(node).unwrap_or(0),
        true,
    ))
}

/// Out-degree centrality: number of outgoing edges (raw count).
///
/// Behavior and conventions:
/// - Directed graphs: counts only outgoing edges.
/// - Undirected graphs: equal to total degree; a self-loop counts as 2.
pub fn out_degree_centrality<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> Result<NodeMap<f64>>
where
    Ty: GraphConstructor<A, W>,
{
    // On undirected graphs out-degree equals total degree, so the self-loop
    // correction applies; on directed graphs outgoing self-loops already count once.
    Ok(degree_map(
        graph,
        |node| graph.out_degree(node).unwrap_or(0),
        true,
    ))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_centrality_empty_graph() {
        use crate::centrality::degree::degree_centrality;
        use crate::core::types::Graph;

        let g = Graph::<i32, f64>::new();
        let result = degree_centrality(&g);

        assert!(result.is_ok());
        let centrality = result.unwrap();
        assert_eq!(centrality.len(), 0);
    }
    use super::*;
    use crate::core::types::{Digraph, Graph};
    #[test]
    fn test_self_loop_directed() {
        let mut g = Digraph::<i32, f64>::new();
        let n = g.add_node(1);
        g.add_edge(n, n, 1.0);
        let d = degree_centrality(&g).unwrap();
        let indeg = in_degree_centrality(&g).unwrap();
        let outdeg = out_degree_centrality(&g).unwrap();
        assert_eq!(d[&n], 2.0);
        assert_eq!(indeg[&n], 1.0);
        assert_eq!(outdeg[&n], 1.0);
    }
    #[test]
    fn test_isolated_and_mixed_degrees_undirected() {
        // A path 0-1-2 plus an isolated node 3: degrees are 1, 2, 1, and 0.
        let mut g = Graph::<i32, f64>::new();
        let a = g.add_node(0);
        let b = g.add_node(1);
        let c = g.add_node(2);
        let iso = g.add_node(3);
        g.add_edge(a, b, 1.0);
        g.add_edge(b, c, 1.0);
        let d = degree_centrality(&g).unwrap();
        assert_eq!(d[&a], 1.0);
        assert_eq!(d[&b], 2.0);
        assert_eq!(d[&c], 1.0);
        assert_eq!(d[&iso], 0.0);
        assert_eq!(d.len(), 4);
    }
    #[test]
    fn test_isolated_and_mixed_degrees_directed() {
        // 0->1, 1->2, plus isolated node 3.
        let mut g = Digraph::<i32, f64>::new();
        let a = g.add_node(0);
        let b = g.add_node(1);
        let c = g.add_node(2);
        let iso = g.add_node(3);
        g.add_edge(a, b, 1.0);
        g.add_edge(b, c, 1.0);
        let deg = degree_centrality(&g).unwrap();
        let indeg = in_degree_centrality(&g).unwrap();
        let outdeg = out_degree_centrality(&g).unwrap();
        assert_eq!(deg[&b], 2.0); // one in, one out
        assert_eq!(indeg[&a], 0.0);
        assert_eq!(outdeg[&a], 1.0);
        assert_eq!(indeg[&c], 1.0);
        assert_eq!(outdeg[&c], 0.0);
        assert_eq!(deg[&iso], 0.0);
    }
    #[test]
    fn test_self_loop_undirected() {
        let mut g = Graph::<i32, f64>::new();
        let n = g.add_node(1);
        g.add_edge(n, n, 1.0);
        let d = degree_centrality(&g).unwrap();
        let indeg = in_degree_centrality(&g).unwrap();
        let outdeg = out_degree_centrality(&g).unwrap();
        assert_eq!(d[&n], 2.0);
        assert_eq!(indeg[&n], 2.0);
        assert_eq!(outdeg[&n], 2.0);
    }
}
