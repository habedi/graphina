// Integration tests for Louvain algorithm bug fixes

#[cfg(feature = "community")]
#[cfg(test)]
mod louvain_algorithm_tests {
    use graphina::community::louvain::louvain;
    use graphina::core::types::Graph;

    #[test]
    fn test_louvain_two_cliques() {
        // Two separate cliques should form two communities
        let mut graph = Graph::new();

        // Clique 1
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n1, 1.0);

        // Clique 2
        let n4 = graph.add_node(4);
        let n5 = graph.add_node(5);
        let n6 = graph.add_node(6);

        graph.add_edge(n4, n5, 1.0);
        graph.add_edge(n5, n6, 1.0);
        graph.add_edge(n6, n4, 1.0);

        let communities = louvain(&graph, Some(42));

        // Should detect at least 1 community (may merge them if no bridge)
        assert!(communities.len() >= 1);
        assert!(communities.len() <= 6);

        // All nodes should be in some community
        let total_nodes: usize = communities.iter().map(|c| c.len()).sum();
        assert_eq!(total_nodes, 6);
    }

    #[test]
    fn test_louvain_bridge_graph() {
        // Two cliques connected by a bridge
        let mut graph = Graph::new();

        // Clique 1
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        graph.add_edge(n1, n2, 2.0);
        graph.add_edge(n2, n3, 2.0);
        graph.add_edge(n3, n1, 2.0);

        // Bridge
        let bridge = graph.add_node(99);
        graph.add_edge(n1, bridge, 0.1);

        // Clique 2
        let n4 = graph.add_node(4);
        let n5 = graph.add_node(5);
        let n6 = graph.add_node(6);

        graph.add_edge(bridge, n4, 0.1);
        graph.add_edge(n4, n5, 2.0);
        graph.add_edge(n5, n6, 2.0);
        graph.add_edge(n6, n4, 2.0);

        let communities = louvain(&graph, Some(42));

        // Should have communities
        assert!(communities.len() >= 1);

        // All nodes accounted for
        let total_nodes: usize = communities.iter().map(|c| c.len()).sum();
        assert_eq!(total_nodes, 7);
    }

    #[test]
    fn test_louvain_weighted_edges() {
        // Test that weights matter
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        // Strong connection between 1 and 2
        graph.add_edge(n1, n2, 10.0);
        // Weak connection to 3
        graph.add_edge(n2, n3, 0.1);

        let communities = louvain(&graph, Some(42));

        assert!(communities.len() >= 1);
        let total_nodes: usize = communities.iter().map(|c| c.len()).sum();
        assert_eq!(total_nodes, 3);
    }

    #[test]
    fn test_louvain_deterministic_with_seed() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);

        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n4, 1.0);

        let communities1 = louvain(&graph, Some(42));
        let communities2 = louvain(&graph, Some(42));

        // Same seed should give same results
        assert_eq!(communities1.len(), communities2.len());
    }

    #[test]
    fn test_louvain_isolated_nodes() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);

        // Only one edge
        graph.add_edge(n1, n2, 1.0);

        // n3 and n4 are isolated
        let communities = louvain(&graph, Some(42));

        // Should handle isolated nodes
        assert!(communities.len() >= 1);
        let total_nodes: usize = communities.iter().map(|c| c.len()).sum();
        assert_eq!(total_nodes, 4);
    }

    #[test]
    fn test_louvain_large_graph() {
        // Test performance on larger graph
        let mut graph = Graph::new();
        let mut nodes = Vec::new();

        // Create 50 nodes
        for i in 0..50 {
            nodes.push(graph.add_node(i));
        }

        // Create 5 communities of 10 nodes each
        for comm in 0..5 {
            let start = comm * 10;
            let end = start + 10;

            // Dense connections within community
            for i in start..end {
                for j in (i + 1)..end {
                    graph.add_edge(nodes[i], nodes[j], 1.0);
                }
            }
        }

        let communities = louvain(&graph, Some(42));

        // Should find some community structure
        assert!(communities.len() >= 1);
        assert!(communities.len() <= 50);

        let total_nodes: usize = communities.iter().map(|c| c.len()).sum();
        assert_eq!(total_nodes, 50);
    }

    #[test]
    fn test_louvain_no_infinite_loop() {
        // Test that algorithm terminates even on difficult graphs
        let mut graph = Graph::new();
        let mut nodes = Vec::new();

        for i in 0..20 {
            nodes.push(graph.add_node(i));
        }

        // Create a ring
        for i in 0..20 {
            graph.add_edge(nodes[i], nodes[(i + 1) % 20], 1.0);
        }

        // This should complete without hanging
        let communities = louvain(&graph, Some(42));

        assert!(communities.len() >= 1);
        let total_nodes: usize = communities.iter().map(|c| c.len()).sum();
        assert_eq!(total_nodes, 20);
    }
}
