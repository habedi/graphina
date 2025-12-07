# Centrality Examples

Use various centrality measures to identify important nodes in a graph.

## Degree Centrality

Computes the centrality of nodes based on the number of connections.

```rust
use graphina::centrality::degree::{degree_centrality, in_degree_centrality, out_degree_centrality};
use graphina::core::types::{Graph, Digraph};

fn main() {
    // 1. Undirected Degree
    let mut g = Graph::new();
    let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
    g.add_edge(nodes[0], nodes[1], ());
    g.add_edge(nodes[0], nodes[2], ());

    let cent = degree_centrality(&g).unwrap();
    println!("Node 1 degree: {}", cent[&nodes[0]]); // 2.0

    // 2. Directed In/Out Degree
    let mut dg = Digraph::new();
    let dnodes = [dg.add_node(1), dg.add_node(2)];
    dg.add_edge(dnodes[0], dnodes[1], ());

    let in_cent = in_degree_centrality(&dg).unwrap();
    let out_cent = out_degree_centrality(&dg).unwrap();

    println!("Node 1 Out-Degree: {}", out_cent[&dnodes[0]]); // 1.0
    println!("Node 2 In-Degree: {}", in_cent[&dnodes[1]]);   // 1.0
}
```

## PageRank

Detailed example of computing PageRank on a directed graph.

```rust
use graphina::centrality::pagerank::pagerank;
use graphina::core::types::Digraph;

fn page_rank_example() {
    let mut graph = Digraph::new();
    let nodes = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();

    // Create a citation-like structure
    let edges = [(0, 1, 1.0), (0, 2, 1.0), (1, 3, 1.0)];
    for (s, d, w) in edges {
        graph.add_edge(nodes[s], nodes[d], w);
    }

    // Compute PageRank (damping: 0.85, iter: 1000, tol: 1e-6)
    let scores = pagerank(&graph, 0.85, 1000, 1e-6_f64).unwrap();

    for n in nodes {
        println!("Node {:?}: {:.4}", n, scores[&n]);
    }
}
```

## Eigenvector Centrality

Measures influence by looking at connections to other high-scoring nodes.

```rust
use graphina::centrality::eigenvector::eigenvector_centrality;
use graphina::core::types::Graph;

fn main() {
    let mut g = Graph::new();
    let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
    g.add_edge(nodes[0], nodes[1], 1.0);
    g.add_edge(nodes[0], nodes[2], 2.0);

    let scores = eigenvector_centrality(&g, 1000, 1e-6).expect("Failed to converge");

    // Node 0 is connected to both 1 and 2, so it should be most central
    assert!(scores[&nodes[0]] > scores[&nodes[1]]);
}
```

## Katz Centrality

A generalization of degree centrality that counts paths of all lengths, penalized by an attenuation factor $\alpha$.

```rust
use graphina::centrality::katz::katz_centrality;
use graphina::core::types::Graph;

fn main() {
    let mut g = Graph::new();
    let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
    g.add_edge(nodes[0], nodes[1], 1.0);
    g.add_edge(nodes[0], nodes[2], 2.0);

    // alpha = 0.1, beta = 1.0
    let scores = katz_centrality(&g, 0.1, Some(&|_| 1.0), 1000, 1e-6).unwrap();

    println!("Node 1 Katz Score: {:.4}", scores[&nodes[0]]);
}
```

## Closeness Centrality

Based on the average shortest path distance to all other nodes.

```rust
use graphina::centrality::closeness::closeness_centrality;
use graphina::core::types::Graph;
use ordered_float::OrderedFloat;

fn main() {
    let mut graph: Graph<i32, OrderedFloat<f64>> = Graph::new();
    let nodes = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();
    let edges = [
        (0, 1, OrderedFloat(1.0)),
        (0, 2, OrderedFloat(1.0)),
        (1, 3, OrderedFloat(1.0))
    ];

    for (s, d, w) in edges {
        graph.add_edge(nodes[s], nodes[d], w);
    }

    let scores = closeness_centrality(&graph).unwrap();
    println!("Node 0 Closeness: {:.4}", scores[&nodes[0]]);
}
```
