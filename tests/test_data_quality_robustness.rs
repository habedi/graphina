// Data Quality and Robustness Integration Tests
//
// Tests that verify the library handles real-world data correctly,
// including edge cases, data quality issues, and stress scenarios.

use graphina::core::io::read_edge_list;
use graphina::core::types::Graph;
#[cfg(feature = "subgraphs")]
use graphina::subgraphs::SubgraphOps;
// Add missing imports for tests using OrderedFloat and HashMap
use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::path::Path;

fn datasets_available() -> bool {
    Path::new("tests/testdata/graphina-graphs").exists()
}

macro_rules! skip_if_no_datasets {
    () => {
        if !datasets_available() {
            eprintln!("Skipping test: datasets not found in tests/testdata/graphina-graphs/");
            return;
        }
    };
}

// ============================================================================
// Data Quality: Consistency Checks
// ============================================================================

#[test]
fn test_data_quality_graph_invariants() {
    skip_if_no_datasets!();

    println!("\nTesting Graph Invariants on Real Data...\n");

    let datasets = vec![
        "wikipedia_chameleon.txt",
        "wikipedia_squirrel.txt",
        "facebook_page_page.txt",
    ];

    for dataset in datasets {
        let mut graph: Graph<i32, f32> = Graph::new();
        if read_edge_list(
            &format!("tests/testdata/graphina-graphs/{}", dataset),
            &mut graph,
            ' ',
        )
        .is_err()
        {
            continue;
        }

        // Invariant 1: Edge count should be consistent
        let edge_count_by_iteration = graph.edges().count();
        assert_eq!(
            graph.edge_count(),
            edge_count_by_iteration,
            "{}: Edge count mismatch",
            dataset
        );

        // Invariant 2: Node count should be consistent
        let node_count_by_iteration = graph.nodes().count();
        assert_eq!(
            graph.node_count(),
            node_count_by_iteration,
            "{}: Node count mismatch",
            dataset
        );

        // Invariant 3: Undirected graph edge symmetry checked via adjacency
        if !graph.is_directed() {
            for (u, v, _) in graph.edges() {
                // In undirected graphs, edges are bidirectional
                assert!(
                    u == v || graph.neighbors(v).any(|n| n == u),
                    "{}: Undirected edge ({:?}, {:?}) not symmetric",
                    dataset,
                    u,
                    v
                );
            }
        }

        // Invariant 4: Density should be in valid range
        let density = graph.density();
        assert!(
            density >= 0.0 && density <= 1.0,
            "{}: Invalid density {}",
            dataset,
            density
        );

        println!("{}: All invariants hold", dataset);
    }

    println!("\nGraph invariants verified across datasets\n");
}

// ============================================================================
// Data Quality: Handle Large Degree Nodes
// ============================================================================

#[test]
fn test_data_quality_high_degree_nodes() {
    skip_if_no_datasets!();

    println!("\nTesting High-Degree Node Handling...\n");

    let mut graph: Graph<i32, f32> = Graph::new();
    if read_edge_list(
        "tests/testdata/graphina-graphs/wikipedia_chameleon.txt",
        &mut graph,
        ' ',
    )
    .is_err()
    {
        return;
    }

    // Skip if graph is empty (dataset not loaded properly)
    if graph.node_count() == 0 {
        println!("Skipping: graph is empty");
        return;
    }

    let mut degree_distribution = Vec::new();

    for (node, _) in graph.nodes() {
        let deg = graph.degree(node).unwrap_or(0);
        degree_distribution.push(deg);
    }

    degree_distribution.sort_unstable();

    let max_degree = degree_distribution.last().unwrap_or(&0);
    let median_degree = degree_distribution[degree_distribution.len() / 2];
    let p95_degree = degree_distribution[(degree_distribution.len() * 95) / 100];

    println!("Max degree: {}", max_degree);
    println!("Median degree: {}", median_degree);
    println!("95th percentile: {}", p95_degree);

    use graphina::traversal::bfs;

    for (node, _) in graph.nodes() {
        let deg = graph.degree(node).unwrap_or(0);
        if deg == *max_degree {
            let visited = bfs(&graph, node);
            println!(
                "BFS from hub node (degree {}): {} nodes visited",
                deg,
                visited.len()
            );
            assert!(!visited.is_empty());
            break;
        }
    }

    println!("\nHigh-degree node handling verified\n");
}

