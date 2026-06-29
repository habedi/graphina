# Connected Components

Connected components are maximal subgraphs where every node is reachable from every other node.

## Function Signature

```python
pg.community.connected_components(graph: PyGraph) -> Dict[int, int]
```

## Parameters

- graph: Undirected graph to analyze

## Returns

Dictionary mapping node IDs to component IDs.

## Description

A connected component is a maximal connected subgraph. For undirected graphs, every node belongs to exactly one
connected component. Nodes in the same component can reach each other by following edges.

## Time Complexity

O(V + E) - linear in graph size

## Space Complexity

O(V)

## Example

```python
import pygraphina as pg

# Create a disconnected graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(8)]

# Component 1
g.add_edge(nodes[0], nodes[1], 1.0)
g.add_edge(nodes[1], nodes[2], 1.0)

# Component 2
g.add_edge(nodes[3], nodes[4], 1.0)
g.add_edge(nodes[4], nodes[5], 1.0)

# Isolated nodes: nodes[6], nodes[7]

# Find connected components
components = pg.community.connected_components(g)

# Group nodes by component
from collections import defaultdict

comp_groups = defaultdict(list)
for node, comp_id in components.items():
    comp_groups[comp_id].append(node)

for comp_id, members in sorted(comp_groups.items()):
    print(f"Component {comp_id}: {sorted(members)}")
```

## Use Cases

- Identifying disconnected subnetworks
- Data quality checks
- Network resilience analysis
- Preprocessing for other algorithms

## Related Concepts

- Weakly Connected Components: For directed graphs (same concept but following edge direction)
- Strongly Connected Components: For directed graphs (nodes that can reach each other)
- Biconnected Components: Subgraphs with no single articulation points

## Comparison

| Metric        | Connected Components |
|---------------|----------------------|
| Time          | O(V + E)             |
| Space         | O(V)                 |
| Parameters    | None                 |
| Deterministic | Yes                  |

## Advantages

- Fast: Linear time complexity O(V + E)
- Simple: Easy to understand and implement
- Exact: Deterministic results
- No parameters: No tuning needed
- Fundamental: Foundation for other algorithms

## When to Use

- Check graph connectivity
- Identify isolated subnetworks
- Preprocess graphs before analysis
- Data quality checks
- Network resilience analysis

## Implementation Notes

- Uses depth-first search (DFS) or breadth-first search (BFS)
- Works on both connected and disconnected graphs
- Returns component IDs starting from 0
- For undirected graphs only
- Deterministic: same result every time

## Common Applications

```python
import pygraphina as pg

# Example: Finding isolated users in social network
g = pg.PyGraph()
users = [g.add_node(i) for i in range(100)]

# Add some edges (friendships)
# ... add edges ...

components = pg.community.connected_components(g)

# Find isolated users
isolated = []
for user, comp_id in components.items():
    # Count users in this component
    comp_size = sum(1 for c_id in components.values() if c_id == comp_id)
    if comp_size == 1:
        isolated.append(user)

print(f"Isolated users: {isolated}")
print(f"Number of distinct networks: {max(components.values()) + 1}")
```

## Edge Cases

- Single node: Component size = 1
- Complete graph: One component with all nodes
- Empty graph: One component per node
- Tree structure: One component if connected, multiple if forest

## Performance Characteristics

| Graph Size | Time | Space |
|-----------|------|-------|
| 1K nodes | < 1ms | ~1KB |
| 10K nodes | ~1ms | ~10KB |
| 100K nodes | ~10ms | ~100KB |
| 1M nodes | ~100ms | ~1MB |

## See Also

- [Louvain Algorithm](louvain.md) - Community detection in connected graphs
- [Label Propagation](label_propagation.md) - General community detection
- [Spectral Clustering](spectral.md) - Mathematical approach to communities
- [Girvan-Newman](girvan_newman.md) - Hierarchical community detection

## Historical Background

Connected components are one of the oldest concepts in graph theory and are fundamental to graph algorithms. Often one of the first algorithms taught in computer science courses.
