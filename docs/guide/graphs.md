# Graph Types in Graphina

Graphina provides a set of robust, statically typed graph structures.
This guide explains which type to use and how they work under the hood.

## The Two Main Types

Unlike NetworkX which has `Graph`, `DiGraph`, `MultiGraph`, and `MultiDiGraph`, Graphina currently focuses on simple
graphs (no multiple edges between the same node pair) to maximize performance.

### `Graph<A, W>` (Undirected)

The `Graph` type represents an **undirected graph**.

- **Edges**: Bidirectional. If you add an edge from `a` to `b`, it implies a connection `b` to `a`.
- **Use Case**: Social networks (friendships), road networks (two-way streets), molecular structures.

```rust
use graphina::core::types::Graph;

// Nodes store &str, Edges store f64 weights
let mut g = Graph::< & str, f64>::new();
```

### `Digraph<A, W>` (Directed)

The `Digraph` type represents a **directed graph**.

- **Edges**: Directional. An edge from `a` to `b` does not imply `b` to `a`.
- **Use Case**: Web pages (hyperlinks), citation networks, Twitter (following), dependency graphs.

```rust
use graphina::core::types::Digraph;

let mut dg = Digraph::< & str, f64>::new();
```

## Performance & Memory Layout

Graphina is built on top of `petgraph`, using a `StableGraph` backend. This has important implications:

1. **Vector-backed storage**: Nodes and edges are stored in `Vec` structures.
2. **Stable Indices**: Removing a node does **not** shift the indices of other nodes. This allows you to hold onto
   `NodeId`s safely.
3. **Cache Locality**: Iterating over nodes or edges is extremely fast due to contiguous memory (mostly).

## NodeId vs Node Values

In Python's NetworkX, you might say:

```python
G.add_node("Alice")
G.add_edge("Alice", "Bob")
```

In Rust / Graphina, "Alice" is the *attribute* (data) associated with the node, but the node itself is identified by a
lightweight `NodeId`.

```rust
let alice_id = graph.add_node("Alice");
let bob_id = graph.add_node("Bob");

// Connect using IDs, not strings
graph.add_edge(alice_id, bob_id, 1.0);
```

This design separates the **topology** (structure) from the **data**, allowing for highly optimized graph algorithms
that operate purely on integers.

## Density

You can check the density of a graph, which is the ratio of existing edges to possible edges.

```rust
let d = graph.density();
println!("Graph density: {:.2}", d);
```
