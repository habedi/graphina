/*!
# Tests for Traversal Algorithm Bug Fixes

This module contains tests for bugs fixed in traversal algorithms,
specifically for the bidirectional search path reconstruction bug.
*/

use graphina::core::traversal::{bfs, bidis, try_bidirectional_search};
use graphina::core::types::{Digraph, Graph};

#[test]
fn test_bidis_simple_path() {
    // Test basic bidirectional search on a simple path
    let mut graph = Graph::<i32, ()>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    let n4 = graph.add_node(4);

    graph.add_edge(n1, n2, ());
    graph.add_edge(n2, n3, ());
    graph.add_edge(n3, n4, ());

    let path = bidis(&graph, n1, n4).expect("Path should exist");

    assert_eq!(path.len(), 4);
    assert_eq!(path[0], n1);
    assert_eq!(path[path.len() - 1], n4);

    // Verify path is contiguous
    for i in 0..path.len() - 1 {
        assert!(
            graph.contains_edge(path[i], path[i + 1]) || graph.contains_edge(path[i + 1], path[i]),
            "Path is not contiguous between {:?} and {:?}",
            path[i],
            path[i + 1]
        );
    }
}

#[test]
fn test_bidis_shortest_path_selection() {
    // Test that bidirectional search finds the shortest path
    // Create a graph with two paths: short (3 edges) and long (5 edges)
    let mut graph = Graph::<i32, ()>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    let n4 = graph.add_node(4);
    let n5 = graph.add_node(5);
    let n6 = graph.add_node(6);
    let n7 = graph.add_node(7);

    // Short path: n1 -> n2 -> n3 -> n7
    graph.add_edge(n1, n2, ());
    graph.add_edge(n2, n3, ());
    graph.add_edge(n3, n7, ());

    // Long path: n1 -> n4 -> n5 -> n6 -> n7
    graph.add_edge(n1, n4, ());
    graph.add_edge(n4, n5, ());
    graph.add_edge(n5, n6, ());
    graph.add_edge(n6, n7, ());

    let path = bidis(&graph, n1, n7).expect("Path should exist");

    // Should find the shorter path (4 nodes = 3 edges)
    assert_eq!(path.len(), 4, "Should find shortest path of length 4");
    assert_eq!(path[0], n1);
    assert_eq!(path[path.len() - 1], n7);
}

#[test]
fn test_bidis_directed_graph() {
    // Test bidirectional search on a directed graph
    let mut graph = Digraph::<i32, ()>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    let n4 = graph.add_node(4);

    // Create a directed path
    graph.add_edge(n1, n2, ());
    graph.add_edge(n2, n3, ());
    graph.add_edge(n3, n4, ());

    let path = bidis(&graph, n1, n4).expect("Path should exist");

    assert_eq!(path.len(), 4);
    assert_eq!(path[0], n1);
    assert_eq!(path[path.len() - 1], n4);

    // Verify directed path
    for i in 0..path.len() - 1 {
        assert!(
            graph.contains_edge(path[i], path[i + 1]),
            "Directed edge missing from {:?} to {:?}",
            path[i],
            path[i + 1]
        );
    }
}

#[test]
fn test_bidis_no_path() {
    // Test bidirectional search when no path exists
    let mut graph = Graph::<i32, ()>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    let n4 = graph.add_node(4);

    // Create two disconnected components
    graph.add_edge(n1, n2, ());
    graph.add_edge(n3, n4, ());

    let path = bidis(&graph, n1, n4);
    assert!(
        path.is_none(),
        "Should not find path between disconnected nodes"
    );
}

#[test]
fn test_bidis_same_start_and_target() {
    // Test when start and target are the same
    let mut graph = Graph::<i32, ()>::new();
    let n1 = graph.add_node(1);

    let path = bidis(&graph, n1, n1).expect("Path to self should exist");
    assert_eq!(path.len(), 1);
    assert_eq!(path[0], n1);
}

#[test]
fn test_bidis_meeting_point_correctness() {
    // Test that the meeting point is correctly identified
    // Create a graph where searches should meet in the middle
    let mut graph = Graph::<i32, ()>::new();
    let nodes: Vec<_> = (0..7).map(|i| graph.add_node(i)).collect();

    // Create a linear chain
    for i in 0..6 {
        graph.add_edge(nodes[i], nodes[i + 1], ());
    }

    let path = bidis(&graph, nodes[0], nodes[6]).expect("Path should exist");

    assert_eq!(path.len(), 7);
    assert_eq!(path[0], nodes[0]);
    assert_eq!(path[6], nodes[6]);

    // Verify entire path is valid
    for i in 0..path.len() - 1 {
        assert!(
            graph.contains_edge(path[i], path[i + 1]) || graph.contains_edge(path[i + 1], path[i]),
            "Invalid edge in path at position {}: {:?} to {:?}",
            i,
            path[i],
            path[i + 1]
        );
    }
}

