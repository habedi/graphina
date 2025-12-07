# Subgraphs and Filtering Examples

Extract specific parts of your graph for focused analysis.

## Extracting an Induced Subgraph

Create a new graph containing only a specific set of nodes and their connections.

```rust
use graphina::core::types::Graph;
use graphina::subgraphs::SubgraphOps;

fn main() {
    let mut graph = Graph::<&str, f64>::new();
    let n1 = graph.add_node("A");
    let n2 = graph.add_node("B");
    let n3 = graph.add_node("C");
    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, n3, 1.0);

    let subset = vec![n1, n2];

    // New graph contains only A and B, and the edge A-B
    let subgraph = graph.subgraph(&subset).unwrap();
    println!("Subgraph nodes: {}", subgraph.node_count());
}
```

## Identifying Neighborhoods (Ego Graph)

Analyze the immediate surroundings of a node `k` hops away.

```rust
use graphina::core::types::Graph;
use graphina::subgraphs::SubgraphOps;

fn main() {
    let mut graph = Graph::<&str, f64>::new();
    let center = graph.add_node("Center");
    let leaf1 = graph.add_node("L1");
    let leaf2 = graph.add_node("L2");

    graph.add_edge(center, leaf1, 1.0);
    graph.add_edge(center, leaf2, 1.0);

    // All nodes within 1 hop of "Center" (radius was corrected to 1 for this small example)
    let ego = graph.ego_graph(center, 1).unwrap();
    println!("Ego network size: {}", ego.node_count());
}
```

## Filtering by Attribute

Keep only nodes or edges that match a predicate.

```rust
use graphina::core::types::Graph;
use graphina::subgraphs::SubgraphOps;

fn main() {
    let mut graph = Graph::<i32, f64>::new();
    // Add nodes 0 to 9
    for i in 0..10 {
        graph.add_node(i);
    }

    // Filter: Keep only even numbers
    let even_graph = graph.filter_nodes(|_id, &val| val % 2 == 0);

    // Filter: Keep edges with weight > 0.5
    let strong_edges = graph.filter_edges(|_u, _v, w| *w > 0.5);
}
```
