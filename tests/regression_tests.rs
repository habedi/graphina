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

// Regression: the generic Dijkstra followed only the stored edge orientation,
// so on an undirected graph a node reached via an edge stored as (other, node)
// could not reach back. Here the edges are stored as (0,1) and (1,2); from node
// 2, Dijkstra must still reach node 0 at distance 2.
#[test]
fn test_dijkstra_undirected_follows_both_directions() {
    use graphina::core::paths::dijkstra;

    let mut g = Graph::<i32, i32>::new();
    let n0 = g.add_node(0);
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    g.add_edge(n0, n1, 1);
    g.add_edge(n1, n2, 1);

    let dist = dijkstra(&g, n2).expect("dijkstra should succeed");
    assert_eq!(dist[&n2], Some(0));
    assert_eq!(dist[&n1], Some(1));
    assert_eq!(dist[&n0], Some(2), "node 0 must be reachable from node 2");
}

// Regression: harmonic centrality summed reciprocal distances over all nodes
// including the source itself, whose distance is 0, yielding 1/0 = infinity for
// every node. In a unit-weight triangle each node's harmonic centrality is
// 1/1 + 1/1 = 2.
#[test]
#[cfg(feature = "centrality")]
fn test_harmonic_centrality_excludes_source() {
    use graphina::centrality::harmonic::harmonic_centrality;
    use ordered_float::OrderedFloat;

    let mut g = Graph::<i32, OrderedFloat<f64>>::new();
    let n0 = g.add_node(0);
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    g.add_edge(n0, n1, OrderedFloat(1.0));
    g.add_edge(n1, n2, OrderedFloat(1.0));
    g.add_edge(n2, n0, OrderedFloat(1.0));

    let hc = harmonic_centrality(&g).expect("harmonic should succeed");
    for n in [n0, n1, n2] {
        assert!(hc[&n].is_finite(), "harmonic centrality must be finite");
        assert!((hc[&n] - 2.0).abs() < 1e-9, "expected 2.0, got {}", hc[&n]);
    }
}

// Regression: unnormalized undirected betweenness did not halve the raw Brandes
// count (which accumulates each shortest path from both endpoints), so values
// were double the standard definition. On the unit-weight path 0-1-2-3 the
// middle nodes have unnormalized betweenness 2.0, not 4.0.
#[test]
#[cfg(feature = "centrality")]
fn test_betweenness_undirected_halving() {
    use graphina::centrality::betweenness::betweenness_centrality;
    use ordered_float::OrderedFloat;

    let mut g = Graph::<i32, OrderedFloat<f64>>::new();
    let nodes: Vec<_> = (0..4).map(|i| g.add_node(i)).collect();
    g.add_edge(nodes[0], nodes[1], OrderedFloat(1.0));
    g.add_edge(nodes[1], nodes[2], OrderedFloat(1.0));
    g.add_edge(nodes[2], nodes[3], OrderedFloat(1.0));

    let bc = betweenness_centrality(&g, false).expect("betweenness should succeed");
    assert!((bc[&nodes[0]] - 0.0).abs() < 1e-9);
    assert!(
        (bc[&nodes[1]] - 2.0).abs() < 1e-9,
        "expected 2.0, got {}",
        bc[&nodes[1]]
    );
    assert!(
        (bc[&nodes[2]] - 2.0).abs() < 1e-9,
        "expected 2.0, got {}",
        bc[&nodes[2]]
    );
    assert!((bc[&nodes[3]] - 0.0).abs() < 1e-9);
}

// Regression: closeness centrality summed reciprocal distances (the harmonic
// centrality formula) instead of computing closeness. On the unit-weight path
// 0-1-2 the endpoint's closeness is (reachable / sum_dist) * (reachable / (n-1))
// = (2/3) * (2/2) = 0.6667, not the harmonic value 1/1 + 1/2 = 1.5.
#[test]
#[cfg(feature = "centrality")]
fn test_closeness_centrality_is_not_harmonic() {
    use graphina::centrality::closeness::closeness_centrality;
    use ordered_float::OrderedFloat;

    let mut g = Graph::<i32, OrderedFloat<f64>>::new();
    let n0 = g.add_node(0);
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    g.add_edge(n0, n1, OrderedFloat(1.0));
    g.add_edge(n1, n2, OrderedFloat(1.0));

    let cc = closeness_centrality(&g).expect("closeness should succeed");
    assert!(
        (cc[&n0] - 2.0 / 3.0).abs() < 1e-9,
        "expected 0.6667, got {}",
        cc[&n0]
    );
    assert!(
        (cc[&n1] - 1.0).abs() < 1e-9,
        "expected 1.0, got {}",
        cc[&n1]
    );
    assert!(
        (cc[&n2] - 2.0 / 3.0).abs() < 1e-9,
        "expected 0.6667, got {}",
        cc[&n2]
    );
}

