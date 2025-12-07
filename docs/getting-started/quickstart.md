# Quick Start with Graphina

Create graphs, add nodes, and run algorithms with Graphina.

## Creating a Graph

Rust requires defined types for node and edge data. Create an undirected graph with `String` nodes and `f64` edge weights:

```rust
use graphina::core::types::Graph;

fn main() {
    // Node type: String, Edge type: f64
    let mut graph = Graph::<String, f64>::new();
}
```

!!! note "Graph Type"
    Use `Digraph` for directed graphs.

## Adding Nodes

`add_node` returns a `NodeId`. Use this ID to reference the node, like when adding edges.

```rust
    // Add nodes
    let n1 = graph.add_node("Alice".to_string());
    let n2 = graph.add_node("Bob".to_string());
    let n3 = graph.add_node("Charlie".to_string());

    println!("Graph has {} nodes", graph.node_count());
```

## Adding Edges

Connect nodes using their `NodeId`s.

```rust
    // Add weighted edges
    graph.add_edge(n1, n2, 1.0); // Alice -- Bob
    graph.add_edge(n2, n3, 2.0); // Bob -- Charlie

    // Add an edge and get its ID
    let e1 = graph.add_edge(n1, n3, 0.5); // Alice -- Charlie

    println!("Graph has {} edges", graph.edge_count());
```

## Examining the Graph

Check for existence, degrees, or neighbors.

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

Graphina provides standard algorithms in modules like `centrality`, `community`, and `paths`.

Calculate PageRank:

```rust
use graphina::centrality::pagerank;

    // Create a simple graph for the example
    let mut graph = Graph::<String, f64>::new();
    let n1 = graph.add_node("Alice".to_string());
    let n2 = graph.add_node("Bob".to_string());
    graph.add_edge(n1, n2, 1.0);

    // Calculate PageRank
    // (graph, damping_factor, max_iterations, tolerance, nstart)
    let scores = pagerank(&graph, 0.85, 100, 1e-6, None).unwrap();

    println!("PageRank scores:");
    for (node_id, score) in scores {
        println!("{}: {:.4}", graph[node_id], score);
    }
```

## Complete Example

Combine creation, population, and analysis:

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

    let ranks = pagerank(&g, 0.85, 100, 1e-6, None).unwrap();
    println!("PageRank for A: {:.4}", ranks.get(&a).unwrap());
}
```

## Advanced Graph Creation

For more control over graph properties (capacity, self-loops, etc.), use the `AdvancedGraphBuilder`.

```rust
use graphina::core::builders::UndirectedGraphBuilder;

fn main() {
    let graph = UndirectedGraphBuilder::<i32, f64>::undirected()
        .with_capacity(100, 200)       // Pre-allocate memory
        .allow_self_loops(false)       // Disallow self-loops
        .allow_parallel_edges(false)   // Disallow parallel edges
        .add_node(1)
        .add_node(2)
        .add_edge(0, 1, 1.0)
        .build()
        .unwrap();

    println!("Graph created with {} nodes", graph.node_count());
}
```