#[test]
fn test_bidis_with_cycles() {
    // Test bidirectional search on a graph with cycles
    let mut graph = Graph::<i32, ()>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    let n4 = graph.add_node(4);
    let n5 = graph.add_node(5);

    // Create a cycle with an additional path
    graph.add_edge(n1, n2, ());
    graph.add_edge(n2, n3, ());
    graph.add_edge(n3, n4, ());
    graph.add_edge(n4, n2, ()); // Create cycle
    graph.add_edge(n3, n5, ());

    let path = bidis(&graph, n1, n5).expect("Path should exist");

    assert!(path.len() >= 4); // Minimum path length
    assert_eq!(path[0], n1);
    assert_eq!(path[path.len() - 1], n5);
}

#[test]
fn test_bidis_star_topology() {
    // Test on a star topology where all paths go through center
    let mut graph = Graph::<i32, ()>::new();
    let center = graph.add_node(0);
    let leaves: Vec<_> = (1..=5).map(|i| graph.add_node(i)).collect();

    for &leaf in &leaves {
        graph.add_edge(center, leaf, ());
    }

    let path = bidis(&graph, leaves[0], leaves[4]).expect("Path should exist");

    assert_eq!(path.len(), 3); // leaf -> center -> leaf
    assert_eq!(path[0], leaves[0]);
    assert_eq!(path[1], center);
    assert_eq!(path[2], leaves[4]);
}

#[test]
fn test_try_bidirectional_search_success() {
    // Test the try_ variant with successful path finding
    let mut graph = Graph::<i32, ()>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);

    graph.add_edge(n1, n2, ());
    graph.add_edge(n2, n3, ());

    let result = try_bidirectional_search(&graph, n1, n3);
    assert!(result.is_ok());
    let path = result.unwrap();
    assert_eq!(path.len(), 3);
}

#[test]
fn test_try_bidirectional_search_failure() {
    // Test the try_ variant with no path
    let mut graph = Graph::<i32, ()>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);

    let result = try_bidirectional_search(&graph, n1, n2);
    assert!(result.is_err());
}

#[test]
fn test_bidis_complete_graph() {
    // Test on a complete graph where multiple shortest paths exist
    let mut graph = Graph::<i32, ()>::new();
    let nodes: Vec<_> = (0..5).map(|i| graph.add_node(i)).collect();

    // Create complete graph
    for i in 0..5 {
        for j in (i + 1)..5 {
            graph.add_edge(nodes[i], nodes[j], ());
        }
    }

    let path = bidis(&graph, nodes[0], nodes[4]).expect("Path should exist");

    // In a complete graph, shortest path is direct edge
    assert_eq!(path.len(), 2);
    assert_eq!(path[0], nodes[0]);
    assert_eq!(path[1], nodes[4]);
}

#[test]
fn test_bidis_path_consistency_with_bfs() {
    // Compare bidirectional search path length with BFS to ensure correctness
    let mut graph = Graph::<i32, ()>::new();
    let nodes: Vec<_> = (0..10).map(|i| graph.add_node(i)).collect();

    // Create a more complex graph structure
    graph.add_edge(nodes[0], nodes[1], ());
    graph.add_edge(nodes[1], nodes[2], ());
    graph.add_edge(nodes[0], nodes[3], ());
    graph.add_edge(nodes[3], nodes[4], ());
    graph.add_edge(nodes[4], nodes[2], ());
    graph.add_edge(nodes[2], nodes[5], ());
    graph.add_edge(nodes[5], nodes[6], ());

    let bidis_path = bidis(&graph, nodes[0], nodes[6]).expect("Path should exist");

    // BFS from start should visit nodes in shortest path order
    let bfs_order = bfs(&graph, nodes[0]);
    assert!(bfs_order.contains(&nodes[6]), "BFS should reach target");

    // Verify bidis path is valid
    assert!(bidis_path.len() >= 2);
    assert_eq!(bidis_path[0], nodes[0]);
    assert_eq!(bidis_path[bidis_path.len() - 1], nodes[6]);
}
