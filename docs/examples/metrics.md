# Graph Metrics Examples

Compute structural properties of your graph.

## Distance Metrics

These metrics help you understand the size and connectivity of the graph.

```rust
use graphina::core::types::Graph;
use graphina::metrics::{diameter, radius, average_path_length};

fn main() {
    let mut graph = Graph::<&str, f64>::new();
    let n1 = graph.add_node("A");
    let n2 = graph.add_node("B");
    let n3 = graph.add_node("C");
    let n4 = graph.add_node("D");

    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, n3, 1.0);
    graph.add_edge(n3, n4, 1.0);

    if let Some(d) = diameter(&graph) {
        println!("Diameter: {}", d);
    }

    if let Some(r) = radius(&graph) {
        println!("Radius: {}", r);
    }

    // Average steps between any two nodes
    if let Some(avg) = average_path_length(&graph) {
        println!("Average Path Length: {:.2}", avg);
    }
}
```

## Clustering and Mixing

Understanding how nodes cluster together.

```rust
use graphina::core::types::Graph;
use graphina::metrics::{
    average_clustering_coefficient,
    transitivity,
    assortativity
};

fn main() {
    let mut graph = Graph::<&str, f64>::new();
    let n1 = graph.add_node("A");
    let n2 = graph.add_node("B");
    let n3 = graph.add_node("C");

    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, n3, 1.0);
    graph.add_edge(n3, n1, 1.0);

    // Local clustering averaged over all nodes
    let avg_cc = average_clustering_coefficient(&graph);
    println!("Avg Clustering Coeff: {:.4}", avg_cc);

    // Global clustering (triangles / connected triples)
    let trans = transitivity(&graph);
    println!("Transitivity: {:.4}", trans);

    // Assortativity (do high-degree nodes connect to high-degree nodes?)
    let r = assortativity(&graph);
    println!("Assortativity: {:.4}", r);
}
```
