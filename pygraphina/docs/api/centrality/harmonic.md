# Harmonic Centrality

Harmonic centrality measures the importance of a node based on the sum of reciprocal distances to all other nodes.

## Function Signature

```python
pg.centrality.harmonic(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]
```

## Parameters

- graph: The graph to analyze (directed or undirected)

## Returns

Dictionary mapping node IDs to their harmonic centrality scores.

## Description

Harmonic centrality is computed as:

```
H(u) = Σ(1 / d(u, v)) for all v ≠ u
```

Where d(u, v) is the shortest distance between nodes u and v. If a node is unreachable, the distance is treated as
infinity, contributing 0 to the sum.

## Time Complexity

O(V·E) - performs shortest path computation from each node

## Use Cases

- Finding central nodes in disconnected graphs (unlike closeness centrality)
- Network analysis where unreachable nodes need special handling
- Infrastructure networks with potential disconnections

## Example

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(5)]

# Add edges
edges = [(0, 1), (1, 2), (2, 3), (3, 4)]
for u, v in edges:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Calculate harmonic centrality
harmonic = pg.centrality.harmonic(g)

# Display scores
for node in sorted(harmonic.keys()):
    print(f"Node {node}: {harmonic[node]:.4f}")
```

## Comparison with Other Centrality Measures

| Measure     | Handles Disconnected            | Computational Cost |
|-------------|---------------------------------|--------------------|
| Harmonic    | Yes                             | O(V·E)             |
| Closeness   | No (undefined for disconnected) | O(V·E)             |
| Betweenness | Yes                             | O(V·E)             |
| Degree      | N/A (local)                     | O(1) per node      |

## References

- Boldi, P., & Vigna, S. (2014). Axioms for centrality. Internet Mathematics, 10(3-4), 222-262.
