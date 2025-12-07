# Graph Generators

Graphina provides generators for creating synthetic graphs with specific properties, useful for testing and analysis.

## Random Graph Models

### Erdős-Rényi (ER)

Generates a random graph where each edge exists with probability `p`.

```rust
use graphina::core::generators::erdos_renyi_graph;
use graphina::core::types::Undirected;

// 100 nodes, edge probability 0.1, seed 42
let g = erdos_renyi_graph::<Undirected>(100, 0.1, 42).unwrap();
```

### Barabási-Albert (BA)

Generates a scale-free network using preferential attachment. New nodes attach to existing high-degree nodes.

```rust
use graphina::core::generators::barabasi_albert_graph;
use graphina::core::types::Undirected;

// 1000 nodes, each new node adds 3 edges, seed 42
let g = barabasi_albert_graph::<Undirected>(1000, 3, 42).unwrap();
```

### Watts-Strogatz (WS)

Generates a small-world network by rewiring a regular lattice ring lattice.

```rust
use graphina::core::generators::watts_strogatz_graph;
use graphina::core::types::Undirected;

// 100 nodes, k=4 nearest neighbors, rewiring probability 0.1, seed 42
let g = watts_strogatz_graph::<Undirected>(100, 4, 0.1, 42).unwrap();
```

## Determinism

All generators accept a `seed` parameter (u64). Using the same seed guarantees the same graph structure, ensuring reproducibility for tests and experiments.

## Common Use Cases

*   **Benchmarking**: Test algorithm performance on graphs of increasing size.
*   **Null Models**: Compare properties of real-world networks against random baselines (e.g., "Is this community structure significant compared to an ER graph?").
*   **Simulations**: Model disease spread or information diffusion on realistic topologies (BA or WS).
