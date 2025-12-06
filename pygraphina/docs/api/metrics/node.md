# Node Metrics

Node metrics characterize individual nodes in the graph.

## Available Metrics

### Degree Information

```python
g.degree(node)           # Total degree of node
# For directed graphs:
g.in_degree(node)        # In-degree
g.out_degree(node)       # Out-degree
```

### Clustering

```python
g.clustering_of(node)    # Local clustering coefficient
g.triangles_of(node)     # Number of triangles through node
```

### Neighbors

```python
g.neighbors(node)        # Adjacent nodes
# For directed graphs:
g.out_neighbors(node)    # Outgoing neighbors
g.in_neighbors(node)     # Incoming neighbors
```

### Attributes

```python
g.get_node_attr(node)    # Node attribute value
g.contains_node(node)    # Does node exist
```

## Examples

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Add some edges
for i in range(9):
    g.add_edge(nodes[i], nodes[i+1], 1.0)
g.add_edge(nodes[0], nodes[5], 1.0)

# Analyze each node
for node in nodes:
    degree = g.degree(node)
    neighbors = g.neighbors(node)
    clustering = g.clustering_of(node)
    triangles = g.triangles_of(node)

    print(f"Node {node}:")
    print(f"  Degree: {degree}")
    print(f"  Neighbors: {neighbors}")
    print(f"  Clustering: {clustering:.3f}")
    print(f"  Triangles: {triangles}")
```

## Interpretation

- **Degree**: Number of connections (importance in many contexts)
- **Clustering**: How densely connected the neighbors are
- **Triangles**: Participation in cohesive groups
- **Neighbors**: Direct connections

## Time Complexity

| Metric     | Complexity |
|------------|------------|
| Degree     | O(1)       |
| Neighbors  | O(degree)  |
| Clustering | O(degree²) |
| Triangles  | O(degree²) |

## Use Cases

- Finding influential nodes (high degree)
- Identifying cohesive groups (high clustering)
- Community structure analysis
- Network resilience (removing high-degree nodes)
