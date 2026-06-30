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
use graphina::approximation::vertex_cover::min_weighted_vertex_cover;

let cover = min_weighted_vertex_cover(&graph);
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

## Other Approximations

### Average Clustering Coefficient

Estimates the average local clustering coefficient.

```rust
use graphina::approximation::clustering::average_clustering;

let avg_cc = average_clustering(&graph);
```

### Local Node Connectivity

Approximates the local node connectivity between two nodes using repeated BFS.

```rust
use graphina::approximation::connectivity::local_node_connectivity;

let conn = local_node_connectivity(&graph, source, target);
```

### Minimum Maximal Matching

Greedy approximation for minimum maximal matching.

```rust
use graphina::approximation::matching::min_maximal_matching;

let matching = min_maximal_matching(&graph);
```

### Ramsey R(2, t)

Approximates the Ramsey number R(2, t) by finding a max clique and max independent set.

```rust
use graphina::approximation::ramsey::ramsey_r2;

let (clique, ind_set) = ramsey_r2(&graph);
```

### Densest Subgraph

Finds a subgraph with maximum average degree using a greedy peeling strategy.

```rust
use graphina::approximation::subgraph::densest_subgraph;

let nodes = densest_subgraph(&graph);
```

### Treewidth

Approximates treewidth using minimum degree or minimum fill-in heuristics.

```rust
use graphina::approximation::treewidth::{treewidth_min_degree, treewidth_min_fill_in};

let (tw, order) = treewidth_min_degree(&graph);
// or
let (tw, order) = treewidth_min_fill_in(&graph);
```
