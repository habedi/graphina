// Test file for verifying bug fixes in Graphina

use graphina::centrality::algorithms::{
    degree_centrality, in_degree_centrality, out_degree_centrality,
};
use graphina::core::types::{Digraph, Graph};

#[test]
fn test_degree_centrality_undirected_no_double_counting() {
    // Bug: degree_centrality was double-counting edges in undirected graphs
    // because flow_edges() returns both (u,v) and (v,u) for each edge

    let mut g: Graph<i32, ()> = Graph::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    // Add edges: n1-n2, n1-n3
    g.add_edge(n1, n2, ());
    g.add_edge(n1, n3, ());

    let centrality = degree_centrality(&g);

    // Node 1 should have degree 2 (connected to 2 nodes)
    // Nodes 2 and 3 should each have degree 1
    assert_eq!(centrality[&n1], 2.0, "Node 1 should have degree 2");
    assert_eq!(centrality[&n2], 1.0, "Node 2 should have degree 1");
    assert_eq!(centrality[&n3], 1.0, "Node 3 should have degree 1");
}

#[test]
fn test_degree_centrality_directed_counts_both() {
    // For directed graphs, degree centrality should count both in and out edges

    let mut g: Digraph<i32, ()> = Digraph::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    // Add directed edges: n1->n2, n1->n3
    g.add_edge(n1, n2, ());
    g.add_edge(n1, n3, ());

    let centrality = degree_centrality(&g);

    // Node 1: out-degree = 2, in-degree = 0, total = 2
    // Nodes 2 and 3: out-degree = 0, in-degree = 1, total = 1
    assert_eq!(centrality[&n1], 2.0, "Node 1 should have total degree 2");
    assert_eq!(centrality[&n2], 1.0, "Node 2 should have total degree 1");
    assert_eq!(centrality[&n3], 1.0, "Node 3 should have total degree 1");
}

#[test]
fn test_degree_centrality_undirected_complex() {
    // More complex test with multiple edges
    let mut g: Graph<i32, f64> = Graph::new();
    let nodes: Vec<_> = (0..5).map(|i| g.add_node(i)).collect();

    // Create a simple path: 0-1-2-3-4
    g.add_edge(nodes[0], nodes[1], 1.0);
    g.add_edge(nodes[1], nodes[2], 1.0);
    g.add_edge(nodes[2], nodes[3], 1.0);
    g.add_edge(nodes[3], nodes[4], 1.0);

    let centrality = degree_centrality(&g);

    // End nodes should have degree 1, middle nodes degree 2
    assert_eq!(centrality[&nodes[0]], 1.0);
    assert_eq!(centrality[&nodes[1]], 2.0);
    assert_eq!(centrality[&nodes[2]], 2.0);
    assert_eq!(centrality[&nodes[3]], 2.0);
    assert_eq!(centrality[&nodes[4]], 1.0);
}

#[test]
fn test_degree_centrality_undirected_star() {
    // Star graph: one central node connected to all others
    let mut g: Graph<i32, ()> = Graph::new();
    let center = g.add_node(0);
    let leaves: Vec<_> = (1..=5).map(|i| g.add_node(i)).collect();

    for &leaf in &leaves {
        g.add_edge(center, leaf, ());
    }

    let centrality = degree_centrality(&g);

    // Center should have degree 5
    assert_eq!(centrality[&center], 5.0);

    // Each leaf should have degree 1
    for &leaf in &leaves {
        assert_eq!(centrality[&leaf], 1.0);
    }
}

#[test]
fn test_degree_centrality_self_loop() {
    // Test self-loops are counted correctly
    let mut g: Graph<i32, ()> = Graph::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);

    g.add_edge(n1, n1, ()); // Self-loop
    g.add_edge(n1, n2, ());

    let centrality = degree_centrality(&g);

    // Self-loop counts once for each endpoint (both are n1), plus edge to n2
    // So n1: 2 (from self-loop) + 1 (from edge to n2) = 3
    assert_eq!(centrality[&n1], 3.0);
    assert_eq!(centrality[&n2], 1.0);
}

#[test]
fn test_in_out_degree_consistency() {
    // For undirected graphs, in and out degree should be equal
    let mut g: Graph<i32, ()> = Graph::new();
    let nodes: Vec<_> = (0..4).map(|i| g.add_node(i)).collect();

    g.add_edge(nodes[0], nodes[1], ());
    g.add_edge(nodes[1], nodes[2], ());
    g.add_edge(nodes[2], nodes[3], ());

    let in_deg = in_degree_centrality(&g);
    let out_deg = out_degree_centrality(&g);

    // For undirected graphs, these should be equal
    for node in &nodes {
        assert_eq!(in_deg[node], out_deg[node]);
    }
}

#[test]
fn test_directed_in_out_degree_separation() {
    // For directed graphs, in and out degree can differ
    let mut g: Digraph<i32, ()> = Digraph::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    // n1 -> n2, n1 -> n3 (n1 has out-degree 2, in-degree 0)
    g.add_edge(n1, n2, ());
    g.add_edge(n1, n3, ());

    let in_deg = in_degree_centrality(&g);
    let out_deg = out_degree_centrality(&g);

    assert_eq!(out_deg[&n1], 2.0);
    assert_eq!(in_deg[&n1], 0.0);

    assert_eq!(out_deg[&n2], 0.0);
    assert_eq!(in_deg[&n2], 1.0);

    assert_eq!(out_deg[&n3], 0.0);
    assert_eq!(in_deg[&n3], 1.0);
}

#[test]
fn test_empty_graph_centrality() {
    // Edge case: empty graph
    let g: Graph<i32, ()> = Graph::new();
    let centrality = degree_centrality(&g);

    assert_eq!(centrality.len(), 0);
}

#[test]
fn test_single_node_centrality() {
    // Edge case: single isolated node
    let mut g: Graph<i32, ()> = Graph::new();
    let n1 = g.add_node(1);

    let centrality = degree_centrality(&g);

    assert_eq!(centrality[&n1], 0.0);
}

#[test]
fn test_complete_graph_centrality() {
    // Complete graph: every node connected to every other
    let mut g: Graph<i32, ()> = Graph::new();
    let nodes: Vec<_> = (0..5).map(|i| g.add_node(i)).collect();

    // Connect every pair
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            g.add_edge(nodes[i], nodes[j], ());
        }
    }

    let centrality = degree_centrality(&g);

    // In a complete graph of n nodes, each node has degree n-1
    for node in &nodes {
        assert_eq!(centrality[node], 4.0);
    }
}
