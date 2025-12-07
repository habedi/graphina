# Louvain Algorithm

The Louvain algorithm is a fast greedy optimization method for finding communities by maximizing modularity.

## Function Signature

```python
pg.community.louvain(
    graph: PyGraph,
resolution: float = 1.0
) -> Dict[int, int]
```

## Parameters

- **graph**: Undirected graph to analyze
- **resolution**: Resolution parameter for community size
    - < 1.0: Larger communities
    - = 1.0: Default (default)
    - > 1.0: Smaller communities

## Returns

Dictionary mapping node IDs to community labels.

## Description

The Louvain algorithm optimizes modularity using a greedy approach:

1. Start with each node as its own community
2. For each node, try moving it to neighboring communities
3. Keep move if modularity increases
4. Repeat until convergence
5. Optionally, recursively apply to supernode graph

## Time Complexity

O(V + E) to O(V log V) depending on implementation

## Space Complexity

O(V + E)

## Example

```python
import pygraphina as pg

# Create a network with natural communities
g = pg.core.karate_club_graph()

# Detect communities with different resolutions
fine = pg.community.louvain(g, resolution=2.0)  # Smaller communities
normal = pg.community.louvain(g, resolution=1.0)  # Default
coarse = pg.community.louvain(g, resolution=0.5)  # Larger communities

# Compare results
from collections import Counter

print(f"Fine (res=2.0): {len(set(fine.values()))} communities")
print(f"Normal (res=1.0): {len(set(normal.values()))} communities")
print(f"Coarse (res=0.5): {len(set(coarse.values()))} communities")

# Analyze community quality
for comm_id, members in Counter(normal.values()).items():
    print(f"Community {comm_id}: {members} nodes")
```

## Advantages

- Very fast - suitable for large graphs
- Good quality communities
- Resolution parameter allows flexibility
- Widely used and well-tested
- Multi-level hierarchical clustering

## Disadvantages

- Non-deterministic (may vary on runs)
- Greedy approach not guaranteed optimal
- Resolution parameter affects results
- May miss small communities with low resolution

## Parameters

### Resolution

The resolution parameter controls community size:

- **0.5**: Larger, coarser communities
- **1.0**: Default, balanced
- **2.0**: Smaller, finer communities

Choose resolution based on application:

- Network analysis: 1.0 (default)
- Finding major groups: 0.5
- Finding micro-communities: 2.0

## When to Use

- Large graphs (suitable for > 100,000 nodes)
- Need fast results
- Exploring different community resolutions
- Production systems

## Reproducibility

For reproducible results with the same graph:

- Results are deterministic with same seed (if available)
- Try multiple resolutions
- Average results if needed

## Comparison

| Algorithm         | Speed     | Quality        | Scalability |
|-------------------|-----------|----------------|-------------|
| Louvain           | Very Fast | Good-Excellent | Excellent   |
| Girvan-Newman     | Slow      | Excellent      | Poor        |
| Label Propagation | Fast      | Good           | Excellent   |
| Spectral          | Medium    | Good           | Medium      |
