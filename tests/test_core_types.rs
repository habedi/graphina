// File: tests/core_types_tests.rs

use graphina::core::types::{Digraph, Graph, NodeId};

#[test]
fn test_digraph() {
    // Create a directed graph with integer node attributes and f32 edge weights.
    let mut dgraph = Digraph::<i32, f32>::new();
    let n1 = dgraph.add_node(1);
    let n2 = dgraph.add_node(2);
    let n3 = dgraph.add_node(3);

    let _e1 = dgraph.add_edge(n1, n2, 1.0);
    let _e2 = dgraph.add_edge(n2, n3, 2.0);
    let _e3 = dgraph.add_edge(n3, n1, 3.0);

    // Verify counts.
    assert_eq!(dgraph.node_count(), 3);
    assert_eq!(dgraph.edge_count(), 3);

    // For a directed graph, neighbors are those with outgoing edges.
    let neighbors_n1: Vec<NodeId> = dgraph.neighbors(n1).collect();
    assert!(neighbors_n1.contains(&n2));
    assert!(!neighbors_n1.contains(&n3));

    // Verify node attributes.
    assert_eq!(*dgraph.node_attr(n1).unwrap(), 1);
    assert_eq!(*dgraph.node_attr(n2).unwrap(), 2);
    assert_eq!(*dgraph.node_attr(n3).unwrap(), 3);

    // Verify edge attributes via dense API.
    let matrix = dgraph.to_adjacency_matrix();
    assert_eq!(matrix[0][1], Some(1.0));
    assert_eq!(matrix[1][2], Some(2.0));
    assert_eq!(matrix[2][0], Some(3.0));

    // Test sparse conversion.
    let sparse = dgraph.to_sparse_adjacency_matrix();
    assert_eq!(sparse.rows(), 3);
    // Check a few nonzero values.
    assert_eq!(sparse.get(0, 1), Some(&1.0));
    assert_eq!(sparse.get(1, 2), Some(&2.0));
    assert_eq!(sparse.get(2, 0), Some(&3.0));

    // Update node attribute and verify.
    let updated = dgraph.update_node(n1, 10);
    assert!(updated, "Update should succeed");
    assert_eq!(*dgraph.node_attr(n1).unwrap(), 10);
}

#[test]
fn test_graph() {
    // Create an undirected graph with string node attributes and unweighted edges.
    let mut graph = Graph::<&str, f32>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    let _e1 = graph.add_edge(a, b, 0.0);
    let _e2 = graph.add_edge(b, c, 0.0);
    let _e3 = graph.add_edge(c, a, 0.0);

    // Verify counts.
    assert_eq!(graph.node_count(), 3);
    // For undirected graphs, our API adds one edge per pair.
    assert_eq!(graph.edge_count(), 3);

    // In an undirected graph, neighbors should include all connected nodes.
    let neighbors_a: Vec<NodeId> = graph.neighbors(a).collect();
    assert!(neighbors_a.contains(&b));
    assert!(neighbors_a.contains(&c));

    // Verify node attributes.
    assert_eq!(graph.node_attr(a).unwrap(), &"A");
    assert_eq!(graph.node_attr(b).unwrap(), &"B");
    assert_eq!(graph.node_attr(c).unwrap(), &"C");

    // For unweighted edges, edge_attr returns the weight (here, 0.0).
    let matrix = graph.to_adjacency_matrix();
    // Each edge appears twice in the dense matrix for an undirected graph.
    assert_eq!(matrix[a.index()][b.index()], Some(0.0));
    assert_eq!(matrix[b.index()][c.index()], Some(0.0));
    assert_eq!(matrix[c.index()][a.index()], Some(0.0));

    // Test sparse conversion.
    let sparse = graph.to_sparse_adjacency_matrix();
    assert_eq!(sparse.rows(), 3);
    // Check one of the symmetric entries.
    assert_eq!(sparse.get(a.index(), b.index()), Some(&0.0));

    // Update node attribute and verify.
    let updated = graph.update_node(a, "Alpha");
    assert!(updated, "Update should succeed");
    assert_eq!(graph.node_attr(a).unwrap(), &"Alpha");
}

#[test]
fn test_removals() {
    // Test removal functionality on a directed graph.
    let mut dgraph = Digraph::<i32, f32>::new();
    let n1 = dgraph.add_node(1);
    let n2 = dgraph.add_node(2);
    let n3 = dgraph.add_node(3);

    let _e1 = dgraph.add_edge(n1, n2, 1.0);
    let e2 = dgraph.add_edge(n2, n3, 2.0);
    let _e3 = dgraph.add_edge(n3, n1, 3.0);

    // Remove an edge and check count.
    let removed_edge = dgraph.remove_edge(e2);
    assert_eq!(removed_edge, Some(2.0));
    assert_eq!(dgraph.edge_count(), 2);

    // Remove a node and verify its attribute is returned.
    let removed_attr = dgraph.remove_node(n2);
    assert_eq!(removed_attr, Some(2));
    // The node count should now reflect the removal.
    assert_eq!(dgraph.node_count(), 2);

    // Subsequent updates to the removed node should fail.
    let update_result = dgraph.update_node(n2, 20);
    assert!(!update_result, "Update on removed node should fail");

    // Ensure the adjacency matrix does not include the removed node.
    let matrix = dgraph.to_adjacency_matrix();
    // The resulting matrix size should match the new node count.
    assert_eq!(matrix.len(), 2);
    // Similarly, verify the sparse matrix.
    let sparse = dgraph.to_sparse_adjacency_matrix();
    assert_eq!(sparse.rows(), 2);

    // Test removal on an undirected graph.
    let mut graph = Graph::<&str, f32>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    let _e1 = graph.add_edge(a, b, 0.0);
    let _e2 = graph.add_edge(b, c, 0.0);
    let _e3 = graph.add_edge(c, a, 0.0);

    // Remove a node.
    let removed_node = graph.remove_node(b);
    assert_eq!(removed_node, Some("B"));
    assert_eq!(graph.node_count(), 2);
    // With the node removed, the edge count should drop.
    // Depending on petgraph's internal behavior, edge_count may be 0 if all incident edges are removed.
    // Here we assert that no edges remain connected to the removed node.
    for (src, tgt, _) in graph.edges() {
        assert!(src != b && tgt != b);
    }
}
