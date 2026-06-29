//! Regression tests to verify bug fixes stay fixed and features continue working.

use graphina::core::types::Graph;

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
