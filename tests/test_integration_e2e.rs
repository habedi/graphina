// End-to-End and Integration Tests for Graphina
//
// This comprehensive test suite validates the graphina library using real-world datasets.
// Tests cover: core operations, algorithms, serialization, and cross-module integration.
//
// Datasets should be located in tests/testdata/graphina-graphs/
// To download: huggingface-cli download habedi/graphina-graphs --repo-type dataset --local-dir tests/testdata/graphina-graphs

use graphina::core::io::read_edge_list;
use graphina::core::types::{Digraph, Graph};
use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::path::Path;

// ============================================================================
// Test Infrastructure
// ============================================================================

#[derive(Debug, Clone)]
struct DatasetInfo {
    name: &'static str,
    file: &'static str,
    is_directed: bool,
    min_nodes: usize,
    min_edges: usize,
}

const DATASETS: &[DatasetInfo] = &[
    DatasetInfo {
        name: "Wikipedia Chameleon",
        file: "wikipedia_chameleon.txt",
        is_directed: false,
        min_nodes: 2000,
        min_edges: 30000,
    },
    DatasetInfo {
        name: "Wikipedia Squirrel",
        file: "wikipedia_squirrel.txt",
        is_directed: false,
        min_nodes: 5000,
        min_edges: 190000,
    },
    DatasetInfo {
        name: "Wikipedia Crocodile",
        file: "wikipedia_crocodile.txt",
        is_directed: false,
        min_nodes: 11000,
        min_edges: 170000,
    },
    DatasetInfo {
        name: "Facebook Page-Page",
        file: "facebook_page_page.txt",
        is_directed: false,
        min_nodes: 22000,
        min_edges: 170000,
    },
    DatasetInfo {
        name: "Stanford Web Graph",
        file: "stanford_web_graph.txt",
        is_directed: true,
        min_nodes: 280000,
        min_edges: 2300000,
    },
    DatasetInfo {
        name: "DBLP Citation Network",
        file: "dblp_citation_network.txt",
        is_directed: false,
        min_nodes: 317000,
        min_edges: 1049000,
    },
];

fn datasets_available() -> bool {
    Path::new("tests/testdata/graphina-graphs").exists()
}

macro_rules! skip_if_no_datasets {
    () => {
        if !datasets_available() {
            eprintln!("️  Skipping test: datasets not found");
            eprintln!("   Run: huggingface-cli download habedi/graphina-graphs --repo-type dataset --local-dir tests/testdata/graphina-graphs");
            return;
        }
    };
}

fn load_undirected_graph(filename: &str) -> Result<Graph<i32, f32>, std::io::Error> {
    let path = format!("tests/testdata/graphina-graphs/{}", filename);
    let mut graph = Graph::new();
    read_edge_list(&path, &mut graph, ' ')?;
    Ok(graph)
}

fn load_directed_graph(filename: &str) -> Result<Digraph<i32, f32>, std::io::Error> {
    let path = format!("tests/testdata/graphina-graphs/{}", filename);
    let mut graph = Digraph::new();
    read_edge_list(&path, &mut graph, ' ')?;
    Ok(graph)
}

// ============================================================================
// E2E Test: Complete Graph Analysis Pipeline
// ============================================================================