// Regression: Katz centrality built a directed-only adjacency matrix, so on an
// undirected graph it was asymmetric and broke the graph's symmetry. Here nodes
// 1 and 3 are symmetric, as are 0 and 4, so their Katz centralities must be
// equal.
#[test]
#[cfg(feature = "centrality")]
fn test_katz_centrality_symmetric_on_undirected() {
    use graphina::centrality::katz::katz_centrality;
    use ordered_float::OrderedFloat;

    let mut g = Graph::<i32, OrderedFloat<f64>>::new();
    let ids: Vec<_> = (0..5).map(|i| g.add_node(i)).collect();
    for (u, v, w) in [
        (0, 1, 1.0),
        (1, 2, 1.0),
        (2, 3, 1.0),
        (1, 3, 2.0),
        (3, 4, 1.0),
    ] {
        g.add_edge(ids[u], ids[v], OrderedFloat(w));
    }

    let kc = katz_centrality(&g, 0.1, None, 2000, 1e-9).expect("katz should succeed");
    assert!(
        (kc[&ids[1]] - kc[&ids[3]]).abs() < 1e-9,
        "symmetric nodes 1 and 3 must be equal: {} vs {}",
        kc[&ids[1]],
        kc[&ids[3]]
    );
    assert!(
        (kc[&ids[0]] - kc[&ids[4]]).abs() < 1e-9,
        "symmetric nodes 0 and 4 must be equal: {} vs {}",
        kc[&ids[0]],
        kc[&ids[4]]
    );
}

// Regression: Laplacian centrality used the formula d^2 + sum(neighbor degrees),
// missing the +d term and the factor of 2 on the neighbor-degree sum. The
// unnormalized Laplacian centrality of a node in an unweighted graph is
// d^2 + d + 2 * sum(neighbor degrees). In a triangle each node has degree 2 and
// two neighbors of degree 2, so the value is 4 + 2 + 2*4 = 14, not 8.
#[test]
#[cfg(feature = "centrality")]
fn test_laplacian_centrality_formula() {
    use graphina::centrality::other::laplacian_centrality;
    use ordered_float::OrderedFloat;

    let mut g = Graph::<i32, OrderedFloat<f64>>::new();
    let n0 = g.add_node(0);
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    g.add_edge(n0, n1, OrderedFloat(1.0));
    g.add_edge(n1, n2, OrderedFloat(1.0));
    g.add_edge(n2, n0, OrderedFloat(1.0));

    let lc = laplacian_centrality(&g).expect("laplacian should succeed");
    for n in [n0, n1, n2] {
        assert!(
            (lc[&n] - 14.0).abs() < 1e-9,
            "expected 14.0, got {}",
            lc[&n]
        );
    }
}

// Regression: VoteRank previously iterated a HashSet (non-deterministic output),
// kept a dead `votes` array, and never reduced neighbors' voting ability or
// stopped when no votes remained, so it elected spurious extra seeds. On a star
// the center is elected first; afterward every leaf's voting ability is spent,
// so the standard algorithm stops, electing exactly one node even when more are
// requested. The result must also be deterministic across runs.
#[test]
#[cfg(feature = "centrality")]
fn test_voterank_stops_and_is_deterministic() {
    use graphina::centrality::other::voterank;

    let mut g = Graph::<i32, f64>::new();
    let center = g.add_node(0);
    let leaves: Vec<_> = (1..=4).map(|i| g.add_node(i)).collect();
    for &leaf in &leaves {
        g.add_edge(center, leaf, 1.0);
    }

    let first = voterank(&g, 4);
    assert_eq!(
        first,
        vec![center],
        "star elects only the center, then stops"
    );

    // Deterministic: repeated calls give the same election.
    for _ in 0..5 {
        assert_eq!(voterank(&g, 4), first);
    }
}

