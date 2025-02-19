use network::graph::{Digraph, Graph, NodeId};

#[test]
fn integration_test_digraph() {
    // Create a directed graph with integer node attributes and f32 edge weights.
    let mut dgraph = Digraph::<i32, f32>::new();
    let n1 = dgraph.add_node(1);
    let n2 = dgraph.add_node(2);
    let n3 = dgraph.add_node(3);

    let e1 = dgraph.add_edge(n1, n2, 1.0);
    let e2 = dgraph.add_edge(n2, n3, 2.0);
    let e3 = dgraph.add_edge(n3, n1, 3.0);

    // Verify counts.
    assert_eq!(dgraph.node_count(), 3);
    assert_eq!(dgraph.edge_count(), 3);

    // For a directed graph, neighbors are those with outgoing edges.
    let neighbors_n1: Vec<NodeId> = dgraph.neighbors(n1).collect();
    assert!(neighbors_n1.contains(&n2));
    assert!(!neighbors_n1.contains(&n3));

    // Verify node attributes.
    assert_eq!(*dgraph.node_attr(n1).unwrap(), 1);
    assert_eq!(*dgraph.node_attr(n2).unwrap(), 2);
    assert_eq!(*dgraph.node_attr(n3).unwrap(), 3);

    // Verify edge attributes.
    assert_eq!(*dgraph.edge_attr(e1).unwrap(), 1.0);
    assert_eq!(*dgraph.edge_attr(e2).unwrap(), 2.0);
    assert_eq!(*dgraph.edge_attr(e3).unwrap(), 3.0);

    // Update node attribute and verify.
    dgraph.update_node(n1, 10);
    assert_eq!(*dgraph.node_attr(n1).unwrap(), 10);
}

#[test]
fn integration_test_graph() {
    // Create an undirected graph with string node attributes and unweighted edges.
    let mut graph = Graph::<&str, ()>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    let e1 = graph.add_edge(a, b, ());
    let e2 = graph.add_edge(b, c, ());
    let e3 = graph.add_edge(c, a, ());

    // Verify counts.
    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 3);

    // In an undirected graph, neighbors should include all connected nodes.
    let neighbors_a: Vec<NodeId> = graph.neighbors(a).collect();
    assert!(neighbors_a.contains(&b));
    assert!(neighbors_a.contains(&c));

    // Verify node attributes.
    assert_eq!(graph.node_attr(a).unwrap(), &"A");
    assert_eq!(graph.node_attr(b).unwrap(), &"B");
    assert_eq!(graph.node_attr(c).unwrap(), &"C");

    // For unweighted edges, edge_attr returns the unit type `()`.
    assert_eq!(graph.edge_attr(e1).unwrap(), &());
    assert_eq!(graph.edge_attr(e2).unwrap(), &());
    assert_eq!(graph.edge_attr(e3).unwrap(), &());

    // Update node attribute and verify.
    graph.update_node(a, "Alpha");
    assert_eq!(graph.node_attr(a).unwrap(), &"Alpha");
}