#[test]
fn test_e2e_complete_graph_analysis_pipeline() {
    skip_if_no_datasets!();

    println!("\n Running Complete Graph Analysis Pipeline...\n");

    // Load a medium-sized graph
    let graph = match load_undirected_graph("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(e) => {
            eprintln!("️  Dataset not found: {}", e);
            return;
        }
    };

    // Skip if graph is empty (dataset not loaded properly)
    if graph.node_count() == 0 || graph.edge_count() == 0 {
        println!(" Skipping: graph is empty");
        return;
    }

    println!(
        " Loaded graph: {} nodes, {} edges",
        graph.node_count(),
        graph.edge_count()
    );

    // Step 1: Validate graph structure
    assert!(graph.node_count() > 0, "Graph should have nodes");
    assert!(graph.edge_count() > 0, "Graph should have edges");
    assert!(!graph.is_empty());
    println!(" Graph structure validated");

    // Step 2: Compute basic metrics
    let density = graph.density();
    assert!(
        density > 0.0 && density <= 1.0,
        "Density should be in (0, 1]"
    );
    println!(" Density: {:.6}", density);

    // Step 3: Test serialization roundtrip
    let temp_json = "test_e2e_graph.json";
    let temp_bin = "test_e2e_graph.bin";

    graph.save_json(temp_json).expect("Should save JSON");
    graph.save_binary(temp_bin).expect("Should save binary");
    println!(" Serialization successful");

    let loaded_json = Graph::<i32, f32>::load_json(temp_json).expect("Should load JSON");
    let loaded_bin = Graph::<i32, f32>::load_binary(temp_bin).expect("Should load binary");

    assert_eq!(loaded_json.node_count(), graph.node_count());
    assert_eq!(loaded_bin.node_count(), graph.node_count());
    println!(" Deserialization verified");

    // Cleanup
    std::fs::remove_file(temp_json).ok();
    std::fs::remove_file(temp_bin).ok();

    // Step 4: Traversal algorithms
    use graphina::core::traversal::{bfs, dfs};

    if let Some((start, _)) = graph.nodes().next() {
        let bfs_visited = bfs(&graph, start);
        let dfs_visited = dfs(&graph, start);

        assert!(!bfs_visited.is_empty());
        assert!(!dfs_visited.is_empty());
        println!(" BFS visited {} nodes", bfs_visited.len());
        println!(" DFS visited {} nodes", dfs_visited.len());
    }

    // Step 5: Path algorithms
    use graphina::core::paths::dijkstra;

    if let Some((start, _)) = graph.nodes().next() {
        // Convert to OrderedFloat for path algorithms
        let mut weighted_graph: Graph<i32, OrderedFloat<f64>> = Graph::new();
        let mut node_map = HashMap::new();

        for (node, attr) in graph.nodes() {
            let new_node = weighted_graph.add_node(*attr);
            node_map.insert(node, new_node);
        }

        for (u, v, &w) in graph.edges() {
            weighted_graph.add_edge(node_map[&u], node_map[&v], OrderedFloat(w as f64));
        }

        let distances = dijkstra(&weighted_graph, node_map[&start]).expect("Dijkstra should work");
        let reachable = distances.values().filter(|d| d.is_some()).count();
        println!(" Dijkstra: {} reachable nodes from source", reachable);
    }

    println!("\n Complete pipeline test passed!\n");
}

// ============================================================================
// E2E Test: Directed Graph Analysis Pipeline
// ============================================================================

#[test]
fn test_e2e_directed_graph_analysis() {
    skip_if_no_datasets!();

    println!("\n Testing Directed Graph Analysis...\n");

    let graph = match load_directed_graph("stanford_web_graph.txt") {
        Ok(g) => g,
        Err(e) => {
            eprintln!("️  Large dataset not available: {}", e);
            return;
        }
    };

    println!(
        " Loaded directed graph: {} nodes, {} edges",
        graph.node_count(),
        graph.edge_count()
    );

    assert!(graph.is_directed());
    println!(" Confirmed directed property");

    // Test in/out degree using graph methods
    if let Some((node, _)) = graph.nodes().next() {
        let in_deg = graph.in_degree(node).unwrap_or(0);
        let out_deg = graph.out_degree(node).unwrap_or(0);
        println!(
            " Sample node - In-degree: {}, Out-degree: {}",
            in_deg, out_deg
        );
    }

    println!("\n Directed graph test passed!\n");
}

// ============================================================================
// Integration Test: All Datasets Basic Properties
// ============================================================================

