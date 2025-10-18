// Integration tests for betweenness centrality bug fixes

#[cfg(feature = "centrality")]
#[cfg(test)]
mod betweenness_centrality_tests {
    use graphina::centrality::betweenness::{betweenness_centrality, edge_betweenness_centrality};
    use graphina::core::types::{Digraph, Graph};
    use ordered_float::OrderedFloat;

    #[test]
    fn test_betweenness_linear_graph() {
        // Linear graph: 1 -- 2 -- 3 -- 4
        // Node 2 and 3 should have higher betweenness
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);

        graph.add_edge(n1, n2, OrderedFloat(1.0));
        graph.add_edge(n2, n3, OrderedFloat(1.0));
        graph.add_edge(n3, n4, OrderedFloat(1.0));

        let result = betweenness_centrality(&graph, false).expect("Should compute betweenness");

        // End nodes should have 0 betweenness
        assert_eq!(result[&n1], 0.0);
        assert_eq!(result[&n4], 0.0);

        // Middle nodes should have positive betweenness
        assert!(result[&n2] > 0.0);
        assert!(result[&n3] > 0.0);
    }

    #[test]
    fn test_betweenness_star_graph() {
        // Star graph: center node should have maximum betweenness
        let mut graph = Graph::new();
        let center = graph.add_node(0);
        let mut leaves = Vec::new();

        for i in 1..6 {
            let leaf = graph.add_node(i);
            graph.add_edge(center, leaf, OrderedFloat(1.0));
            leaves.push(leaf);
        }

        let result = betweenness_centrality(&graph, false).expect("Should compute betweenness");

        // Center should have maximum betweenness
        let center_betweenness = result[&center];
        for leaf in leaves {
            assert!(center_betweenness > result[&leaf]);
            assert_eq!(result[&leaf], 0.0); // Leaves have 0 betweenness
        }
    }

    #[test]
    fn test_betweenness_normalized() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        graph.add_edge(n1, n2, OrderedFloat(1.0));
        graph.add_edge(n2, n3, OrderedFloat(1.0));

        let normalized = betweenness_centrality(&graph, true).expect("Should compute");
        let unnormalized = betweenness_centrality(&graph, false).expect("Should compute");

        // Normalized values should be non-negative
        for (_node, &value) in &normalized {
            assert!(value >= 0.0, "Normalized value should be non-negative");
        }

        // Unnormalized should be larger (or equal)
        for node in [n1, n2, n3] {
            assert!(unnormalized[&node] >= normalized[&node]);
        }
    }

    #[test]
    fn test_edge_betweenness_basic() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);

        // Create a diamond shape
        graph.add_edge(n1, n2, OrderedFloat(1.0));
        graph.add_edge(n1, n3, OrderedFloat(1.0));
        graph.add_edge(n2, n4, OrderedFloat(1.0));
        graph.add_edge(n3, n4, OrderedFloat(1.0));

        let result = edge_betweenness_centrality(&graph, false).expect("Should compute");

        // All edges should have some betweenness
        assert!(result.len() > 0);
        for ((_u, _v), &value) in &result {
            assert!(value >= 0.0);
        }
    }

    #[test]
    fn test_betweenness_directed_vs_undirected() {
        // Compare directed and undirected graphs
        let mut ugraph = Graph::new();
        let un1 = ugraph.add_node(1);
        let un2 = ugraph.add_node(2);
        let un3 = ugraph.add_node(3);
        ugraph.add_edge(un1, un2, OrderedFloat(1.0));
        ugraph.add_edge(un2, un3, OrderedFloat(1.0));

        let mut dgraph = Digraph::new();
        let dn1 = dgraph.add_node(1);
        let dn2 = dgraph.add_node(2);
        let dn3 = dgraph.add_node(3);
        dgraph.add_edge(dn1, dn2, OrderedFloat(1.0));
        dgraph.add_edge(dn2, dn3, OrderedFloat(1.0));

        let u_result = betweenness_centrality(&ugraph, true).expect("Should compute");
        let d_result = betweenness_centrality(&dgraph, true).expect("Should compute");

        // Both should return valid results
        assert_eq!(u_result.len(), 3);
        assert_eq!(d_result.len(), 3);
    }

    #[test]
    fn test_betweenness_complete_graph() {
        // In a complete graph, all nodes have equal betweenness (0 for K_n)
        let mut graph = Graph::new();
        let mut nodes = Vec::new();

        for i in 0..5 {
            nodes.push(graph.add_node(i));
        }

        // Connect all pairs
        for i in 0..5 {
            for j in (i + 1)..5 {
                graph.add_edge(nodes[i], nodes[j], OrderedFloat(1.0));
            }
        }

        let result = betweenness_centrality(&graph, false).expect("Should compute");

        // In complete graph, betweenness is 0 (no node lies on shortest path between others)
        for &node in &nodes {
            assert_eq!(result[&node], 0.0);
        }
    }

    #[test]
    fn test_betweenness_empty_graph() {
        let graph: Graph<i32, OrderedFloat<f64>> = Graph::new();
        let result = betweenness_centrality(&graph, false);
        assert!(result.is_err(), "Should return error for empty graph");
    }

    #[test]
    fn test_betweenness_single_node() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);

        let result = betweenness_centrality(&graph, false).expect("Should compute");
        assert_eq!(result[&n1], 0.0);
    }
}
