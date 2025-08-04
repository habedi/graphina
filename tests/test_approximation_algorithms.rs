//! Integration tests for the approximation and heuristic algorithms.
//! These tests create small graphs using the core `Graph` type (an alias for
//! `BaseGraph<A, W, Undirected>`) and verify that the approximation functions
//! return expected results.
//!
//! To run the tests, execute: `cargo test --test approximation_algorithms_tests`

use graphina::approximation::algorithms::*;
use graphina::core::types::{Graph, NodeId};
use ordered_float::OrderedFloat;
use std::collections::HashSet;

/// Helper: Creates a triangle graph with f64 weights.
fn triangle_graph_f64() -> Graph<&'static str, f64> {
    let mut g = Graph::<&'static str, f64>::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    let c = g.add_node("c");
    g.add_edge(a, b, 1.0);
    g.add_edge(b, c, 1.0);
    g.add_edge(c, a, 1.0);
    g
}

/// Helper: Creates a diamond graph (ordered weights) with nodes a, b, c, d.
/// Edges: (a,b), (a,c), (b,d), (c,d) each with weight 1.
fn diamond_graph_ordered() -> Graph<&'static str, OrderedFloat<f64>> {
    let mut g = Graph::<&'static str, OrderedFloat<f64>>::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    let c = g.add_node("c");
    let d = g.add_node("d");
    g.add_edge(a, b, OrderedFloat(1.0));
    g.add_edge(a, c, OrderedFloat(1.0));
    g.add_edge(b, d, OrderedFloat(1.0));
    g.add_edge(c, d, OrderedFloat(1.0));
    g
}

/// Helper: Creates a line graph (ordered weights) with nodes a, b, c, d.
/// Edges: (a,b), (b,c), (c,d) each with weight 1.
fn line_graph_ordered() -> Graph<&'static str, OrderedFloat<f64>> {
    let mut g = Graph::<&'static str, OrderedFloat<f64>>::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    let c = g.add_node("c");
    let d = g.add_node("d");
    g.add_edge(a, b, OrderedFloat(1.0));
    g.add_edge(b, c, OrderedFloat(1.0));
    g.add_edge(c, d, OrderedFloat(1.0));
    g
}

/// Helper: Creates a star graph (f64 weights) with a center and three leaves.
fn star_graph_f64() -> Graph<&'static str, f64> {
    let mut g = Graph::<&'static str, f64>::new();
    let center = g.add_node("center");
    let leaf1 = g.add_node("leaf1");
    let leaf2 = g.add_node("leaf2");
    let leaf3 = g.add_node("leaf3");
    g.add_edge(center, leaf1, 1.0);
    g.add_edge(center, leaf2, 1.0);
    g.add_edge(center, leaf3, 1.0);
    g
}

#[test]
fn test_local_node_connectivity() {
    let g = diamond_graph_ordered();
    let nodes: Vec<NodeId> = g.nodes().map(|(u, _)| u).collect();
    // Expect two vertex-disjoint paths from node "a" to node "d"
    let connectivity = local_node_connectivity(&g, nodes[0], nodes[3]);
    assert_eq!(connectivity, 2);

    // For a line graph, there is only one simple path.
    let g_line = line_graph_ordered();
    let nodes_line: Vec<NodeId> = g_line.nodes().map(|(u, _)| u).collect();
    let connectivity_line = local_node_connectivity(&g_line, nodes_line[0], nodes_line[3]);
    assert_eq!(connectivity_line, 1);
}

#[test]
fn test_maximum_independent_set() {
    let g = triangle_graph_f64();
    let mis = maximum_independent_set(&g);
    // In a triangle graph, a maximum independent set contains 1 node.
    assert_eq!(mis.len(), 1);

    let star = star_graph_f64();
    let mis_star = maximum_independent_set(&star);
    let nodes: Vec<NodeId> = star.nodes().map(|(u, _)| u).collect();
    // Assuming the center was added first, it should not be in the independent set.
    let center = nodes[0];
    assert!(!mis_star.contains(&center));
    // The independent set should then contain all leaves.
    assert_eq!(mis_star.len(), nodes.len() - 1);
}

#[test]
fn test_max_clique() {
    let g = triangle_graph_f64();
    let clique = max_clique(&g);
    // For a triangle, the maximum clique is all 3 nodes.
    assert_eq!(clique.len(), 3);
}

#[test]
fn test_clique_removal() {
    let g = triangle_graph_f64();
    let cliques = clique_removal(&g);
    // For a triangle, one clique covering all nodes should be removed.
    assert_eq!(cliques.len(), 1);
    assert_eq!(cliques[0].len(), 3);
}

#[test]
fn test_large_clique_size() {
    let g_triangle = triangle_graph_f64();
    let size_triangle = large_clique_size(&g_triangle);
    assert_eq!(size_triangle, 3);

    let star = star_graph_f64();
    let size_star = large_clique_size(&star);
    // In a star graph, the largest clique is any edge (size 2).
    assert_eq!(size_star, 2);
}

#[test]
fn test_average_clustering() {
    let g_triangle = triangle_graph_f64();
    let clustering_triangle = average_clustering(&g_triangle);
    // In a triangle, every node's neighbors are fully connected.
    assert!((clustering_triangle - 1.0).abs() < 1e-6);

    // Create a line graph of 3 nodes (clustering should be 0).
    let mut g_line = Graph::<&str, f64>::new();
    let a = g_line.add_node("a");
    let b = g_line.add_node("b");
    let c = g_line.add_node("c");
    g_line.add_edge(a, b, 1.0);
    g_line.add_edge(b, c, 1.0);
    let clustering_line = average_clustering(&g_line);
    assert!((clustering_line - 0.0).abs() < 1e-6);
}