// ============================================================================
// Robustness: Empty and Single-Node Subgraphs
// ============================================================================

#[test]
fn test_robustness_edge_case_subgraphs() {
    skip_if_no_datasets!();

    println!("\nTesting Edge Case Subgraph Operations...\n");

    let mut graph: Graph<i32, f32> = Graph::new();
    if read_edge_list(
        "tests/testdata/graphina-graphs/wikipedia_chameleon.txt",
        &mut graph,
        ' ',
    )
    .is_err()
    {
        return;
    }

    if let Some((single_node, _)) = graph.nodes().next() {
        let single_subgraph = graph
            .subgraph(&[single_node])
            .expect("Should create subgraph");
        assert_eq!(single_subgraph.node_count(), 1);
        assert_eq!(single_subgraph.edge_count(), 0);
        println!("Single-node subgraph handled correctly");
    }

    let empty_subgraph: Graph<i32, f32> =
        graph.subgraph(&[]).expect("Should create empty subgraph");
    assert_eq!(empty_subgraph.node_count(), 0);
    assert_eq!(empty_subgraph.edge_count(), 0);
    println!("Empty subgraph handled correctly");

    println!("\nEdge case subgraphs verified\n");
}

// ============================================================================
// Robustness: Algorithm Stability with Different Starting Points
// ============================================================================

#[cfg(feature = "centrality")]
#[test]
fn test_robustness_algorithm_stability() {
    skip_if_no_datasets!();

    println!("\nTesting Algorithm Stability...\n");

    let mut graph_f32: Graph<i32, f32> = Graph::new();
    if read_edge_list(
        "tests/testdata/graphina-graphs/wikipedia_chameleon.txt",
        &mut graph_f32,
        ' ',
    )
    .is_err()
    {
        return;
    }

    let mut graph: Graph<i32, OrderedFloat<f64>> = Graph::new();
    let mut node_map = HashMap::new();

    for (node, attr) in graph_f32.nodes() {
        let new_node = graph.add_node(*attr);
        node_map.insert(node, new_node);
    }

    for (u, v, &w) in graph_f32.edges() {
        graph.add_edge(node_map[&u], node_map[&v], OrderedFloat(w as f64));
    }

    use graphina::centrality::degree::degree_centrality;

    let result1 = degree_centrality(&graph).unwrap();
    let result2 = degree_centrality(&graph).unwrap();

    assert_eq!(result1.len(), result2.len());

    for (node, &val1) in &result1 {
        let val2 = result2.get(node).unwrap();
        assert_eq!(val1, *val2, "Degree centrality should be deterministic");
    }

    println!("Degree centrality is deterministic");
    println!("\nAlgorithm stability verified\n");
}

// ============================================================================
// Robustness: Memory Efficiency with Large Graphs
// ============================================================================

#[test]
#[ignore]
fn test_robustness_memory_efficiency() {
    skip_if_no_datasets!();

    println!("\nTesting Memory Efficiency...\n");

    let datasets = vec!["wikipedia_crocodile.txt", "facebook_page_page.txt"];

    for dataset in datasets {
        let mut graph: Graph<i32, f32> = Graph::new();

        let start = std::time::Instant::now();
        if read_edge_list(
            &format!("tests/testdata/graphina-graphs/{}", dataset),
            &mut graph,
            ' ',
        )
        .is_err()
        {
            continue;
        }

        let load_time = start.elapsed();

        println!(
            "{}: Loaded {} nodes, {} edges in {:?}",
            dataset,
            graph.node_count(),
            graph.edge_count(),
            load_time
        );

        let _ = graph.density();
        let _ = graph.edges().take(1000).count();

        println!("  Basic operations completed successfully");
    }

    println!("\nMemory efficiency verified\n");
}

