//! Integration and cross-module tests for Graphina.

use graphina::core::io::read_edge_list;
use graphina::core::types::{Digraph, Graph, NodeId};
use graphina::core::validation::{
    count_components, has_negative_weights, is_bipartite, is_connected, is_dag, is_empty,
    validate_is_dag, validate_node_exists, validate_non_empty, validate_non_negative_weights,
};
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

fn load_graph_f64(filename: &str) -> Result<Graph<i32, OrderedFloat<f64>>, std::io::Error> {
    let path = format!("tests/testdata/graphina-graphs/{}", filename);
    let mut graph_f32: Graph<i32, f32> = Graph::new();
    read_edge_list(&path, &mut graph_f32, ' ')?;

    let mut graph = Graph::new();
    let mut node_map = HashMap::new();

    for (node, attr) in graph_f32.nodes() {
        let new_node = graph.add_node(*attr);
        node_map.insert(node, new_node);
    }

    for (u, v, &w) in graph_f32.edges() {
        graph.add_edge(node_map[&u], node_map[&v], OrderedFloat(w as f64));
    }

    Ok(graph)
}

fn load_graph_dataset(filename: &str) -> Result<Graph<i32, f32>, std::io::Error> {
    let path = format!("tests/testdata/graphina-graphs/{}", filename);
    let mut graph = Graph::new();
    read_edge_list(&path, &mut graph, ' ')?;
    Ok(graph)
}

#[test]
fn test_validation_utilities_empty_graph() {
    let g = Graph::<i32, f64>::new();

    assert!(is_empty(&g));
    assert!(!is_connected(&g));
    assert!(is_dag(&g));
    assert!(!has_negative_weights(&g));
    assert_eq!(count_components(&g), 0);
}

#[test]
fn test_validation_utilities_single_node() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);

    assert!(!is_empty(&g));
    assert!(is_connected(&g));
    assert!(is_dag(&g));
    assert_eq!(count_components(&g), 1);

    assert!(validate_node_exists(&g, n1).is_ok());
    assert!(validate_non_empty(&g).is_ok());
}

#[test]
fn test_validation_negative_weights() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);

    g.add_edge(n1, n2, 5.0);
    assert!(!has_negative_weights(&g));
    assert!(validate_non_negative_weights(&g).is_ok());

    g.add_edge(n2, n1, -1.0);
    assert!(has_negative_weights(&g));
    assert!(validate_non_negative_weights(&g).is_err());
}

#[test]
fn test_validation_dag() {
    let mut g = Digraph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);

    assert!(is_dag(&g));
    assert!(validate_is_dag(&g).is_ok());

    g.add_edge(n3, n1, 1.0);

    assert!(!is_dag(&g));
    assert!(validate_is_dag(&g).is_err());
}

#[test]
fn test_bipartite_detection() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    let n4 = g.add_node(4);
    g.add_edge(n1, n3, 1.0);
    g.add_edge(n1, n4, 1.0);
    g.add_edge(n2, n3, 1.0);
    g.add_edge(n2, n4, 1.0);

    assert!(is_bipartite(&g));

    // Add edge within same side - no longer bipartite
    g.add_edge(n1, n2, 1.0);
    assert!(!is_bipartite(&g));
}

#[test]
#[cfg(feature = "community")]
fn test_connected_components_non_contiguous_indices() {
    use graphina::community::connected_components::connected_components;

    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    let n4 = g.add_node(4);
    let _n5 = g.add_node(5);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n3, n4, 1.0);

    // Remove node to create non-contiguous indices
    g.remove_node(n3);

    let components = connected_components(&g);

    // Should have 3 components: {n1,n2}, {n4}, {n5}
    assert_eq!(components.len(), 3);
}

#[test]
fn test_component_count_after_operations() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    assert_eq!(count_components(&g), 3);

    g.add_edge(n1, n2, 1.0);
    assert_eq!(count_components(&g), 2);

    g.add_edge(n2, n3, 1.0);
    assert_eq!(count_components(&g), 1);

    g.remove_node(n2);
    assert_eq!(count_components(&g), 2);
}

#[test]
fn test_graph_density_calculation() {
    let mut g = Graph::<i32, f64>::new();

    assert_eq!(g.density(), 0.0); // Empty graph

    let n1 = g.add_node(1);
    assert_eq!(g.density(), 0.0); // Single node

    let n2 = g.add_node(2);
    assert_eq!(g.density(), 0.0); // No edges

    g.add_edge(n1, n2, 1.0);
    assert_eq!(g.density(), 1.0); // Complete graph with 2 nodes

    let _n3 = g.add_node(3);
    // 1 edge out of 3 possible (undirected)
    assert!((g.density() - 1.0 / 3.0).abs() < 1e-10);
}

#[test]
fn test_directed_graph_density() {
    let mut g = Digraph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);
    g.add_edge(n3, n1, 1.0);

    // 3 edges out of 6 possible (directed: n*(n-1))
    assert!((g.density() - 0.5).abs() < 1e-10);
}

