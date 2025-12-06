//! Regression tests to verify bug fixes stay fixed and features continue working.

use graphina::core::types::{Digraph, Graph};

#[test]
#[cfg(feature = "community")]
fn test_louvain_with_removed_nodes() {
    use graphina::community::louvain::louvain;

    let mut g = Graph::<i32, f64>::new();
    let n0 = g.add_node(0);
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    let n4 = g.add_node(4);

    g.add_edge(n0, n1, 1.0);
    g.add_edge(n1, n2, 1.0);
    g.add_edge(n3, n4, 1.0);

    g.remove_node(n2);

    let communities = louvain(&g, Some(42)).unwrap();
    assert!(!communities.is_empty());
}

#[test]
fn test_undirected_degree_consistency() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);

    g.add_edge(n1, n2, 5.0);

    assert_eq!(g.degree(n1), Some(1));
    assert_eq!(g.degree(n2), Some(1));
    assert_eq!(g.edge_count(), 1);
}

#[test]
#[cfg(feature = "centrality")]
fn test_centrality_empty_graph() {
    use graphina::centrality::degree::degree_centrality;

    let g = Graph::<i32, f64>::new();
    let result = degree_centrality(&g);

    assert!(result.is_ok());
    let centrality = result.unwrap();
    assert_eq!(centrality.len(), 0);
}

#[test]
fn test_dijkstra_negative_weights() {
    use graphina::core::paths::dijkstra_path_f64;

    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);

    g.add_edge(n1, n2, -5.0);

    let result = dijkstra_path_f64(&g, n1, None);
    assert!(result.is_err());
}

#[test]
fn test_self_loop_handling() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);

    g.add_edge(n1, n1, 1.0);

    assert!(g.degree(n1).unwrap() > 0);
    assert!(g.contains_edge(n1, n1));
}

#[test]
fn test_directed_edge_finding() {
    let mut g = Digraph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);

    g.add_edge(n1, n2, 1.0);

    assert!(g.contains_edge(n1, n2));
    assert!(!g.contains_edge(n2, n1));
}

#[test]
fn test_iterator_safety() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);

    let nodes: Vec<_> = g.nodes().map(|(id, _)| id).collect();

    for node in nodes {
        g.remove_node(node);
    }

    assert_eq!(g.node_count(), 0);
}

#[test]
#[cfg(feature = "parallel")]
fn test_parallel_vs_sequential_consistency() {
    use graphina::parallel::degrees_parallel;

    let mut g = Graph::<i32, f64>::new();
    let nodes: Vec<_> = (0..100).map(|i| g.add_node(i)).collect();

    for i in 0..100 {
        for j in (i + 1)..100 {
            if (i * j) % 7 == 0 {
                g.add_edge(nodes[i], nodes[j], 1.0);
            }
        }
    }

    let parallel_degrees = degrees_parallel(&g);

    let sequential_degrees: std::collections::HashMap<_, _> = g
        .nodes()
        .map(|(id, _)| (id, g.degree(id).unwrap()))
        .collect();

    for (node, deg) in &sequential_degrees {
        assert_eq!(parallel_degrees.get(node), Some(deg));
    }
}

#[test]
fn test_nodemap_with_deleted_nodes() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    let map = g.to_nodemap(|_, val| *val * 2);

    g.remove_node(n2);

    assert_eq!(map.get(&n1), Some(&2));
    assert_eq!(map.get(&n3), Some(&6));
}

#[test]
fn test_dag_validation() {
    use graphina::core::validation::is_dag;

    let mut g = Digraph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);

    assert!(is_dag(&g));

    g.add_edge(n3, n1, 1.0);

    assert!(!is_dag(&g));
}

#[test]
fn test_serialization_special_values() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);

    g.add_edge(n1, n2, 0.0);

    let json = g.to_serializable();
    let json_str = serde_json::to_string(&json).unwrap();

    assert!(!json_str.contains("NaN"));
    assert!(!json_str.contains("Infinity"));
}

// Centrality Module Bug Fixes

#[test]
#[cfg(feature = "centrality")]
fn test_pagerank_with_deleted_nodes() {
    use graphina::centrality::pagerank::pagerank;

    let mut graph: Digraph<i32, f64> = Digraph::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    let n4 = graph.add_node(4);

    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, n3, 1.0);
    graph.add_edge(n3, n4, 1.0);
    graph.add_edge(n4, n1, 1.0);

    graph.remove_node(n2);

    let pr = pagerank(&graph, 0.85, 100, 1e-6, None).unwrap();

    assert!(!pr.contains_key(&n2));
    assert!(pr.contains_key(&n1));
    assert!(pr.contains_key(&n3));
    assert!(pr.contains_key(&n4));

    let sum: f64 = pr.values().sum();
    assert!((sum - 1.0).abs() < 1e-4);
}

