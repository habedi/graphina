//! Tests for bug fixes in the approximation module
//!
//! This test suite verifies that the approximation algorithms handle edge cases correctly,
//! particularly empty graphs and graphs with removed nodes.

#[cfg(test)]
mod approximation_bug_fixes {
    use graphina::approximation::clique::{clique_removal, large_clique_size, max_clique};
    use graphina::approximation::treewidth::{treewidth_min_degree, treewidth_min_fill_in};
    use graphina::core::types::Graph;

    /// Test that max_clique handles empty graphs without panicking
    #[test]
    fn test_max_clique_empty_graph() {
        let graph: Graph<i32, f64> = Graph::new();
        let clique = max_clique(&graph);
        assert!(clique.is_empty(), "Empty graph should return empty clique");
    }

    /// Test that max_clique handles single-node graphs correctly
    #[test]
    fn test_max_clique_single_node() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let clique = max_clique(&graph);
        assert_eq!(clique.len(), 1);
        assert!(clique.contains(&n1));
    }

    /// Test that max_clique handles graphs with isolated nodes
    #[test]
    fn test_max_clique_isolated_nodes() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        // Only connect n1 and n2
        graph.add_edge(n1, n2, 1.0);

        let clique = max_clique(&graph);
        assert!(clique.len() >= 1, "Should find at least one node");
        assert!(
            clique.len() <= 2,
            "Clique cannot be larger than connected component"
        );
    }

    /// Test that max_clique finds a complete triangle
    #[test]
    fn test_max_clique_complete_triangle() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        // Create a complete triangle
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n1, 1.0);

        let clique = max_clique(&graph);
        assert_eq!(
            clique.len(),
            3,
            "Triangle should be detected as a clique of size 3"
        );
    }

    /// Test that max_clique handles graphs with deleted nodes
    #[test]
    fn test_max_clique_with_deleted_nodes() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);

        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n4, 1.0);

        // Remove a node
        graph.remove_node(n2);

        // Should not panic
        let clique = max_clique(&graph);
        assert!(
            !clique.contains(&n2),
            "Deleted node should not be in clique"
        );
    }

    /// Test clique_removal on empty graph
    #[test]
    fn test_clique_removal_empty() {
        let graph: Graph<i32, f64> = Graph::new();
        let cliques = clique_removal(&graph);
        assert!(cliques.is_empty(), "Empty graph should return no cliques");
    }

    /// Test clique_removal on simple graph
    #[test]
    fn test_clique_removal_simple() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        graph.add_edge(n1, n2, 1.0);

        let cliques = clique_removal(&graph);
        assert!(!cliques.is_empty(), "Should find at least one clique");
    }

    /// Test large_clique_size on various graphs
    #[test]
    fn test_large_clique_size() {
        // Empty graph
        let graph: Graph<i32, f64> = Graph::new();
        assert_eq!(large_clique_size(&graph), 0);

        // Single node
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        assert_eq!(large_clique_size(&graph), 1);

        // Two connected nodes
        let n2 = graph.add_node(2);
        graph.add_edge(n1, n2, 1.0);
        assert_eq!(large_clique_size(&graph), 2);
    }

    /// Test treewidth_min_degree on empty graph
    #[test]
    fn test_treewidth_min_degree_empty() {
        let graph: Graph<i32, f64> = Graph::new();
        let (tw, order) = treewidth_min_degree(&graph);
        assert_eq!(tw, 0, "Empty graph should have treewidth 0");
        assert!(order.is_empty(), "Empty graph should return empty order");
    }

    /// Test treewidth_min_degree on single node
    #[test]
    fn test_treewidth_min_degree_single_node() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let (tw, order) = treewidth_min_degree(&graph);
        assert_eq!(tw, 0, "Single node should have treewidth 0");
        assert_eq!(order.len(), 1);
        assert_eq!(order[0], n1);
    }

    /// Test treewidth_min_degree on path graph
    #[test]
    fn test_treewidth_min_degree_path() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);

        let (tw, order) = treewidth_min_degree(&graph);
        assert!(tw <= 1, "Path graph should have treewidth at most 1");
        assert_eq!(order.len(), 3);
    }

    /// Test treewidth_min_degree on complete graph K4
    #[test]
    fn test_treewidth_min_degree_complete() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);

        // Complete graph K4
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n1, n3, 1.0);
        graph.add_edge(n1, n4, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n2, n4, 1.0);
        graph.add_edge(n3, n4, 1.0);

        let (tw, order) = treewidth_min_degree(&graph);
        assert_eq!(tw, 3, "Complete graph K4 should have treewidth 3");
        assert_eq!(order.len(), 4);
    }

    /// Test treewidth_min_fill_in on empty graph
    #[test]
    fn test_treewidth_min_fill_in_empty() {
        let graph: Graph<i32, f64> = Graph::new();
        let (tw, order) = treewidth_min_fill_in(&graph);
        assert_eq!(tw, 0);
        assert!(order.is_empty());
    }

    /// Test treewidth_min_fill_in on single node
    #[test]
    fn test_treewidth_min_fill_in_single_node() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let (tw, order) = treewidth_min_fill_in(&graph);
        assert_eq!(tw, 0);
        assert_eq!(order.len(), 1);
    }

    /// Test treewidth algorithms with isolated nodes
    #[test]
    fn test_treewidth_isolated_nodes() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        // Only connect n1 and n2, n3 is isolated
        graph.add_edge(n1, n2, 1.0);

        let (tw1, order1) = treewidth_min_degree(&graph);
        assert_eq!(order1.len(), 3);

        let (tw2, order2) = treewidth_min_fill_in(&graph);
        assert_eq!(order2.len(), 3);

        // Both should handle isolated nodes without panicking
        assert!(tw1 <= 1);
        assert!(tw2 <= 1);
    }

    /// Test that treewidth algorithms handle graphs with deleted nodes
    #[test]
    fn test_treewidth_with_deleted_nodes() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);

        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n4, 1.0);

        // Remove a node
        graph.remove_node(n2);

        // Should not panic
        let (tw1, order1) = treewidth_min_degree(&graph);
        let (tw2, order2) = treewidth_min_fill_in(&graph);

        assert!(!order1.contains(&n2));
        assert!(!order2.contains(&n2));
        assert_eq!(order1.len(), 3);
        assert_eq!(order2.len(), 3);
    }
}
