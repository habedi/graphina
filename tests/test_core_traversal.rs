use graphina::core::traversal::{bfs, bidis, dfs, iddfs};
use graphina::core::types::{Digraph, Graph};

/// Helper function to build a simple undirected graph:
///
/// Graph structure:
/// 1 -- 2
/// |  /
/// 3 -- 4
fn build_undirected_graph() -> Graph<i32, f32> {
    let mut g = Graph::<i32, f32>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    let n4 = g.add_node(4);
    g.add_edge(n1, n2, 1.0);
    g.add_edge(n1, n3, 1.0);
    g.add_edge(n2, n3, 1.0);
    g.add_edge(n3, n4, 1.0);
    g
}

/// Helper function to build a simple directed graph:
///
/// Graph structure:
/// 1 -> 2 -> 4
///  \
///   -> 3 -^
fn build_directed_graph() -> Digraph<i32, f32> {
    let mut g = Digraph::<i32, f32>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    let n4 = g.add_node(4);
    g.add_edge(n1, n2, 1.0);
    g.add_edge(n1, n3, 1.0);
    g.add_edge(n2, n4, 1.0);
    g.add_edge(n3, n4, 1.0);
    g
}

#[test]
fn test_bfs() {
    let graph = build_undirected_graph();
    // Start at node with value 1. (We know it was the first added.)
    let start = graph.nodes().next().unwrap().0;
    let order = bfs(&graph, start);
    // In an undirected graph, BFS starting at node 1 should at least visit all nodes.
    let visited: Vec<i32> = order
        .iter()
        .filter_map(|nid| graph.node_attr(*nid))
        .cloned()
        .collect();
    // Check that all expected node values are present.
    for v in [1, 2, 3, 4] {
        assert!(visited.contains(&v), "BFS did not visit node {}", v);
    }
}

#[test]
fn test_dfs() {
    let graph = build_undirected_graph();
    let start = graph.nodes().next().unwrap().0;
    let order = dfs(&graph, start);
    // Ensure that DFS visits all nodes.
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
    let graph = build_directed_graph();
    // Try to find a path from 1 to 4 with max_depth 3.
    let start = graph.nodes().find(|(_nid, &val)| val == 1).unwrap().0;
    let target = graph.nodes().find(|(_nid, &val)| val == 4).unwrap().0;
    let path_opt = iddfs(&graph, start, target, 3);
    assert!(path_opt.is_some(), "IDDFS did not find a path");
    let path = path_opt.unwrap();
    // Validate that the path starts with 1 and ends with 4.
    assert_eq!(graph.node_attr(path.first().cloned().unwrap()), Some(&1));
    assert_eq!(graph.node_attr(path.last().cloned().unwrap()), Some(&4));
}

#[test]
fn test_bidirectional_search() {
    let graph = build_directed_graph();
    // Bidirectional search on directed graphs: find path from 1 to 4.
    let start = graph.nodes().find(|(_, &val)| val == 1).unwrap().0;
    let target = graph.nodes().find(|(_, &val)| val == 4).unwrap().0;
    let path_opt = bidis(&graph, start, target);
    assert!(
        path_opt.is_some(),
        "Bidirectional search did not find a path"
    );
    let path = path_opt.unwrap();
    // Check that the path starts with 1 and ends with 4.
    assert_eq!(graph.node_attr(path.first().cloned().unwrap()), Some(&1));
    assert_eq!(graph.node_attr(path.last().cloned().unwrap()), Some(&4));
    // Optionally, print the path for visual inspection.
    let values: Vec<_> = path
        .iter()
        .filter_map(|nid| graph.node_attr(*nid))
        .cloned()
        .collect();
    println!("Bidirectional search path: {:?}", values);
}
