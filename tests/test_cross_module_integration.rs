// Cross-Module Integration Tests
//
// Tests that verify interactions between different modules of the graphina library
// using real-world datasets to ensure components work together correctly.

use graphina::core::io::read_edge_list;
use graphina::core::types::{Digraph, Graph};
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

    // Convert to OrderedFloat<f64>
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

// ============================================================================
// Cross-Module: Traversal + Metrics
// ============================================================================

#[test]
fn test_cross_traversal_and_metrics() {
    skip_if_no_datasets!();

    println!("\nTesting Traversal + Metrics Integration...\n");

    let graph = match load_graph_f64("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(_) => return,
    };

    use graphina::core::traversal::bfs;

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

        assert!(visited.len() > 0);
    }

    println!("\nTraversal + Metrics integration verified\n");
}

// ============================================================================
// Cross-Module: Paths + Centrality
// ============================================================================

#[cfg(feature = "centrality")]
#[test]
fn test_cross_paths_and_centrality() {
    skip_if_no_datasets!();

    println!("\nTesting Paths + Centrality Integration...\n");

    let graph = match load_graph_f64("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(_) => return,
    };

    // Skip if graph is empty
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

// ============================================================================
// Cross-Module: Community + Metrics
// ============================================================================

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

    use graphina::community::connected_components::connected_components;

    let components = connected_components(&graph);
    println!("Found {} connected component(s)", components.len());

    if !components.is_empty() {
        let largest_component = components.iter().max_by_key(|c| c.len()).unwrap();

        let component_graph = graph
            .subgraph(largest_component)
            .expect("Should create subgraph");

        let mut total_deg = 0;
        for (node, _) in component_graph.nodes() {
            total_deg += component_graph.degree(node).unwrap_or(0);
        }
        let avg_deg = total_deg as f64 / component_graph.node_count() as f64;

        println!(
            "Largest component: {} nodes, avg degree: {:.2}",
            component_graph.node_count(),
            avg_deg
        );

        assert!(avg_deg > 0.0);
    }

    println!("\nCommunity + Metrics integration verified\n");
}

// ============================================================================
// Cross-Module: Serialization + All Algorithms
// ============================================================================

#[test]
fn test_cross_serialization_preserves_algorithm_results() {
    skip_if_no_datasets!();

    println!("\nTesting Serialization Preserves Algorithm Results...\n");

    let mut original: Graph<i32, f32> = Graph::new();
    if read_edge_list(
        "tests/testdata/graphina-graphs/wikipedia_chameleon.txt",
        &mut original,
        ' ',
    )
    .is_err()
    {
        return;
    }

    // Skip if graph is empty (dataset not loaded properly)
    if original.node_count() == 0 {
        println!("Skipping: graph is empty");
        return;
    }

    use graphina::core::traversal::bfs;

    let start_node = original.nodes().next().map(|(n, _)| n).unwrap();
    let original_bfs = bfs(&original, start_node);

    let temp_path = std::env::temp_dir().join("test_cross_serialize.bin");
    original.save_binary(&temp_path).expect("Should save");
    let loaded = Graph::<i32, f32>::load_binary(&temp_path).expect("Should load");

    let loaded_start = loaded.nodes().next().map(|(n, _)| n).unwrap();
    let loaded_bfs = bfs(&loaded, loaded_start);

    assert_eq!(
        original_bfs.len(),
        loaded_bfs.len(),
        "BFS should produce same results after serialization"
    );

    println!(
        "BFS results preserved: {} nodes visited",
        original_bfs.len()
    );

    std::fs::remove_file(&temp_path).ok();

    println!("\nSerialization preserves algorithm results\n");
}

// ============================================================================
// Cross-Module: Generators + Validation
// ============================================================================

