/*!
# Integration Tests: Regressions

This test suite contains regression tests for community detection, graph generators,
and other algorithms to ensure bugs don't reappear and features continue to work correctly.

## Coverage:
- Community detection with edge cases (deleted nodes, multigraphs)
- Graph generators (Barabási-Albert, Watts-Strogatz, etc.)
- Algorithm correctness with various graph structures
*/

use graphina::core::types::{Graph, NodeId};

// ============================================================================
// Community Detection Regressions
// ============================================================================

#[test]
#[cfg(feature = "community")]
fn test_girvan_newman_with_deleted_nodes() {
    use graphina::community::girvan_newman::girvan_newman;

    let mut g: Graph<i32, f64> = Graph::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    let n4 = g.add_node(4);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);

    g.remove_node(n2);

    let communities = girvan_newman(&g, 2);

    let mut seen = std::collections::HashSet::<NodeId>::new();
    for c in &communities {
        for &nid in c {
            seen.insert(nid);
        }
    }

    assert!(!seen.contains(&n2));
    assert!(seen.contains(&n1));
    assert!(seen.contains(&n4));
}

#[test]
#[cfg(feature = "community")]
fn test_spectral_clustering_multigraph_and_deletions() {
    use graphina::community::spectral::spectral_clustering;

    let mut g: Graph<i32, f64> = Graph::new();
    let a = g.add_node(1);
    let b = g.add_node(2);
    let c = g.add_node(3);
    let d = g.add_node(4);

    g.add_edge(a, b, 1.0);
    g.add_edge(a, b, 2.0);
    g.add_edge(b, c, 1.0);
    g.add_edge(c, d, 1.0);

    g.remove_node(c);

    let clusters = spectral_clustering(&g, 2, Some(42));

    let mut seen = std::collections::HashSet::new();
    for cls in &clusters {
        for &nid in cls {
            seen.insert(nid);
        }
    }
    assert!(!seen.contains(&c));
}

#[test]
#[cfg(feature = "community")]
fn test_label_propagation_stability() {
    use graphina::community::label_propagation::label_propagation;

    let mut g: Graph<i32, f64> = Graph::new();
    let nodes: Vec<_> = (0..10).map(|i| g.add_node(i)).collect();

    // Create two clear communities
    for i in 0..4 {
        for j in (i + 1)..5 {
            g.add_edge(nodes[i], nodes[j], 1.0);
        }
    }

    for i in 5..9 {
        for j in (i + 1)..10 {
            g.add_edge(nodes[i], nodes[j], 1.0);
        }
    }

    // Weak inter-community link
    g.add_edge(nodes[2], nodes[7], 0.1);

    let communities = label_propagation(&g, 100, Some(42));

    assert!(!communities.is_empty());
    assert!(communities.len() <= 10);
}

#[test]
#[cfg(feature = "community")]
fn test_modularity_optimization_convergence() {
    use graphina::community::louvain::louvain;

    let mut g: Graph<i32, f64> = Graph::new();
    let nodes: Vec<_> = (0..20).map(|i| g.add_node(i)).collect();

    // Create modular structure
    for i in 0..10 {
        for j in (i + 1)..10 {
            g.add_edge(nodes[i], nodes[j], 1.0);
        }
    }

    for i in 10..20 {
        for j in (i + 1)..20 {
            g.add_edge(nodes[i], nodes[j], 1.0);
        }
    }

    g.add_edge(nodes[5], nodes[15], 0.5);

    let communities = louvain(&g, Some(42));

    assert!(!communities.is_empty());
    assert!(communities.len() >= 2);
}

// ============================================================================
// Graph Generator Regressions
// ============================================================================

#[test]
fn test_barabasi_albert_large_graph_completes_and_counts() {
    use graphina::core::generators::barabasi_albert_graph;
    use graphina::core::types::Undirected;

    let n = 200;
    let m = 3;
    let seed = 12345;
    let g = barabasi_albert_graph::<Undirected>(n, m, seed).expect("BA generator should succeed");

    assert_eq!(g.node_count(), n);

    let expected_edges = (m * (m - 1) / 2) + (n - m) * m;
    assert_eq!(g.edge_count(), expected_edges);
}

