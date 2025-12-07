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

    let cities = vec!["A", "B", "C", "D"];
    let nodes: Vec<_> = cities.iter().map(|&name| graph.add_node(name)).collect();

    // Add edges (simulating distances)
    // ... setup edges ...

    let start_node = nodes[0];

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
    // ... setup graph ...

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
    // ... setup graph ...

    let clique = max_clique(&graph);
    println!("Found clique of size: {}", clique.len());
}
```
