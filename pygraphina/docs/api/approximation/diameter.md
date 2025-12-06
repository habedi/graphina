# Diameter Approximation

The diameter is the longest shortest path in a graph. Computing exact diameter requires O(V·E) time, so approximation is often used.

## Overview

Graph diameter measures the maximum distance between any two nodes. It characterizes network communication latency and overall network scale.

## Time Complexity

Exact: O(V·E)
Approximation: O(V) to O(V log V) depending on method

## Example

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(20)]

# Create a path
for i in range(19):
    g.add_edge(nodes[i], nodes[i+1], 1.0)

# Add shortcuts
g.add_edge(nodes[0], nodes[10], 1.0)
g.add_edge(nodes[10], nodes[19], 1.0)

# Exact diameter
diameter = g.diameter()
print(f"Exact diameter: {diameter}")

# Approximate diameter
approx_diameter = pg.approximation.approximate_diameter(g)
print(f"Approximate diameter: {approx_diameter}")
```

## Use Cases

- Large network diameter estimation
- Network scale assessment
- Communication latency estimation
- Network design evaluation

## Diameter vs Radius

- **Diameter**: Maximum shortest path length
- **Radius**: Minimum eccentricity (distance to farthest node from a center)
- Radius ≤ Diameter ≤ 2·Radius