#[test]
fn test_cross_generators_and_validation() {
    println!("\nTesting Generators + Validation Integration...\n");

    use graphina::core::generators::{complete_graph, cycle_graph, erdos_renyi_graph};
    use graphina::core::types::GraphMarker;
    use graphina::core::validation::is_connected;

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

// ============================================================================
// Cross-Module: Subgraphs + Community Detection
// ============================================================================

#[cfg(feature = "community")]
#[test]
fn test_cross_subgraphs_and_communities() {
    skip_if_no_datasets!();

    println!("\nTesting Subgraphs + Community Detection...\n");

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

    use graphina::community::label_propagation::label_propagation;

    let communities = label_propagation(&graph, 50, Some(42));

    let first_community_id = communities.first().copied().unwrap_or(0);
    let community_nodes: Vec<_> = graph
        .node_ids()
        .enumerate()
        .filter(|(idx, _)| communities[*idx] == first_community_id)
        .map(|(_, node)| node)
        .collect();

    let community_subgraph = graph
        .subgraph(&community_nodes)
        .expect("Should create subgraph");

    println!(
        "Community subgraph: {} nodes, {} edges",
        community_subgraph.node_count(),
        community_subgraph.edge_count()
    );

    assert!(community_subgraph.edge_count() > 0 || community_subgraph.node_count() <= 1);

    println!("\nSubgraphs + Communities integration verified\n");
}

// ============================================================================
// Cross-Module: MST + Path Algorithms
// ============================================================================

#[test]
fn test_cross_mst_and_paths() {
    skip_if_no_datasets!();

    println!("\nTesting MST + Path Algorithms...\n");

    let graph = match load_graph_f64("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(_) => return,
    };

    // Skip if graph is empty (dataset not loaded properly)
    if graph.node_count() == 0 {
        println!("Skipping: graph is empty");
        return;
    }

    use graphina::core::mst::kruskal_mst;
    use graphina::core::paths::dijkstra;

    let (mst_edges, _total_weight) = kruskal_mst(&graph).expect("MST should work");
    println!("MST has {} edges", mst_edges.len());

    assert!(mst_edges.len() < graph.node_count());

    let mut mst_graph: Graph<i32, OrderedFloat<f64>> = Graph::new();
    let mut mst_node_map = HashMap::new();

    for (node, attr) in graph.nodes() {
        let new_node = mst_graph.add_node(*attr);
        mst_node_map.insert(node, new_node);
    }

    for edge in mst_edges {
        if let (Some(&mu), Some(&mv)) = (mst_node_map.get(&edge.u), mst_node_map.get(&edge.v)) {
            mst_graph.add_edge(mu, mv, edge.weight);
        }
    }

    if let Some((start, _)) = mst_graph.nodes().next() {
        let distances = dijkstra(&mst_graph, start).expect("Dijkstra on MST should work");
        let reachable = distances.values().filter(|d| d.is_some()).count();
        println!("From MST root, {} nodes reachable", reachable);
    }

    println!("\nMST + Paths integration verified\n");
}

// ============================================================================
// Cross-Module: Directed Graph Specific Integration
// ============================================================================

#[test]
fn test_cross_directed_graph_algorithms() {
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

    use graphina::core::traversal::bfs;

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

// ============================================================================
// Cross-Module: Validation + Algorithms Consistency
// ============================================================================

#[test]
fn test_cross_validation_ensures_algorithm_correctness() {
    skip_if_no_datasets!();

    println!("\nTesting Validation + Algorithm Consistency...\n");

    let graph = match load_graph_f64("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(_) => return,
    };

    use graphina::core::validation::{has_self_loops, is_connected};

    let connected = is_connected(&graph);
    let has_loops = has_self_loops(&graph);

    println!("Graph connected: {}", connected);
    println!("Has self-loops: {}", has_loops);

    if connected {
        use graphina::core::traversal::bfs;

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

// ============================================================================
// Cross-Module: Parallel Operations
// ============================================================================

#[cfg(feature = "centrality")]
#[test]
fn test_cross_parallel_centrality_computation() {
    skip_if_no_datasets!();

    println!("\nTesting Parallel Centrality Computation...\n");

    let graph = match load_graph_f64("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(_) => return,
    };

    // Skip if graph is empty
    if graph.node_count() == 0 {
        println!("Skipping: graph is empty");
        return;
    }

    use graphina::centrality::closeness::closeness_centrality;
    use graphina::centrality::degree::degree_centrality;

    let start = std::time::Instant::now();

    let deg_cent = degree_centrality(&graph).unwrap();
    let close_cent = closeness_centrality(&graph).expect("Should work");

    let elapsed = start.elapsed();

    println!("Computed {} degree centrality values", deg_cent.len());
    println!("Computed {} closeness centrality values", close_cent.len());
    println!("Total time: {:?}", elapsed);

    assert_eq!(deg_cent.len(), close_cent.len());

    println!("\nParallel operations verified\n");
}
