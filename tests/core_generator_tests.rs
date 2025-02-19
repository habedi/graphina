// File: tests/core_generators_tests.rs

use graphina::core::generators::{
    barabasi_albert_graph, bipartite_graph, complete_graph, cycle_graph, erdos_renyi_graph,
    star_graph, watts_strogatz_graph,
};
use graphina::core::types::{Directed, Undirected};

#[test]
fn test_erdos_renyi_directed() {
    let graph = erdos_renyi_graph::<Directed>(3, 1.0, 42)
        .expect("Failed to generate directed Erdős–Rényi graph");
    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 6);
}

#[test]
fn test_erdos_renyi_undirected() {
    let graph = erdos_renyi_graph::<Undirected>(3, 1.0, 42)
        .expect("Failed to generate undirected Erdős–Rényi graph");
    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 3);
}

#[test]
fn test_complete_graph_directed() {
    let graph = complete_graph::<Directed>(4).expect("Failed to generate directed complete graph");
    assert_eq!(graph.node_count(), 4);
    assert_eq!(graph.edge_count(), 12);
}

#[test]
fn test_complete_graph_undirected() {
    let graph =
        complete_graph::<Undirected>(4).expect("Failed to generate undirected complete graph");
    assert_eq!(graph.node_count(), 4);
    assert_eq!(graph.edge_count(), 6);
}

#[test]
fn test_bipartite_graph() {
    let graph =
        bipartite_graph::<Undirected>(3, 2, 1.0, 42).expect("Failed to generate bipartite graph");
    assert_eq!(graph.node_count(), 5);
    assert_eq!(graph.edge_count(), 6);
}

#[test]
fn test_star_graph() {
    let graph = star_graph::<Undirected>(5).expect("Failed to generate star graph");
    assert_eq!(graph.node_count(), 5);
    assert_eq!(graph.edge_count(), 4);
}

#[test]
fn test_cycle_graph() {
    let graph = cycle_graph::<Undirected>(5).expect("Failed to generate cycle graph");
    assert_eq!(graph.node_count(), 5);
    assert_eq!(graph.edge_count(), 5);
}

#[test]
fn test_watts_strogatz_graph() {
    let n = 10;
    let k = 4; // must be even and less than n
    let beta = 0.5;
    let seed = 42;
    let graph = watts_strogatz_graph::<Undirected>(n, k, beta, seed)
        .expect("Failed to generate Watts–Strogatz graph");
    assert_eq!(graph.node_count(), n);
    // Initially, the ring lattice has n*k/2 edges; rewiring may add additional edges.
    assert!(graph.edge_count() >= n * k / 2);
}

#[test]
fn test_barabasi_albert_graph() {
    let n = 20;
    let m = 3;
    let seed = 42;
    let graph = barabasi_albert_graph::<Undirected>(n, m, seed)
        .expect("Failed to generate Barabási–Albert graph");
    assert_eq!(graph.node_count(), n);
    // Starting complete graph of m nodes has m*(m-1)/2 edges.
    // Each new node (n-m) adds m edges.
    let expected_edges = (m * (m - 1) / 2) + (n - m) * m;
    assert_eq!(graph.edge_count(), expected_edges);
}
