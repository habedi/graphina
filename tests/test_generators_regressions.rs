// Integration tests for generator regressions and API contracts

use graphina::core::generators::{barabasi_albert_graph, watts_strogatz_graph};
use graphina::core::types::Undirected;

#[test]
fn test_barabasi_albert_large_graph_completes_and_counts() {
    let n = 200;
    let m = 3;
    let seed = 12345;
    let g = barabasi_albert_graph::<Undirected>(n, m, seed).expect("BA generator should succeed");
    assert_eq!(g.node_count(), n);
    // initial complete graph edges + added m edges for each new node
    let expected_edges = (m * (m - 1) / 2) + (n - m) * m;
    assert_eq!(g.edge_count(), expected_edges);
}

#[test]
fn test_watts_strogatz_edge_count_is_reasonable() {
    let n = 100;
    let k = 6; // even, < n
    let beta = 0.2;
    let seed = 7;
    let g =
        watts_strogatz_graph::<Undirected>(n, k, beta, seed).expect("WS generator should succeed");
    assert_eq!(g.node_count(), n);
    // The ring lattice has n*k/2 undirected edges; rewiring preserves count
    assert_eq!(g.edge_count(), n * k / 2);
}