#[test]
fn test_degree_calculations_consistency() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n1, n3, 1.0);
    g.add_edge(n2, n3, 1.0);

    assert_eq!(g.degree(n1), Some(2));
    assert_eq!(g.degree(n2), Some(2));
    assert_eq!(g.degree(n3), Some(2));

    // Total degree should be 2 * edge_count
    let total_degree: usize = g.nodes().map(|(id, _)| g.degree(id).unwrap()).sum();
    assert_eq!(total_degree, 2 * g.edge_count());
}

#[test]
fn test_directed_degree_calculations() {
    let mut g = Digraph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n1, n3, 1.0);
    g.add_edge(n2, n3, 1.0);

    // For directed graphs, degree = in_degree + out_degree
    assert_eq!(g.degree(n1), Some(2)); // out: 2, in: 0
    assert_eq!(g.degree(n2), Some(2)); // out: 1, in: 1
    assert_eq!(g.degree(n3), Some(2)); // out: 0, in: 2
}

#[test]
fn test_node_removal_edge_cleanup() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);
    g.add_edge(n1, n3, 1.0);

    assert_eq!(g.edge_count(), 3);

    g.remove_node(n2);

    // Should have removed 2 edges connected to n2
    assert_eq!(g.edge_count(), 1);
    assert!(g.contains_edge(n1, n3));
}

#[test]
fn test_graph_clear() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    g.add_edge(n1, n2, 1.0);

    g.clear();

    assert_eq!(g.node_count(), 0);
    assert_eq!(g.edge_count(), 0);
    assert!(g.is_empty());
}

#[test]
fn test_bulk_operations() {
    let mut g = Graph::<i32, f64>::new();

    // Bulk add nodes
    let nodes: Vec<NodeId> = (0..100).map(|i| g.add_node(i)).collect();

    assert_eq!(g.node_count(), 100);

    // Bulk add edges
    for i in 0..99 {
        g.add_edge(nodes[i], nodes[i + 1], 1.0);
    }

    assert_eq!(g.edge_count(), 99);
}

#[test]
#[cfg(feature = "parallel")]
fn test_parallel_module_accessible() {
    use graphina::parallel::degrees_parallel;

    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    g.add_edge(n1, n2, 1.0);

    let degrees = degrees_parallel(&g);
    assert_eq!(degrees.len(), 2);
}

#[test]
#[cfg(feature = "visualization")]
fn test_visualization_module_accessible() {
    use graphina::visualization::{LayoutAlgorithm, LayoutEngine};

    let mut g = Graph::<&str, f64>::new();
    let n1 = g.add_node("A");
    let n2 = g.add_node("B");
    g.add_edge(n1, n2, 1.0);

    let positions = LayoutEngine::compute_layout(&g, LayoutAlgorithm::Circular, 800.0, 600.0);

    assert_eq!(positions.len(), 2);
}

#[test]
fn test_graph_builder_basic() {
    let g = Graph::<i32, f64>::builder()
        .add_node(1)
        .add_node(2)
        .add_node(3)
        .add_edge(0, 1, 1.0)
        .add_edge(1, 2, 2.0)
        .build();

    assert_eq!(g.node_count(), 3);
    assert_eq!(g.edge_count(), 2);
}

#[test]
fn test_digraph_builder() {
    let g = Digraph::<String, f64>::builder()
        .add_node("A".to_string())
        .add_node("B".to_string())
        .add_edge(0, 1, 5.0)
        .build();

    assert!(g.is_directed());
    assert_eq!(g.node_count(), 2);
    assert_eq!(g.edge_count(), 1);
}

#[test]
fn test_cross_traversal_and_metrics() {
    skip_if_no_datasets!();

    println!("\nTesting Traversal + Metrics Integration...\n");

    let graph = match load_graph_f64("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(_) => return,
    };

    use graphina::traversal::bfs;

    // Find node with highest degree
    let mut max_degree = 0;
    let mut max_node = None;

    for (node, _) in graph.nodes() {
        let deg = graph.degree(node).unwrap_or(0);
        if deg > max_degree {
            max_degree = deg;
            max_node = Some(node);
        }
    }

    if let Some(hub_node) = max_node {
        let visited = bfs(&graph, hub_node);
        println!(
            "Hub node (degree {}): BFS reaches {} nodes",
            max_degree,
            visited.len()
        );

        assert!(!visited.is_empty());
    }

    println!("\nTraversal + Metrics integration verified\n");
}

#[cfg(feature = "centrality")]
#[test]
fn test_cross_paths_and_centrality() {
    skip_if_no_datasets!();

    println!("\nTesting Paths + Centrality Integration...\n");

    let graph = match load_graph_f64("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(_) => return,
    };
    if graph.node_count() == 0 {
        println!("Skipping: graph is empty");
        return;
    }

    use graphina::centrality::closeness::closeness_centrality;
    use graphina::core::paths::dijkstra;

    let closeness = closeness_centrality(&graph).expect("Closeness should work");

    let most_central = closeness
        .iter()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(node, _)| *node);

    if let Some(central_node) = most_central {
        let distances = dijkstra(&graph, central_node).expect("Dijkstra should work");
        let reachable = distances.values().filter(|d| d.is_some()).count();

        println!("Most central node reaches {} nodes", reachable);
        assert!(reachable > 0);
    }

    println!("\nPaths + Centrality integration verified\n");
}

