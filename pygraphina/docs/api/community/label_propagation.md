# Label Propagation

Label propagation is a fast, iterative community detection algorithm that assigns nodes to communities based on their
neighbors.

## Function Signature

```python
pg.community.label_propagation(
    graph: PyGraph,
    max_iter: int,
    seed: int = None
) -> Dict[int, int]
```

## Parameters

- graph: Undirected graph to analyze
- max_iter: Maximum number of iterations
- seed: Optional random seed for reproducibility (default: None)

## Returns

Dictionary mapping node IDs to community labels (integers).

## Description

Label propagation is a simple and efficient algorithm that works as follows:

1. Initially, each node has a unique label
2. In each iteration, nodes update their label to the most frequent label among their neighbors
3. Process repeats until convergence or max iterations

## Time Complexity

O(k·(V + E)) where k is the number of iterations

## Space Complexity

O(V)

## Advantages

- Fast execution
- No parameters to tune (except max_iters)
- Works well for many real-world networks
- Probabilistic approach is parameter-free

## Disadvantages

- Results can be non-deterministic
- May not find globally optimal communities
- Sensitive to label initialization

## Example

```python
import pygraphina as pg

# Create a graph with community structure
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Community 1: nodes 0-3
for i in range(4):
    for j in range(i + 1, 4):
        g.add_edge(nodes[i], nodes[j], 1.0)

# Community 2: nodes 4-7
for i in range(4, 8):
    for j in range(i + 1, 8):
        g.add_edge(nodes[i], nodes[j], 1.0)

# Bridge edge
g.add_edge(nodes[3], nodes[4], 1.0)

# Detect communities
communities = pg.community.label_propagation(g, max_iter=100)

# Display results
from collections import Counter

print(f"Communities detected: {len(set(communities.values()))}")
for comm_id, count in Counter(communities.values()).items():
    print(f"  Community {comm_id}: {count} nodes")
```

## When to Use

- Need fast community detection
- Graph is moderately large (10k-100k+ nodes)
- Seeking multiple community detection options
- Want parameter-free algorithm

## Comparison with Other Methods

| Algorithm         | Speed     | Quality   | Parameters |
|-------------------|-----------|-----------|------------|
| Label Propagation | Very Fast | Good      | Minimal    |
| Louvain           | Fast      | Excellent | Resolution |
| Girvan-Newman     | Slow      | Good      | None       |
| Spectral          | Medium    | Good      | k          |
