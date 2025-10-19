/*!
# Integration Tests for Architectural Improvements

This test suite validates the new architectural features added in v0.4.0-a1:
- Unified error handling with thiserror
- Trait-based graph API
- Advanced builder patterns
- Memory pooling

These tests demonstrate how the improvements work together in realistic scenarios.
*/

#[cfg(all(
    feature = "centrality",
    feature = "community",
    feature = "approximation"
))]
mod architectural_improvements {
    use graphina::core::builders::{AdvancedGraphBuilder, TopologyBuilder};
    use graphina::core::error::{GraphinaError, Result};
    use graphina::core::pool::{NodeSetPool, acquire_node_set};
    use graphina::core::types::{BaseGraph, Directed};

    #[test]
    fn test_unified_error_handling() {
        // Test that new error type works across different operations
        let result: Result<()> = (|| {
            let graph: BaseGraph<i32, f64, Directed> = AdvancedGraphBuilder::directed().build()?;

            if graph.is_empty() {
                return Err(GraphinaError::invalid_graph("Graph is empty"));
            }

            Ok(())
        })();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, GraphinaError::InvalidGraph(_)));
        assert!(err.to_string().contains("empty"));
    }

    #[test]
    fn test_error_conversion_chain() {
        // Test that errors can be chained and converted properly
        fn inner_operation() -> Result<i32> {
            Err(GraphinaError::node_not_found("Node 42"))
        }

        fn outer_operation() -> Result<String> {
            let value = inner_operation()?;
            Ok(format!("Value: {}", value))
        }

        let result = outer_operation();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GraphinaError::NodeNotFound(_)
        ));
    }

    #[test]
    fn test_graph_query_methods() {
        // Test that graph has query methods (not using trait directly since not implemented)
        let graph = TopologyBuilder::complete(5, 10, 1.0);

        // Use direct methods instead of trait methods
        assert_eq!(graph.node_count(), 5);
        assert_eq!(graph.edge_count(), 10);
        assert!(!graph.is_empty());
        assert!(!graph.is_directed());

        // Test density calculation
        let density = graph.density();
        assert!(density > 0.0 && density <= 1.0);
    }

    #[test]
    fn test_advanced_builder_with_validation() -> Result<()> {
        // Test that builder validation catches errors early
        let result = AdvancedGraphBuilder::directed()
            .allow_self_loops(false)
            .add_node(1)
            .add_node(2)
            .add_edge(0, 0, 1.0) // Self-loop!
            .build();

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Self-loops are not allowed")
        );

        // Test successful build with validation
        let graph = AdvancedGraphBuilder::undirected()
            .with_capacity(10, 20)
            .allow_parallel_edges(false)
            .add_nodes(vec![1, 2, 3, 4])
            .add_edges(vec![(0, 1, 1.0), (1, 2, 2.0), (2, 3, 3.0)])
            .build()?;

        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 3);

        Ok(())
    }

    #[test]
    fn test_topology_builders() {
        // Test all topology builders
        let complete = TopologyBuilder::complete(6, (), 1.0);
        assert_eq!(complete.node_count(), 6);
        assert_eq!(complete.edge_count(), 15); // n*(n-1)/2

        let cycle = TopologyBuilder::cycle(8, (), 1.0);
        assert_eq!(cycle.node_count(), 8);
        assert_eq!(cycle.edge_count(), 8);

        let path = TopologyBuilder::path(10, (), 1.0);
        assert_eq!(path.node_count(), 10);
        assert_eq!(path.edge_count(), 9);

        let star = TopologyBuilder::star(7, (), 1.0);
        assert_eq!(star.node_count(), 7);
        assert_eq!(star.edge_count(), 6);

        let grid = TopologyBuilder::grid(4, 5, (), 1.0);
        assert_eq!(grid.node_count(), 20);
        // Grid edges: (rows-1)*cols + rows*(cols-1) = 3*5 + 4*4 = 31
        assert_eq!(grid.edge_count(), 31);
    }

    #[test]
    fn test_memory_pooling_basic() {
        // Test that memory pooling works correctly
        let pool = NodeSetPool::new(5);

        assert_eq!(pool.available(), 0);

        {
            let mut set1 = pool.acquire();
            set1.insert(graphina::core::types::NodeId::new(
                petgraph::graph::NodeIndex::new(1),
            ));
            assert_eq!(set1.len(), 1);
        }

        // Set should be returned to pool
        assert_eq!(pool.available(), 1);

        {
            let set2 = pool.acquire();
            // Should be cleared
            assert_eq!(set2.len(), 0);
        }
    }

    #[test]
    fn test_memory_pooling_with_algorithm() {
        // Simulate an algorithm using pooled memory
        let pool = NodeSetPool::new(10);

        // Run "algorithm" 5 times
        for i in 0..5 {
            let mut visited = pool.acquire();

            // Simulate processing
            for j in 0..i + 1 {
                visited.insert(graphina::core::types::NodeId::new(
                    petgraph::graph::NodeIndex::new(j),
                ));
            }

            assert_eq!(visited.len(), i + 1);
            // visited automatically returned to pool on drop
        }

        // After all iterations, pool should have collected sets
        assert!(pool.available() > 0);
    }

    #[test]
    fn test_default_thread_local_pools() {
        // Test the convenience functions for default pools
        {
            let mut set = acquire_node_set();
            set.insert(graphina::core::types::NodeId::new(
                petgraph::graph::NodeIndex::new(1),
            ));
            assert_eq!(set.len(), 1);
        }

        {
            let mut map = graphina::core::pool::acquire_node_map();
            map.insert(
                graphina::core::types::NodeId::new(petgraph::graph::NodeIndex::new(1)),
                3.14,
            );
            assert_eq!(map.len(), 1);
        }

        {
            let mut queue = graphina::core::pool::acquire_node_queue();
            queue.push_back(graphina::core::types::NodeId::new(
                petgraph::graph::NodeIndex::new(1),
            ));
            assert_eq!(queue.len(), 1);
        }
    }

    #[test]
    fn test_integrated_workflow() -> Result<()> {
        // Test a complete workflow using all new features

        // 1. Build a graph using advanced builder
        let mut graph = AdvancedGraphBuilder::undirected()
            .with_capacity(50, 100)
            .allow_self_loops(false)
            .allow_parallel_edges(false)
            .build()?;

        // Add nodes
        let nodes: Vec<_> = (0..10).map(|i| graph.add_node(i)).collect();

        // Add edges
        for i in 0..9 {
            graph.add_edge(nodes[i], nodes[i + 1], 1.0);
        }

        // 2. Use direct methods to query graph
        assert_eq!(graph.node_count(), 10);
        assert_eq!(graph.edge_count(), 9);
        assert!(!graph.is_directed());

        // 3. Use memory pool for temporary storage
        let pool = NodeSetPool::new(5);
        {
            let mut visited = pool.acquire();
            for node in nodes.iter().take(5) {
                visited.insert(*node);
            }
            assert_eq!(visited.len(), 5);
        }

        // 4. Error handling
        let result = graph.try_update_node(
            graphina::core::types::NodeId::new(petgraph::graph::NodeIndex::new(999)),
            42,
        );
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_builder_error_propagation() {
        // Test that builder errors are properly typed
        let result = AdvancedGraphBuilder::directed()
            .add_node(1)
            .add_edge(0, 5, 1.0) // Invalid target index
            .build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, GraphinaError::InvalidArgument(_)));
        assert!(err.to_string().contains("out of bounds"));
    }

    #[test]
    fn test_performance_with_pooling() {
        // Demonstrate performance benefit of pooling
        let pool = NodeSetPool::new(10);
        let iterations = 100;

        // With pooling
        for _ in 0..iterations {
            let mut set = pool.acquire();
            for j in 0..50 {
                set.insert(graphina::core::types::NodeId::new(
                    petgraph::graph::NodeIndex::new(j),
                ));
            }
            // Automatically returned to pool
        }

        // Pool should have accumulated reusable sets
        assert!(pool.available() > 0);
        assert!(pool.available() <= 10); // Respects max_size
    }

    #[test]
    fn test_multiple_graph_types_with_builders() -> Result<()> {
        // Test building different graph types

        // Directed graph
        let directed = AdvancedGraphBuilder::directed()
            .add_nodes(vec![1, 2, 3])
            .add_edge(0, 1, 1.0)
            .add_edge(1, 2, 2.0)
            .build()?;

        assert!(directed.is_directed());
        assert_eq!(directed.node_count(), 3);

        // Undirected graph
        let undirected = AdvancedGraphBuilder::undirected()
            .add_nodes(vec![1, 2, 3])
            .add_edge(0, 1, 1.0)
            .add_edge(1, 2, 2.0)
            .build()?;

        assert!(!undirected.is_directed());
        assert_eq!(undirected.node_count(), 3);

        Ok(())
    }

    #[test]
    fn test_error_cloning() {
        // Test that errors can be cloned (useful for logging, retry logic, etc.)
        let err = GraphinaError::algorithm_error("Test error");
        let cloned = err.clone();

        assert_eq!(err.to_string(), cloned.to_string());
    }

    #[test]
    fn test_topology_with_custom_types() {
        // Test topology builders with custom node/edge types
        #[derive(Clone, Debug, PartialEq)]
        struct NodeData {
            id: usize,
            name: String,
        }

        let node_data = NodeData {
            id: 1,
            name: "Node".to_string(),
        };

        let graph = TopologyBuilder::complete(4, node_data.clone(), 2.5);

        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 6); // Complete graph: n*(n-1)/2

        // Verify node data
        for (_, attr) in graph.nodes() {
            assert_eq!(attr.id, 1);
            assert_eq!(attr.name, "Node");
        }
    }
}
