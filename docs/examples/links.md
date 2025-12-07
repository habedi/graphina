# Link Prediction Examples

Link prediction algorithms help identify missing or future connections in a network.

## Predicting Friends with Jaccard Coefficient

The Jaccard coefficient is a classic way to measure similarity between two sets of neighbors.

```rust
use graphina::core::types::Graph;
use graphina::links::similarity::jaccard_coefficient;

fn main() {
    let mut graph = Graph::<&str, f64>::new();
    let n0 = graph.add_node("Alice");
    let n1 = graph.add_node("Bob");
    let n2 = graph.add_node("Charlie");
    let n3 = graph.add_node("David");

    // Alice and Bob share Charlie as a friend
    graph.add_edge(n0, n2, 1.0);
    graph.add_edge(n1, n2, 1.0);

    // Run prediction for all pairs
    let predictions = jaccard_coefficient(&graph, None);

    for ((u, v), score) in predictions {
        // Filter out existing edges if desired
        if !graph.contains_edge(u, v) && u != v {
            println!("Score between {:?} and {:?}: {:.4}", u, v, score);
        }
    }
}
```

## Using Resource Allocation Index

The Resource Allocation (RA) index is often more effective than Jaccard for social networks as it penalizes common neighbors that are "hubs" (have high degree).

```rust
use graphina::core::types::Graph;
use graphina::links::allocation::resource_allocation_index;

fn main() {
    let mut graph = Graph::<&str, f64>::new();
    let n0 = graph.add_node("Alice");
    let n1 = graph.add_node("Bob");
    let n2 = graph.add_node("Charlie");

    // Alice and Bob both know Charlie
    graph.add_edge(n0, n2, 1.0);
    graph.add_edge(n1, n2, 1.0);

    let ra_scores = resource_allocation_index(&graph, None);
    // Process scores...
}
```

## Community-Aware Prediction

If you have community labels, you can use Soundarajan-Hopcroft variants to boost scores for nodes in the same community.

```rust
use graphina::core::types::Graph;
use graphina::links::allocation::ra_index_soundarajan_hopcroft;

fn main() {
    let mut graph = Graph::<&str, f64>::new();
    let mut graph = Graph::<&str, f64>::new();
    let n1 = graph.add_node("A");
    let n2 = graph.add_node("B");

    // Two nodes in the same community (e.g., both even indices)
    let n3 = graph.add_node("C");
    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, n3, 1.0);

    // Mock community assignment closure
    let community_map = |node_id| {
        // In reality, lookup from a HashMap or property
        if node_id.index() % 2 == 0 { 1 } else { 2 }
    };

    let scores = ra_index_soundarajan_hopcroft(&graph, None, community_map);
}
```
