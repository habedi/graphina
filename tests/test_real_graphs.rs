// Integration Tests with Real-World Graph Datasets
//
// This test suite uses real-world graph datasets from the graphina-graphs repository.
// The datasets should be located in tests/testdata/graphina-graphs/
//
// To download the datasets, run:
// huggingface-cli download habedi/graphina-graphs --repo-type dataset --local-dir tests/testdata/graphina-graphs
//
// Tests are automatically skipped if the datasets are not present.

use graphina::core::io::read_edge_list;
use graphina::core::types::{Digraph, Graph};
use std::path::Path;

// Helper function to check if datasets directory exists
fn datasets_available() -> bool {
    Path::new("tests/testdata/graphina-graphs").exists()
}

// Helper macro to skip tests if datasets not available
macro_rules! skip_if_no_datasets {
    () => {
        if !datasets_available() {
            eprintln!("Skipping test: datasets not found in tests/testdata/graphina-graphs/");
            eprintln!("To download: huggingface-cli download habedi/graphina-graphs --repo-type dataset --local-dir tests/testdata/graphina-graphs");
            return;
        }
    };
}

// Helper function to load a graph from dataset
fn load_graph_dataset(filename: &str) -> Result<Graph<i32, f32>, std::io::Error> {
    let path = format!("tests/testdata/graphina-graphs/{}", filename);
    let mut graph = Graph::new();
    read_edge_list(&path, &mut graph, ' ')?;
    Ok(graph)
}

#[allow(dead_code)]
fn load_digraph_dataset(filename: &str) -> Result<Digraph<i32, f32>, std::io::Error> {
    let path = format!("tests/testdata/graphina-graphs/{}", filename);
    let mut graph = Digraph::new();
    read_edge_list(&path, &mut graph, ' ')?;
    Ok(graph)
}

// ============================================================================
// Core Operations on Real Graphs
// ============================================================================

#[test]
fn test_load_and_analyze_karate_club() {
    skip_if_no_datasets!();

    let graph = match load_graph_dataset("karate_club.txt") {
        Ok(g) => g,
        Err(_) => {
            eprintln!("Skipping: karate_club.txt not found");
            return;
        }
    };

    // Zachary's Karate Club has 34 nodes and 78 edges
    assert!(graph.node_count() > 0, "Graph should have nodes");
    assert!(graph.edge_count() > 0, "Graph should have edges");

    // Basic properties
    assert!(!graph.is_empty());
    assert!(graph.density() > 0.0);
    assert!(graph.density() <= 1.0);

    println!(
        "Karate Club: {} nodes, {} edges, density: {:.4}",
        graph.node_count(),
        graph.edge_count(),
        graph.density()
    );
}

#[test]
fn test_load_multiple_datasets() {
    skip_if_no_datasets!();

    let datasets = vec![
        "karate_club.txt",
        "dolphins.txt",
        "les_miserables.txt",
        "football.txt",
    ];

    for dataset_name in datasets {
        match load_graph_dataset(dataset_name) {
            Ok(graph) => {
                assert!(graph.node_count() > 0);
                assert!(graph.edge_count() > 0);
                println!(
                    "{}: {} nodes, {} edges",
                    dataset_name,
                    graph.node_count(),
                    graph.edge_count()
                );
            }
            Err(_) => {
                eprintln!("Dataset {} not found, skipping", dataset_name);
            }
        }
    }
}

// ============================================================================
// Traversal Algorithms on Real Graphs
// ============================================================================

#[test]
fn test_bfs_on_real_graph() {
    skip_if_no_datasets!();

    use graphina::core::traversal::bfs;

    let graph = match load_graph_dataset("karate_club.txt") {
        Ok(g) => g,
        Err(_) => {
            eprintln!("Skipping: karate_club.txt not found");
            return;
        }
    };

    if let Some((start_node, _)) = graph.nodes().next() {
        let visited = bfs(&graph, start_node);

        assert!(!visited.is_empty());
        assert!(visited.len() <= graph.node_count());

        println!("BFS from first node visited {} nodes", visited.len());
    }
}

#[test]
fn test_dfs_on_real_graph() {
    skip_if_no_datasets!();

    use graphina::core::traversal::dfs;

    let graph = match load_graph_dataset("dolphins.txt") {
        Ok(g) => g,
        Err(_) => {
            eprintln!("Skipping: dolphins.txt not found");
            return;
        }
    };

    if let Some((start_node, _)) = graph.nodes().next() {
        let visited = dfs(&graph, start_node);

        assert!(!visited.is_empty());
        assert!(visited.len() <= graph.node_count());

        println!("DFS from first node visited {} nodes", visited.len());
    }
}

