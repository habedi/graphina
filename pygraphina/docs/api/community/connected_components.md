# Connected Components

Connected components are maximal subgraphs where every node is reachable from every other node.

## Function Signature

```python
pg.community.connected_components(graph: PyGraph) -> Dict[int, int]
```

## Parameters

- **graph**: Undirected graph to analyze

## Returns

Dictionary mapping node IDs to component IDs.

## Description

A connected component is a maximal connected subgraph. For undirected graphs, every node belongs to exactly one connected component. Nodes in the same component can reach each other by following edges.

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

- **Weakly Connected Components**: For directed graphs (same concept but following edge direction)
- **Strongly Connected Components**: For directed graphs (nodes that can reach each other)
- **Biconnected Components**: Subgraphs with no single articulation points

## Comparison

| Metric | Connected Components |
|--------|---------------------|
| Time | O(V + E) |
| Space | O(V) |
| Parameters | None |
| Deterministic | Yes |
