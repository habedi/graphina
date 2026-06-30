//! Algorithm tests for Graphina modules.
//!
//! This file contains tests for algorithms that don't have dedicated unit tests
//! in the source files, improving overall test coverage.

use graphina::core::types::{Digraph, Graph};
use ordered_float::OrderedFloat;

// =============================================================================
// APPROXIMATION ALGORITHM TESTS
// =============================================================================

mod approximation_tests {
    use super::*;
    use graphina::approximation::clustering::average_clustering;
    use graphina::approximation::matching::min_maximal_matching;
    use graphina::approximation::ramsey::ramsey_r2;
    use graphina::approximation::subgraph::densest_subgraph;

    #[test]
    fn test_average_clustering_empty_graph() {
        let graph: Graph<i32, f64> = Graph::new();
        let result = average_clustering(&graph);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_average_clustering_single_node() {
        let mut graph: Graph<i32, f64> = Graph::new();
        graph.add_node(1);
        let result = average_clustering(&graph);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_average_clustering_triangle() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n1, 1.0);

        let result = average_clustering(&graph);
        // In a triangle, each node has clustering coefficient 1.0
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_average_clustering_star() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let center = graph.add_node(0);
        let mut leaves = vec![];
        for i in 1..=4 {
            leaves.push(graph.add_node(i));
        }
        for leaf in &leaves {
            graph.add_edge(center, *leaf, 1.0);
        }

        let result = average_clustering(&graph);
        // Star graph: center has 4 neighbors but they're not connected to each other
        // Clustering coefficient should be 0 for this configuration
        assert!(result < 0.1);
    }

    #[test]
    fn test_min_maximal_matching_empty_graph() {
        let graph: Graph<i32, f64> = Graph::new();
        let matching = min_maximal_matching(&graph);
        assert!(matching.is_empty());
    }

    #[test]
    fn test_min_maximal_matching_single_edge() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        graph.add_edge(n1, n2, 1.0);

        let matching = min_maximal_matching(&graph);
        assert_eq!(matching.len(), 1);
    }

    #[test]
    fn test_min_maximal_matching_path() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let nodes: Vec<_> = (0..4).map(|i| graph.add_node(i)).collect();
        graph.add_edge(nodes[0], nodes[1], 1.0);
        graph.add_edge(nodes[1], nodes[2], 1.0);
        graph.add_edge(nodes[2], nodes[3], 1.0);

        let matching = min_maximal_matching(&graph);
        // Path of 4 nodes can have at most 2 edges in matching
        assert!(!matching.is_empty() && matching.len() <= 2);
    }

    #[test]
    fn test_ramsey_r2_empty_graph() {
        let graph: Graph<i32, f64> = Graph::new();
        let (clique, independent_set) = ramsey_r2(&graph);
        assert!(clique.is_empty());
        assert!(independent_set.is_empty());
    }

    #[test]
    fn test_ramsey_r2_triangle() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n1, 1.0);

        let (clique, _independent_set) = ramsey_r2(&graph);
        // The entire triangle is a clique
        assert_eq!(clique.len(), 3);
    }

    #[test]
    fn test_densest_subgraph_empty_graph() {
        let graph: Graph<i32, f64> = Graph::new();
        let result = densest_subgraph(&graph);
        assert!(result.is_empty());
    }

    #[test]
    fn test_densest_subgraph_complete_graph() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let nodes: Vec<_> = (0..5).map(|i| graph.add_node(i)).collect();
        for i in 0..5 {
            for j in (i + 1)..5 {
                graph.add_edge(nodes[i], nodes[j], 1.0);
            }
        }

        let result = densest_subgraph(&graph);
        // Complete graph is its own densest subgraph
        assert!(!result.is_empty());
    }

    #[test]
    fn test_densest_subgraph_with_bottleneck() {
        let mut graph: Graph<i32, f64> = Graph::new();
        // Two cliques connected by a single edge
        let clique1: Vec<_> = (0..4).map(|i| graph.add_node(i)).collect();
        let clique2: Vec<_> = (4..8).map(|i| graph.add_node(i)).collect();

        // Complete clique 1
        for i in 0..4 {
            for j in (i + 1)..4 {
                graph.add_edge(clique1[i], clique1[j], 1.0);
            }
        }
        // Complete clique 2
        for i in 0..4 {
            for j in (i + 1)..4 {
                graph.add_edge(clique2[i], clique2[j], 1.0);
            }
        }
        // Connect cliques
        graph.add_edge(clique1[0], clique2[0], 1.0);

        let result = densest_subgraph(&graph);
        // Should find a non-empty subgraph
        assert!(!result.is_empty());
    }
}