#[test]
#[cfg(feature = "centrality")]
fn test_eigenvector_with_deleted_nodes() {
    use graphina::centrality::eigenvector::eigenvector_centrality;

    let mut graph: Graph<i32, f64> = Graph::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    let n4 = graph.add_node(4);
    let n5 = graph.add_node(5);

    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n1, n3, 1.0);
    graph.add_edge(n1, n4, 1.0);
    graph.add_edge(n1, n5, 1.0);

    graph.remove_node(n5);

    let eig = eigenvector_centrality(&graph, 100, 1e-6).unwrap();

    assert!(eig[&n1] > eig[&n2]);
    assert!(eig[&n1] > eig[&n3]);
    assert!(eig[&n1] > eig[&n4]);
    assert!(!eig.contains_key(&n5));
}

#[test]
#[cfg(feature = "centrality")]
fn test_eigenvector_issue_21_regression() {
    // Regression test for Issue #21: "Using Vec to return centrality might cause error for graph with removed node"
    // Ensures that creating a "gap" in NodeIds by removing an intermediate node doesn't cause out-of-bounds access.
    use graphina::centrality::eigenvector::eigenvector_centrality;

    let mut g = Graph::<i32, f64>::new();

    let n0 = g.add_node(0);
    let n1 = g.add_node(1);
    let n2 = g.add_node(4);
    let n3 = g.add_node(9);

    g.add_edge(n0, n3, 1.0);
    g.add_edge(n1, n2, 1.0);
    g.add_edge(n3, n1, 1.0);

    // Remove an intermediate node (n1) to create a gap if IDs were treated as dense indices
    g.remove_node(n1);

    // This should not panic
    let result = eigenvector_centrality(&g, 1000, 1e-6);
    assert!(result.is_ok());

    let centrality = result.unwrap();
    // n1 should not be in the result
    assert!(!centrality.contains_key(&n1));
    // Remaining nodes should be present
    assert!(centrality.contains_key(&n0));
    assert!(centrality.contains_key(&n2));
    assert!(centrality.contains_key(&n3));
}

#[test]
#[cfg(feature = "centrality")]
fn test_katz_with_deleted_nodes() {
    use graphina::centrality::katz::katz_centrality;

    let mut graph: Digraph<i32, f64> = Digraph::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    let n4 = graph.add_node(4);

    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, n3, 1.0);
    graph.add_edge(n3, n4, 1.0);

    graph.remove_node(n2);

    let katz = katz_centrality(&graph, 0.1, None, 100, 1e-6).unwrap();

    assert!(katz.contains_key(&n1));
    assert!(!katz.contains_key(&n2));
    assert!(katz.contains_key(&n3));
    assert!(katz.contains_key(&n4));
}

#[test]
#[cfg(feature = "centrality")]
fn test_betweenness_centrality_two_nodes_division_by_zero_fix() {
    use graphina::centrality::betweenness::betweenness_centrality;
    use ordered_float::OrderedFloat;

    let mut graph = Graph::<i32, OrderedFloat<f64>>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);

    graph.add_edge(n1, n2, OrderedFloat(1.0));

    let result = betweenness_centrality(&graph, true);
    assert!(result.is_ok());

    let centrality = result.unwrap();
    assert_eq!(centrality.len(), 2);
    assert_eq!(*centrality.get(&n1).unwrap(), 0.0);
    assert_eq!(*centrality.get(&n2).unwrap(), 0.0);
}

// Graph Generator Regressions

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
    assert_eq!(g.edge_count(), 0);
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
    assert_eq!(g.edge_count(), n * (n - 1) / 2);
}

// Community Detection Regressions

#[test]
#[cfg(feature = "community")]
fn test_girvan_newman_with_deleted_nodes() {
    use graphina::community::girvan_newman::girvan_newman;
    use graphina::core::types::NodeId;

    let mut g: Graph<i32, f64> = Graph::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    let n4 = g.add_node(4);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);

    g.remove_node(n2);

    let communities = girvan_newman(&g, 2).unwrap();

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
fn test_label_propagation_stability() {
    use graphina::community::label_propagation::label_propagation;

    let mut g: Graph<i32, f64> = Graph::new();
    let nodes: Vec<_> = (0..10).map(|i| g.add_node(i)).collect();

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

    g.add_edge(nodes[2], nodes[7], 0.1);

    let communities = label_propagation(&g, 100, Some(42)).unwrap();
    assert!(!communities.is_empty());
    assert!(communities.len() <= 10);
}

// MST and Path Algorithm Consistency

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

    assert_eq!(weight_kruskal, weight_prim);
    assert_eq!(weight_kruskal, weight_boruvka);
}

#[test]
#[cfg(feature = "traversal")]
fn test_traversal_algorithms_find_same_paths() {
    use graphina::traversal::{bfs, dfs};

    let mut g: Graph<i32, ()> = Graph::new();
    let nodes: Vec<_> = (0..5).map(|i| g.add_node(i)).collect();

    g.add_edge(nodes[0], nodes[1], ());
    g.add_edge(nodes[0], nodes[2], ());
    g.add_edge(nodes[1], nodes[3], ());
    g.add_edge(nodes[1], nodes[4], ());

    let bfs_result = bfs(&g, nodes[0]);
    let dfs_result = dfs(&g, nodes[0]);

    assert_eq!(bfs_result.len(), 5);
    assert_eq!(dfs_result.len(), 5);

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
