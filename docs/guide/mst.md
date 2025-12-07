# Minimum Spanning Tree (MST)

Minimum Spanning Tree algorithms find a subset of edges that connect all nodes in a graph with the minimum possible total edge weight, without forming any cycles.

## Algorithms

### Prim's Algorithm

Prim's algorithm grows the MST from a starting node, adding the cheapest edge at each step.

```rust
use graphina::core::types::{Graph, NodeId};
use graphina::mst::prim_mst;
use ordered_float::OrderedFloat;

fn main() {
    let mut g = Graph::<i32, OrderedFloat<f64>>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);

    g.add_edge(n1, n2, OrderedFloat(1.0));
    g.add_edge(n1, n3, OrderedFloat(3.0));
    g.add_edge(n2, n3, OrderedFloat(2.0)); // Cheaper path via n2

    let (mst_edges, total_weight) = prim_mst(&g).unwrap();
    println!("Total Weight: {}", total_weight);
}
```

### Kruskal's Algorithm

Kruskal's algorithm sorts all edges by weight and adds them if they don't form a cycle.

```rust
use graphina::mst::kruskal_mst;

let (mst_edges, total_weight) = kruskal_mst(&g).unwrap();
```

### Borůvka's Algorithm (Parallel)

A parallel implementation that is efficient for large graphs. It works by finding the minimum weight edge incident to each component in parallel.

!!! note "Parallel Feature"
    Requires the `parallel` feature and `W` must implement `Send + Sync`.

```rust
use graphina::mst::boruvka_mst;

let (mst_edges, total_weight) = boruvka_mst(&g).unwrap();
```

## Weight Type Requirements

Edge weights `W` must implement `Ord`. For floating point numbers, use `OrderedFloat` or a similar wrapper to provide total ordering.

```rust
use ordered_float::OrderedFloat;
let mut g = Graph::<i32, OrderedFloat<f64>>::new();
let u = g.add_node(1);
let v = g.add_node(2);
g.add_edge(u, v, OrderedFloat(1.5));
```
