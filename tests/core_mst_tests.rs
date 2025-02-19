use graphina::core::mst::{boruvka_mst, kruskal_mst, prim_mst};
use graphina::core::types::Graph;
use ordered_float::OrderedFloat;

/// Builds a connected undirected graph with 4 nodes.
/// Graph structure (nodes 0,1,2,3):
///   0 -- 1: 1.0
///   0 -- 2: 2.0
///   1 -- 2: 2.0
///   1 -- 3: 3.0
///   2 -- 3: 1.0
/// The optimal MST should have 3 edges with total weight: 1.0 + 1.0 + 2.0 = 4.0.
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

/// Builds a disconnected undirected graph with 4 nodes divided into 2 components:
/// Component 1: nodes 0 and 1 connected by an edge with weight 1.0.
/// Component 2: nodes 2 and 3 connected by an edge with weight 2.0.
/// The MST forest should have (n - k) = 4 - 2 = 2 edges with total weight: 1.0 + 2.0 = 3.0.
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
    let (mst_edges, total_weight) = boruvka_mst(&graph);
    // For 4 connected nodes, MST should have 3 edges.
    assert_eq!(mst_edges.len(), 3, "Boruvka MST should have 3 edges");
    // Expected total weight: 1.0 + 1.0 + 2.0 = 4.0.
    assert!(
        (total_weight.0 - 4.0).abs() < 1e-6,
        "Boruvka MST total weight expected to be 4.0, got {}",
        total_weight.0
    );
}

#[test]
fn test_kruskal_mst_connected() {
    let graph = build_connected_graph();
    let (mst_edges, total_weight) = kruskal_mst(&graph);
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
    let (mst_edges, total_weight) = prim_mst(&graph);
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
    let (mst_edges, total_weight) = boruvka_mst(&graph);
    // For a disconnected graph with 4 nodes and 2 components, MST forest should have 2 edges.
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
    let (mst_edges, total_weight) = kruskal_mst(&graph);
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
    let (mst_edges, total_weight) = prim_mst(&graph);
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
