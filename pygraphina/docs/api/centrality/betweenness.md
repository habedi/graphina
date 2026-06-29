# Betweenness Centrality

Betweenness centrality measures how often a node appears on shortest paths between other nodes.

## Function Signature

```python
pg.centrality.betweenness(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]
```

## Parameters

- graph: The graph to analyze

## Returns

Dictionary mapping node IDs to betweenness centrality scores.

## Description

For each node u, betweenness is computed as:

```
BC(u) = Σ σ(s,t|u) / σ(s,t)
```

Where σ(s,t|u) is the number of shortest paths from s to t through u, and σ(s,t) is the total number of shortest paths
from s to t.

## Time Complexity

O(V·E) - shortest path computation from each node

## Space Complexity

O(V²) - storing all-pairs distances

## Example

```python
import pygraphina as pg

# Create a graph with one central node
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(7)]

# Star topology: node 0 is center
for i in range(1, 7):
    g.add_edge(nodes[0], nodes[i], 1.0)

# Node 0 has high betweenness (appears on all paths between other nodes)
betweenness = pg.centrality.betweenness(g, True)
for node, score in sorted(betweenness.items()):
    print(f"Node {node}: {score:.4f}")
```

## Use Cases

- Finding bridge nodes in social networks
- Identifying critical nodes in transportation networks
- Network bottleneck detection
- Community detection (using edge betweenness)

## Advantages

- Identifies important bridge nodes
- Well-defined mathematically
- Captures global structure

## Disadvantages

- Computationally expensive (O(V·E))
- Not suitable for very large graphs
- Sensitive to network structure

## Comparison with Other Centralities

| Centrality  | Computes                      | Best For             |
|-------------|-------------------------------|----------------------|
| Betweenness | Shortest path frequency       | Bridges/connectors   |
| Closeness   | Distance to all others        | Central hubs         |
| Degree      | Direct connections            | Immediate importance |
| Eigenvector | Connection to important nodes | Influence            |
| PageRank    | Random walk probability       | Web ranking          |
