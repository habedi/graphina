# Graph Traversal

Graph traversal algorithms systematically visit all nodes in a graph, exploring the graph structure.

## Available Algorithms

| Algorithm | Time | Space | Usage |
|-----------|------|-------|-------|
| BFS | O(V+E) | O(V) | Level-order exploration |
| DFS | O(V+E) | O(V) | Deep exploration |
| IDDFS | O(V+E) | O(d) | Limited memory |
| Bidirectional | O(V+E) | O(V) | Point-to-point paths |

## Common Usage Pattern

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Add edges
for i in range(9):
    g.add_edge(nodes[i], nodes[i+1], 1.0)

# Traverse from starting node
bfs_order = g.bfs(nodes[0])
dfs_order = g.dfs(nodes[0])

print(f"BFS: {bfs_order}")
print(f"DFS: {dfs_order}")
```

## Breadth-First Search (BFS)

- Explores nodes level by level
- FIFO queue structure
- Finds shortest paths in unweighted graphs
- Memory-intensive for wide graphs

## Depth-First Search (DFS)

- Explores as far as possible before backtracking
- LIFO stack structure
- Memory-efficient for deep graphs
- Useful for topological sorting

## Iterative Deepening DFS (IDDFS)

- Combines DFS space efficiency with BFS completeness
- Limited by maximum depth
- Good for memory-constrained environments

## Bidirectional Search

- Searches from both start and goal simultaneously
- Meets in the middle
- Faster than single-direction for some cases
- Good for point-to-point queries

## Applications

- Finding connected components
- Detecting cycles
- Reachability analysis
- Path finding
- Network exploration
- Topological sorting

## Example: Finding All Reachable Nodes

```python
import pygraphina as pg

g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Create a graph with isolated components
for i in range(5):
    g.add_edge(nodes[i], nodes[i+1], 1.0)

# Find all nodes reachable from node 0
reachable = g.bfs(nodes[0])
print(f"Reachable from 0: {reachable}")

# For disconnected graph, only get one component
assert 6 not in reachable
```