// ============================================================================
// Path Algorithms on Real Graphs
// ============================================================================

#[test]
fn test_dijkstra_on_real_graph() {
    skip_if_no_datasets!();

    use graphina::core::paths::dijkstra;
    use ordered_float::OrderedFloat;

    // Load as weighted graph
    let path = "tests/testdata/graphina-graphs/karate_club.txt";
    let mut graph: Graph<i32, OrderedFloat<f64>> = Graph::new();

    if read_edge_list(path, &mut graph, ' ').is_err() {
        eprintln!("Skipping: karate_club.txt not found");
        return;
    }

    if let Some((start_node, _)) = graph.nodes().next() {
        let distances = dijkstra(&graph, start_node).expect("Dijkstra should succeed");

        // Check that source has distance 0
        assert_eq!(distances[&start_node], Some(OrderedFloat(0.0)));

        // Count reachable nodes
        let reachable = distances.values().filter(|d| d.is_some()).count();
        println!("Dijkstra: {} nodes reachable from source", reachable);

        assert!(reachable > 0);
    }
}

// ============================================================================
// Centrality Measures on Real Graphs
// ============================================================================

#[cfg(feature = "centrality")]
#[test]
fn test_degree_centrality_on_real_graph() {
    skip_if_no_datasets!();

    use graphina::centrality::degree::degree_centrality;

    let graph = match load_graph_dataset("karate_club.txt") {
        Ok(g) => g,
        Err(_) => {
            eprintln!("Skipping: karate_club.txt not found");
            return;
        }
    };

    let centrality = degree_centrality(&graph).unwrap();

    assert_eq!(centrality.len(), graph.node_count());

    // Find node with highest degree centrality
    let max_centrality = centrality
        .values()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    println!("Max degree centrality: {}", max_centrality);
    assert!(*max_centrality > 0.0);
}

#[cfg(feature = "centrality")]
#[test]
fn test_betweenness_centrality_on_real_graph() {
    skip_if_no_datasets!();

    use graphina::centrality::betweenness::betweenness_centrality;
    use ordered_float::OrderedFloat;

    // Load graph with f32 weights first, then convert
    let graph_f32 = match load_graph_dataset("karate_club.txt") {
        Ok(g) => g,
        Err(_) => {
            eprintln!("Skipping: karate_club.txt not found");
            return;
        }
    };

    // Convert to OrderedFloat<f64> for betweenness
    let mut graph: Graph<i32, OrderedFloat<f64>> = Graph::new();
    let mut node_map = std::collections::HashMap::new();

    for (node, attr) in graph_f32.nodes() {
        let new_node = graph.add_node(*attr);
        node_map.insert(node, new_node);
    }

    for (u, v, &w) in graph_f32.edges() {
        graph.add_edge(node_map[&u], node_map[&v], OrderedFloat(w as f64));
    }

    let centrality = betweenness_centrality(&graph, true).expect("Should compute betweenness");

    assert_eq!(centrality.len(), graph.node_count());

    // All values should be non-negative
    for &value in centrality.values() {
        assert!(value >= 0.0);
    }

    println!(
        "Computed betweenness centrality for {} nodes",
        centrality.len()
    );
}

// ============================================================================
// Community Detection on Real Graphs
// ============================================================================

#[cfg(feature = "community")]
#[test]
fn test_louvain_on_karate_club() {
    skip_if_no_datasets!();

    use graphina::community::louvain::louvain;

    // Load graph with f32 weights
    let graph_f32 = match load_graph_dataset("karate_club.txt") {
        Ok(g) => g,
        Err(_) => {
            eprintln!("Skipping: karate_club.txt not found");
            return;
        }
    };

    // Convert to f64 for louvain
    let mut graph: Graph<i32, f64> = Graph::new();
    let mut node_map = std::collections::HashMap::new();

    for (node, attr) in graph_f32.nodes() {
        let new_node = graph.add_node(*attr);
        node_map.insert(node, new_node);
    }

    for (u, v, &w) in graph_f32.edges() {
        graph.add_edge(node_map[&u], node_map[&v], w as f64);
    }

    let communities = louvain(&graph, Some(42));

    assert!(!communities.is_empty());

    // All nodes should be in some community
    let total_nodes: usize = communities.iter().map(|c| c.len()).sum();
    assert_eq!(total_nodes, graph.node_count());

    println!("Louvain found {} communities", communities.len());

    // Print community sizes
    for (i, comm) in communities.iter().enumerate() {
        println!("  Community {}: {} nodes", i, comm.len());
    }
}

