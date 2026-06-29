# Clustering Coefficient Approximation

Clustering coefficient measures the density of triangles around each node. For large graphs, sampling-based
approximation is used.

## Function Signature

```python
pg.approximation.average_clustering_approx(
    graph: PyGraph,
num_samples: int = 1000
) -> float
```

## Parameters

- graph: The graph to analyze
- num_samples: Number of nodes to sample (default: 1000)

## Returns

Approximate average clustering coefficient (0 to 1).

## Description

The clustering coefficient for a node is the fraction of possible triangles through that node that actually exist.

Exact computation requires checking all triangles (O(V³) worst case). The approximation samples nodes and computes exact
clustering for those.

## Time Complexity

O(num_samples · d²) where d is average degree

## Space Complexity

O(V)

## Example

```python
import pygraphina as pg

# Create a graph with triangles
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(100)]

# Add edges creating clustering
for i in range(50):
    for j in range(i + 1, min(i + 5, 50)):
        g.add_edge(nodes[i], nodes[j], 1.0)

# Compute approximate clustering
clustering = pg.approximation.average_clustering_approx(g, num_samples=100)
print(f"Average clustering coefficient: {clustering:.4f}")

# Exact computation (for comparison on small graphs)
exact = g.average_clustering()
print(f"Exact clustering coefficient: {exact:.4f}")
print(f"Error: {abs(clustering - exact):.4f}")
```

## Accuracy

- More samples = higher accuracy but slower
- Default 1000 samples good for most graphs
- Adjust based on graph size and accuracy needs

## Use Cases

- Large network analysis
- Small-world network detection
- Community structure assessment
- Network evolution tracking

## Related Metrics

- Local Clustering: Per-node clustering coefficient
- Transitivity: Global clustering (ratio of triangles to triples)
