// tests/paths_algorithms.rs

use graphina::core::paths::{a_star, bellman_ford, dijkstra, floyd_warshall, ida_star, johnson};
use graphina::core::types::{Digraph, NodeId};
use ordered_float::OrderedFloat;

/// Helper: builds a test graph with OrderedFloat<f64> weights.
/// Nodes: 0, 1, 2, 3
/// Edges:
///   0 -> 1 with weight 1.0
///   0 -> 2 with weight 4.0
///   1 -> 2 with weight 2.0
///   1 -> 3 with weight 6.0
///   2 -> 3 with weight 3.0
fn build_test_graph_ordered() -> Digraph<i32, OrderedFloat<f64>> {
    let mut graph: Digraph<i32, OrderedFloat<f64>> = Digraph::default();
    let n0 = graph.add_node(0);
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    graph.add_edge(n0, n1, OrderedFloat(1.0));
    graph.add_edge(n0, n2, OrderedFloat(4.0));
    graph.add_edge(n1, n2, OrderedFloat(2.0));
    graph.add_edge(n1, n3, OrderedFloat(6.0));
    graph.add_edge(n2, n3, OrderedFloat(3.0));
    graph
}

/// Helper: builds a test graph with f64 weights (for IDA*).
fn build_test_graph_f64() -> Digraph<i32, f64> {
    let mut graph: Digraph<i32, f64> = Digraph::default();
    let n0 = graph.add_node(0);
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    graph.add_edge(n0, n1, 1.0);
    graph.add_edge(n0, n2, 4.0);
    graph.add_edge(n1, n2, 2.0);
    graph.add_edge(n1, n3, 6.0);
    graph.add_edge(n2, n3, 3.0);
    graph
}

#[test]
fn test_dijkstra_directed() {
    let graph = build_test_graph_ordered();
    let n0 = graph.nodes().find(|(node, _)| node.index() == 0).unwrap().0;
    let n3 = graph.nodes().find(|(node, _)| node.index() == 3).unwrap().0;
    let dist = dijkstra(&graph, n0);
    // Expected shortest distance from node 0 to 3: 1.0 + 2.0 + 3.0 = 6.0.
    assert_eq!(dist[n3.index()], Some(OrderedFloat(6.0)));
}

#[test]
fn test_bellman_ford_directed() {
    let graph = build_test_graph_ordered();
    let n0 = graph.nodes().find(|(node, _)| node.index() == 0).unwrap().0;
    let n3 = graph.nodes().find(|(node, _)| node.index() == 3).unwrap().0;
    let dist = bellman_ford(&graph, n0).expect("No negative cycle");
    assert_eq!(dist[n3.index()], Some(OrderedFloat(6.0)));
}

#[test]
fn test_a_star_directed() {
    let graph = build_test_graph_ordered();
    let n0 = graph.nodes().find(|(node, _)| node.index() == 0).unwrap().0;
    let n3 = graph.nodes().find(|(node, _)| node.index() == 3).unwrap().0;
    // Zero heuristic for simplicity.
    let result = a_star(&graph, n0, n3, |_| OrderedFloat(0.0));
    assert!(result.is_some());
    let (cost, path) = result.unwrap();
    assert_eq!(cost, OrderedFloat(6.0));
    // Expected path: [n0, n1, n2, n3]
    let expected: Vec<NodeId> = graph.nodes().map(|(node, _)| node).collect();
    assert_eq!(path, vec![n0, expected[1], expected[2], n3]);
}

#[test]
fn test_floyd_warshall_directed() {
    let graph = build_test_graph_ordered();
    let n0 = graph.nodes().find(|(node, _)| node.index() == 0).unwrap().0;
    let n3 = graph.nodes().find(|(node, _)| node.index() == 3).unwrap().0;
    let matrix = floyd_warshall(&graph).expect("No negative cycle");
    assert_eq!(matrix[n0.index()][n3.index()], Some(OrderedFloat(6.0)));
}

#[test]
fn test_johnson_directed() {
    let graph = build_test_graph_ordered();
    let n0 = graph.nodes().find(|(node, _)| node.index() == 0).unwrap().0;
    let n3 = graph.nodes().find(|(node, _)| node.index() == 3).unwrap().0;
    let matrix = johnson(&graph).expect("No negative cycle");
    assert_eq!(matrix[n0.index()][n3.index()], Some(OrderedFloat(6.0)));
}

#[test]
fn test_ida_star_directed() {
    // For IDA*, we use a graph with f64 weights.
    let graph = build_test_graph_f64();
    let n0 = graph.nodes().find(|(node, _)| node.index() == 0).unwrap().0;
    let n3 = graph.nodes().find(|(node, _)| node.index() == 3).unwrap().0;
    // Use a zero heuristic.
    let result = ida_star(&graph, n0, n3, |_| 0.0);
    assert!(result.is_some());
    let (cost, path) = result.unwrap();
    assert_eq!(cost, 6.0);
    let expected: Vec<NodeId> = graph.nodes().map(|(node, _)| node).collect();
    assert_eq!(path, vec![n0, expected[1], expected[2], n3]);
}