#[cfg(feature = "community")]
#[test]
fn test_louvain_on_dolphins() {
    skip_if_no_datasets!();

    use graphina::community::louvain::louvain;

    // Load graph with f32 weights
    let graph_f32 = match load_graph_dataset("dolphins.txt") {
        Ok(g) => g,
        Err(_) => {
            eprintln!("Skipping: dolphins.txt not found");
            return;
        }
    };

    // Convert to f64 for louvain
    let mut graph: Graph<i32, f64> = Graph::new();
    let mut node_map = std::collections::HashMap::new();

    for (node, attr) in graph_f32.nodes() {
        let new_node = graph.add_node(*attr);
        node_map.insert(node, new_node);
    }

    for (u, v, &w) in graph_f32.edges() {
        graph.add_edge(node_map[&u], node_map[&v], w as f64);
    }

    let communities = louvain(&graph, Some(42));

    assert!(!communities.is_empty());

    let total_nodes: usize = communities.iter().map(|c| c.len()).sum();
    assert_eq!(total_nodes, graph.node_count());

    println!(
        "Dolphins network: {} communities found in {} nodes",
        communities.len(),
        graph.node_count()
    );
}

// ============================================================================
// Graph Properties on Real Graphs
// ============================================================================

#[test]
fn test_graph_properties_on_real_graphs() {
    skip_if_no_datasets!();

    let datasets = vec!["karate_club.txt", "dolphins.txt"];

    for dataset_name in datasets {
        match load_graph_dataset(dataset_name) {
            Ok(graph) => {
                let n = graph.node_count();
                let m = graph.edge_count();
                let density = graph.density();

                println!("\n{}", dataset_name);
                println!("  Nodes: {}", n);
                println!("  Edges: {}", m);
                println!("  Density: {:.4}", density);

                // Compute degree distribution
                let mut degrees: Vec<usize> = Vec::new();
                for (node, _) in graph.nodes() {
                    if let Some(deg) = graph.degree(node) {
                        degrees.push(deg);
                    }
                }

                if !degrees.is_empty() {
                    degrees.sort_unstable();
                    let min_deg = degrees[0];
                    let max_deg = degrees[degrees.len() - 1];
                    let avg_deg: f64 = degrees.iter().sum::<usize>() as f64 / degrees.len() as f64;

                    println!(
                        "  Degree: min={}, max={}, avg={:.2}",
                        min_deg, max_deg, avg_deg
                    );

                    assert!(min_deg <= max_deg);
                    assert!(avg_deg >= min_deg as f64);
                    assert!(avg_deg <= max_deg as f64);
                }
            }
            Err(_) => {
                eprintln!("Dataset {} not found, skipping", dataset_name);
            }
        }
    }
}

// ============================================================================
// Performance Tests
// ============================================================================

#[test]
fn test_large_graph_loading_performance() {
    skip_if_no_datasets!();

    use std::time::Instant;

    // Try to load larger datasets if available
    let datasets = vec!["football.txt", "les_miserables.txt"];

    for dataset_name in datasets {
        match load_graph_dataset(dataset_name) {
            Ok(graph) => {
                let start = Instant::now();

                // Perform some operations
                let _n = graph.node_count();
                let _m = graph.edge_count();
                let _d = graph.density();

                // Count all neighbors
                let mut total_neighbors = 0;
                for (node, _) in graph.nodes() {
                    total_neighbors += graph.neighbors(node).count();
                }

                let duration = start.elapsed();

                println!(
                    "{}: {} nodes, {} edges, {}ms to analyze",
                    dataset_name,
                    graph.node_count(),
                    graph.edge_count(),
                    duration.as_millis()
                );

                assert!(total_neighbors > 0);
            }
            Err(_) => {
                eprintln!("Dataset {} not found, skipping", dataset_name);
            }
        }
    }
}

// ============================================================================
// Correctness Tests with Known Properties
// ============================================================================

#[test]
fn test_karate_club_known_properties() {
    skip_if_no_datasets!();

    let graph = match load_graph_dataset("karate_club.txt") {
        Ok(g) => g,
        Err(_) => {
            eprintln!("Skipping: karate_club.txt not found");
            return;
        }
    };

    // Zachary's Karate Club is known to have 34 nodes
    // The exact number of edges depends on how the data is formatted
    // but should be around 78 for the standard dataset
    if graph.node_count() == 34 {
        println!("✓ Karate Club has expected 34 nodes");

        // Should be an undirected graph
        assert!(!graph.is_directed());

        // Should be connected (all nodes reachable from any node)
        use graphina::core::traversal::bfs;
        if let Some((start, _)) = graph.nodes().next() {
            let visited = bfs(&graph, start);
            assert_eq!(visited.len(), 34, "Karate Club should be fully connected");
        }
    }
}
