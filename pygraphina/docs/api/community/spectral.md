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
- Optimal for certain graph cuts

## Disadvantages

- Requires knowing k in advance
- Computationally expensive for large graphs
- O(V³) complexity limits scalability
- Sensitive to graph structure

## When to Use

- Medium-sized graphs (< 10,000 nodes)
- Known number of communities
- Need mathematical rigor
- Graph has balanced community sizes

## Comparison with Other Methods

| Method | Speed | Scalability | Quality | Requires k |
|--------|-------|-------------|---------|-----------|
| Spectral | Slow | Poor | Very Good | Yes |
| Louvain | Fast | Excellent | Excellent | No |
| Label Propagation | Very Fast | Excellent | Good | No |
| Girvan-Newman | Very Slow | Very Poor | Good | Yes |

## Mathematical Background

The algorithm is based on the graph Laplacian:

```
L = D - A
```

Where:
- D is the degree matrix (diagonal)
- A is the adjacency matrix

The eigenvectors of L provide information about graph structure. Nodes with similar eigenvector values tend to be in the same community.

## Practical Notes

- Initialize k-means carefully for stable results
- Normalize eigenvectors for consistent clustering
- Works on both connected and disconnected graphs
- May need preprocessing for weighted graphs

## Implementation Notes

- Uses spectral decomposition of Laplacian matrix
- Applies k-means to eigenvector coordinates
- Returns community labels (0 to k-1)
- Deterministic for a given input

## See Also

- [Louvain Algorithm](louvain.md) - Faster practical alternative
- [Label Propagation](label_propagation.md) - Fast scalable method
- [Girvan-Newman](girvan_newman.md) - Hierarchical divisive approach
- [Connected Components](connected_components.md) - Finding trivial communities

## References

- Ng, A. Y., Jordan, M. I., & Weiss, Y. (2001). On spectral clustering: Analysis and an algorithm.
- von Luxburg, U. (2007). A tutorial on spectral clustering.

## When to Use

- Known number of communities
- Well-separated clusters expected
- Smaller graphs (< 10,000 nodes)
- Need theoretical guarantees
