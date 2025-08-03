use graphina::core::paths::{a_star, bellman_ford, dijkstra, floyd_warshall, ida_star, johnson};
use graphina::core::types::{Digraph, NodeId};
use ordered_float::OrderedFloat;
use std::collections::HashMap;

fn build_test_graph_ordered() -> (Digraph<i32, OrderedFloat<f64>>, HashMap<i32, NodeId>) {
    let mut graph: Digraph<i32, OrderedFloat<f64>> = Digraph::default();
    let mut nodes = HashMap::new();
    nodes.insert(0, graph.add_node(0));
    nodes.insert(1, graph.add_node(1));
    nodes.insert(2, graph.add_node(2));
    nodes.insert(3, graph.add_node(3));
    graph.add_edge(nodes[&0], nodes[&1], OrderedFloat(1.0));
    graph.add_edge(nodes[&0], nodes[&2], OrderedFloat(4.0));
    graph.add_edge(nodes[&1], nodes[&2], OrderedFloat(2.0));
    graph.add_edge(nodes[&1], nodes[&3], OrderedFloat(6.0));
    graph.add_edge(nodes[&2], nodes[&3], OrderedFloat(3.0));
    (graph, nodes)
}

fn build_test_graph_f64() -> (Digraph<i32, f64>, HashMap<i32, NodeId>) {
    let mut graph: Digraph<i32, f64> = Digraph::default();
    let mut nodes = HashMap::new();
    nodes.insert(0, graph.add_node(0));
    nodes.insert(1, graph.add_node(1));
    nodes.insert(2, graph.add_node(2));
    nodes.insert(3, graph.add_node(3));
    graph.add_edge(nodes[&0], nodes[&1], 1.0);
    graph.add_edge(nodes[&0], nodes[&2], 4.0);
    graph.add_edge(nodes[&1], nodes[&2], 2.0);
    graph.add_edge(nodes[&1], nodes[&3], 6.0);
    graph.add_edge(nodes[&2], nodes[&3], 3.0);
    (graph, nodes)
}

#[test]
fn test_dijkstra_directed() {
    let (graph, nodes) = build_test_graph_ordered();
    let n0 = nodes[&0];
    let n3 = nodes[&3];
    let dist = dijkstra(&graph, n0).unwrap();
    assert_eq!(dist[n3.index()], Some(OrderedFloat(6.0)));
}

#[test]
fn test_bellman_ford_directed() {
    let (graph, nodes) = build_test_graph_ordered();
    let n0 = nodes[&0];
    let n3 = nodes[&3];
    let dist = bellman_ford(&graph, n0).expect("No negative cycle");
    assert_eq!(dist[n3.index()], Some(OrderedFloat(6.0)));
}

#[test]
fn test_a_star_directed() {
    let (graph, nodes) = build_test_graph_ordered();
    let n0 = nodes[&0];
    let n1 = nodes[&1];
    let n2 = nodes[&2];
    let n3 = nodes[&3];
    let result = a_star(&graph, n0, n3, |_| OrderedFloat(0.0));
    assert!(result.is_ok());
    let path_opt = result.unwrap();
    assert!(path_opt.is_some());
    let (cost, path) = path_opt.unwrap();
    assert_eq!(cost, OrderedFloat(6.0));
    assert_eq!(path, vec![n0, n1, n2, n3]);
}

#[test]
fn test_floyd_warshall_directed() {
    let (graph, nodes) = build_test_graph_ordered();
    let n0 = nodes[&0];
    let n3 = nodes[&3];
    let matrix = floyd_warshall(&graph).expect("No negative cycle");
    assert_eq!(matrix[n0.index()][n3.index()], Some(OrderedFloat(6.0)));
}

#[test]
fn test_johnson_directed() {
    let (graph, nodes) = build_test_graph_ordered();
    let n0 = nodes[&0];
    let n3 = nodes[&3];
    let matrix = johnson(&graph).expect("No negative cycle");
    assert_eq!(matrix[n0.index()][n3.index()], Some(OrderedFloat(6.0)));
}

#[test]
fn test_ida_star_directed() {
    let (graph, nodes) = build_test_graph_f64();
    let n0 = nodes[&0];
    let n1 = nodes[&1];
    let n2 = nodes[&2];
    let n3 = nodes[&3];
    let result = ida_star(&graph, n0, n3, |_| 0.0);
    assert!(result.is_ok());
    let path_opt = result.unwrap();
    assert!(path_opt.is_some());
    let (cost, path) = path_opt.unwrap();
    assert_eq!(cost, 6.0);
    assert_eq!(path, vec![n0, n1, n2, n3]);
}
