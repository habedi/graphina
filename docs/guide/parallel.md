# Parallel Processing

Graphina provides multi-threaded implementations of common algorithms using [Rayon](https://github.com/rayon-rs/rayon).
This allows you to leverage all CPU cores for processing large graphs.

## Enabling Parallel Support

The parallel feature must be enabled in your `Cargo.toml` (if not enabled by default):

```toml
[dependencies]
graphina = { version = "0.3.0-alpha.4", features = ["parallel"] }
```

## Available Algorithms

The `graphina::parallel` module mirrors many algorithms from the main modules but executes them in parallel.

### Parallel PageRank

Significantly faster than the single-threaded version for graphs with millions of nodes.

```rust
use graphina::core::types::Digraph;
use graphina::parallel::pagerank_parallel;

let mut g = Digraph::<&str, f64>::new();
// Add some nodes
let n1 = g.add_node("A");
let n2 = g.add_node("B");
g.add_edge(n1, n2, 1.0);

let ranks = pagerank_parallel(&g, 0.85, 100, 1e-6, None);
```

### Parallel Connected Components

Finds connected components in parallel.

```rust
use graphina::parallel::components::connected_components_parallel;

let components = connected_components_parallel(&graph);
```

### Parallel Breadth-First Search (BFS)

Performing BFS from multiple sources concurrently.

```rust
use graphina::parallel::bfs::bfs_parallel;

let mut g = Digraph::<i32, f64>::new();
let start_nodes = vec![n1, n2];
let visited = bfs_parallel(&g, start_nodes);
```

## When to use Parallelism?

Parallelism implies overhead. Use it when:

*   The graph has > 100,000 nodes.
*   The algorithm is computationally intensive (e.g., Betweenness Centrality).

## Thread Safety

Graphina's `Graph` and `Digraph` types are thread-safe (implement `Sync`) as long as the node attributes (`A`) and edge
weights (`W`) are also `Sync`. This allows them to be shared across threads efficiently.