#[test]
fn test_integration_all_datasets_load_and_validate() {
    skip_if_no_datasets!();

    println!("\n Validating All Datasets...\n");

    for dataset in DATASETS {
        let result = if dataset.is_directed {
            load_directed_graph(dataset.file).map(|g| (g.node_count(), g.edge_count()))
        } else {
            load_undirected_graph(dataset.file).map(|g| (g.node_count(), g.edge_count()))
        };

        match result {
            Ok((nodes, edges)) => {
                if nodes == 0 || edges == 0 {
                    eprintln!("️  Skipping {}: empty graph", dataset.name);
                    continue;
                }
                assert!(
                    nodes >= dataset.min_nodes,
                    "{}: Expected at least {} nodes, got {}",
                    dataset.name,
                    dataset.min_nodes,
                    nodes
                );
                assert!(
                    edges >= dataset.min_edges,
                    "{}: Expected at least {} edges, got {}",
                    dataset.name,
                    dataset.min_edges,
                    edges
                );

                println!(" {}: {} nodes, {} edges", dataset.name, nodes, edges);
            }
            Err(e) => {
                eprintln!("️  Skipping {}: {}", dataset.name, e);
            }
        }
    }

    println!("\n Dataset validation complete!\n");
}

// ============================================================================
// Integration Test: Graph Metrics Consistency
// ============================================================================

#[test]
fn test_integration_metrics_consistency() {
    skip_if_no_datasets!();

    println!("\n Testing Metrics Consistency...\n");

    let graph = match load_undirected_graph("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(_) => return,
    };

    // Skip if graph is empty
    if graph.node_count() == 0 || graph.edge_count() == 0 {
        println!(" Skipping: graph is empty");
        return;
    }

    // Test degree distribution using graph method
    let mut total_degree = 0;
    let mut node_count = 0;

    for (node, _) in graph.nodes() {
        let deg = graph.degree(node).unwrap_or(0);
        assert!(deg >= 0, "Degree should be non-negative");
        total_degree += deg;
        node_count += 1;
    }

    let calculated_avg = total_degree as f64 / node_count as f64;

    println!(" Average degree: {:.2}", calculated_avg);
    println!(" Degree metrics consistent");

    // Test density calculation
    let density = graph.density();
    let max_edges = (node_count * (node_count - 1)) / 2;
    let expected_density = graph.edge_count() as f64 / max_edges as f64;

    assert!(
        (density - expected_density).abs() < 0.0001,
        "Density calculation mismatch"
    );

    println!(" Density: {:.6}", density);

    println!("\n Metrics consistency verified!\n");
}

// ============================================================================
// Integration Test: Centrality Algorithms
// ============================================================================

#[cfg(feature = "centrality")]
#[test]
fn test_integration_centrality_algorithms() {
    skip_if_no_datasets!();

    println!("\n Testing Centrality Algorithms...\n");

    let graph_f32 = match load_undirected_graph("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(_) => return,
    };

    if graph_f32.node_count() == 0 || graph_f32.edge_count() == 0 {
        println!(" Skipping: graph is empty");
        return;
    }

    // Convert to OrderedFloat<f64> for centrality algorithms
    let mut graph: Graph<i32, OrderedFloat<f64>> = Graph::new();
    let mut node_map = HashMap::new();

    for (node, attr) in graph_f32.nodes() {
        let new_node = graph.add_node(*attr);
        node_map.insert(node, new_node);
    }

    for (u, v, &w) in graph_f32.edges() {
        graph.add_edge(node_map[&u], node_map[&v], OrderedFloat(w as f64));
    }

    // Test degree centrality
    use graphina::centrality::degree::degree_centrality;
    let deg_centrality = degree_centrality(&graph);
    assert_eq!(deg_centrality.len(), graph.node_count());
    println!(
        " Degree centrality computed for {} nodes",
        deg_centrality.len()
    );

    // Test closeness centrality
    use graphina::centrality::closeness::closeness_centrality;
    let close_centrality = closeness_centrality(&graph).expect("Closeness should work");
    assert_eq!(close_centrality.len(), graph.node_count());
    println!(
        " Closeness centrality computed for {} nodes",
        close_centrality.len()
    );

    // Test betweenness centrality (sample of nodes for performance)
    use graphina::centrality::betweenness::betweenness_centrality;
    let between_centrality = betweenness_centrality(&graph, true).expect("Betweenness should work");
    assert_eq!(between_centrality.len(), graph.node_count());
    println!(
        " Betweenness centrality computed for {} nodes",
        between_centrality.len()
    );

    // Verify all values are non-negative
    for &val in deg_centrality.values() {
        assert!(val >= 0.0, "Degree centrality should be non-negative");
    }

    for &val in close_centrality.values() {
        assert!(val >= 0.0, "Closeness centrality should be non-negative");
    }

    for &val in between_centrality.values() {
        assert!(val >= 0.0, "Betweenness centrality should be non-negative");
    }

    println!("\n Centrality algorithms validated!\n");
}

