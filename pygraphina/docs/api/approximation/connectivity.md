# Connectivity Approximation

Connectivity measures how robust a graph is to node or edge removal.

## Overview

Connectivity concepts include:

- Node connectivity: minimum nodes to remove to disconnect graph
- Edge connectivity: minimum edges to remove to disconnect graph
- Component connectivity: connections between components

## Time Complexity

Depends on specific algorithm, generally O(V·E)

## Use Cases

- Network resilience analysis
- Critical node identification
- Network robustness assessment
- Vulnerability analysis

## Example

```python
import pygraphina as pg

# Create a connected graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Create backbone connections
for i in range(9):
    g.add_edge(nodes[i], nodes[i+1], 1.0)

# Add cross-connections
g.add_edge(nodes[0], nodes[5], 1.0)
g.add_edge(nodes[4], nodes[9], 1.0)

# Analyze connectivity
num_components = g.count_components()
is_connected = g.is_connected()

print(f"Connected: {is_connected}")
print(f"Components: {num_components}")
```

## Related Concepts

- **Min-cut**: Minimum edges to remove for disconnection
- **Max-flow**: Maximum flow through graph
- **Bridge**: Edge whose removal increases components
- **Articulation point**: Node whose removal increases components