#[test]
fn test_densest_subgraph() {
    let g = triangle_graph_f64();
    let densest = densest_subgraph(&g, None);
    let nodes: HashSet<NodeId> = g.nodes().map(|(u, _)| u).collect();
    // For a triangle, the densest subgraph should be the entire graph.
    assert_eq!(densest, nodes);
}

#[test]
fn test_diameter() {
    let g_line = line_graph_ordered();
    // In a line graph of 4 nodes with unit weights, the diameter is 3.
    let diam = diameter(&g_line).unwrap();
    assert!((diam - 3.0).abs() < 1e-6);
}

#[test]
fn test_min_weighted_vertex_cover() {
    let g = triangle_graph_f64();
    let cover = min_weighted_vertex_cover(&g, None);
    // For a triangle, the minimal vertex cover is of size 2.
    assert_eq!(
        cover.len(),
        2,
        "Expected vertex cover size 2, got {:?}",
        cover.len()
    );
}

#[test]
fn test_min_maximal_matching() {
    let g = triangle_graph_f64();
    let matching = min_maximal_matching(&g);
    // In a triangle, a maximal matching consists of 1 edge.
    assert_eq!(matching.len(), 1);
}

#[test]
fn test_ramsey_r2() {
    let g = triangle_graph_f64();
    let (clique, independent_set) = ramsey_r2(&g);
    // For a triangle, maximum clique size is 3 and maximum independent set size is 1.
    assert_eq!(clique.len(), 3);
    assert_eq!(independent_set.len(), 1);
}

#[test]
fn test_greedy_tsp() {
    // Create a cycle graph (complete graph) with 4 nodes (OrderedFloat weights).
    let g = {
        let mut g = Graph::<&str, OrderedFloat<f64>>::new();
        let a = g.add_node("a");
        let b = g.add_node("b");
        let c = g.add_node("c");
        let d = g.add_node("d");
        g.add_edge(a, b, OrderedFloat(1.0));
        g.add_edge(b, c, OrderedFloat(1.0));
        g.add_edge(c, d, OrderedFloat(1.0));
        g.add_edge(d, a, OrderedFloat(1.0));
        // Add extra edges to make the graph complete.
        g.add_edge(a, c, OrderedFloat(2.0));
        g.add_edge(b, d, OrderedFloat(2.0));
        g
    };
    let nodes: Vec<NodeId> = g.nodes().map(|(u, _)| u).collect();
    let start = nodes[0];
    let (tour, cost) = greedy_tsp(&g, start).unwrap();
    // The tour should start and end at the starting node.
    assert_eq!(tour.first(), Some(&start));
    assert_eq!(tour.last(), Some(&start));
    // The tour should include all nodes.
    let unique: HashSet<_> = tour.into_iter().collect();
    assert!(unique.len() >= nodes.len());
    assert!(cost > 0.0);
}

#[test]
fn test_simulated_annealing_tsp() {
    // For the placeholder, simulated annealing TSP returns the initial cycle.
    let mut g = Graph::<&str, f64>::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    let c = g.add_node("c");
    g.add_edge(a, b, 1.0);
    g.add_edge(b, c, 1.0);
    g.add_edge(c, a, 1.0);
    let nodes: Vec<NodeId> = g.nodes().map(|(u, _)| u).collect();
    // Define an initial cycle.
    let init_cycle = vec![nodes[0], nodes[1], nodes[2], nodes[0]];
    let (cycle, cost) = simulated_annealing_tsp(&g, init_cycle.clone()).unwrap();
    assert_eq!(cycle, init_cycle);
    assert!((cost - 3.0).abs() < 1e-6);
}

#[test]
fn test_threshold_accepting_tsp() {
    // For the placeholder, threshold accepting TSP returns the initial cycle.
    let mut g = Graph::<&str, f64>::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    let c = g.add_node("c");
    g.add_edge(a, b, 1.0);
    g.add_edge(b, c, 1.0);
    g.add_edge(c, a, 1.0);
    let nodes: Vec<NodeId> = g.nodes().map(|(u, _)| u).collect();
    let init_cycle = vec![nodes[0], nodes[1], nodes[2], nodes[0]];
    let (cycle, cost) = threshold_accepting_tsp(&g, init_cycle.clone()).unwrap();
    assert_eq!(cycle, init_cycle);
    assert!((cost - 3.0).abs() < 1e-6);
}

#[test]
fn test_treewidth_min_degree() {
    // For a line graph of 4 nodes, the treewidth should be 1.
    let g = {
        let mut g = Graph::<&str, f64>::new();
        let a = g.add_node("a");
        let b = g.add_node("b");
        let c = g.add_node("c");
        let d = g.add_node("d");
        g.add_edge(a, b, 1.0);
        g.add_edge(b, c, 1.0);
        g.add_edge(c, d, 1.0);
        g
    };
    let (tw, order) = treewidth_min_degree(&g);
    assert_eq!(tw, 1);
    assert_eq!(order.len(), g.node_count());
}

#[test]
fn test_treewidth_min_fill_in() {
    // For a line graph of 4 nodes, the treewidth should also be 1.
    let g = {
        let mut g = Graph::<&str, f64>::new();
        let a = g.add_node("a");
        let b = g.add_node("b");
        let c = g.add_node("c");
        let d = g.add_node("d");
        g.add_edge(a, b, 1.0);
        g.add_edge(b, c, 1.0);
        g.add_edge(c, d, 1.0);
        g
    };
    let (tw, order) = treewidth_min_fill_in(&g);
    assert_eq!(tw, 1);
    assert_eq!(order.len(), g.node_count());
}
