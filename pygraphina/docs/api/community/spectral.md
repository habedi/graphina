# Spectral Clustering

Spectral clustering uses the graph Laplacian matrix to partition the graph into communities.

## Function Signature

```python
pg.community.spectral_clustering(
    graph: PyGraph,
    k: int
) -> Dict[int, int]
```

## Parameters

- **graph**: Undirected graph to analyze
- **k**: Number of communities to find

## Returns

Dictionary mapping node IDs to community labels (0 to k-1).

## Description

Spectral clustering works by:

1. Computing the graph Laplacian matrix
2. Finding the k smallest eigenvectors
3. Using k-means clustering on the eigenvector coordinates
4. Assigning nodes to clusters

## Time Complexity

O(V³) for eigenvalue decomposition

## Space Complexity

O(V²) for the Laplacian matrix

## Example

```python
import pygraphina as pg

# Create a graph with clear cluster structure
g = pg.PyGraph()

# Define clusters
cluster1 = [g.add_node(i) for i in range(5)]
cluster2 = [g.add_node(i+5) for i in range(5)]
cluster3 = [g.add_node(i+10) for i in range(5)]

# Connect within clusters densely
for cluster in [cluster1, cluster2, cluster3]:
    for i in range(len(cluster)):
        for j in range(i+1, len(cluster)):
            g.add_edge(cluster[i], cluster[j], 1.0)

# Add weak inter-cluster edges
g.add_edge(cluster1[-1], cluster2[0], 0.5)
g.add_edge(cluster2[-1], cluster3[0], 0.5)

# Detect communities
communities = pg.community.spectral_clustering(g, k=3)

# Verify
from collections import Counter
print(f"Found {len(set(communities.values()))} communities")
print(f"Size distribution: {Counter(communities.values())}")
```

## Advantages

- Theoretically well-founded
- Works well for balanced clusters
- Handles complex network structures

## Disadvantages

- Requires knowing k in advance
- Computationally expensive for large graphs
- Assumes well-separated clusters

## When to Use

- Known number of communities
- Well-separated clusters expected
- Smaller graphs (< 10,000 nodes)
- Need theoretical guarantees
