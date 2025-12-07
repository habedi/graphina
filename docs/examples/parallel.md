# Parallel Processing Examples

Leverage multi-core CPUs for large graph analysis using Rayon.

> [!IMPORTANT]
> Ensure the `parallel` feature is enabled in your `Cargo.toml`.

## Parallel PageRank

Calculates PageRank significantly faster on large graphs.

```rust
use graphina::core::types::Digraph;
use graphina::parallel::pagerank_parallel;

fn main() {
    let mut graph = Digraph::<&str, f64>::new();

    // Imagine adding 100,000 nodes and edges...
    // Create a larger graph
    for i in 0..100 {
        let n = graph.add_node("Node");
        if i > 0 {
            // Connect to previous to form a chain/line
            // In a real scenario, you'd add many edges
        }
    }

    // Arguments: graph, damping, iteration, tolerance, nstart
    match pagerank_parallel(&graph, 0.85, 100, 1e-6, None) {
        Ok(scores) => {
            println!("Computed {} ranks.", scores.len());
        },
        Err(e) => println!("Error: {}", e),
    }
}
```

## Parallel BFS

Run BFS from multiple source nodes simultaneously. Useful for distance estimations or analyzing spread.

```rust
use graphina::core::types::Digraph;
use graphina::parallel::bfs::bfs_parallel;

fn main() {
    let mut graph = Digraph::<&str, f64>::new();
    let n1 = graph.add_node("A");
    let n2 = graph.add_node("B");
    graph.add_edge(n1, n2, 1.0);

    let start_nodes = vec![n1];

    // visited is a HashSet of all nodes reachable from ANY start node
    let visited = bfs_parallel(&graph, start_nodes);
    println!("Total reachable nodes: {}", visited.len());
}
```