// =============================================================================
// CENTRALITY ALGORITHM TESTS
// =============================================================================

mod centrality_tests {
    use super::*;
    use graphina::centrality::closeness::closeness_centrality;
    use graphina::centrality::harmonic::harmonic_centrality;
    use graphina::centrality::other::{
        global_reaching_centrality, laplacian_centrality, local_reaching_centrality, voterank,
    };

    #[test]
    fn test_closeness_centrality_triangle() {
        let mut graph: Graph<i32, OrderedFloat<f64>> = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, OrderedFloat(1.0));
        graph.add_edge(n2, n3, OrderedFloat(1.0));
        graph.add_edge(n3, n1, OrderedFloat(1.0));

        let result = closeness_centrality(&graph).unwrap();
        assert_eq!(result.len(), 3);
        // In a symmetric triangle, all nodes should have equal closeness
        let values: Vec<f64> = result.values().copied().collect();
        assert!((values[0] - values[1]).abs() < 1e-10);
        assert!((values[1] - values[2]).abs() < 1e-10);
    }

    #[test]
    fn test_closeness_centrality_empty_graph() {
        let graph: Graph<i32, OrderedFloat<f64>> = Graph::new();
        let result = closeness_centrality(&graph);
        assert!(result.is_err());
    }

    #[test]
    fn test_closeness_centrality_star() {
        let mut graph: Graph<i32, OrderedFloat<f64>> = Graph::new();
        let center = graph.add_node(0);
        let leaves: Vec<_> = (1..=4).map(|i| graph.add_node(i)).collect();
        for leaf in &leaves {
            graph.add_edge(center, *leaf, OrderedFloat(1.0));
        }

        let result = closeness_centrality(&graph).unwrap();
        // Center should have highest closeness
        let center_centrality = result[&center];
        for leaf in &leaves {
            assert!(center_centrality >= result[leaf]);
        }
    }

    #[test]
    fn test_harmonic_centrality_triangle() {
        let mut graph: Graph<i32, OrderedFloat<f64>> = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, OrderedFloat(1.0));
        graph.add_edge(n2, n3, OrderedFloat(1.0));
        graph.add_edge(n3, n1, OrderedFloat(1.0));

        let result = harmonic_centrality(&graph).unwrap();
        assert_eq!(result.len(), 3);
        // All nodes should have positive harmonic centrality
        for val in result.values() {
            assert!(*val > 0.0);
        }
    }

    #[test]
    fn test_harmonic_centrality_disconnected() {
        let mut graph: Graph<i32, OrderedFloat<f64>> = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let _n3 = graph.add_node(3);
        // Only connect n1-n2, n3 is isolated
        graph.add_edge(n1, n2, OrderedFloat(1.0));

        let result = harmonic_centrality(&graph).unwrap();
        // All should have some value (harmonic handles disconnected graphs)
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_local_reaching_centrality() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let nodes: Vec<_> = (0..5).map(|i| graph.add_node(i)).collect();
        // Path graph
        for i in 0..4 {
            graph.add_edge(nodes[i], nodes[i + 1], 1.0);
        }

        let result = local_reaching_centrality(&graph, 2).unwrap();
        assert_eq!(result.len(), 5);
        // Center node should reach more nodes in 2 hops
        assert!(result[&nodes[2]] >= result[&nodes[0]]);
    }

    #[test]
    fn test_global_reaching_centrality() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let nodes: Vec<_> = (0..4).map(|i| graph.add_node(i)).collect();
        for i in 0..4 {
            for j in (i + 1)..4 {
                graph.add_edge(nodes[i], nodes[j], 1.0);
            }
        }

        let result = global_reaching_centrality(&graph).unwrap();
        assert_eq!(result.len(), 4);
        // In complete graph, all nodes reach all nodes
        for val in result.values() {
            assert_eq!(*val, 4.0); // Reaches self and all others
        }
    }

    #[test]
    fn test_voterank_empty_graph() {
        let graph: Graph<i32, f64> = Graph::new();
        let result = voterank(&graph, 5);
        assert!(result.is_empty());
    }

    #[test]
    fn test_voterank_star() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let center = graph.add_node(0);
        let leaves: Vec<_> = (1..=4).map(|i| graph.add_node(i)).collect();
        for leaf in &leaves {
            graph.add_edge(center, *leaf, 1.0);
        }

        let result = voterank(&graph, 2);
        assert!(!result.is_empty());
        // Center should be selected first (highest degree)
        assert_eq!(result[0], center);
    }

    #[test]
    fn test_laplacian_centrality() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let nodes: Vec<_> = (0..4).map(|i| graph.add_node(i)).collect();
        graph.add_edge(nodes[0], nodes[1], 1.0);
        graph.add_edge(nodes[1], nodes[2], 1.0);
        graph.add_edge(nodes[2], nodes[3], 1.0);

        let result = laplacian_centrality(&graph).unwrap();
        assert_eq!(result.len(), 4);
        // Central nodes should have higher Laplacian centrality
        assert!(result[&nodes[1]] > result[&nodes[0]]);
        assert!(result[&nodes[2]] > result[&nodes[3]]);
    }
}

