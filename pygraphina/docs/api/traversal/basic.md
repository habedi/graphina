# Breadth-First and Depth-First Search

BFS and DFS are fundamental graph traversal algorithms for exploring nodes systematically.

## Function Signatures

```python
graph.bfs(start: int) -> List[int]
graph.dfs(start: int) -> List[int]
graph.bidirectional_search(start: int, target: int) -> Optional[List[int]]
```

## Parameters

- **start**: Starting node ID
- **target**: Target node ID (for bidirectional search)

## Returns

List of node IDs in traversal or search order.

## Breadth-First Search (BFS)

Explores nodes level by level, one distance at a time.

**Data Structure**: Queue (FIFO)

**Properties**:

- Finds shortest path in unweighted graphs
- Level-order exploration
- Memory-intensive for wide graphs

**Example**:

```python
import pygraphina as pg

# Create a tree structure
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(7)]

# Build tree: 0 is root
g.add_edge(nodes[0], nodes[1], 1.0)
g.add_edge(nodes[0], nodes[2], 1.0)
g.add_edge(nodes[1], nodes[3], 1.0)
g.add_edge(nodes[1], nodes[4], 1.0)
g.add_edge(nodes[2], nodes[5], 1.0)
g.add_edge(nodes[2], nodes[6], 1.0)

# BFS from root gives level-order
bfs = g.bfs(nodes[0])
print(f"BFS: {bfs}")
# Output: [0, 1, 2, 3, 4, 5, 6]
```

## Depth-First Search (DFS)

Explores as deep as possible before backtracking.

**Data Structure**: Stack (LIFO) or Recursion

**Properties**:

- Memory-efficient for deep graphs
- Useful for cycle detection
- Useful for topological sorting

**Example**:

```python
import pygraphina as pg

# Create a path graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(5)]

# Create path: 0-1-2-3-4
for i in range(4):
    g.add_edge(nodes[i], nodes[i + 1], 1.0)

# Add a cross-edge
g.add_edge(nodes[0], nodes[3], 1.0)

# DFS explores deeply
dfs = g.dfs(nodes[0])
print(f"DFS: {dfs}")
# Output depends on implementation order
```

## Bidirectional Search

Search from both start and target, meeting in the middle.

**Advantages**:

- Faster than single-direction for some cases
- Effective for point-to-point queries

**Example**:

```python
# Find shortest path using bidirectional search
start, target = nodes[0], nodes[4]
path = g.bidirectional_search(start, target)
if path:
    print(f"Path: {path}")
else:
    print("No path found")
```

## Time and Space Complexity

| Algorithm     | Time     | Space             |
|---------------|----------|-------------------|
| BFS           | O(V + E) | O(V)              |
| DFS           | O(V + E) | O(h) (h = height) |
| Bidirectional | O(V + E) | O(V)              |

## Use Cases

### BFS

- Finding shortest paths (unweighted)
- Level-order analysis
- Connected components
- Breadth-based exploration

### DFS

- Topological sorting
- Cycle detection
- Strongly connected components
- Depth-based exploration

### Bidirectional

- Point-to-point shortest paths
- Early termination in large graphs

## IDDFS (Iterative Deepening DFS)

Combine DFS memory efficiency with BFS completeness:

```python
graph.iddfs(start: int, target: int, max_depth: int) -> Optional[List[int]]
```

Perform DFS with increasing depth limits until target found.

**Best for**:

- Unknown search depth
- Memory-constrained environments
- Complete but memory-efficient search

## Comparison

| Property                   | BFS  | DFS     | Bidirectional |
|----------------------------|------|---------|---------------|
| Shortest Path (Unweighted) | Yes  | No      | Yes           |
| Space Efficient            | No   | Yes     | No            |
| Finds All Paths            | Yes  | Yes     | No            |
| Memory Usage               | O(V) | O(h)    | O(V)          |
| Parallelizable             | Yes  | Limited | Moderate      |
