// File: tests/links_algorithms_tests.rs

use graphina::core::types::{Graph, NodeId};
use graphina::links::algorithms::*;

// Helper: builds a simple undirected graph with 4 nodes.
// The graph structure:
//   1
//  / \
// 0---2
// |
// 3
//
// Neighbors:
// - Node 0: {1,2,3}
// - Node 1: {0,2}
// - Node 2: {0,1}
// - Node 3: {0}
fn build_test_graph() -> Graph<i32, f64> {
    let mut graph: Graph<i32, f64> = Graph::default();
    let n0 = graph.add_node(0);
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    // Add undirected edges (the underlying implementation for undirected graphs
    // automatically makes the connections symmetric).
    graph.add_edge(n0, n1, 1.0);
    graph.add_edge(n0, n2, 1.0);
    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n0, n3, 1.0);
    graph
}

// A simple community function: nodes 0,1,2 in community 0, node 3 in community 1.
fn community(n: NodeId) -> u8 {
    if n.index() == 3 { 1 } else { 0 }
}

// Helper for approximate equality of f64 values.
fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() < tol
}

#[test]
fn test_resource_allocation_index() {
    let graph = build_test_graph();
    let scores = resource_allocation_index(&graph, None);
    // For nodes 1 and 2: their neighbors are {0} in common.
    // Node 0's degree is 3, so expected score = 1/3.
    let mut found = false;
    for ((u, v), score) in scores {
        if u.index() == 1 && v.index() == 2 {
            assert!(approx_eq(score, 1.0 / 3.0, 1e-5));
            found = true;
        }
    }
    assert!(
        found,
        "Pair (1,2) not found in resource allocation index results"
    );
}

#[test]
fn test_jaccard_coefficient() {
    let graph = build_test_graph();
    let scores = jaccard_coefficient(&graph, None);
    // For nodes 1 and 2:
    // N(1) = {0,2}, N(2) = {0,1} => Intersection = {0} (size 1), Union = {0,1,2} (size 3)
    // Expected score = 1/3.
    let mut found = false;
    for ((u, v), score) in scores {
        if u.index() == 1 && v.index() == 2 {
            assert!(approx_eq(score, 1.0 / 3.0, 1e-5));
            found = true;
        }
    }
    assert!(found, "Pair (1,2) not found in Jaccard coefficient results");
}

#[test]
fn test_adamic_adar_index() {
    let graph = build_test_graph();
    let scores = adamic_adar_index(&graph, None);
    // For nodes 1 and 2: common neighbor is node 0 (degree 3)
    // Expected score = 1 / ln(3).
    let expected = 1.0 / (3.0_f64).ln();
    let mut found = false;
    for ((u, v), score) in scores {
        if u.index() == 1 && v.index() == 2 {
            assert!(approx_eq(score, expected, 1e-5));
            found = true;
        }
    }
    assert!(found, "Pair (1,2) not found in Adamic-Adar index results");
}

#[test]
fn test_preferential_attachment() {
    let graph = build_test_graph();
    let scores = preferential_attachment(&graph, None);
    // For nodes 1 and 2: degree(1)=2, degree(2)=2, expected score = 4.
    let mut found = false;
    for ((u, v), score) in scores {
        if u.index() == 1 && v.index() == 2 {
            assert_eq!(score, 4.0);
            found = true;
        }
    }
    assert!(
        found,
        "Pair (1,2) not found in preferential attachment results"
    );
}

#[test]
fn test_cn_soundarajan_hopcroft() {
    let graph = build_test_graph();
    let scores = cn_soundarajan_hopcroft(&graph, None, community);
    // For nodes 1 and 2: common neighbor is node 0.
    // community(1)=0, community(2)=0, community(0)=0, so count = 1.
    let mut found = false;
    for ((u, v), score) in scores {
        if u.index() == 1 && v.index() == 2 {
            assert_eq!(score, 1.0);
            found = true;
        }
    }
    assert!(
        found,
        "Pair (1,2) not found in CN Soundarajan-Hopcroft results"
    );
}

#[test]
fn test_ra_index_soundarajan_hopcroft() {
    let graph = build_test_graph();
    let scores = ra_index_soundarajan_hopcroft(&graph, None, community);
    // For nodes 1 and 2: common neighbor is node 0 qualifies, so expected score = 1 / degree(0) = 1/3.
    let mut found = false;
    for ((u, v), score) in scores {
        if u.index() == 1 && v.index() == 2 {
            assert!(approx_eq(score, 1.0 / 3.0, 1e-5));
            found = true;
        }
    }
    assert!(
        found,
        "Pair (1,2) not found in RA index Soundarajan-Hopcroft results"
    );
}

#[test]
fn test_within_inter_cluster() {
    let graph = build_test_graph();
    // For nodes 1 and 2: common neighbors = {0}.
    // For delta = 1.0, expected score = (1 + 1) / (0 + 1) = 2.
    let scores = within_inter_cluster(&graph, None, community, 1.0);
    let mut found = false;
    for ((u, v), score) in scores {
        if u.index() == 1 && v.index() == 2 {
            assert_eq!(score, 2.0);
            found = true;
        }
    }
    assert!(
        found,
        "Pair (1,2) not found in within-inter-cluster results"
    );
}

#[test]
fn test_common_neighbor_centrality() {
    let graph = build_test_graph();
    // For nodes 1 and 2: common neighbors = {0} so count = 1.
    // With alpha = 1.0, expected score = 1^1 = 1.
    let scores = common_neighbor_centrality(&graph, None, 1.0);
    let mut found = false;
    for ((u, v), score) in scores {
        if u.index() == 1 && v.index() == 2 {
            assert_eq!(score, 1.0);
            found = true;
        }
    }
    assert!(
        found,
        "Pair (1,2) not found in common neighbor centrality results"
    );
}