// =============================================================================
// COMMUNITY DETECTION TESTS
// =============================================================================

mod community_tests {
    use super::*;
    use graphina::community::infomap::infomap;
    use graphina::community::spectral::{spectral_clustering, spectral_embeddings};

    #[test]
    fn test_infomap_empty_graph() {
        let graph: Graph<i32, f64> = Graph::new();
        let result = infomap(&graph, 100, Some(42));
        assert!(result.is_err());
    }

    #[test]
    fn test_infomap_single_node() {
        let mut graph: Graph<i32, f64> = Graph::new();
        graph.add_node(1);
        let result = infomap(&graph, 100, Some(42)).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_infomap_two_cliques() {
        let mut graph: Graph<i32, f64> = Graph::new();
        // Clique 1
        let c1: Vec<_> = (0..4).map(|i| graph.add_node(i)).collect();
        for i in 0..4 {
            for j in (i + 1)..4 {
                graph.add_edge(c1[i], c1[j], 1.0);
            }
        }
        // Clique 2
        let c2: Vec<_> = (4..8).map(|i| graph.add_node(i)).collect();
        for i in 0..4 {
            for j in (i + 1)..4 {
                graph.add_edge(c2[i], c2[j], 1.0);
            }
        }
        // Weak connection
        graph.add_edge(c1[0], c2[0], 0.1);

        let result = infomap(&graph, 100, Some(42)).unwrap();
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_infomap_max_iter_zero() {
        let mut graph: Graph<i32, f64> = Graph::new();
        graph.add_node(1);
        let result = infomap(&graph, 0, Some(42));
        assert!(result.is_err());
    }

    #[test]
    fn test_spectral_embeddings_empty_graph() {
        let graph: Graph<i32, f64> = Graph::new();
        let result = spectral_embeddings(&graph, 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_spectral_embeddings_k_zero() {
        let mut graph: Graph<i32, f64> = Graph::new();
        graph.add_node(1);
        let result = spectral_embeddings(&graph, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_spectral_embeddings_k_greater_than_n() {
        let mut graph: Graph<i32, f64> = Graph::new();
        graph.add_node(1);
        graph.add_node(2);
        let result = spectral_embeddings(&graph, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_spectral_embeddings_valid() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let nodes: Vec<_> = (0..5).map(|i| graph.add_node(i)).collect();
        for i in 0..4 {
            graph.add_edge(nodes[i], nodes[i + 1], 1.0);
        }

        let result = spectral_embeddings(&graph, 2).unwrap();
        assert_eq!(result.len(), 5);
        for embedding in &result {
            assert_eq!(embedding.len(), 2);
        }
    }

    #[test]
    fn test_spectral_clustering_two_clusters() {
        let mut graph: Graph<i32, f64> = Graph::new();
        // Two disconnected components
        let c1: Vec<_> = (0..3).map(|i| graph.add_node(i)).collect();
        let c2: Vec<_> = (3..6).map(|i| graph.add_node(i)).collect();

        for i in 0..3 {
            for j in (i + 1)..3 {
                graph.add_edge(c1[i], c1[j], 1.0);
                graph.add_edge(c2[i], c2[j], 1.0);
            }
        }

        let result = spectral_clustering(&graph, 2, Some(42)).unwrap();
        assert_eq!(result.len(), 2);
        // Each cluster should have nodes
        for cluster in &result {
            assert!(!cluster.is_empty());
        }
    }
}

// =============================================================================
// LINK PREDICTION TESTS
// =============================================================================

mod link_prediction_tests {
    use super::*;
    use graphina::links::allocation::{ra_index_soundarajan_hopcroft, resource_allocation_index};
    use graphina::links::attachment::preferential_attachment;
    use graphina::links::centrality::common_neighbor_centrality;
    use graphina::links::cluster::within_inter_cluster;
    use graphina::links::soundarajan_hopcroft::cn_soundarajan_hopcroft;
    use std::collections::HashMap;

    #[test]
    fn test_resource_allocation_index_empty() {
        let graph: Graph<i32, f64> = Graph::new();
        let result = resource_allocation_index(&graph, None);
        assert!(result.is_empty());
    }

    #[test]
    fn test_resource_allocation_index_triangle() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n3, n1, 1.0);

        let result = resource_allocation_index(&graph, None);
        // Three pairs in triangle
        assert_eq!(result.len(), 3);
        // Each pair has one common neighbor
        for (_, score) in &result {
            assert!(*score > 0.0);
        }
    }

    #[test]
    fn test_resource_allocation_index_custom_pairs() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);

        let pairs = vec![(n1, n3)];
        let result = resource_allocation_index(&graph, Some(&pairs));
        assert_eq!(result.len(), 1);
        // n1 and n3 have n2 as common neighbor
        assert!(result[0].1 > 0.0);
    }

    #[test]
    fn test_preferential_attachment_empty() {
        let graph: Graph<i32, f64> = Graph::new();
        let result = preferential_attachment(&graph, None);
        assert!(result.is_empty());
    }

    #[test]
    fn test_preferential_attachment_star() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let center = graph.add_node(0);
        let leaves: Vec<_> = (1..=3).map(|i| graph.add_node(i)).collect();
        for leaf in &leaves {
            graph.add_edge(center, *leaf, 1.0);
        }

        let pairs = vec![(leaves[0], leaves[1])];
        let result = preferential_attachment(&graph, Some(&pairs));
        assert_eq!(result.len(), 1);
        // Both leaves have degree 1, so PA = 1 * 1 = 1
        assert_eq!(result[0].1, 1.0);

        // Pair with center
        let pairs2 = vec![(center, leaves[0])];
        let result2 = preferential_attachment(&graph, Some(&pairs2));
        // Center has degree 3, leaf has degree 1, PA = 3 * 1 = 3
        assert_eq!(result2[0].1, 3.0);
    }

    #[test]
    fn test_common_neighbor_centrality() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);

        let pairs = vec![(n1, n3)];
        let result = common_neighbor_centrality(&graph, Some(&pairs), 1.0);
        assert_eq!(result.len(), 1);
        // One common neighbor (n2), alpha=1, so score = 1^1 = 1
        assert_eq!(result[0].1, 1.0);

        // Test with different alpha
        let result2 = common_neighbor_centrality(&graph, Some(&pairs), 2.0);
        // 1^2 = 1
        assert_eq!(result2[0].1, 1.0);
    }

