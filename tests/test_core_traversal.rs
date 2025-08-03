use graphina::core::traversal::{bfs, bidis, dfs, iddfs};
use graphina::core::types::{Digraph, Graph, NodeId};
use std::collections::HashMap;

fn build_undirected_graph() -> (Graph<i32, f32>, HashMap<i32, NodeId>) {
    let mut g = Graph::<i32, f32>::new();
    let mut nodes = HashMap::new();
    nodes.insert(1, g.add_node(1));
    nodes.insert(2, g.add_node(2));
    nodes.insert(3, g.add_node(3));
    nodes.insert(4, g.add_node(4));
    g.add_edge(nodes[&1], nodes[&2], 1.0);
    g.add_edge(nodes[&1], nodes[&3], 1.0);
    g.add_edge(nodes[&2], nodes[&3], 1.0);
    g.add_edge(nodes[&3], nodes[&4], 1.0);
    (g, nodes)
}

fn build_directed_graph() -> (Digraph<i32, f32>, HashMap<i32, NodeId>) {
    let mut g = Digraph::<i32, f32>::new();
    let mut nodes = HashMap::new();
    nodes.insert(1, g.add_node(1));
    nodes.insert(2, g.add_node(2));
    nodes.insert(3, g.add_node(3));
    nodes.insert(4, g.add_node(4));
    g.add_edge(nodes[&1], nodes[&2], 1.0);
    g.add_edge(nodes[&1], nodes[&3], 1.0);
    g.add_edge(nodes[&2], nodes[&4], 1.0);
    g.add_edge(nodes[&3], nodes[&4], 1.0);
    (g, nodes)
}

#[test]
fn test_bfs() {
    let (graph, nodes) = build_undirected_graph();
    let start = nodes[&1];
    let order = bfs(&graph, start);
    let visited: Vec<i32> = order
        .iter()
        .filter_map(|nid| graph.node_attr(*nid))
        .cloned()
        .collect();
    for v in [1, 2, 3, 4] {
        assert!(visited.contains(&v), "BFS did not visit node {}", v);
    }
}

#[test]
fn test_dfs() {
    let (graph, nodes) = build_undirected_graph();
    let start = nodes[&1];
    let order = dfs(&graph, start);
    let visited: Vec<i32> = order
        .iter()
        .filter_map(|nid| graph.node_attr(*nid))
        .cloned()
        .collect();
    for v in [1, 2, 3, 4] {
        assert!(visited.contains(&v), "DFS did not visit node {}", v);
    }
}

#[test]
fn test_iddfs() {
    let (graph, nodes) = build_directed_graph();
    let start = nodes[&1];
    let target = nodes[&4];
    let path_opt = iddfs(&graph, start, target, 3);
    assert!(path_opt.is_some(), "IDDFS did not find a path");
    let path = path_opt.unwrap();
    assert_eq!(path.first(), Some(&start));
    assert_eq!(path.last(), Some(&target));
}

#[test]
fn test_bidirectional_search() {
    let (graph, nodes) = build_directed_graph();
    let start = nodes[&1];
    let target = nodes[&4];
    let path_opt = bidis(&graph, start, target);
    assert!(
        path_opt.is_some(),
        "Bidirectional search did not find a path"
    );
    let path = path_opt.unwrap();
    assert_eq!(path.first(), Some(&start));
    assert_eq!(path.last(), Some(&target));
}
