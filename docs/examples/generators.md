# Graph Generators Examples

Graph generators are essential for testing algorithms and creating benchmarks.
This example demonstrates how to generate various random graphs and analyze their properties.

```rust
use graphina::core::generators::{erdos_renyi_graph, barabasi_albert_graph, watts_strogatz_graph};
use graphina::core::types::Undirected;
use graphina::metrics::{diameter, average_clustering_coefficient};

fn main() {
    let seed = 42;
    let n = 1000;

    // 1. Erdős-Rényi Graph (Random)
    // n=1000, p=0.01
    // Features: Low clustering, Poisson degree distribution
    println!("Generating Erdős-Rényi graph...");
    let er_graph = erdos_renyi_graph::<Undirected>(n, 0.01, seed).unwrap();
    println!("  ER Edges: {}", er_graph.edge_count());
    println!("  ER Clustering: {:.4}", average_clustering_coefficient(&er_graph));

    // 2. Barabási-Albert Graph (Scale-Free)
    // n=1000, edges per new node=3
    // Features: Hubs, Power-law degree distribution
    println!("\nGenerating Barabási-Albert graph...");
    let ba_graph = barabasi_albert_graph::<Undirected>(n, 3, seed).unwrap();
    println!("  BA Edges: {}", ba_graph.edge_count());
    // BA graphs typically have higher max degree due to hubs
    let max_degree = ba_graph.nodes().map(|(id, _)| ba_graph.degree(id)).max().unwrap_or(0);
    println!("  BA Max Degree: {}", max_degree);

    // 3. Watts-Strogatz Graph (Small World)
    // n=1000, k=10 neighbors, p=0.1 rewiring
    // Features: High clustering like regular lattice, short paths like random graph
    println!("\nGenerating Watts-Strogatz graph...");
    let ws_graph = watts_strogatz_graph::<Undirected>(n, 10, 0.1, seed).unwrap();
    println!("  WS Edges: {}", ws_graph.edge_count());
    println!("  WS Clustering: {:.4}", average_clustering_coefficient(&ws_graph));

    // Comparison summary
    println!("\nSummary:");
    println!("Scale-free networks (BA) model social networks well.");
    println!("Small-world networks (WS) model biological and neural networks well.");
    println!("Random graphs (ER) serve as good mathematical baselines.");
}
```
