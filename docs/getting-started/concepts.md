# Core Concepts in Graphina

Graphina is designed to be fast, memory-efficient, and easy to use, but coming from dynamic libraries like NetworkX, some concepts might be new.

## Graph Types

Graphina provides two primary graph structures, both of which are generic over node attributes (`A`) and edge weights (`W`).

*   `Graph<A, W>`: An undirected graph. Edges have no direction; `add_edge(a, b, w)` effectively connects $a \leftrightarrow b$.
*   `Digraph<A, W>`: A directed graph. Edges go from source to target. `neighbors(u)` returns only outgoing neighbors (successors).

## Strongly Typed Data

Unlike Python where a graph can hold mixed types (like strings, ints, objects), Graphina graphs are strongly typed.

```rust
// A social network: Nodes are people (String), Edges are relationship strength (f64)
let g = Graph::<String, f64>::new();

// A road map: Nodes are intersections (Point struct), Edges are distances (u32)
let g = Graph::<Point, u32>::new();
```

This design allows Graphina to optimize memory layout and guarantee data consistency at compile time.

## References via NodeId

In NetworkX, the node *value* (like "Alice") is often the identifier.
In Graphina, adding a node transfers ownership of the data to the graph and returns a `NodeId`.

```rust
let id = graph.add_node("Data");
```

This `NodeId` is a lightweight handle (essentially an integer index) that permits $O(1)$ access to the node.
You use this ID for all subsequent graph operations:

*   Creating edges (`graph.add_edge(id1, id2, ...)`)
*   Querying neighbors (`graph.neighbors(id1)`)
*   Algorithms (`pagerank(&graph, ...)`)

## The `try_` API vs Option/Result Returning Methods

Graphina offers two styles of interaction for mutative operations:

Use the `try_` variants or `Option/Result` returning methods for robustness:

*   `graph.remove_node(...)` -> Returns `Option<A>`.
*   `graph.try_remove_node(...)` -> Returns `Result<A, GraphinaError>`. Recommended for production applications where you need to handle errors or propagate them via `?`.

## Performance

Graphina wraps `petgraph`'s `StableGraph`, meaning:

*   Fast Lookups: Nodes and edges are stored in vectors.
*   Stability: Removing a node does not invalidate the `NodeId`s of other nodes.
