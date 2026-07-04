# Graph Types

Graphina provides statically typed graph structures, so you can select the appropriate type for your use case.

## Main Types

### `Graph<A, W>` (Undirected)

`Graph` represents a simple undirected graph.

- Edges: Bidirectional. Adding an edge from `a` to `b` implies a connection `b` to `a`.
- Use Cases: Social networks, road networks, and molecular structures.

```rust
use graphina::core::types::Graph;

// Nodes store &str, Edges store f64 weights
let mut g = Graph::<&str, f64>::new();
```

### `Digraph<A, W>` (Directed)

`Digraph` represents a simple directed graph.

- Edges: Directional. An edge from `a` to `b` does not imply `b` to `a`.
- Use Cases: Web pages, citation networks, and dependency graphs.

```rust
use graphina::core::types::Digraph;

let mut dg = Digraph::<&str, f64>::new();
```
