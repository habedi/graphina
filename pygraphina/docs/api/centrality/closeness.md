# Closeness Centrality

Closeness centrality measures how close a node is to all other nodes on average.

## Function Signature

```python
pg.centrality.closeness(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]
```

## Parameters

- **graph**: The graph to analyze

## Returns

Dictionary mapping node IDs to closeness centrality scores (0 to 1).

## Description

Closeness is computed as:

```
C(u) = (n-1) / Σ d(u, v)
```

Where d(u, v) is the shortest distance between u and v, and n is the number of nodes.

Higher scores indicate nodes closer to all others.

## Time Complexity

O(V·E) - shortest path computation from each node

## Space Complexity

O(V)

## Example

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(5)]

# Create a path: 0-1-2-3-4
for i in range(4):
    g.add_edge(nodes[i], nodes[i+1], 1.0)

# Middle node has highest closeness
closeness = pg.centrality.closeness(g)
for node, score in sorted(closeness.items()):
    print(f"Node {node}: {score:.4f}")
```

## Use Cases

- Finding central hubs in networks
- Optimal placement of facilities
- Communication efficiency analysis
- Identifying nodes with shortest average distance to others

## Advantages

- Intuitive interpretation (inverse of average distance)
- Captures global structure
- Works for directed and undirected graphs

## Disadvantages

- Undefined for disconnected components (if unreachable)
- Computationally expensive for large graphs
- Sensitive to network connectivity

## Variants

- **Harmonic Centrality**: Handles disconnected graphs better
- **Normalized Closeness**: Scales by network size
