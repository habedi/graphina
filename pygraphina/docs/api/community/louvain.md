# Louvain Algorithm

The Louvain algorithm is a fast greedy optimization method for finding communities by maximizing modularity.

## Function Signature

```python
pg.community.louvain(
    graph: PyGraph,
    seed: Optional[int] = None
) -> List[List[int]]
```

## Parameters

- graph: Undirected graph to analyze
- seed: Random seed for reproducibility (optional)

## Returns

List of communities, where each community is a list of node IDs. Each node appears in exactly one community.

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
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(34)]

# Add edges to create a graph with community structure
# (simplified karate club-like structure)
edges = [
    (0, 1), (0, 2), (0, 3), (0, 4), (1, 2), (1, 3), (2, 3),
    (5, 6), (5, 7), (5, 8), (6, 7), (6, 8), (7, 8),
    (4, 5), (3, 9), (9, 10), (10, 11)
]
for u, v in edges:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Detect communities
communities = pg.community.louvain(g, seed=42)

# Analyze results
print(f"Found {len(communities)} communities")
for comm_id, members in enumerate(communities):
    print(f"Community {comm_id}: {len(members)} nodes - {members[:5]}...")
```

## Advantages

- Very fast - suitable for large graphs
- Good quality communities
- Widely used and well-tested
- Multi-level hierarchical clustering
- Reproducible with seed parameter

## Disadvantages

- Non-deterministic without seed
- Greedy approach not guaranteed optimal
- May produce different results on different runs without fixed seed

## Parameters

### Seed

The seed parameter controls randomness:

- None: Non-deterministic behavior (different results each run)
- Integer: Reproducible results with same seed value

Use a seed when you need:

- Reproducible research results
- Consistent testing
- Comparable experiments

Omit seed when you want:

- Multiple different community structures
- Robustness testing across runs

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
