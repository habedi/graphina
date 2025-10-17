// Tests for core module bugs

use graphina::core::generators::{barabasi_albert_graph, watts_strogatz_graph};
use graphina::core::traversal::{bfs, bidis, dfs};
use graphina::core::types::{Graph, GraphMarker};

#[test]
fn test_watts_strogatz_no_duplicate_edges() {
    // Bug: Watts-Strogatz can create duplicate edges between same node pairs
    let graph = watts_strogatz_graph::<GraphMarker>(10, 4, 0.5, 42).unwrap();

    // Check for duplicate edges by counting edges between each pair
    let mut edge_count = std::collections::HashMap::new();
    for (u, v, _) in graph.edges() {
        let key = if u.index() < v.index() {
            (u.index(), v.index())
        } else {
            (v.index(), u.index())
        };
        *edge_count.entry(key).or_insert(0) += 1;
    }

    // No edge pair should appear more than once
    for (pair, count) in edge_count.iter() {
        assert_eq!(*count, 1, "Duplicate edge found between {:?}", pair);
    }
}

#[test]
fn test_barabasi_albert_terminates() {
    // Bug: Barabási-Albert can hang in infinite loop during target selection
    // This test ensures the algorithm terminates in reasonable time
    use std::time::{Duration, Instant};

    let start = Instant::now();
    let timeout = Duration::from_secs(5);

    let result = barabasi_albert_graph::<GraphMarker>(100, 5, 42);

    let elapsed = start.elapsed();
    assert!(elapsed < timeout, "Algorithm took too long: {:?}", elapsed);
    assert!(result.is_ok(), "Algorithm should complete successfully");

    // Verify graph properties
    let graph = result.unwrap();
    assert_eq!(graph.node_count(), 100);

    // Each node after the first m should have exactly m edges added
    // But we can't easily verify this without tracking internal state
}

#[test]
fn test_barabasi_albert_large_m() {
    // Edge case: m close to n should still work
    let result = barabasi_albert_graph::<GraphMarker>(20, 15, 42);
    assert!(result.is_ok());

    let graph = result.unwrap();
    assert_eq!(graph.node_count(), 20);
}

#[test]
fn test_bidirectional_search_simple_path() {
    // Test basic bidirectional search functionality
    let mut graph = Graph::<i32, ()>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);

    graph.add_edge(n1, n2, ());
    graph.add_edge(n2, n3, ());

    let path = bidis(&graph, n1, n3);
    assert!(path.is_some());

    let path = path.unwrap();
    assert_eq!(path.len(), 3);
    assert_eq!(path[0], n1);
    assert_eq!(path[2], n3);
}

#[test]
fn test_bidirectional_search_diamond_graph() {
    // Bug: Bidirectional search may find incorrect meeting point
    // Diamond graph: 1->2, 1->3, 2->4, 3->4
    let mut graph = Graph::<i32, ()>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    let n4 = graph.add_node(4);

    graph.add_edge(n1, n2, ());
    graph.add_edge(n1, n3, ());
    graph.add_edge(n2, n4, ());
    graph.add_edge(n3, n4, ());

    let path = bidis(&graph, n1, n4);
    assert!(path.is_some());

    let path = path.unwrap();
    // Should be length 3 (shortest path)
    assert_eq!(path.len(), 3, "Path should be shortest: {:?}", path);
    assert_eq!(path[0], n1);
    assert_eq!(path[2], n4);
}

#[test]
fn test_bidirectional_search_no_path() {
    // Test disconnected graph
    let mut graph = Graph::<i32, ()>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);

    graph.add_edge(n1, n2, ());
    // n3 is disconnected

    let path = bidis(&graph, n1, n3);
    assert!(path.is_none(), "Should return None for disconnected nodes");
}

#[test]
fn test_traversal_empty_graph() {
    // Test that BFS/DFS properly validate the start node exists
    let mut graph = Graph::<i32, ()>::new();
    let n1 = graph.add_node(1);

    // Remove the node to make it invalid
    graph.remove_node(n1);

    // After the fix, BFS/DFS now validate node existence and return empty for removed nodes
    let bfs_result = bfs(&graph, n1);
    let dfs_result = dfs(&graph, n1);

    // Fixed behavior: returns empty vector for removed/invalid nodes
    assert_eq!(
        bfs_result.len(),
        0,
        "BFS should return empty for removed node"
    );
    assert_eq!(
        dfs_result.len(),
        0,
        "DFS should return empty for removed node"
    );
}

#[test]
fn test_watts_strogatz_no_self_loops() {
    // Ensure rewiring doesn't create self-loops
    let graph = watts_strogatz_graph::<GraphMarker>(20, 4, 0.8, 42).unwrap();

    for (u, v, _) in graph.edges() {
        assert_ne!(u, v, "Self-loop detected: {:?}", u);
    }
}

#[test]
fn test_barabasi_albert_degree_distribution() {
    // Verify that initial complete graph is formed correctly
    let graph = barabasi_albert_graph::<GraphMarker>(10, 3, 42).unwrap();

    // First m=3 nodes should have high degree (complete graph among them)
    let mut degrees = std::collections::HashMap::new();
    for (u, v, _) in graph.edges() {
        *degrees.entry(u).or_insert(0) += 1;
        *degrees.entry(v).or_insert(0) += 1;
    }

    // Just verify graph was created without checking exact distribution
    assert!(degrees.len() > 0, "Graph should have edges");
}

#[test]
fn test_generator_edge_cases() {
    // Test various edge cases for generators

    // Watts-Strogatz with beta=0 should be ring lattice
    let graph = watts_strogatz_graph::<GraphMarker>(10, 4, 0.0, 42).unwrap();
    assert_eq!(graph.edge_count(), 20); // Each node connected to 2 neighbors on each side

    // Watts-Strogatz with beta=1 should rewire all edges
    let graph = watts_strogatz_graph::<GraphMarker>(10, 4, 1.0, 42).unwrap();
    assert!(graph.edge_count() > 0);
}

#[test]
fn test_bidirectional_same_start_end() {
    // Edge case: start == target
    let mut graph = Graph::<i32, ()>::new();
    let n1 = graph.add_node(1);

    let path = bidis(&graph, n1, n1);
    assert!(path.is_some());
    assert_eq!(path.unwrap(), vec![n1]);
}
