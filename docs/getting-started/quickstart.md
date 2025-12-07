# Quick Start with Graphina (Rust)

This guide will walk you through creating your first graph with Graphina, adding nodes and edges, and running basic algorithms.

## Creating a Graph

Unlike dynamic languages where graphs can hold any data, Rust requires us to define the types of data our nodes and edges will hold.

Let's create an undirected graph where nodes store names (`String`) and edges store weights (`f64`).

```rust
use graphina::core::types::Graph;

fn main() {
    // Create an empty graph
    // Node type: String, Edge type: f64
    let mut graph = Graph::<String, f64>::new();
}
```

> [!NOTE]
> For directed graphs, use `Digraph` instead of `Graph`.

## Adding Nodes

In Graphina, adding a node returns a `NodeId`. You use this ID to refer to the node later (like when creating edges).
This differs from libraries like NetworkX where you might use the node value itself as the identifier.

```rust
    // Add nodes
    let n1 = graph.add_node("Alice".to_string());
    let n2 = graph.add_node("Bob".to_string());
    let n3 = graph.add_node("Charlie".to_string());

    println!("Graph has {} nodes", graph.node_count());
```

## Adding Edges

Edges connect two nodes using their `NodeId`s. You also provide the edge weight/attribute.

```rust
    // Add weighted edges
    graph.add_edge(n1, n2, 1.0); // Alice -- Bob
    graph.add_edge(n2, n3, 2.0); // Bob -- Charlie

    // Add an edge and get its ID
    let e1 = graph.add_edge(n1, n3, 0.5); // Alice -- Charlie

    println!("Graph has {} edges", graph.edge_count());
```

## Examining the Graph

You can check if nodes or edges exist, and inspect their degrees or neighbors.

```rust
    // Check degree
    let degree = graph.degree(n2);
    println!("Bob's degree: {:?}", degree); // Some(2)

    // Iterate over neighbors
    println!("Alice's neighbors:");
    for neighbor_id in graph.neighbors(n1) {
        // Access the node data using the index operator
        println!("- {}", graph[neighbor_id]);
    }
```

## Running Algorithms

Graphina comes with a suite of standard algorithms in modules like `centrality`, `community`, and `paths`.

Here is how to calculate PageRank:

```rust
use graphina::centrality::pagerank;

    // ... (graph creation code)

    // Calculate PageRank
    // (graph, damping_factor, max_iterations, tolerance)
    let scores = pagerank(&graph, 0.85, 100, 1e-6);

    println!("PageRank scores:");
    for (node_id, score) in scores {
        println!("{}: {:.4}", graph[node_id], score);
    }
```

## Complete Example

Putting it all together:

```rust
use graphina::core::types::Graph;
use graphina::centrality::pagerank;

fn main() {
    let mut g = Graph::<&str, f64>::new();

    let a = g.add_node("A");
    let b = g.add_node("B");
    let c = g.add_node("C");

    g.add_edge(a, b, 1.0);
    g.add_edge(b, c, 1.0);
    g.add_edge(c, a, 1.0);

    println!("Nodes: {}, Edges: {}", g.node_count(), g.edge_count());

    let ranks = pagerank(&g, 0.85, 100, 1e-6);
    println!("PageRank for A: {:.4}", ranks.get(&a).unwrap());
}
```
