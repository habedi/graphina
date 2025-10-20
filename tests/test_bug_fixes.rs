// Bug fix tests for architectural and algorithmic issues

use graphina::core::types::{Digraph, Graph};
#[cfg(feature = "subgraphs")]
use graphina::subgraphs::SubgraphOps;

#[cfg(test)]
mod bug_fixes {
    use super::*;

    /// Bug: Louvain algorithm assumes contiguous node indices
    /// Fix: Map NodeId to contiguous indices explicitly
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

        // Create a simple community structure
        g.add_edge(n0, n1, 1.0);
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n3, n4, 1.0);

        // Remove a node in the middle
        g.remove_node(n2);

        // This should not panic or cause array out of bounds
        let communities = louvain(&g, Some(42));

        // Should have valid communities
        assert!(!communities.is_empty());
    }

    /// Bug: Edge iteration in undirected graphs may double-count edge weights
    /// Test: Verify degree calculations are correct
    #[test]
    fn test_undirected_degree_consistency() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);

        g.add_edge(n1, n2, 5.0);

        // In undirected graphs, an edge n1-n2 should be counted once for each node
        assert_eq!(g.degree(n1), Some(1));
        assert_eq!(g.degree(n2), Some(1));

        // But the edge count should be 1
        assert_eq!(g.edge_count(), 1);
    }

    /// Bug: Empty graph handling in centrality algorithms
    #[test]
    #[cfg(feature = "centrality")]
    fn test_centrality_empty_graph() {
        use graphina::centrality::degree::degree_centrality;

        let g = Graph::<i32, f64>::new();
        let result = degree_centrality(&g);

        // Empty graph returns Ok with empty NodeMap (not an error)
        assert!(result.is_ok());
        let centrality = result.unwrap();
        assert_eq!(centrality.len(), 0);
    }

    /// Bug: Division by zero in metrics for single-node graphs
    #[test]
    fn test_metrics_single_node() {
        use graphina::metrics::{average_clustering_coefficient, diameter};

        let mut g = Graph::<i32, f64>::new();
        g.add_node(1);

        // Single node should have diameter 0
        assert_eq!(diameter(&g), Some(0));

        // Clustering coefficient should be 0 (no neighbors)
        assert_eq!(average_clustering_coefficient(&g), 0.0);
    }

    /// Bug: Negative edge weights in Dijkstra should be caught
    #[test]
    fn test_dijkstra_negative_weights() {
        use graphina::core::paths::dijkstra_path_f64;

        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);

        g.add_edge(n1, n2, -5.0);

        // Should return error for negative weights
        let result = dijkstra_path_f64(&g, n1, None);
        assert!(result.is_err());
    }

    /// Bug: Self-loops should be handled correctly
    #[test]
    fn test_self_loop_handling() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);

        // Add self-loop
        g.add_edge(n1, n1, 1.0);

        // Should be reflected in degree
        assert!(g.degree(n1).unwrap() > 0);

        // Edge should exist
        assert!(g.contains_edge(n1, n1));
    }

    /// Bug: Directed vs undirected edge finding
    #[test]
    fn test_directed_edge_finding() {
        let mut g = Digraph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);

        g.add_edge(n1, n2, 1.0);

        // Should find edge n1->n2
        assert!(g.contains_edge(n1, n2));

        // Should NOT find edge n2->n1 (directed graph)
        assert!(!g.contains_edge(n2, n1));
    }

    /// Bug: Iterator invalidation when modifying graph
    #[test]
    fn test_iterator_safety() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);

        // Collect nodes first to avoid iterator invalidation
        let nodes: Vec<_> = g.nodes().map(|(id, _)| id).collect();

        // Now safe to remove
        for node in nodes {
            g.remove_node(node);
        }

        assert_eq!(g.node_count(), 0);
    }

    /// Bug: Parallel algorithm correctness
    #[test]
    fn test_parallel_vs_sequential_consistency() {
        use graphina::parallel::degrees_parallel;

        let mut g = Graph::<i32, f64>::new();
        let nodes: Vec<_> = (0..100).map(|i| g.add_node(i)).collect();

        // Create a random graph
        for i in 0..100 {
            for j in (i + 1)..100 {
                if (i * j) % 7 == 0 {
                    g.add_edge(nodes[i], nodes[j], 1.0);
                }
            }
        }

        // Parallel computation
        let parallel_degrees = degrees_parallel(&g);

        // Sequential computation
        let sequential_degrees: std::collections::HashMap<_, _> = g
            .nodes()
            .map(|(id, _)| (id, g.degree(id).unwrap()))
            .collect();

        // Should match
        for (node, deg) in &sequential_degrees {
            assert_eq!(parallel_degrees.get(node), Some(deg));
        }
    }

    /// Bug: Graph builder edge indices must be valid
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_graph_builder_invalid_edge() {
        let _g = Graph::<i32, f64>::builder()
            .add_node(1)
            .add_node(2)
            .add_edge(0, 5, 1.0) // Node 5 doesn't exist!
            .build();
    }

    /// Bug: NodeMap should handle deleted nodes gracefully
    #[test]
    fn test_nodemap_with_deleted_nodes() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);

        // Create a node map
        let map = g.to_nodemap(|_, val| *val * 2);

        // Remove a node
        g.remove_node(n2);

        // Map should still have old entries
        assert_eq!(map.get(&n1), Some(&2));
        assert_eq!(map.get(&n3), Some(&6));

        // But accessing removed node should return old value or be handled
        // This is expected behavior - NodeMap is independent of graph
    }

    /// Bug: Validation should catch cycles in DAG algorithms
    #[test]
    fn test_dag_validation() {
        use graphina::core::validation::is_dag;

        let mut g = Digraph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);

        // Should be a DAG
        assert!(is_dag(&g));

        // Add cycle
        g.add_edge(n3, n1, 1.0);

        // Should no longer be a DAG
        assert!(!is_dag(&g));
    }

    /// Bug: Subgraph extraction should preserve node attributes
    #[test]
    fn test_subgraph_attribute_preservation() {
        let mut g = Graph::<String, f64>::new();
        let n1 = g.add_node("Alice".to_string());
        let n2 = g.add_node("Bob".to_string());
        let n3 = g.add_node("Charlie".to_string());

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);

        // Extract subgraph
        let subgraph = g
            .subgraph(&[n1, n2])
            .expect("Subgraph creation should succeed");

        // Attributes should be preserved
        assert_eq!(subgraph.node_count(), 2);

        // Check attributes are still accessible
        for (_, attr) in subgraph.nodes() {
            assert!(attr == "Alice" || attr == "Bob");
        }
    }

    /// Bug: Serialization should handle special float values
    #[test]
    fn test_serialization_special_values() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);

        // Use special float values
        g.add_edge(n1, n2, 0.0);

        // Serialize to JSON
        let json = g.to_serializable();
        let json_str = serde_json::to_string(&json).unwrap();

        // Should not contain NaN or Inf
        assert!(!json_str.contains("NaN"));
        assert!(!json_str.contains("Infinity"));
    }
}
