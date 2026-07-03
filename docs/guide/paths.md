# Path Finding

Graphina provides a suite of algorithms for finding shortest paths and traversing graphs.

## Dijkstra's Algorithm

Finds the shortest paths from a source node to all other nodes (or a target node) in a graph with non-negative edge
weights.

### Function Signature

```rust
pub fn dijkstra<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    source: NodeId
) -> Result<NodeMap<Option<W>>>
```

The generic `dijkstra` requires a totally ordered weight type (`W: Ord`), so wrap floating-point
weights in `ordered_float::OrderedFloat`. For plain `f64` graphs, `dijkstra_path_f64` computes the
same distances (plus the predecessor trace) without the wrapper.

### Example

```rust
use graphina::core::paths::dijkstra_path_f64;

let (cost, _trace) = dijkstra_path_f64(&graph, start_node, None)?;

if let Some(distance) = cost.get(&end_node).copied().flatten() {
    println!("Shortest distance: {}", distance);
} else {
    println!("Node is unreachable");
}
```

## A* (A-Star) Search

Finds the shortest path to a specific target using a heuristic function to guide the search.
Faster than Dijkstra if you have a good heuristic (like Euclidean distance for maps).

Like `dijkstra`, `a_star` requires a totally ordered weight type, so convert `f64` weights to
`OrderedFloat<f64>` first. The heuristic must never overestimate the true remaining distance.

```rust
use graphina::core::paths::a_star;
use graphina::core::types::NodeId;
use ordered_float::OrderedFloat;

let g_ord = graph.convert::<OrderedFloat<f64>>();

// Heuristic function: a lower bound on the distance from u to the target,
// for example the Euclidean distance when nodes have coordinates
let heuristic = |u: NodeId| OrderedFloat((x1 - x2).hypot(y1 - y2));

let path = a_star(&g_ord, start, end, heuristic)?;
```

## Bellman-Ford

Computes shortest paths from a single source in graphs that may contain negative edge weights.
It can also detect negative cycles.

```rust
use graphina::core::paths::bellman_ford;

match bellman_ford(&graph, start_node) {
    Some(distances) => println!("Calculated distances"),
    None => println!("Negative cycle detected!"),
}
```

## Floyd-Warshall

Computes all-pairs shortest paths. Returns a matrix (map of maps) of distances.
Note: This is $O(V^3)$, so use only on small graphs (< 500-1000 nodes).

```rust
use graphina::core::paths::floyd_warshall;

let all_paths = floyd_warshall(&graph);
```

## Johnson's Algorithm

Computes all-pairs shortest paths in sparse graphs that may contain negative weights (but no negative cycles).
It uses Bellman-Ford to reweight the graph, then runs Dijkstra from every node.

```rust
use graphina::core::paths::johnson;

let all_paths = johnson(&graph);
```