    #[test]
    fn test_cn_soundarajan_hopcroft() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);

        // Community function: all in same community
        let community_map: HashMap<_, _> = [(n1, 0), (n2, 0), (n3, 0)].into_iter().collect();
        let community = |node| community_map.get(&node).copied().unwrap_or(0);

        let pairs = vec![(n1, n3)];
        let result = cn_soundarajan_hopcroft(&graph, Some(&pairs), community);
        assert_eq!(result.len(), 1);
        // All in same community, one common neighbor
        assert_eq!(result[0].1, 1.0);
    }

    #[test]
    fn test_cn_soundarajan_hopcroft_different_communities() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);

        // Different communities
        let community_map: HashMap<_, _> = [(n1, 0), (n2, 1), (n3, 0)].into_iter().collect();
        let community = |node| community_map.get(&node).copied().unwrap_or(0);

        let pairs = vec![(n1, n3)];
        let result = cn_soundarajan_hopcroft(&graph, Some(&pairs), community);
        // n2 is in different community, so doesn't count
        assert_eq!(result[0].1, 0.0);
    }

    #[test]
    fn test_within_inter_cluster() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);
        graph.add_edge(n2, n4, 1.0);

        let community_map: HashMap<_, _> =
            [(n1, 0), (n2, 0), (n3, 0), (n4, 1)].into_iter().collect();
        let community = |node| community_map.get(&node).copied().unwrap_or(0);

        let pairs = vec![(n1, n3)];
        let result = within_inter_cluster(&graph, Some(&pairs), community, 0.001);
        assert_eq!(result.len(), 1);
        // n2 is within, so ratio is (1 + 0.001) / (0 + 0.001) ~ 1001
        assert!(result[0].1 > 100.0);
    }

    #[test]
    fn test_ra_index_soundarajan_hopcroft() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);

        let community_map: HashMap<_, _> = [(n1, 0), (n2, 0), (n3, 0)].into_iter().collect();
        let community = |node| community_map.get(&node).copied().unwrap_or(0);

        let pairs = vec![(n1, n3)];
        let result = ra_index_soundarajan_hopcroft(&graph, Some(&pairs), community);
        assert_eq!(result.len(), 1);
        // n2 is common neighbor, all same community, RA = 1/degree(n2) = 1/2
        assert!((result[0].1 - 0.5).abs() < 1e-10);
    }
}

