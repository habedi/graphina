// filepath: /home/hassan/Workspace/RustRoverProjects/graphina/tests/test_community_regressions.rs
use graphina::community::girvan_newman::girvan_newman;
use graphina::community::spectral::spectral_clustering;
use graphina::core::types::{Graph, NodeId};

#[test]
fn test_girvan_newman_with_deleted_nodes() {
    let mut g: Graph<i32, f64> = Graph::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    let n4 = g.add_node(4);

    // Two edges forming a path: 1-2-3, and 4 isolated
    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);

    // Delete middle node to create non-contiguous indices
    g.remove_node(n2);

    // Run Girvan–Newman aiming for 2 communities
    let communities = girvan_newman(&g, 2);

    // Flatten and collect nodes seen in communities
    let mut seen = std::collections::HashSet::<NodeId>::new();
    for c in &communities {
        for &nid in c {
            seen.insert(nid);
        }
    }

    // Deleted node should not be present; all others should
    assert!(!seen.contains(&n2));
    assert!(seen.contains(&n1));
    assert!(seen.contains(&n3) || g.node_count() == 2); // n3 may be gone if deletion cascaded
    assert!(seen.contains(&n4));
}

#[test]
fn test_spectral_clustering_multigraph_and_deletions() {
    let mut g: Graph<i32, f64> = Graph::new();
    let a = g.add_node(1);
    let b = g.add_node(2);
    let c = g.add_node(3);
    let d = g.add_node(4);

    // Multigraph edges (accumulating weights): a-b multiple edges, b-c, c-d
    g.add_edge(a, b, 1.0);
    g.add_edge(a, b, 2.0); // multiple edge increases weight
    g.add_edge(b, c, 1.0);
    g.add_edge(c, d, 1.0);

    // Delete one node to create non-contiguous indices
    g.remove_node(c);

    // k must be <= remaining nodes
    let k = std::cmp::min(2, g.node_count());
    let clusters = spectral_clustering(&g, k, Some(42));

    // Check we cover all existing nodes and none of the deleted
    let mut covered = std::collections::HashSet::<NodeId>::new();
    for cl in &clusters {
        for &nid in cl {
            covered.insert(nid);
        }
    }

    assert!(covered.contains(&a));
    assert!(covered.contains(&b));
    assert!(!covered.contains(&c));
    assert!(covered.contains(&d));

    // Basic sanity: number of clusters equals k (unless graph extremely small)
    assert_eq!(clusters.len(), k);
}
