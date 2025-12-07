# Parallel Processing

Graphina provides multi-threaded implementations of common algorithms using [Rayon](https://github.com/rayon-rs/rayon).
This allows you to leverage all CPU cores for processing large graphs.

## Enabling Parallel Support

The parallel feature must be enabled in your `Cargo.toml` (if not enabled by default):

```toml
[dependencies]
graphina = { version = "0.3.0", features = ["parallel"] }
```

## Available Algorithms

The `graphina::parallel` module mirrors many algorithms from the main modules but executes them in parallel.

### Parallel PageRank

Significantly faster than the single-threaded version for graphs with millions of nodes.

```rust
use graphina::core::types::Digraph;
use graphina::parallel::pagerank_parallel;

let mut g = Digraph::<&str, f64>::new();
// ... load graph ...

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

- **Small graphs (< 10,000 nodes)**: The overhead of managing threads might make parallel algorithms slower. Use the
  standard sequential versions.
- **Large graphs (> 100,000 nodes)**: Parallel algorithms will likely provide a significant speedup.

## Thread Safety

Graphina's `Graph` and `Digraph` types are thread-safe (implement `Sync`) as long as the node attributes (`A`) and edge
weights (`W`) are also `Sync`. This allows them to be shared across threads efficiently.