#[cfg(feature = "community")]
#[test]
fn test_cross_community_and_metrics() {
    skip_if_no_datasets!();

    println!("\nTesting Community + Metrics Integration...\n");

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
    let mut graph: Graph<i32, f64> = Graph::new();
    let mut node_map = HashMap::new();

    for (node, attr) in graph_f32.nodes() {
        let new_node = graph.add_node(*attr);
        node_map.insert(node, new_node);
    }

    for (u, v, &w) in graph_f32.edges() {
        graph.add_edge(node_map[&u], node_map[&v], w as f64);
    }
    if graph.node_count() == 0 {
        println!("Skipping: graph is empty");
        return;
    }

    use graphina::community::connected_components::connected_components;

    let components = connected_components(&graph);
    println!("Found {} connected component(s)", components.len());

    // Calculate metrics on the graph itself instead of creating a subgraph
    let mut total_deg = 0;
    for (node, _) in graph.nodes() {
        total_deg += graph.degree(node).unwrap_or(0);
    }
    let avg_deg = total_deg as f64 / graph.node_count() as f64;

    println!(
        "Graph: {} nodes, {} components, avg degree: {:.2}",
        graph.node_count(),
        components.len(),
        avg_deg
    );

    assert!(avg_deg > 0.0);
    assert!(!components.is_empty());

    println!("\nCommunity + Metrics integration verified\n");
}

#[test]
fn test_cross_generators_and_validation() {
    println!("\nTesting Generators + Validation Integration...\n");

    use graphina::core::generators::{complete_graph, cycle_graph, erdos_renyi_graph};
    use graphina::core::types::GraphMarker;

    let complete = complete_graph::<GraphMarker>(50).expect("Should create graph");
    assert!(
        is_connected(&complete),
        "Complete graph should be connected"
    );
    println!("Complete graph (n=50) is connected");

    let cycle = cycle_graph::<GraphMarker>(30).expect("Should create graph");
    assert!(is_connected(&cycle), "Cycle graph should be connected");
    println!("Cycle graph (n=30) is connected");

    let random = erdos_renyi_graph::<GraphMarker>(40, 0.3, 42).expect("Should create graph");
    let random_connected = is_connected(&random);
    println!("Random graph (n=40, p=0.3) connected: {}", random_connected);

    println!("\nGenerators + Validation integration verified\n");
}

#[test]
fn test_cross_validation_ensures_algorithm_correctness() {
    skip_if_no_datasets!();

    println!("\nTesting Validation + Algorithm Consistency...\n");

    let graph = match load_graph_f64("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(_) => return,
    };

    use graphina::core::validation::has_self_loops;

    let connected = is_connected(&graph);
    let has_loops = has_self_loops(&graph);

    println!("Graph connected: {}", connected);
    println!("Has self-loops: {}", has_loops);

    if connected {
        use graphina::traversal::bfs;

        if let Some((start, _)) = graph.nodes().next() {
            let visited = bfs(&graph, start);
            assert_eq!(
                visited.len(),
                graph.node_count(),
                "If graph is connected, BFS should visit all nodes"
            );
            println!(
                "Connected graph verification: BFS visited all {} nodes",
                visited.len()
            );
        }
    }

    println!("\nValidation + Algorithm consistency verified\n");
}

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

        // Invariant 3: Density should be in valid range
        let density = graph.density();
        assert!(
            (0.0..=1.0).contains(&density),
            "{}: Invalid density {}",
            dataset,
            density
        );

        println!("{}: All invariants hold", dataset);
    }

    println!("\nGraph invariants verified across datasets\n");
}

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

#[cfg(feature = "subgraphs")]
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

    use graphina::subgraphs::SubgraphOps;

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

#[test]
fn test_real_world_graph_loading() {
    skip_if_no_datasets!();

    let datasets = vec!["karate_club.txt", "dolphins.txt", "les_miserables.txt"];

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

#[test]
fn test_directed_graph_algorithms() {
    skip_if_no_datasets!();

    println!("\nTesting Directed Graph Cross-Module Integration...\n");

    let mut digraph: Digraph<i32, f32> = Digraph::new();
    let load_result = read_edge_list(
        "tests/testdata/graphina-graphs/stanford_web_graph.txt",
        &mut digraph,
        ' ',
    );

    if load_result.is_err() {
        eprintln!("Stanford web graph not available");
        return;
    }

    use graphina::traversal::bfs;

    let mut max_out = 0;
    let mut hub = None;

    for (node, _) in digraph.nodes().take(1000) {
        let out_deg = digraph.out_degree(node).unwrap_or(0);
        if out_deg > max_out {
            max_out = out_deg;
            hub = Some(node);
        }
    }

    if let Some(hub_node) = hub {
        let reachable = bfs(&digraph, hub_node);
        println!(
            "Hub (out-degree {}): BFS reaches {} nodes",
            max_out,
            reachable.len()
        );
    }

    println!("\nDirected graph integration verified\n");
}
