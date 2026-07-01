# Graph Traversal

Traverse graphs using standard algorithms like Breadth-First Search (BFS) and Depth-First Search (DFS).

## Basic Traversals

### Breadth-First Search (BFS)

Explores the graph layer by layer, visiting all neighbors of a node before moving to the next level.

```rust
use graphina::traversal::bfs;

let order = bfs(&graph, start_node);
println!("Visit order: {:?}", order);
```

### Depth-First Search (DFS)

Explores as far as possible along each branch before backtracking.

```rust
use graphina::traversal::dfs;

let order = dfs(&graph, start_node);
```

## Advanced Algorithms

### Iterative Deepening DFS (IDDFS)

Combines the space efficiency of DFS with the optimality of BFS. It repeatedly runs DFS with increasing depth limits.

```rust
use graphina::traversal::iddfs;

// Find path with max depth limit of 10
if let Some(path) = iddfs(&graph, start, target, 10) {
    println!("Found path: {:?}", path);
}
```

### Bidirectional Search

Runs two simultaneous searches: one forward from the source and one backward from the target, stopping when they meet. This can be significantly faster than standard BFS for finding shortest paths.

```rust
use graphina::traversal::bidis;

if let Some(path) = bidis(&graph, start, target) {
    println!("Shortest path: {:?}", path);
}
```