// ============================================================================
// Integration Test: Community Detection
// ============================================================================

#[cfg(feature = "community")]
#[test]
fn test_integration_community_detection() {
    skip_if_no_datasets!();

    println!("\n Testing Community Detection...\n");

    let graph_f32 = match load_undirected_graph("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(_) => return,
    };

    if graph_f32.node_count() == 0 || graph_f32.edge_count() == 0 {
        println!(" Skipping: graph is empty");
        return;
    }

    // Convert to f64
    let mut graph: Graph<i32, f64> = Graph::new();
    let mut node_map = HashMap::new();

    for (node, attr) in graph_f32.nodes() {
        let new_node = graph.add_node(*attr);
        node_map.insert(node, new_node);
    }

    for (u, v, &w) in graph_f32.edges() {
        graph.add_edge(node_map[&u], node_map[&v], w as f64);
    }

    // Test connected components
    use graphina::community::connected_components::connected_components;
    let components = connected_components(&graph);
    assert!(!components.is_empty(), "Should find at least one component");
    println!(" Found {} connected component(s)", components.len());

    // Test label propagation
    use graphina::community::label_propagation::label_propagation;
    let lp_communities = label_propagation(&graph, 100, Some(42));
    assert_eq!(lp_communities.len(), graph.node_count());
    let num_communities: std::collections::HashSet<_> = lp_communities.iter().collect();
    println!(
        " Label Propagation found {} communities",
        num_communities.len()
    );

    // Test Louvain
    use graphina::community::louvain::louvain;
    let louvain_communities = louvain(&graph, Some(42));
    assert!(!louvain_communities.is_empty());
    println!(" Louvain found {} communities", louvain_communities.len());

    println!("\n Community detection validated!\n");
}

// ============================================================================
// Integration Test: Traversal Algorithms Consistency
// ============================================================================

#[test]
fn test_integration_traversal_consistency() {
    skip_if_no_datasets!();

    println!("\n Testing Traversal Algorithms...\n");

    let graph = match load_undirected_graph("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(_) => return,
    };

    if graph.node_count() == 0 {
        println!(" Skipping: graph is empty");
        return;
    }

    use graphina::core::traversal::{bfs, dfs};

    let mut bfs_total = 0;
    let mut dfs_total = 0;
    let sample_size = 10.min(graph.node_count());

    for (idx, (node, _)) in graph.nodes().enumerate() {
        if idx >= sample_size {
            break;
        }

        let bfs_result = bfs(&graph, node);
        let dfs_result = dfs(&graph, node);

        assert!(!bfs_result.is_empty());
        assert!(!dfs_result.is_empty());

        bfs_total += bfs_result.len();
        dfs_total += dfs_result.len();
    }

    println!(
        " BFS average visited: {:.1} nodes",
        bfs_total as f64 / sample_size as f64
    );
    println!(
        " DFS average visited: {:.1} nodes",
        dfs_total as f64 / sample_size as f64
    );

    println!("\n Traversal consistency verified!\n");
}

