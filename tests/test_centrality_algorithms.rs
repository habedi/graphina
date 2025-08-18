use graphina::centrality::algorithms::*;
use graphina::core::types::Digraph;
use ordered_float::OrderedFloat;

/// Build a strongly connected test graph with f64 weights.
/// This graph is used for tests that don't require total ordering on weights.
fn build_test_graph_f64() -> Digraph<i32, f64> {
    let mut graph: Digraph<i32, f64> = Digraph::default();
    let n0 = graph.add_node(0);
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    // Create a strongly connected directed graph.
    graph.add_edge(n0, n1, 1.0);
    graph.add_edge(n0, n2, 1.0);
    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n1, n3, 1.0);
    graph.add_edge(n2, n0, 1.0);
    graph.add_edge(n2, n3, 1.0);
    graph.add_edge(n3, n0, 1.0);
    graph.add_edge(n3, n1, 1.0);
    graph
}

/// Build a strongly connected test graph with OrderedFloat<f64> weights.
/// Use this for tests that require an ordered weight type (e.g., closeness and harmonic centrality).
fn build_test_graph_ordered() -> Digraph<i32, OrderedFloat<f64>> {
    let mut graph: Digraph<i32, OrderedFloat<f64>> = Digraph::default();
    let n0 = graph.add_node(0);
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    // Create a strongly connected directed graph.
    graph.add_edge(n0, n1, OrderedFloat(1.0));
    graph.add_edge(n0, n2, OrderedFloat(1.0));
    graph.add_edge(n1, n2, OrderedFloat(1.0));
    graph.add_edge(n1, n3, OrderedFloat(1.0));
    graph.add_edge(n2, n0, OrderedFloat(1.0));
    graph.add_edge(n2, n3, OrderedFloat(1.0));
    graph.add_edge(n3, n0, OrderedFloat(1.0));
    graph.add_edge(n3, n1, OrderedFloat(1.0));
    graph
}

/// Helper for approximate floating-point comparisons.
fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() < tol
}

#[test]
fn test_degree_centrality() {
    let graph = build_test_graph_f64();
    let deg = degree_centrality(&graph);
    // In our graph, each node has 2 out-edges and 2 in-edges, so degree should be 4.
    for d in deg {
        assert_eq!(d.1, 4.0);
    }
}

#[test]
fn test_closeness_centrality() {
    let graph = build_test_graph_f64();
    let closeness = closeness_centrality(&graph, false).unwrap();
    // In our strongly connected graph with all edges = 1.0,
    // each node's distances: two neighbors at 1 and one at 2 -> sum = 4.
    // Closeness = (n-1)/sum = 3/4 = 0.75.
    for (_, c) in closeness {
        assert!(approx_eq(c, 0.75, 1e-6));
    }
}

#[test]
fn test_betweenness_centrality() {
    let graph = build_test_graph_f64();
    let bc = betweenness_centrality(&graph);
    // In a symmetric strongly connected graph, betweenness scores should be non-negative.
    for score in bc {
        assert!(score >= 0.0);
    }
}

#[test]
fn test_eigenvector_centrality() {
    let graph = build_test_graph_f64();
    let ev = eigenvector_centrality(&graph, 20, false);
    // Check that we have 4 scores and all are positive.
    assert_eq!(ev.len(), 4);
    for (_, score) in ev.iter() {
        assert!(*score > 0.0);
    }
}

#[test]
fn test_pagerank() {
    let graph = build_test_graph_f64();
    let pr = pagerank(&graph, 0.85, 50);
    let total: f64 = pr.iter().sum();
    assert!(approx_eq(total, 1.0, 1e-6));
    for score in pr {
        assert!(score > 0.0);
    }
}

#[test]
fn test_katz_centrality() {
    let graph = build_test_graph_f64();
    let kc = katz_centrality(&graph, 0.1, 1.0, 50, false, true);
    assert_eq!(kc.len(), 4);
    for (_, score) in kc {
        assert!(score > 0.0);
    }
}

#[test]
fn test_harmonic_centrality() {
    let graph = build_test_graph_ordered();
    let hc = harmonic_centrality(&graph).unwrap();
    // For our graph, for each node: distances: two neighbors at 1 and one at 2, so harmonic centrality ~ 1/1 + 1/1 + 1/2 = 2.5.
    for score in hc {
        assert!(approx_eq(score, 2.5, 1e-6));
    }
}
