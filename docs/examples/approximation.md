# Approximation Algorithms Examples

Examples of using Graphina to solve NP-hard problems using approximation heuristics.

## Solving Traveling Salesman Problem (TSP)

Find an approximate shortest path that visits every node exactly once.

```rust
use graphina::core::types::Graph;
use graphina::approximation::tsp::greedy_tsp;
use ordered_float::OrderedFloat;

fn main() {
    // Note: TSP generally requires a complete or dense graph with metric weights.
    // Weights must be Ord, so we use OrderedFloat<f64>.
    let mut graph = Graph::<&str, OrderedFloat<f64>>::new();

    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");

    // Add edges forming a cycle/tour
    graph.add_edge(a, b, OrderedFloat(1.0));
    graph.add_edge(b, c, OrderedFloat(1.0));
    graph.add_edge(c, d, OrderedFloat(1.0));
    graph.add_edge(d, a, OrderedFloat(1.0));
    // Cross edges
    graph.add_edge(a, c, OrderedFloat(1.5));
    graph.add_edge(b, d, OrderedFloat(1.5));

    let start_node = a;

    if let Ok((tour, cost)) = greedy_tsp(&graph, start_node) {
        println!("Tour order: {:?}", tour);
        println!("Total cost: {}", cost);
    } else {
        println!("No tour found (graph might not be connected)");
    }
}
```

## Minimizing Vertex Cover

Find the smallest set of nodes that "covers" (touches) every edge in the graph.

```rust
use graphina::core::types::Graph;
use graphina::approximation::vertex_cover::min_vertex_cover;

fn main() {
    let mut graph = Graph::<i32, f64>::new();
    let mut graph = Graph::<i32, f64>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    graph.add_edge(n1, n2, 1.0);

    let cover = min_vertex_cover(&graph);
    println!("Vertex cover size: {}", cover.len());
    println!("Nodes in cover: {:?}", cover);
}
```

## Finding Maximum Clique

Find a large clique (subset of fully connected nodes) in the graph.

```rust
use graphina::core::types::Graph;
use graphina::approximation::clique::max_clique;

fn main() {
    let mut graph = Graph::<i32, f64>::new();
    let mut graph = Graph::<i32, f64>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);

    // Create a clique
    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, n3, 1.0);
    graph.add_edge(n3, n1, 1.0);

    let clique = max_clique(&graph);
    println!("Found clique of size: {}", clique.len());
}
```
