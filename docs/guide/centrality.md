# Centrality Algorithms

Centrality measures identify the most important nodes in a graph.
Graphina provides implementations of standard centrality metrics.

## PageRank

PageRank computes the importance of nodes based on incoming links, modeling a random surfer.

### Function Signature

```rust
pub fn pagerank<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    damping: f64,
    max_iter: usize,
    tolerance: f64,
    nstart: Option<&NodeMap<f64>>,
) -> Result<NodeMap<f64>>
```

### Example

```rust
use graphina::core::types::Digraph;
use graphina::centrality::pagerank;
use graphina::centrality::degree_centrality; // Added for degree_centrality
use graphina::centrality::DegreeDirection; // Added for DegreeDirection

let mut g = Digraph::<&str, f64>::new();
let n1 = g.add_node("A");
let n2 = g.add_node("B");
let n3 = g.add_node("C");
let n4 = g.add_node("D");

g.add_edge(n1, n2, 1.0);
g.add_edge(n1, n3, 1.0);
g.add_edge(n2, n3, 1.0);
g.add_edge(n3, n4, 1.0);
g.add_edge(n4, n1, 1.0);

let scores = pagerank(&g, 0.85, 100, 1e-6, None).unwrap();

for (node, score) in scores {
    println!("Node {:?} has PageRank {:.4}", node, score);
}

// Calculate Degree Centrality for the same graph
let degree_scores = degree_centrality(&g, DegreeDirection::Successors);
println!("Degree Centrality (Successors): {:?}", degree_scores);
```

## Betweenness Centrality

Betweenness centrality quantifies the influence of a node over the flow of information between other nodes.
It counts the fraction of shortest paths that pass through a node.

### Use Case

Finding bridges or bottlenecks in a network (for example, a critical router in a network topology).

```rust
use graphina::centrality::betweenness;

let scores = betweenness(&g);
```

## Degree Centrality

The simplest measure: the number of edges connected to a node.

- For **Directed** graphs: often split into In-Degree and Out-Degree.
- For **Undirected** graphs: just Degree.

```rust
use graphina::centrality::degree;

let scores = degree(&g);
```

## Eigenvector Centrality

Determines importance based on connections to other high-scoring nodes. It is similar to PageRank but without the
damping factor/random jump.

```rust
use graphina::centrality::eigenvector;

// (graph, max_iterations, tolerance)
let scores = eigenvector(&g, 100, 1e-6);
```

## Closeness Centrality

node is central if it is close to all other nodes. It is defined as the reciprocal of the sum of shortest path
distances.

```rust
use graphina::centrality::closeness;

let scores = closeness(&g);
```
