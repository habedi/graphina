# Independent Set Approximation

An independent set is a subset of nodes with no edges between them. Maximum independent set is NP-hard.

## Overview

Finding maximum independent sets is computationally hard, but approximation algorithms provide good solutions quickly.

## Time Complexity

Polynomial approximation algorithms

## Example

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Add edges
edges = [(0,1), (1,2), (3,4), (4,5), (6,7)]
for u, v in edges:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Find independent set (greedy approximation)
# Verify no edges within set
print("Independent sets in this graph would exclude connected nodes")
```

## Use Cases

- Scheduling problems (non-conflicting tasks)
- Matching/pairing problems
- Set packing
- Graph coloring lower bounds

## Relationship to Other Problems

- Complement of Vertex Cover
- Maximum Independent Set ≤ n - Minimum Vertex Cover
- Approximation ratio depends on graph structure
