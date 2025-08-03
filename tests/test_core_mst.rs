use graphina::core::mst::{boruvka_mst, kruskal_mst, prim_mst};
use graphina::core::types::Graph;
use ordered_float::OrderedFloat;

fn build_connected_graph() -> Graph<i32, OrderedFloat<f64>> {
    let mut graph: Graph<i32, OrderedFloat<f64>> = Graph::default();
    let n0 = graph.add_node(0);
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);

    graph.add_edge(n0, n1, OrderedFloat(1.0));
    graph.add_edge(n0, n2, OrderedFloat(2.0));
    graph.add_edge(n1, n2, OrderedFloat(2.0));
    graph.add_edge(n1, n3, OrderedFloat(3.0));
    graph.add_edge(n2, n3, OrderedFloat(1.0));

    graph
}

fn build_disconnected_graph() -> Graph<i32, OrderedFloat<f64>> {
    let mut graph: Graph<i32, OrderedFloat<f64>> = Graph::default();
    let n0 = graph.add_node(0);
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);

    graph.add_edge(n0, n1, OrderedFloat(1.0));
    graph.add_edge(n2, n3, OrderedFloat(2.0));

    graph
}

#[test]
fn test_boruvka_mst_connected() {
    let graph = build_connected_graph();
    let (mst_edges, total_weight) = boruvka_mst(&graph).unwrap();
    assert_eq!(mst_edges.len(), 3, "Boruvka MST should have 3 edges");
    assert!(
        (total_weight.0 - 4.0).abs() < 1e-6,
        "Boruvka MST total weight expected to be 4.0, got {}",
        total_weight.0
    );
}

#[test]
fn test_kruskal_mst_connected() {
    let graph = build_connected_graph();
    let (mst_edges, total_weight) = kruskal_mst(&graph).unwrap();
    assert_eq!(mst_edges.len(), 3, "Kruskal MST should have 3 edges");
    assert!(
        (total_weight.0 - 4.0).abs() < 1e-6,
        "Kruskal MST total weight expected to be 4.0, got {}",
        total_weight.0
    );
}

#[test]
fn test_prim_mst_connected() {
    let graph = build_connected_graph();
    let (mst_edges, total_weight) = prim_mst(&graph).unwrap();
    assert_eq!(mst_edges.len(), 3, "Prim MST should have 3 edges");
    assert!(
        (total_weight.0 - 4.0).abs() < 1e-6,
        "Prim MST total weight expected to be 4.0, got {}",
        total_weight.0
    );
}

#[test]
fn test_boruvka_mst_disconnected() {
    let graph = build_disconnected_graph();
    let (mst_edges, total_weight) = boruvka_mst(&graph).unwrap();
    assert_eq!(
        mst_edges.len(),
        2,
        "Boruvka MST in disconnected graph should have 2 edges"
    );
    assert!(
        (total_weight.0 - 3.0).abs() < 1e-6,
        "Boruvka MST total weight expected to be 3.0, got {}",
        total_weight.0
    );
}

#[test]
fn test_kruskal_mst_disconnected() {
    let graph = build_disconnected_graph();
    let (mst_edges, total_weight) = kruskal_mst(&graph).unwrap();
    assert_eq!(
        mst_edges.len(),
        2,
        "Kruskal MST in disconnected graph should have 2 edges"
    );
    assert!(
        (total_weight.0 - 3.0).abs() < 1e-6,
        "Kruskal MST total weight expected to be 3.0, got {}",
        total_weight.0
    );
}

#[test]
fn test_prim_mst_disconnected() {
    let graph = build_disconnected_graph();
    let (mst_edges, total_weight) = prim_mst(&graph).unwrap();
    assert_eq!(
        mst_edges.len(),
        2,
        "Prim MST in disconnected graph should have 2 edges"
    );
    assert!(
        (total_weight.0 - 3.0).abs() < 1e-6,
        "Prim MST total weight expected to be 3.0, got {}",
        total_weight.0
    );
}
