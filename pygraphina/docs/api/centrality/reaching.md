# Reaching Centrality

Reaching centrality measures the proportion of the other nodes a node can reach within a given distance.

## Function Signatures

```python
pg.centrality.local_reaching_centrality(graph: Union[PyGraph, PyDiGraph], distance: int) -> Dict[int, float]
pg.centrality.global_reaching_centrality(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]
```

## Parameters

- graph: The graph to analyze
- distance: Maximum number of hops to consider (local variant only)

## Returns

Dictionary mapping node IDs to reaching centrality scores.

## Description

Reaching centrality is the fraction of the other nodes reachable within a distance threshold, following the
NetworkX definition for unweighted graphs:

- Local Reaching: Limited to the given number of hops
- Global Reaching: All reachable nodes, with no hop limit

## Time Complexity

O(V·E) for global reaching

## Example

```python
import pygraphina as pg

# Create a network with varying connectivity
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Create clusters with limited bridges
for i in range(5):
    g.add_edge(nodes[i], nodes[(i+1)%5], 1.0)

for i in range(5, 10):
    g.add_edge(nodes[i], nodes[(i+1-5)%5 + 5], 1.0)

# Add bridge
g.add_edge(nodes[0], nodes[5], 1.0)

# Local reaching focuses on a bounded neighborhood
local = pg.centrality.local_reaching_centrality(g, 1)

# Global reaching counts all reachable nodes
global_reaching = pg.centrality.global_reaching_centrality(g)

for node in nodes:
    print(f"Node {node}: local={local[node]:.2f}, global={global_reaching[node]:.2f}")
```

## Use Cases

- Measuring accessibility in networks
- Evaluating network connectivity
- Analyzing information spread potential
- Network design optimization

## Advantages

- Simple interpretation
- No parameters to tune
- Fast computation
- Handles disconnected components naturally

## Disadvantages

- Less nuanced than other centralities
- Doesn't account for path weights
- Binary (reachable or not)
