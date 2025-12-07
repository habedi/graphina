# Quick Start

Here's how to create a simple graph and run an algorithm.

```rust
use graphina::core::types::Graph;

fn main() {
    let mut graph = Graph::<i32, f64>::new();

    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);

    graph.add_edge(n1, n2, 1.0);

    println!("Graph has {} nodes", graph.node_count());
}
```