// Regression: personalized PageRank redistributed the rank mass of dangling
// nodes uniformly instead of by the personalization vector. With no edges every
// node is dangling, so all mass teleports by personalization and the result must
// equal the (normalized) personalization vector exactly.
#[test]
#[cfg(feature = "centrality")]
fn test_personalized_pagerank_dangling_uses_personalization() {
    use graphina::centrality::personalized::personalized_pagerank;

    let mut g = Graph::<i32, f64>::new();
    let nodes: Vec<_> = (0..3).map(|i| g.add_node(i)).collect();

    let p = vec![0.5, 0.3, 0.2];
    let pr = personalized_pagerank(&g, Some(p.clone()), 0.85, 1e-12, 2000)
        .expect("personalized pagerank should succeed");
    for (i, &want) in p.iter().enumerate() {
        assert!(
            (pr[&nodes[i]] - want).abs() < 1e-9,
            "node {i}: expected {want}, got {}",
            pr[&nodes[i]]
        );
    }
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

// Regression: prim_mst dropped edges incident to a freshly added node when the
// edge was stored with that node as the target. On a connected graph it
// returned a partial tree (here 2 edges instead of 5). The spanning tree of
// this connected, 6-node graph must have 5 edges and total weight 19.
#[test]
#[cfg(feature = "mst")]
fn test_prim_mst_undirected_target_edges() {
    use graphina::mst::{kruskal_mst, prim_mst};
    use ordered_float::OrderedFloat;

    let mut g: Graph<i32, OrderedFloat<f64>> = Graph::new();
    let nodes: Vec<_> = (0..6).map(|i| g.add_node(i)).collect();
    for (u, v, w) in [
        (0, 4, 5.0),
        (0, 5, 2.0),
        (1, 5, 1.0),
        (2, 4, 10.0),
        (3, 4, 1.0),
    ] {
        g.add_edge(nodes[u], nodes[v], OrderedFloat(w));
    }

    let (prim_edges, prim_weight) = prim_mst(&g).unwrap();
    assert_eq!(prim_edges.len(), 5);
    assert_eq!(prim_weight, OrderedFloat(19.0));

    let (kruskal_edges, kruskal_weight) = kruskal_mst(&g).unwrap();
    assert_eq!(prim_edges.len(), kruskal_edges.len());
    assert_eq!(prim_weight, kruskal_weight);
}

// Regression: boruvka_mst used the raw union-find parent pointer instead of the
// canonical root to group nodes by component. After the first round the parent
// array is not path-compressed, so cheapest-edge selection mis-grouped nodes,
// missed valid merges, and returned a forest with too few edges (here 9 instead
// of 10). This connected, 11-node graph must yield a spanning tree of 10 edges
// and total weight 25.
#[test]
#[cfg(feature = "mst")]
fn test_boruvka_mst_canonical_root_grouping() {
    use graphina::mst::{boruvka_mst, kruskal_mst};
    use ordered_float::OrderedFloat;

    let edges = [
        (0, 2, 4.0),
        (0, 3, 1.0),
        (0, 4, 4.0),
        (0, 5, 4.0),
        (0, 6, 3.0),
        (1, 2, 8.0),
        (1, 3, 6.0),
        (1, 4, 5.0),
        (1, 5, 4.0),
        (1, 6, 10.0),
        (1, 7, 1.0),
        (1, 8, 7.0),
        (2, 3, 7.0),
        (2, 4, 7.0),
        (2, 5, 9.0),
        (2, 8, 8.0),
        (2, 9, 1.0),
        (2, 10, 3.0),
        (3, 4, 9.0),
        (3, 5, 10.0),
        (3, 10, 5.0),
        (4, 6, 5.0),
        (4, 9, 7.0),
        (4, 10, 5.0),
        (5, 6, 7.0),
        (5, 7, 7.0),
        (5, 8, 5.0),
        (5, 9, 4.0),
        (5, 10, 5.0),
        (6, 7, 6.0),
        (6, 8, 2.0),
        (6, 9, 5.0),
        (6, 10, 4.0),
        (7, 9, 2.0),
        (7, 10, 9.0),
        (8, 10, 9.0),
        (9, 10, 10.0),
    ];

    let mut g: Graph<i32, OrderedFloat<f64>> = Graph::new();
    let nodes: Vec<_> = (0..11).map(|i| g.add_node(i)).collect();
    for (u, v, w) in edges {
        g.add_edge(nodes[u], nodes[v], OrderedFloat(w));
    }

    let (boruvka_edges, boruvka_weight) = boruvka_mst(&g).unwrap();
    assert_eq!(boruvka_edges.len(), 10);
    assert_eq!(boruvka_weight, OrderedFloat(25.0));

    let (kruskal_edges, kruskal_weight) = kruskal_mst(&g).unwrap();
    assert_eq!(boruvka_edges.len(), kruskal_edges.len());
    assert_eq!(boruvka_weight, kruskal_weight);
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

#[test]
#[cfg(feature = "metrics")]
fn test_assortativity_is_symmetric_newman_coefficient() {
    // Degree assortativity is the Pearson correlation over the symmetric joint
    // degree distribution of edge endpoints: each undirected edge contributes
    // both orderings. A star is perfectly disassortative (a degree-k hub joined
    // only to degree-1 leaves), so its coefficient is exactly -1.0. The earlier
    // implementation correlated a single edge ordering, giving the two endpoints
    // different means; on a star that collapsed the variance to zero and
    // returned 0.0 instead of -1.0.
    use graphina::metrics::assortativity;

    let mut g: Graph<i32, f64> = Graph::new();
    let center = g.add_node(0);
    for i in 1..=4 {
        let leaf = g.add_node(i);
        g.add_edge(center, leaf, 1.0);
    }

    let r = assortativity(&g);
    assert!(
        (r - (-1.0)).abs() < 1e-9,
        "star degree assortativity should be -1.0, got {r}"
    );
}
