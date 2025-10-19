## Graphina Examples

This directory contains runnable examples that demonstrate common Graphina workflows.

- centrality.rs: Compute various centrality measures
- core_io.rs: Read/write graphs from/to files
- path_dijkstra.rs: Shortest path computations
- visualization.rs: ASCII/HTML/SVG/PNG/D3 visualizations (requires `visualization` feature)
- generators.rs: Random/structured graph generators with deterministic seeds

### Highlights

- Ergonomic builders

```rust
use graphina::core::builders::UndirectedGraphBuilder;

let g = UndirectedGraphBuilder::<i32, f64>::undirected()
    .with_capacity(4, 3)
    .add_node(1)
    .add_node(2)
    .add_edge(0, 1, 1.0)
    .build()
    .unwrap();
```

- Seeded generators (deterministic)

```rust
use graphina::core::generators::{erdos_renyi_graph, barabasi_albert_graph};
use graphina::core::types::Undirected;

let er = erdos_renyi_graph::<Undirected>(100, 0.2, 42).unwrap();
let ba = barabasi_albert_graph::<Undirected>(500, 3, 42).unwrap();
```

### Running

```bash
# Run a single example (enable features as needed)
cargo run --features all --example centrality

# Visualization example (requires visualization feature)
cargo run --features visualization --example visualization

# Run all examples
make run-examples
```

Tip: Many examples use deterministic seeds for reproducibility. You can adjust sizes via environment variables (see generators.rs).