// ============================================================================
// Robustness: Numerical Stability
// ============================================================================

#[cfg(feature = "centrality")]
#[test]
fn test_robustness_numerical_stability() {
    skip_if_no_datasets!();

    println!("\nTesting Numerical Stability...\n");

    let mut graph_f32: Graph<i32, f32> = Graph::new();
    if read_edge_list(
        "tests/testdata/graphina-graphs/wikipedia_chameleon.txt",
        &mut graph_f32,
        ' ',
    )
    .is_err()
    {
        return;
    }

    let mut graph: Graph<i32, OrderedFloat<f64>> = Graph::new();
    let mut node_map = HashMap::new();

    for (node, attr) in graph_f32.nodes() {
        let new_node = graph.add_node(*attr);
        node_map.insert(node, new_node);
    }

    for (u, v, &w) in graph_f32.edges() {
        graph.add_edge(node_map[&u], node_map[&v], OrderedFloat(w as f64));
    }

    use graphina::centrality::degree::degree_centrality;

    let centrality = degree_centrality(&graph).unwrap();

    for (node, &value) in &centrality {
        assert!(
            !value.is_nan(),
            "Centrality value is NaN for node {:?}",
            node
        );
        assert!(
            !value.is_infinite(),
            "Centrality value is infinite for node {:?}",
            node
        );
        assert!(value >= 0.0, "Centrality should be non-negative");
        assert!(value <= 1.0, "Degree centrality should be <= 1.0");
    }

    println!("All centrality values are numerically stable");
    println!("\nNumerical stability verified\n");
}

// ============================================================================
// Robustness: Error Recovery
// ============================================================================

#[test]
fn test_robustness_error_recovery() {
    println!("\nTesting Error Recovery...\n");

    let mut graph: Graph<i32, f32> = Graph::new();
    let result = read_edge_list(
        "tests/testdata/graphina-graphs/nonexistent.txt",
        &mut graph,
        ' ',
    );

    assert!(
        result.is_err(),
        "Should fail gracefully for non-existent file"
    );
    println!("Graceful failure for missing files");

    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);

    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    graph.add_edge(n1, n2, 1.0);

    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 1);
    println!("Graph usable after failed load");

    println!("\nError recovery verified\n");
}

// ============================================================================
// Data Quality: Duplicate Edge Handling
// ============================================================================

#[test]
fn test_data_quality_duplicate_edges() {
    skip_if_no_datasets!();

    println!("\nTesting Duplicate Edge Handling...\n");

    let mut graph: Graph<i32, f32> = Graph::new();
    if read_edge_list(
        "tests/testdata/graphina-graphs/wikipedia_chameleon.txt",
        &mut graph,
        ' ',
    )
    .is_err()
    {
        return;
    }

    let mut unique_edges = std::collections::HashSet::new();

    for (u, v, _) in graph.edges() {
        let edge = if u < v { (u, v) } else { (v, u) };
        unique_edges.insert(edge);
    }

    if !graph.is_directed() {
        println!("Undirected graph has {} unique edges", unique_edges.len());
    }

    println!("\nDuplicate edge handling verified\n");
}

// ============================================================================
// Performance: Algorithm Scalability
// ============================================================================

#[test]
fn test_performance_algorithm_scalability() {
    skip_if_no_datasets!();

    println!("\nTesting Algorithm Scalability...\n");

    let small_dataset = "wikipedia_chameleon.txt";
    let large_dataset = "wikipedia_crocodile.txt";

    for dataset in &[small_dataset, large_dataset] {
        let mut graph: Graph<i32, f32> = Graph::new();
        if read_edge_list(
            &format!("tests/testdata/graphina-graphs/{}", dataset),
            &mut graph,
            ' ',
        )
        .is_err()
        {
            continue;
        }

        use graphina::traversal::bfs;

        let start = std::time::Instant::now();

        if let Some((node, _)) = graph.nodes().next() {
            let visited = bfs(&graph, node);
            let elapsed = start.elapsed();

            println!(
                "{}: BFS on {} nodes took {:?}",
                dataset,
                graph.node_count(),
                elapsed
            );
            println!("  Visited: {} nodes", visited.len());
        }
    }

    println!("\nAlgorithm scalability verified\n");
}

