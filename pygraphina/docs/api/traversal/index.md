# Traversal Algorithms

Graph traversal and search algorithms for exploring graph structure.

!!! info "Available as Graph Methods"
    Traversal methods are available directly on graph objects. See the [Graph API Reference](../graph.md) for complete details.

## Overview

PyGraphina provides several graph traversal algorithms:

| Algorithm | Method | Time | Use Case |
|-----------|--------|------|----------|
| Breadth-First Search | `g.bfs(start)` | O(V+E) | Level-by-level exploration |
| Depth-First Search | `g.dfs(start)` | O(V+E) | Deep path exploration |
| Iterative Deepening DFS | `g.iddfs(start, target, max_depth)` | O(b^d) | Memory-efficient search |
| Bidirectional Search | `g.bidirectional_search(start, target)` | O(b^(d/2)) | Fast shortest path |

Where:
- V = number of nodes
- E = number of edges
- b = branching factor
- d = depth of solution

## Breadth-First Search (BFS)

Explores nodes level by level, visiting all neighbors before going deeper.

### Method

```python
g.bfs(start: int) -> List[int]
```

Parameters:
- `start`: Starting node ID

Returns:
- `List[int]`: Nodes in BFS traversal order

Raises:
- `ValueError`: If start node doesn't exist

### Example

```python
import pygraphina as pg

g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(7)]

# Create a tree structure
#       0
#      / \
#     1   2
#    / \   \
#   3   4   5
#           |
#           6

g.add_edge(nodes[0], nodes[1], 1.0)
g.add_edge(nodes[0], nodes[2], 1.0)
g.add_edge(nodes[1], nodes[3], 1.0)
g.add_edge(nodes[1], nodes[4], 1.0)
g.add_edge(nodes[2], nodes[5], 1.0)
g.add_edge(nodes[5], nodes[6], 1.0)

order = g.bfs(nodes[0])
print(order)  # [0, 1, 2, 3, 4, 5, 6]
# Level 0: [0]
# Level 1: [1, 2]
# Level 2: [3, 4, 5]
# Level 3: [6]
```

### Use Cases

- Finding shortest paths in unweighted graphs
- Level-order traversal
- Finding connected components
- Testing graph connectivity

## Depth-First Search (DFS)

Explores as deep as possible along each branch before backtracking.

### Method

```python
g.dfs(start: int) -> List[int]
```

Parameters:
- `start`: Starting node ID

Returns:
- `List[int]`: Nodes in DFS traversal order

### Example

```python
order = g.dfs(nodes[0])
print(order)  # [0, 1, 3, 4, 2, 5, 6]
# Goes deep along one path before backtracking
```

### Use Cases

- Cycle detection
- Topological sorting
- Finding articulation points
- Maze solving

## Iterative Deepening DFS

Combines advantages of BFS and DFS with limited memory usage.

### Method

```python
g.iddfs(start: int, target: int, max_depth: int) -> List[int]
```

Parameters:
- `start`: Starting node ID
- `target`: Target node ID
- `max_depth`: Maximum search depth

Returns:
- `List[int]`: Path from start to target. Raises `ValueError` if no path is found within max_depth.

### Example

```python
try:
    path = g.iddfs(nodes[0], nodes[6], max_depth=10)
    print(f"Path found: {path}")
    print(f"Length: {len(path) - 1}")
except ValueError:
    print("No path found within depth limit")
```

### Use Cases

- Finding paths with depth constraints
- Memory-limited environments
- Exploring large graphs incrementally

## Bidirectional Search

Searches from both start and target simultaneously, meeting in the middle.

### Method

```python
g.bidirectional_search(start: int, target: int) -> List[int]
```

Parameters:
- `start`: Starting node ID
- `target`: Target node ID

Returns:
- `List[int]`: Path from start to target. Raises `ValueError` if no path exists.

### Example

```python
try:
    path = g.bidirectional_search(nodes[0], nodes[6])
    print(f"Shortest path: {path}")
    print(f"Distance: {len(path) - 1}")
except ValueError:
    print("No path exists")
```

### Use Cases

- Fast shortest path finding
- Large graphs where target is known
- Reducing search space

## Comparison

| Algorithm | Memory | Speed | Path Length | Best For |
|-----------|--------|-------|-------------|----------|
| BFS | O(V) | Fast | Shortest (unweighted) | Small/medium graphs |
| DFS | O(h) | Fast | Not shortest | Deep searches, cycles |
| IDDFS | O(d) | Medium | Shortest | Memory-limited |
| Bidirectional | O(V) | Very Fast | Shortest | Known target |

Where:
- V = nodes
- h = maximum depth
- d = solution depth

## Complete Example

```python
import pygraphina as pg

# Create a social network
g = pg.PyGraph()
people = ["Alice", "Bob", "Charlie", "David", "Eve", "Frank"]
nodes = {person: g.add_node(i) for i, person in enumerate(people)}

# Add friendships
friendships = [
    ("Alice", "Bob"),
    ("Alice", "Charlie"),
    ("Bob", "David"),
    ("Charlie", "David"),
    ("David", "Eve"),
    ("Eve", "Frank"),
]
for p1, p2 in friendships:
    g.add_edge(nodes[p1], nodes[p2], 1.0)

# BFS: Find all people reachable from Alice
reachable = g.bfs(nodes["Alice"])
print(f"Alice can reach: {[people[i] for i in reachable]}")

# DFS: Explore network depth-first
order = g.dfs(nodes["Alice"])
print(f"DFS order: {[people[i] for i in order]}")

# Find path from Alice to Frank
path = g.bidirectional_search(nodes["Alice"], nodes["Frank"])
if path:
    path_names = [people[i] for i in path]
    print(f"Shortest path: {' -> '.join(path_names)}")
    print(f"Degrees of separation: {len(path) - 1}")
```

## See Also

- [Graph API Reference](../graph.md) - Full method documentation
- [Path Algorithms](../core/paths.md) - Weighted shortest paths
- [Subgraphs](../subgraphs/index.md) - Extracting graph portions