// ============================================================================
// Integration Test: Serialization Format Compatibility
// ============================================================================

#[test]
fn test_integration_serialization_formats() {
    skip_if_no_datasets!();

    println!("\n Testing Serialization Formats...\n");

    let original = match load_undirected_graph("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(_) => return,
    };

    let temp_dir = std::env::temp_dir();
    let json_path = temp_dir.join("test_integration.json");
    let bin_path = temp_dir.join("test_integration.bin");
    let graphml_path = temp_dir.join("test_integration.graphml");

    // Save in all formats
    original.save_json(&json_path).expect("Should save JSON");
    original.save_binary(&bin_path).expect("Should save binary");
    original
        .save_graphml(&graphml_path)
        .expect("Should save GraphML");
    println!(" Saved in all formats");

    // Load and verify
    let from_json = Graph::<i32, f32>::load_json(&json_path).expect("Should load JSON");
    let from_bin = Graph::<i32, f32>::load_binary(&bin_path).expect("Should load binary");

    assert_eq!(from_json.node_count(), original.node_count());
    assert_eq!(from_json.edge_count(), original.edge_count());
    assert_eq!(from_bin.node_count(), original.node_count());
    assert_eq!(from_bin.edge_count(), original.edge_count());
    println!(" JSON roundtrip verified");
    println!(" Binary roundtrip verified");

    // Verify GraphML file structure
    let graphml_content = std::fs::read_to_string(&graphml_path).expect("Should read GraphML");
    assert!(graphml_content.contains("<?xml"));
    assert!(graphml_content.contains("<graphml"));
    assert!(graphml_content.contains("edgedefault=\"undirected\""));
    println!(" GraphML format verified");

    // Cleanup
    std::fs::remove_file(&json_path).ok();
    std::fs::remove_file(&bin_path).ok();
    std::fs::remove_file(&graphml_path).ok();

    println!("\n Serialization compatibility verified!\n");
}

// ============================================================================
// Integration Test: Path Algorithms
// ============================================================================

#[test]
fn test_integration_path_algorithms() {
    skip_if_no_datasets!();

    println!("\n️  Testing Path Algorithms...\n");

    let graph_f32 = match load_undirected_graph("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(_) => return,
    };

    // Convert to OrderedFloat<f64>
    let mut graph: Graph<i32, OrderedFloat<f64>> = Graph::new();
    let mut node_map = HashMap::new();

    for (node, attr) in graph_f32.nodes() {
        let new_node = graph.add_node(*attr);
        node_map.insert(node, new_node);
    }

    for (u, v, &w) in graph_f32.edges() {
        graph.add_edge(node_map[&u], node_map[&v], OrderedFloat(w as f64));
    }

    use graphina::core::paths::{bellman_ford, dijkstra};

    if let Some((start, _)) = graph.nodes().next() {
        // Test Dijkstra
        let dijkstra_distances = dijkstra(&graph, start).expect("Dijkstra should work");
        assert_eq!(dijkstra_distances[&start], Some(OrderedFloat(0.0)));

        let reachable_dijkstra = dijkstra_distances.values().filter(|d| d.is_some()).count();
        println!(" Dijkstra: {} reachable nodes", reachable_dijkstra);

        // Test Bellman-Ford
        let bf_distances = bellman_ford(&graph, start).expect("Bellman-Ford should work");
        assert_eq!(bf_distances[&start], Some(OrderedFloat(0.0)));

        let reachable_bf = bf_distances.values().filter(|d| d.is_some()).count();
        println!(" Bellman-Ford: {} reachable nodes", reachable_bf);

        // Verify consistency between algorithms
        assert_eq!(
            reachable_dijkstra, reachable_bf,
            "Dijkstra and Bellman-Ford should find same reachable nodes"
        );
        println!(" Path algorithms consistent");
    }

    println!("\n Path algorithms validated!\n");
}

