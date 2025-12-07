# Graph Types in Graphina

Graphina provides robust, statically typed graph structures.
Select the appropriate type for your use case.

## Main Types

Graphina focuses on simple graphs to maximize performance. Multiple edges between the same node pair are not supported.

### `Graph<A, W>` (Undirected)

`Graph` represents an **undirected graph**.

- **Edges**: Bidirectional. Adding an edge from `a` to `b` implies a connection `b` to `a`.
- **Use Cases**: Social networks, road networks, molecular structures.

```rust
use graphina::core::types::Graph;

// Nodes store &str, Edges store f64 weights
let mut g = Graph::< & str, f64>::new();
```

### `Digraph<A, W>` (Directed)

`Digraph` represents a **directed graph**.

- **Edges**: Directional. An edge from `a` to `b` does not imply `b` to `a`.
- **Use Cases**: Web pages, citation networks, dependency graphs.

```rust
use graphina::core::types::Digraph;

let mut dg = Digraph::< & str, f64>::new();
```

## Performance and Memory Layout

Graphina uses a `StableGraph` backend from `petgraph`.

1. **Vector-backed storage**: Nodes and edges use `Vec` structures.
2. **Stable Indices**: Removing a node does **not** shift other indices. Safely retain `NodeId`s.
3. **Cache Locality**: Contiguous memory usage improves iteration performance.

## NodeId vs Node Values

NetworkX adds nodes by value:

```python
G.add_node("Alice")
G.add_edge("Alice", "Bob")
```

Graphina separates topology from data. "Alice" is an attribute; the node is identified by a lightweight `NodeId`.

```rust
let alice_id = graph.add_node("Alice");
let bob_id = graph.add_node("Bob");

// Connect using IDs, not strings
graph.add_edge(alice_id, bob_id, 1.0);
```

This design separates **topology** from **data**, enabling optimized integer-based algorithms.

## Density

Check density (ratio of existing to possible edges).

```rust
let d = graph.density();
println!("Graph density: {:.2}", d);
```