#[test]
fn test_watts_strogatz_edge_count_is_reasonable() {
    use graphina::core::generators::watts_strogatz_graph;
    use graphina::core::types::Undirected;

    let n = 100;
    let k = 6;
    let beta = 0.2;
    let seed = 7;

    let g =
        watts_strogatz_graph::<Undirected>(n, k, beta, seed).expect("WS generator should succeed");

    assert_eq!(g.node_count(), n);
    assert_eq!(g.edge_count(), n * k / 2);
}

#[test]
fn test_erdos_renyi_with_zero_probability() {
    use graphina::core::generators::erdos_renyi_graph;
    use graphina::core::types::Undirected;

    let n = 50;
    let p = 0.0;
    let seed = 123;

    let g = erdos_renyi_graph::<Undirected>(n, p, seed).expect("ER generator should succeed");

    assert_eq!(g.node_count(), n);
    assert_eq!(g.edge_count(), 0); // No edges with p=0
}

#[test]
fn test_erdos_renyi_with_full_probability() {
    use graphina::core::generators::erdos_renyi_graph;
    use graphina::core::types::Undirected;

    let n = 20;
    let p = 1.0;
    let seed = 456;

    let g = erdos_renyi_graph::<Undirected>(n, p, seed).expect("ER generator should succeed");

    assert_eq!(g.node_count(), n);
    // Complete graph has n*(n-1)/2 edges
    assert_eq!(g.edge_count(), n * (n - 1) / 2);
}

#[test]
fn test_barabasi_albert_min_degree() {
    use graphina::core::generators::barabasi_albert_graph;
    use graphina::core::types::Undirected;

    let n = 50;
    let m = 4;
    let seed = 789;

    let g = barabasi_albert_graph::<Undirected>(n, m, seed).expect("BA generator should succeed");

    // Every node added after initial clique should have degree >= m
    let mut low_degree_count = 0;
    for (node, _) in g.nodes() {
        if let Some(deg) = g.degree(node) {
            if deg < m {
                low_degree_count += 1;
            }
        }
    }

    // Only the initial clique nodes might have degree < m in some cases
    assert!(low_degree_count <= m);
}

#[test]
fn test_watts_strogatz_regularity_with_zero_rewiring() {
    use graphina::core::generators::watts_strogatz_graph;
    use graphina::core::types::Undirected;

    let n = 30;
    let k = 4;
    let beta = 0.0; // No rewiring
    let seed = 999;

    let g =
        watts_strogatz_graph::<Undirected>(n, k, beta, seed).expect("WS generator should succeed");

    // With beta=0, should be a regular ring lattice
    // Every node should have degree k
    for (node, _) in g.nodes() {
        assert_eq!(g.degree(node), Some(k));
    }
}

#[test]
fn test_generator_with_small_parameters() {
    use graphina::core::generators::{barabasi_albert_graph, watts_strogatz_graph};
    use graphina::core::types::Undirected;

    // Test edge cases with small graphs
    let ba = barabasi_albert_graph::<Undirected>(5, 2, 111);
    assert!(ba.is_ok());
    assert_eq!(ba.unwrap().node_count(), 5);

    let ws = watts_strogatz_graph::<Undirected>(6, 2, 0.3, 222);
    assert!(ws.is_ok());
    assert_eq!(ws.unwrap().node_count(), 6);
}

#[test]
fn test_generator_error_handling() {
    use graphina::core::generators::{barabasi_albert_graph, watts_strogatz_graph};
    use graphina::core::types::Undirected;

    // m > n should fail
    let result = barabasi_albert_graph::<Undirected>(5, 10, 333);
    assert!(result.is_err());

    // k >= n should fail
    let result = watts_strogatz_graph::<Undirected>(10, 10, 0.5, 444);
    assert!(result.is_err());

    // Odd k should fail
    let result = watts_strogatz_graph::<Undirected>(10, 3, 0.5, 555);
    assert!(result.is_err());
}

