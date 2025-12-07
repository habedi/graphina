# Graph Traversal Examples

Walk through the graph to find paths or visit nodes.

## Breadth-First Search (BFS)

Visit neighbors layer by layer. Good for finding shortest paths in unweighted graphs.

```rust
use graphina::core::types::Graph;
use graphina::traversal::bfs;

fn main() {
    let mut graph = Graph::<&str, f64>::new();
    let start = graph.add_node("Start");
    let end = graph.add_node("End");

    graph.add_edge(start, end, 1.0);

    // Returns a Vec<NodeId> in visitation order
    let visited_order = bfs(&graph, start);
    println!("Visited {} nodes via BFS", visited_order.len());
}
```

## Depth-First Search (DFS)

Explore as deep as possible before backtracking.

```rust
use graphina::core::types::Graph;
use graphina::traversal::dfs;

fn main() {
    let mut graph = Graph::<&str, f64>::new();
    let start = graph.add_node("Start");

    let visited_order = dfs(&graph, start);
}
```

## Bidirectional Search

Find the shortest path between two nodes by searching from both ends simultaneously.

```rust
use graphina::core::types::Graph;
use graphina::traversal::bidis;

fn main() {
    let mut graph = Graph::<&str, f64>::new();
    let start = graph.add_node("Start");
    let end = graph.add_node("End");

    // Connect them
    let mid = graph.add_node("Mid");
    graph.add_edge(start, mid, 1.0);
    graph.add_edge(mid, end, 1.0);

    if let Some(path) = bidis(&graph, start, end) {
        println!("Shortest path length (hops): {}", path.len() - 1);
        println!("Path: {:?}", path);
    } else {
        println!("No path found.");
    }
}
```
