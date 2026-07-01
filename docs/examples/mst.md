# Minimum Spanning Tree Examples

Find the subset of edges that connects the graph with minimal total weight.

## Prim's Algorithm

Use Prim's for dense graphs or when you want to grow a tree from a specific node.

```rust
use graphina::core::types::Graph;
use graphina::mst::prim_mst;
use ordered_float::OrderedFloat;

fn main() {
    // Note: Weights must be Ord (OrderedFloat for f64)
    let mut graph = Graph::<&str, OrderedFloat<f64>>::new();

    let n1 = graph.add_node("A");
    let n2 = graph.add_node("B");
    let n3 = graph.add_node("C");

    graph.add_edge(n1, n2, OrderedFloat(1.0));
    graph.add_edge(n2, n3, OrderedFloat(2.0));
    graph.add_edge(n1, n3, OrderedFloat(10.0));

    // Returns (Vec<MstEdge<W>>, total_weight)
    if let Ok((edges, total)) = prim_mst(&graph) {
        println!("MST Total Weight: {}", total);
        for edge in edges {
            println!("Edge: {:?} - {:?} (wt: {})", edge.u, edge.v, edge.weight);
        }
    }
}
```

## Kruskal's Algorithm

Kruskal's is efficient for sparse graphs and works by sorting edges.

```rust
use graphina::core::types::Graph;
use graphina::mst::kruskal_mst;
use ordered_float::OrderedFloat;

fn main() {
    let mut graph = Graph::<&str, OrderedFloat<f64>>::new();
    let n1 = graph.add_node("A");
    let n2 = graph.add_node("B");
    graph.add_edge(n1, n2, OrderedFloat(1.0));

    if let Ok((edges, total)) = kruskal_mst(&graph) {
        println!("Kruskal MST Weight: {}", total);
    }
}
```
