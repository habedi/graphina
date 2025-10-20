//! Integration tests for architectural fixes and bug corrections made in 2025
//!
//! This test suite validates critical fixes including:
//! - Connected components performance bug (O(n*m) -> O(n+m))
//! - Non-contiguous node index handling
//! - Validation utilities
//! - GraphinaGraph trait consistency

use graphina::core::types::{Digraph, Graph, NodeId};
use graphina::core::validation::{
    count_components, has_negative_weights, is_bipartite, is_connected, is_dag, is_empty,
    validate_connected, validate_is_dag, validate_node_exists, validate_non_empty,
    validate_non_negative_weights,
};

#[cfg(feature = "community")]
use graphina::community::connected_components::connected_components;

#[test]
#[cfg(feature = "community")]
fn test_connected_components_performance_fix() {
    // Previous implementation had O(n*m) complexity due to iterating
    // over all edges for each node. This should now be O(n+m).
    let mut g = Graph::<i32, f64>::new();

    // Create a large graph with multiple components
    let mut nodes = Vec::new();
    for i in 0..1000 {
        nodes.push(g.add_node(i));
    }

    // Create 10 components of 100 nodes each
    for comp in 0..10 {
        for i in 0..99 {
            let idx1 = comp * 100 + i;
            let idx2 = comp * 100 + i + 1;
            g.add_edge(nodes[idx1], nodes[idx2], 1.0);
        }
    }

    let components = connected_components(&g);
    assert_eq!(components.len(), 10);

    for component in &components {
        assert_eq!(component.len(), 100);
    }
}

#[test]
#[cfg(feature = "community")]
fn test_connected_components_non_contiguous_indices() {
    // Bug fix: Handle graphs where nodes have been removed (non-contiguous indices)
    let mut g = Graph::<i32, f64>::new();
    let nodes: Vec<NodeId> = (0..20).map(|i| g.add_node(i)).collect();

    // Create two components: chain 0-1-2-3-4-5-6-7-8-9 and chain 10-11-12-13-14-15-16-17-18-19
    for i in 0..9 {
        g.add_edge(nodes[i], nodes[i + 1], 1.0);
    }
    for i in 10..19 {
        g.add_edge(nodes[i], nodes[i + 1], 1.0);
    }

    // Remove nodes that split the chains
    // Removing node 2 splits first chain into: {0,1} and {3,4,5,6,7,8,9}
    // Removing node 5 splits second part into: {3,4} and {6,7,8,9}
    // Removing node 12 splits second chain into: {10,11} and {13,14,15,16,17,18,19}
    // Removing node 15 splits second part into: {13,14} and {16,17,18,19}
    g.remove_node(nodes[2]);
    g.remove_node(nodes[5]);
    g.remove_node(nodes[12]);
    g.remove_node(nodes[15]);

    // After removals we have 6 components: {0,1}, {3,4}, {6,7,8,9}, {10,11}, {13,14}, {16,17,18,19}
    let components = connected_components(&g);
    assert_eq!(components.len(), 6);

    // Verify no component contains a removed node
    for component in &components {
        for &node in component {
            assert!(g.contains_node(node), "Component contains removed node");
        }
    }
}

#[test]
fn test_validation_utilities_empty_graph() {
    let g = Graph::<i32, f64>::new();

    assert!(is_empty(&g));
    assert!(!is_connected(&g));
    assert_eq!(count_components(&g), 0);

    assert!(validate_non_empty(&g).is_err());
    assert!(validate_connected(&g).is_err());
}

#[test]
fn test_validation_utilities_single_node() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);

    assert!(!is_empty(&g));
    assert!(is_connected(&g));
    assert_eq!(count_components(&g), 1);

    assert!(validate_non_empty(&g).is_ok());
    assert!(validate_node_exists(&g, n1).is_ok());
}

#[test]
fn test_validation_negative_weights() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);

    g.add_edge(n1, n2, 5.0);
    assert!(!has_negative_weights(&g));
    assert!(validate_non_negative_weights(&g).is_ok());

    g.add_edge(n2, n1, -2.5);
    assert!(has_negative_weights(&g));
    assert!(validate_non_negative_weights(&g).is_err());
}

#[test]
fn test_validation_dag() {
    let mut g = Digraph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    let n4 = g.add_node(4);

    // Build a DAG
    g.add_edge(n1, n2, 1.0);
    g.add_edge(n1, n3, 1.0);
    g.add_edge(n2, n4, 1.0);
    g.add_edge(n3, n4, 1.0);

    assert!(is_dag(&g));
    assert!(validate_is_dag(&g).is_ok());

    // Add cycle
    g.add_edge(n4, n1, 1.0);
    assert!(!is_dag(&g));
    assert!(validate_is_dag(&g).is_err());
}

#[test]
fn test_bipartite_detection() {
    let mut g = Graph::<i32, f64>::new();

    // Create K_{3,3} (complete bipartite graph)
    let left: Vec<NodeId> = (0..3).map(|i| g.add_node(i)).collect();
    let right: Vec<NodeId> = (3..6).map(|i| g.add_node(i)).collect();

    for &l in &left {
        for &r in &right {
            g.add_edge(l, r, 1.0);
        }
    }

    assert!(is_bipartite(&g));

    // Add edge within left partition - breaks bipartiteness
    g.add_edge(left[0], left[1], 1.0);
    assert!(!is_bipartite(&g));
}

