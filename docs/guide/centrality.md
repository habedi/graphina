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
use graphina::centrality::pagerank::pagerank;
use graphina::centrality::degree::degree_centrality;

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
let degree_scores = degree_centrality(&g).unwrap();
println!("Degree Centrality: {:?}", degree_scores);
```

## Betweenness Centrality

Betweenness centrality quantifies the influence of a node over the flow of information between other nodes.
It counts the fraction of shortest paths that pass through a node.

### Use Case

Finding bridges or bottlenecks in a network (for example, a critical router in a network topology).

```rust
use graphina::centrality::betweenness::betweenness_centrality;

let scores = betweenness_centrality(&g, true).unwrap();
```

## Degree Centrality

The simplest measure: the number of edges connected to a node.

- For Directed graphs: often split into In-Degree and Out-Degree.
- For Undirected graphs: just Degree.

```rust
use graphina::centrality::degree::degree_centrality;

let scores = degree_centrality(&g).unwrap();
```

## Eigenvector Centrality

Determines importance based on connections to other high-scoring nodes. It is similar to PageRank but without the
damping factor/random jump.

```rust
use graphina::centrality::eigenvector::eigenvector_centrality;

// (graph, max_iterations, tolerance)
let scores = eigenvector_centrality(&g, 100, 1e-6).unwrap();
```

## Closeness Centrality

A node is central if it is close to all other nodes. It is defined as the reciprocal of the sum of shortest path
distances.

```rust
use graphina::centrality::closeness::closeness_centrality;

let scores = closeness_centrality(&g).unwrap();
```

## Katz Centrality

Computes the relative influence of a node by measuring the number of walks of length $k$ between node pairs, attenuated by a factor $\alpha$.

```rust
use graphina::centrality::katz::katz_centrality;

// Arguments: graph, alpha, beta closure, max_iter, tolerance
let scores = katz_centrality(&g, 0.1, None, 1000, 1e-9).unwrap();
```

## Harmonic Centrality

A variant of closeness centrality designed for disconnected graphs. It sums the reciprocals of the shortest path distances.

```rust
use graphina::centrality::harmonic::harmonic_centrality;

let scores = harmonic_centrality(&g).unwrap();
```

## Personalized PageRank

Computes a PageRank vector biased towards a set of target nodes defined by a personalization vector.

```rust
use graphina::centrality::personalized::personalized_pagerank;

let personalization = vec![0.8, 0.2]; // mapped to nodes in order
let scores = personalized_pagerank(&g, Some(personalization), 0.85, 1e-6, 100).unwrap();
```

## VoteRank

Identifies a set of influential node seeds using a voting mechanism where elected nodes weaken their neighbors' voting weights.

```rust
use graphina::centrality::other::voterank;

// Returns top 3 seeds as a Vec<NodeId>
let seeds = voterank(&g, 3);
```

## Reaching Centrality

Local reaching centrality measures the fraction of nodes that can be reached from a node within a given distance. Global reaching centrality considers the entire graph.

```rust
use graphina::centrality::other::{local_reaching_centrality, global_reaching_centrality};

// local reaching centrality within 2 steps
let local_scores = local_reaching_centrality(&g, 2).unwrap();

// global reaching centrality
let global_scores = global_reaching_centrality(&g).unwrap();
```

## Laplacian Centrality

Measures the drop in the Laplacian energy of a graph when a node is removed.

```rust
use graphina::centrality::other::laplacian_centrality;

let scores = laplacian_centrality(&g).unwrap();
```
