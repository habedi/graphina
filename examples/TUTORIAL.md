## Graphina Examples and Tutorials

This directory contains examples that show how to use Graphina for various graph analysis tasks.

### Getting Started

#### Basic Examples

1. **[core_io.rs](core_io.rs)** - Reading and writing graphs from files
2. **[centrality.rs](centrality.rs)** - Computing node centrality measures
3. **[path_dijkstra.rs](path_dijkstra.rs)** - Finding shortest paths
4. **[visualization.rs](visualization.rs)** - Visualizing graphs

#### Advanced Examples

1. **[centrality_eigenvector.rs](centrality_eigenvector.rs)** - Eigenvector centrality analysis
2. **[centrality_katz.rs](centrality_katz.rs)** - Katz centrality for influence measurement

### Quick Start Tutorial

#### Creating Your First Graph

```rust
use graphina::core::types::Graph;

fn main() {
    // Create a new undirected graph
    let mut graph = Graph::<String, f64>::new();

    // Add nodes representing people in a social network
    let alice = graph.add_node("Alice".to_string());
    let bob = graph.add_node("Bob".to_string());
    let charlie = graph.add_node("Charlie".to_string());
    let diana = graph.add_node("Diana".to_string());

    // Add edges representing friendships (with interaction weights)
    graph.add_edge(alice, bob, 5.0);
    graph.add_edge(alice, charlie, 3.0);
    graph.add_edge(bob, charlie, 4.0);
    graph.add_edge(charlie, diana, 2.0);

    println!("Social network created with {} people and {} connections",
             graph.node_count(), graph.edge_count());
}
```

#### Finding Important Nodes

```rust
use graphina::core::types::Graph;
use graphina::centrality::degree::degree_centrality;
use graphina::centrality::pagerank::pagerank;

fn analyze_network() {
    let mut graph = Graph::<String, f64>::new();
    // ... add nodes and edges ...

    // Find most connected nodes
    let degree_scores = degree_centrality(&graph).unwrap();

    // Find most influential nodes
    let pagerank_scores = pagerank(&graph, 0.85, 100, 1e-6).unwrap();

    for (node, _attr) in graph.nodes() {
        println!("Node {}: degree={:.2}, pagerank={:.4}",
                 node.index(),
                 degree_scores.get(&node).unwrap_or(&0.0),
                 pagerank_scores.get(&node).unwrap_or(&0.0)
        );
    }
}
```

#### Community Detection

```rust
use graphina::core::types::Graph;
use graphina::community::louvain::louvain;

fn find_communities() {
    let mut graph = Graph::<String, f64>::new();
    // ... build a social network ...

    // Detect communities
    let communities = louvain(&graph, Some(42));

    println!("Found {} communities:", communities.len());
    for (i, community) in communities.iter().enumerate() {
        println!("Community {}: {} members", i, community.len());
    }
}
```

#### Link Prediction

```rust
use graphina::core::types::Graph;
use graphina::links::similarity::{
    jaccard_coefficient,
    adamic_adar_index,
    common_neighbors,
};

fn predict_future_connections() {
    let graph = Graph::<i32, f64>::new();
    // ... build network ...

    let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();

    // Compare similarity metrics for potential links
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            let n1 = nodes[i];
            let n2 = nodes[j];

            if !graph.contains_edge(n1, n2) {
                let jaccard = jaccard_coefficient(&graph, n1, n2);
                let adamic = adamic_adar_index(&graph, n1, n2);
                let common = common_neighbors(&graph, n1, n2).len();

                if common > 0 {
                    println!("Potential link {}-{}: jaccard={:.3}, adamic={:.3}, common={}",
                             n1.index(), n2.index(), jaccard, adamic, common);
                }
            }
        }
    }
}
```

### Performance Tips

#### 1. Use Bulk Operations

```rust
// Instead of:
for i in 0..1000 {
graph.add_node(i);
}

// Use:
let nodes = graph.add_nodes_bulk( & (0..1000).collect::<Vec<_ > > ());
```

#### 2. Pre-allocate When Possible

```rust
// If you know the graph size upfront:
let graph = Graph::<i32, f64>::with_capacity(1000, 5000);
```

#### 3. Use Parallel Algorithms for Large Graphs

```rust
use graphina::core::parallel::{
    degrees_parallel,
    pagerank_parallel,
    clustering_coefficients_parallel,
};

let degrees = degrees_parallel( & large_graph);
let pagerank_scores = pagerank_parallel( & large_graph, 0.85, 100, 1e-6);
```

### Common Patterns

#### Pattern 1: Graph Pipeline

```rust
use graphina::core::types::Graph;
use graphina::core::generators::barabasi_albert_graph;

fn analysis_pipeline() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Generate or load graph
    let graph = barabasi_albert_graph::<i32, f64>(1000, 3, Some(42));

    // 2. Compute metrics
    let diameter = graphina::core::metrics::diameter(&graph);
    let clustering = graphina::core::metrics::average_clustering_coefficient(&graph);

    // 3. Find communities
    let communities = graphina::community::louvain::louvain(&graph, Some(42));

    // 4. Identify important nodes
    let centrality = graphina::centrality::degree::degree_centrality(&graph)?;

    // 5. Export results
    let json = graph.to_serializable();
    std::fs::write("graph.json", serde_json::to_string_pretty(&json)?)?;

    Ok(())
}
```

#### Pattern 2: Real-World Network Analysis

```rust
use graphina::core::types::Graph;
use graphina::core::io::read_edge_list;

fn analyze_real_network() -> Result<(), Box<dyn std::error::Error>> {
    // Load from file
    let mut graph = Graph::<i32, f64>::new();
    read_edge_list("network.txt", &mut graph, ',')?;

    println!("Network Statistics:");
    println!("  Nodes: {}", graph.node_count());
    println!("  Edges: {}", graph.edge_count());
    println!("  Density: {:.4}", graph.density());

    // Analyze structure
    let components = graphina::community::connected_components::connected_components(&graph);
    println!("  Connected components: {}", components.len());

    // Find influential nodes
    let pagerank = graphina::centrality::pagerank::pagerank(&graph, 0.85, 100, 1e-6)?;
    let mut ranked: Vec<_> = pagerank.iter().collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    println!("\nTop 10 most influential nodes:");
    for (node, score) in ranked.iter().take(10) {
        println!("  Node {}: {:.6}", node.index(), score);
    }

    Ok(())
}
```

### Running the Examples

To run any example:

```bash
## Run a specific example
cargo run --features all --example centrality

## Run with visualization
cargo run --features all --example visualization

## Run all examples
make run-examples
```
