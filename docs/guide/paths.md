# Path Finding

Graphina provides a suite of algorithms for finding shortest paths and traversing graphs.

## Dijkstra's Algorithm

Finds the shortest paths from a source node to all other nodes (or a target node) in a graph with **non-negative** edge
weights.

### Function Signature

```rust
pub fn dijkstra<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    source: NodeId
) -> Result<NodeMap<Option<W>>>
```

### Example

```rust
use graphina::core::paths::dijkstra;

let result = dijkstra(&graph, start_node)?;

if let Some(cost) = result.get(&end_node).unwrap() {
    println!("Shortest distance: {}", cost);
} else {
    println!("Node is unreachable");
}
```

## A* (A-Star) Search

Finds the shortest path to a specific target using a heuristic function to guide the search.
Faster than Dijkstra if you have a good heuristic (e.g., Euclidean distance for maps).

```rust
use graphina::core::paths::a_star;

// Heuristic function: estimate distance from u to target
let heuristic = |u: NodeId| -> f64 {
    // ... calculate distance
    0.0
};

let path = a_star(&graph, start, end, heuristic)?;
```

## Bellman-Ford

Computes shortest paths from a single source in graphs that may contain **negative** edge weights.
It can also detect negative cycles.

```rust
use graphina::core::paths::bellman_ford;

match bellman_ford(&graph, start_node) {
    Some(distances) => println!("Calculated distances"),
    None => println!("Negative cycle detected!"),
}
```

## Floyd-Warshall

Computes **all-pairs** shortest paths. Returns a matrix (map of maps) of distances.
Note: This is $O(V^3)$, so use only on small graphs (< 500-1000 nodes).

```rust
use graphina::core::paths::floyd_warshall;

let all_paths = floyd_warshall(&graph);
```
