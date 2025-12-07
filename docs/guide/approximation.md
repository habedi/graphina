# Approximation Algorithms

Graphina provides approximation algorithms for several NP-hard graph problems.

## Traveling Salesman Problem (TSP)

Finds an approximate solution to the TSP (shortest tour visiting all nodes).

### Greedy Algorithm

Constructs a tour by repeatedly visiting the nearest unvisited node.

```rust
use graphina::approximation::tsp::greedy_tsp;
use ordered_float::OrderedFloat;

// Weights must implementation Ord (use OrderedFloat for f64)
let g_ord = graph.convert::<OrderedFloat<f64>>();
if let Ok((tour, cost)) = greedy_tsp(&g_ord, start_node) {
    println!("Tour: {:?}, Cost: {}", tour, cost);
}
```

## Vertex Cover

Finds a subset of nodes such that every edge in the graph is incident to at least one node in the subset.

```rust
use graphina::approximation::vertex_cover::min_vertex_cover;

let cover = min_vertex_cover(&graph);
```

## Maximum Independent Set

Finds a set of nodes where no two nodes in the set are adjacent.

```rust
use graphina::approximation::independent_set::max_independent_set;

let set = max_independent_set(&graph);
```

## Maximum Clique

Finds the largest subset of nodes where every node is connected to every other node in the subset.

```rust
use graphina::approximation::clique::max_clique;

let clique = max_clique(&graph);
```
