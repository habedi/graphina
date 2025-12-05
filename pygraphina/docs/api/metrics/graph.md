# Graph Metrics

Graph metrics characterize properties of the entire graph.

## Available Metrics

### Basic Properties

```python
g.node_count()           # Number of nodes
g.edge_count()           # Number of edges
g.density()              # Edge density (0 to 1)
g.is_directed()          # Whether graph is directed
g.is_empty()             # Whether graph has no nodes
```

### Connectivity

```python
g.is_connected()         # Is graph fully connected
g.count_components()     # Number of connected components
```

### Path-Based

```python
g.diameter()             # Maximum shortest path
g.radius()               # Minimum eccentricity
g.average_path_length()  # Mean shortest path
```

### Clustering

```python
g.average_clustering()   # Mean clustering coefficient
g.transitivity()         # Fraction of triangles
```

### Structural

```python
g.assortativity()        # Degree correlation
g.has_negative_weights() # Contains negative edges
g.has_self_loops()       # Contains self-loops
g.is_bipartite()         # Can be 2-colored
```

## Examples

```python
import pygraphina as pg

# Create a test graph
g = pg.core.barabasi_albert_graph(100, 3, seed=42)

# Basic metrics
print(f"Nodes: {g.node_count()}")
print(f"Edges: {g.edge_count()}")
print(f"Density: {g.density():.4f}")

# Connectivity
print(f"Connected: {g.is_connected()}")
print(f"Components: {g.count_components()}")

# Path metrics
print(f"Diameter: {g.diameter()}")
print(f"Radius: {g.radius()}")

# Clustering
print(f"Avg Clustering: {g.average_clustering():.4f}")
print(f"Transitivity: {g.transitivity():.4f}")

# Structural
print(f"Assortativity: {g.assortativity():.4f}")
```

## Interpretation

- **Density**: How many edges vs maximum possible
- **Diameter**: Maximum communication distance
- **Clustering**: Local transitivity (friend of friend is friend)
- **Assortativity**: High-degree nodes connect to high-degree (positive) or low-degree (negative) nodes

## Time Complexity

| Metric | Complexity |
|--------|-----------|
| Basic | O(1) to O(V) |
| Diameter | O(V·E) |
| Clustering | O(V·d²) |
| Path Length | O(V·E) |
| Assortativity | O(E) |
