# Traveling Salesman Problem (TSP) Approximation

The Traveling Salesman Problem seeks the shortest tour visiting all nodes exactly once. It is NP-hard, so approximation
is used for large instances.

## Overview

TSP has many practical applications. Exact solutions are only feasible for small instances (< 1000 nodes). For larger
problems, approximation algorithms are necessary.

## Approximation Algorithms

### MST-Based Approximation

- Use minimum spanning tree as starting point
- Approximation ratio: 2 for metric TSP
- Time complexity: O(V² log V)

## Example

```python
import pygraphina as pg

# Create a small TSP instance
g = pg.PyGraph()
cities = [g.add_node(i) for i in range(8)]

# Add edges with distances
edges = [
    (0, 1, 10), (0, 2, 15), (1, 2, 12),
    (2, 3, 20), (3, 4, 25), (4, 5, 15),
    (5, 6, 18), (6, 7, 22), (7, 0, 28)
]

for u, v, w in edges:
    g.add_edge(cities[u], cities[v], float(w))

# Approximate TSP solution
# Would use heuristic like 2-opt, genetic algorithm, etc.
print("TSP on small instance can use exact solver")
print("TSP on large instance requires approximation")
```

## Time Complexity

Approximation: O(V² log V)

## Use Cases

- Vehicle routing
- Delivery optimization
- Circuit board drilling
- DNA sequencing
- Tour planning

## Notes

- Metric TSP has 2-approximation
- Non-metric TSP is harder to approximate
- Many heuristics exist (nearest neighbor, 2-opt, etc.)