// ============================================================================
// Algorithm Correctness Regressions
// ============================================================================

#[test]
#[cfg(feature = "mst")]
fn test_mst_algorithms_consistency() {
    use graphina::mst::{boruvka_mst, kruskal_mst, prim_mst};
    use ordered_float::OrderedFloat;

    let mut g: Graph<i32, OrderedFloat<f64>> = Graph::new();
    let nodes: Vec<_> = (0..6).map(|i| g.add_node(i)).collect();

    g.add_edge(nodes[0], nodes[1], OrderedFloat(1.0));
    g.add_edge(nodes[0], nodes[2], OrderedFloat(4.0));
    g.add_edge(nodes[1], nodes[2], OrderedFloat(2.0));
    g.add_edge(nodes[1], nodes[3], OrderedFloat(5.0));
    g.add_edge(nodes[2], nodes[3], OrderedFloat(3.0));
    g.add_edge(nodes[2], nodes[4], OrderedFloat(6.0));
    g.add_edge(nodes[3], nodes[4], OrderedFloat(7.0));
    g.add_edge(nodes[3], nodes[5], OrderedFloat(8.0));
    g.add_edge(nodes[4], nodes[5], OrderedFloat(9.0));

    let (_, weight_kruskal) = kruskal_mst(&g).unwrap();
    let (_, weight_prim) = prim_mst(&g).unwrap();
    let (_, weight_boruvka) = boruvka_mst(&g).unwrap();

    // All three algorithms should find the same MST weight
    assert_eq!(weight_kruskal, weight_prim);
    assert_eq!(weight_kruskal, weight_boruvka);
}

#[test]
#[cfg(feature = "traversal")]
fn test_traversal_algorithms_find_same_paths() {
    use graphina::traversal::{bfs, dfs};

    let mut g: Graph<i32, ()> = Graph::new();
    let nodes: Vec<_> = (0..5).map(|i| g.add_node(i)).collect();

    // Create a tree
    g.add_edge(nodes[0], nodes[1], ());
    g.add_edge(nodes[0], nodes[2], ());
    g.add_edge(nodes[1], nodes[3], ());
    g.add_edge(nodes[1], nodes[4], ());

    let bfs_result = bfs(&g, nodes[0]);
    let dfs_result = dfs(&g, nodes[0]);

    // Both should visit all nodes
    assert_eq!(bfs_result.len(), 5);
    assert_eq!(dfs_result.len(), 5);

    // Both should find the same set of nodes
    let bfs_set: std::collections::HashSet<_> = bfs_result.iter().collect();
    let dfs_set: std::collections::HashSet<_> = dfs_result.iter().collect();
    assert_eq!(bfs_set, dfs_set);
}

#[test]
fn test_shortest_path_algorithms_consistency() {
    use graphina::core::paths::{bellman_ford, dijkstra_path_f64};

    let mut g: Graph<i32, f64> = Graph::new();
    let nodes: Vec<_> = (0..4).map(|i| g.add_node(i)).collect();

    g.add_edge(nodes[0], nodes[1], 1.0);
    g.add_edge(nodes[1], nodes[2], 2.0);
    g.add_edge(nodes[2], nodes[3], 3.0);
    g.add_edge(nodes[0], nodes[3], 10.0);

    let dijkstra_result = dijkstra_path_f64(&g, nodes[0], None).unwrap();
    let bellman_ford_result = bellman_ford(&g, nodes[0]).unwrap();

    // Both should find the same shortest path distances
    for (node, _) in g.nodes() {
        let dij_dist = dijkstra_result.0.get(&node).and_then(|&d| d);
        let bf_dist = bellman_ford_result.get(&node).and_then(|&d| d);

        match (dij_dist, bf_dist) {
            (Some(d1), Some(d2)) => assert!((d1 - d2).abs() < 1e-10),
            (None, None) => {}
            _ => panic!("Algorithms disagree on reachability"),
        }
    }
}