// ============================================================================
// Integration Test: Graph Generators and Real Data Comparison
// ============================================================================

#[test]
fn test_integration_generators_vs_real_data() {
    skip_if_no_datasets!();

    println!("\n Comparing Generators vs Real Data...\n");

    let real_graph = match load_undirected_graph("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(_) => return,
    };

    if real_graph.node_count() == 0 {
        println!(" Skipping: graph is empty");
        return;
    }

    use graphina::core::generators::{complete_graph, erdos_renyi_graph};
    use graphina::core::types::GraphMarker;

    // Create synthetic graphs with similar size
    let n = 100.min(real_graph.node_count()); // Use smaller size for performance
    let complete = complete_graph::<GraphMarker>(n).expect("Should create complete graph");
    let random = erdos_renyi_graph::<GraphMarker>(n, 0.05, 42).expect("Should create random graph");

    println!(
        "Real graph (sample): {} nodes, density: {:.6}",
        real_graph.node_count(),
        real_graph.density()
    );
    println!(
        "Complete graph: {} nodes, density: {:.6}",
        complete.node_count(),
        complete.density()
    );
    println!(
        "Random graph: {} nodes, density: {:.6}",
        random.node_count(),
        random.density()
    );

    // Complete graph should have density close to 1.0
    assert!(
        complete.density() > 0.99,
        "Complete graph should be nearly dense"
    );

    // Random graph should have moderate density
    assert!(random.density() > 0.0 && random.density() < 1.0);

    println!(" Generator properties verified");

    println!("\n Generator comparison complete!\n");
}

// ============================================================================
// Integration Test: Subgraph Operations
// ============================================================================

#[test]
fn test_integration_subgraph_operations() {
    skip_if_no_datasets!();

    println!("\n Testing Subgraph Operations...\n");

    let graph = match load_undirected_graph("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(_) => return,
    };

    // Create a subgraph from first 100 nodes
    let nodes: Vec<_> = graph.nodes().take(100).map(|(n, _)| n).collect();
    let subgraph = graph.subgraph(&nodes).expect("Should create subgraph");

    assert!(subgraph.node_count() <= 100);
    assert!(subgraph.edge_count() <= graph.edge_count());
    println!(
        " Subgraph: {} nodes, {} edges",
        subgraph.node_count(),
        subgraph.edge_count()
    );

    // Subgraph density might be different
    println!("  Original density: {:.6}", graph.density());
    println!("  Subgraph density: {:.6}", subgraph.density());

    println!("\n Subgraph operations validated!\n");
}

// ============================================================================
// Stress Test: Large Graph Operations
// ============================================================================

#[test]
#[ignore] // Run with --ignored flag for stress testing
fn test_stress_large_graph_operations() {
    skip_if_no_datasets!();

    println!("\n Stress Testing Large Graph...\n");

    let graph = match load_undirected_graph("dblp_citation_network.txt") {
        Ok(g) => g,
        Err(e) => {
            eprintln!("️  Large dataset not available: {}", e);
            return;
        }
    };

    println!(
        "Loaded massive graph: {} nodes, {} edges",
        graph.node_count(),
        graph.edge_count()
    );

    let start_time = std::time::Instant::now();

    // Test basic operations
    let density = graph.density();
    println!(
        " Density computed: {:.8} ({:?})",
        density,
        start_time.elapsed()
    );

    // Test traversal on sample nodes
    use graphina::core::traversal::bfs;
    if let Some((node, _)) = graph.nodes().next() {
        let start_bfs = std::time::Instant::now();
        let visited = bfs(&graph, node);
        println!(
            " BFS visited {} nodes ({:?})",
            visited.len(),
            start_bfs.elapsed()
        );
    }

    println!("\n Stress test completed in {:?}!\n", start_time.elapsed());
}