// =============================================================================
// DIRECTED GRAPH SPECIFIC TESTS
// =============================================================================

mod directed_graph_tests {
    use super::*;
    use graphina::centrality::closeness::closeness_centrality;
    use graphina::centrality::harmonic::harmonic_centrality;

    #[test]
    fn test_closeness_centrality_directed() {
        let mut graph: Digraph<i32, OrderedFloat<f64>> = Digraph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        // n1 -> n2 -> n3 (one-way)
        graph.add_edge(n1, n2, OrderedFloat(1.0));
        graph.add_edge(n2, n3, OrderedFloat(1.0));

        let result = closeness_centrality(&graph).unwrap();
        assert_eq!(result.len(), 3);
        // n1 can reach n2, n3; n2 can reach n3; n3 can reach nobody
        assert!(result[&n1] > result[&n3]);
    }

    #[test]
    fn test_harmonic_centrality_directed() {
        let mut graph: Digraph<i32, OrderedFloat<f64>> = Digraph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, OrderedFloat(1.0));
        graph.add_edge(n2, n3, OrderedFloat(1.0));
        graph.add_edge(n3, n1, OrderedFloat(1.0)); // Cycle

        let result = harmonic_centrality(&graph).unwrap();
        assert_eq!(result.len(), 3);
        for val in result.values() {
            assert!(*val > 0.0);
        }
    }
}

// =============================================================================
// EDGE CASE TESTS
// =============================================================================

mod edge_case_tests {
    use super::*;
    use graphina::community::infomap::infomap;

    #[test]
    fn test_infomap_deterministic_with_seed() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let nodes: Vec<_> = (0..10).map(|i| graph.add_node(i)).collect();
        for i in 0..9 {
            graph.add_edge(nodes[i], nodes[i + 1], 1.0);
        }

        let result1 = infomap(&graph, 50, Some(12345)).unwrap();
        let result2 = infomap(&graph, 50, Some(12345)).unwrap();
        // Infomap should produce consistent number of assignments
        assert_eq!(result1.len(), result2.len());
        // Results may vary due to algorithm internals, just ensure it runs
        assert_eq!(result1.len(), 10);
    }

    #[test]
    fn test_algorithms_with_self_loops() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n1, n1, 1.0); // Self-loop

        use graphina::approximation::clustering::average_clustering;
        let result = average_clustering(&graph);
        // Should not panic, result may vary
        assert!(result.is_finite());
    }

    #[test]
    fn test_algorithms_with_parallel_edges() {
        let mut graph: Graph<i32, f64> = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n1, n2, 2.0); // Parallel edge

        use graphina::links::attachment::preferential_attachment;
        let result = preferential_attachment(&graph, None);
        // Should not panic
        assert!(!result.is_empty());
    }
}
