# Approximation Algorithms

Approximation algorithms find near-optimal solutions for computationally hard problems in polynomial time.

## Overview

Many graph problems are NP-hard (clique, vertex cover, traveling salesman problem). For practical applications,
approximation algorithms provide good solutions quickly instead of spending exponential time on exact solutions.

## Available Algorithms

| Algorithm         | Problem                 | Approximation Ratio | Time Complexity |
|-------------------|-------------------------|---------------------|-----------------|
| Large Clique      | Maximum Clique          | Heuristic           | O(V²)           |
| Vertex Cover      | Minimum Vertex Cover    | 2-approximation     | O(V + E)        |
| Clustering        | Graph Clustering        | Approximation       | O(V²)           |
| Connectivity      | Graph Connectivity      | Approximation       | O(V + E)        |
| Diameter          | Graph Diameter          | Approximation       | O(V²)           |
| Independent Set   | Maximum Independent Set | Approximation       | O(V²)           |
| Treewidth         | Graph Treewidth         | Approximation       | O(V³)           |
| Densest Subgraph  | Densest Subgraph        | 2-approximation     | O(V²)           |
| Ramsey R(2,2)     | Clique/Independent Set  | log(n) guarantee    | O(V²)           |

## When to Use Approximation Algorithms

### Use Approximation When:

- Problem is NP-hard
- Exact solution takes too long
- Near-optimal solution is acceptable
- Need answer quickly

### Use Exact Algorithms When:

- Problem is polynomial-solvable
- Optimality is critical
- Graph is small enough
- Running time is not critical

## Common Usage Pattern

```python
import pygraphina as pg

# Create a test graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(20)]

# Add random edges
import random

random.seed(42)
for i in range(100):
    u, v = random.randint(0, 19), random.randint(0, 19)
    if u != v:
        g.add_edge(nodes[u], nodes[v], 1.0)

# Use approximation algorithms
clique_size = pg.approximation.large_clique_size(g)
vc_size = len(pg.approximation.min_weighted_vertex_cover(g))
diameter_approx = pg.approximation.diameter(g)

print(f"Approximate clique size: {clique_size}")
print(f"Approximate vertex cover: {vc_size}")
print(f"Approximate diameter: {diameter_approx}")
```

## Approximation Guarantees

Some algorithms have proven approximation ratios:

- Vertex Cover: 2-approximation (always within 2x optimal)
- TSP (MST-based): 2-approximation for metric TSP
- Independent Set: Depends on algorithm

Others use heuristics with no proven guarantee but work well in practice.

## Trade-offs

| Factor      | Better With Approximation | Better With Exact |
|-------------|---------------------------|-------------------|
| Speed       | Yes                       | No                |
| Optimality  | No                        | Yes               |
| Scalability | Yes                       | No                |
| Theory      | Varies                    | Strong            |

## References

See individual algorithm pages for specific approximation ratios and techniques.