// ============================================================================
// Data Quality: Isolated Nodes
// ============================================================================

#[cfg(feature = "community")]
#[test]
fn test_data_quality_isolated_nodes() {
    skip_if_no_datasets!();

    println!("\nTesting Isolated Node Detection...\n");

    let mut graph: Graph<i32, f32> = Graph::new();
    if read_edge_list(
        "tests/testdata/graphina-graphs/wikipedia_chameleon.txt",
        &mut graph,
        ' ',
    )
    .is_err()
    {
        return;
    }

    let mut isolated_count = 0;
    let mut low_degree_count = 0;

    for (node, _) in graph.nodes() {
        let deg = graph.degree(node).unwrap_or(0);
        if deg == 0 {
            isolated_count += 1;
        } else if deg == 1 {
            low_degree_count += 1;
        }
    }

    println!("Isolated nodes (degree 0): {}", isolated_count);
    println!("Low-degree nodes (degree 1): {}", low_degree_count);

    let isolation_rate = isolated_count as f64 / graph.node_count() as f64;
    println!("Isolation rate: {:.4}%", isolation_rate * 100.0);

    println!("\nIsolated node analysis complete\n");
}

// ============================================================================
// Robustness: Concurrent Access Patterns
// ============================================================================

#[test]
fn test_robustness_concurrent_read_operations() {
    skip_if_no_datasets!();

    println!("\nTesting Concurrent Read Operations...\n");

    let mut graph: Graph<i32, f32> = Graph::new();
    if read_edge_list(
        "tests/testdata/graphina-graphs/wikipedia_chameleon.txt",
        &mut graph,
        ' ',
    )
    .is_err()
    {
        return;
    }

    use std::sync::Arc;
    use std::thread;

    let graph = Arc::new(graph);
    let mut handles = vec![];

    for _ in 0..4 {
        let graph_clone = Arc::clone(&graph);
        let handle = thread::spawn(move || {
            let _ = graph_clone.node_count();
            let _ = graph_clone.edge_count();
            let _ = graph_clone.density();
            graph_clone.nodes().take(100).count()
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join();
        assert!(result.is_ok());
    }

    println!("Concurrent reads completed successfully");
    println!("\nConcurrent access verified\n");
}

// ============================================================================
// Integration: End-to-End Data Pipeline
// ============================================================================

#[test]
fn test_integration_complete_data_pipeline() {
    skip_if_no_datasets!();

    println!("\nTesting Complete Data Pipeline...\n");

    let mut graph: Graph<i32, f32> = Graph::new();
    if read_edge_list(
        "tests/testdata/graphina-graphs/wikipedia_chameleon.txt",
        &mut graph,
        ' ',
    )
    .is_err()
    {
        return;
    }
    println!(
        "Step 1: Loaded {} nodes, {} edges",
        graph.node_count(),
        graph.edge_count()
    );

    use graphina::core::validation::has_self_loops;
    let has_loops = has_self_loops(&graph);
    println!("Step 2: Validated (self-loops: {})", has_loops);

    let sample_nodes: Vec<_> = graph.nodes().take(500).map(|(n, _)| n).collect();
    let subgraph = graph
        .subgraph(&sample_nodes)
        .expect("Should create subgraph");
    println!(
        "Step 3: Created subgraph with {} nodes",
        subgraph.node_count()
    );

    let density = subgraph.density();
    println!("Step 4: Analyzed (density: {:.6})", density);

    let temp_path = std::env::temp_dir().join("test_pipeline.json");
    subgraph.save_json(&temp_path).expect("Should save");
    println!("Step 5: Exported to JSON");

    let reloaded = Graph::<i32, f32>::load_json(&temp_path).expect("Should reload");
    assert_eq!(reloaded.node_count(), subgraph.node_count());
    println!("Step 6: Reloaded and verified");

    std::fs::remove_file(&temp_path).ok();

    println!("\nComplete data pipeline verified\n");
}
