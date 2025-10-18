use graphina::core::paths::dijkstra;
use graphina::core::types::Digraph;
use ordered_float::OrderedFloat;

#[test]
fn dijkstra_handles_noncontiguous_node_indices() {
    // Build a small graph
    let mut g: Digraph<i32, OrderedFloat<f64>> = Digraph::new();
    let n0 = g.add_node(0);
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);

    // Edges: 0->1 (1.0), 1->2 (1.0), 0->2 (3.0)
    g.add_edge(n0, n1, OrderedFloat(1.0));
    g.add_edge(n1, n2, OrderedFloat(1.0));
    g.add_edge(n0, n2, OrderedFloat(3.0));

    // Remove node n1 to create a gap in StableGraph indices
    let _ = g.remove_node(n1);

    // Run Dijkstra from n0 and assert distance to n2 is still correct
    let dist = dijkstra(&g, n0).expect("dijkstra should succeed on nonnegative weights");
    assert_eq!(dist[&n0], Some(OrderedFloat(0.0)));
    assert_eq!(dist[&n2], Some(OrderedFloat(3.0)));

    // NodeId for removed node n1 should not be present in the NodeMap
    assert!(dist.get(&n1).is_none());
}