#[test]
fn test_graph_density_calculation() {
    let mut g = Graph::<i32, f64>::new();

    // Empty graph
    assert_eq!(g.density(), 0.0);

    // Single node
    g.add_node(1);
    assert_eq!(g.density(), 0.0);

    // Complete graph K_4 on a fresh graph (avoid the extra single node)
    let mut k4 = Graph::<i32, f64>::new();
    let nodes: Vec<NodeId> = (0..4).map(|i| k4.add_node(i)).collect();
    for i in 0..4 {
        for j in (i + 1)..4 {
            k4.add_edge(nodes[i], nodes[j], 1.0);
        }
    }

    // K_4 has 6 edges, max possible is 4*3/2 = 6 for undirected
    // Density = 2*6 / (4*3) = 1.0
    assert!((k4.density() - 1.0).abs() < 1e-10);
}

#[test]
fn test_directed_graph_density() {
    let mut g = Digraph::<i32, f64>::new();
    let nodes: Vec<NodeId> = (0..4).map(|i| g.add_node(i)).collect();

    // Add all possible directed edges
    for i in 0..4 {
        for j in 0..4 {
            if i != j {
                g.add_edge(nodes[i], nodes[j], 1.0);
            }
        }
    }

    // Complete directed graph: 12 edges, max possible is 4*3 = 12
    // Density = 12 / 12 = 1.0
    assert!((g.density() - 1.0).abs() < 1e-10);
}

#[test]
fn test_node_removal_edge_cleanup() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 2.0);
    g.add_edge(n1, n3, 3.0);

    assert_eq!(g.node_count(), 3);
    assert_eq!(g.edge_count(), 3);

    // Remove middle node
    g.remove_node(n2);

    assert_eq!(g.node_count(), 2);
    // Edges involving n2 should be removed
    assert_eq!(g.edge_count(), 1);

    assert!(g.contains_node(n1));
    assert!(!g.contains_node(n2));
    assert!(g.contains_node(n3));
}

#[test]
fn test_degree_calculations_consistency() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n1, n3, 1.0);
    g.add_edge(n2, n3, 1.0);

    // For undirected graphs, degree = in_degree = out_degree
    assert_eq!(g.degree(n1), Some(2));
    assert_eq!(g.in_degree(n1), Some(2));
    assert_eq!(g.out_degree(n1), Some(2));

    // Node with highest degree
    let degrees: Vec<usize> = vec![n1, n2, n3]
        .iter()
        .map(|&n| g.degree(n).unwrap())
        .collect();
    assert_eq!(degrees, vec![2, 2, 2]); // Complete triangle
}

#[test]
fn test_directed_degree_calculations() {
    let mut g = Digraph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    // n1 -> n2 -> n3 -> n1 (cycle)
    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);
    g.add_edge(n3, n1, 1.0);

    // Each node has in_degree=1, out_degree=1, total_degree=2
    assert_eq!(g.in_degree(n1), Some(1));
    assert_eq!(g.out_degree(n1), Some(1));
    assert_eq!(g.degree(n1), Some(2));

    assert_eq!(g.in_degree(n2), Some(1));
    assert_eq!(g.out_degree(n2), Some(1));
    assert_eq!(g.degree(n2), Some(2));
}

#[test]
fn test_graph_clear() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    g.add_edge(n1, n2, 1.0);

    assert_eq!(g.node_count(), 2);
    assert_eq!(g.edge_count(), 1);

    g.clear();

    assert_eq!(g.node_count(), 0);
    assert_eq!(g.edge_count(), 0);
    assert!(g.is_empty());
}

#[test]
fn test_bulk_operations() {
    let mut g = Graph::<i32, f64>::new();

    // Bulk add nodes
    let nodes = g.add_nodes_bulk(&[1, 2, 3, 4, 5]);
    assert_eq!(nodes.len(), 5);
    assert_eq!(g.node_count(), 5);

    // Bulk add edges
    let edges = g.add_edges_bulk(&[
        (nodes[0], nodes[1], 1.0),
        (nodes[1], nodes[2], 2.0),
        (nodes[2], nodes[3], 3.0),
        (nodes[3], nodes[4], 4.0),
    ]);
    assert_eq!(edges.len(), 4);
    assert_eq!(g.edge_count(), 4);
}

#[test]
fn test_retain_operations() {
    let mut g = Graph::<i32, f64>::new();
    let nodes: Vec<NodeId> = (0..10).map(|i| g.add_node(i)).collect();

    for i in 0..9 {
        g.add_edge(nodes[i], nodes[i + 1], i as f64);
    }

    // Retain only even-valued nodes
    g.retain_nodes(|_id, attr| attr % 2 == 0);

    assert_eq!(g.node_count(), 5); // 0, 2, 4, 6, 8

    // Edges should be retained only if both endpoints exist
    // Original edges: 0-1, 1-2, 2-3, 3-4, 4-5, 5-6, 6-7, 7-8, 8-9
    // After filter: edges with both endpoints even remain
    assert!(g.edge_count() < 9);
}

#[test]
fn test_component_count_after_operations() {
    let mut g = Graph::<i32, f64>::new();

    // Create 3 separate components
    let c1: Vec<NodeId> = (0..5).map(|i| g.add_node(i)).collect();
    let c2: Vec<NodeId> = (5..10).map(|i| g.add_node(i)).collect();
    let c3: Vec<NodeId> = (10..15).map(|i| g.add_node(i)).collect();

    // Connect each component internally
    for i in 0..4 {
        g.add_edge(c1[i], c1[i + 1], 1.0);
        g.add_edge(c2[i], c2[i + 1], 1.0);
        g.add_edge(c3[i], c3[i + 1], 1.0);
    }

    assert_eq!(count_components(&g), 3);

    // Bridge two components
    g.add_edge(c1[4], c2[0], 1.0);
    assert_eq!(count_components(&g), 2);

    // Bridge all components
    g.add_edge(c2[4], c3[0], 1.0);
    assert_eq!(count_components(&g), 1);
    assert!(is_connected(&g));
}
