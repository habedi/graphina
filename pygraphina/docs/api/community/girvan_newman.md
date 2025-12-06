# Girvan-Newman Algorithm

Girvan-Newman is a divisive algorithm that detects communities by iteratively removing edges with high betweenness
centrality.

## Function Signature

```python
pg.community.girvan_newman(
    graph: PyGraph,
    num_communities: int
) -> Dict[int, int]
```

## Parameters

- **graph**: Undirected graph to analyze
- **num_communities**: Number of communities to find

## Returns

Dictionary mapping node IDs to community labels.

## Description

The algorithm works as follows:

1. Calculate betweenness centrality for all edges
2. Remove the edge with highest betweenness
3. Recalculate betweenness for remaining edges
4. Repeat until the desired number of communities is reached

## Time Complexity

O(V·E²) - very expensive for large graphs

## Space Complexity

O(V·E)

## Example

```python
import pygraphina as pg

# Create a graph with clear community structure
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(8)]

# Community 1: nodes 0-2
for i in range(3):
    for j in range(i+1, 3):
        g.add_edge(nodes[i], nodes[j], 1.0)

# Community 2: nodes 3-5
for i in range(3, 6):
    for j in range(i+1, 6):
        g.add_edge(nodes[i], nodes[j], 1.0)

# Community 3: nodes 6-7
g.add_edge(nodes[6], nodes[7], 1.0)

# Bridges between communities
g.add_edge(nodes[2], nodes[3], 1.0)  # Bridge 1-2
g.add_edge(nodes[5], nodes[6], 1.0)  # Bridge 2-3

# Detect communities
communities = pg.community.girvan_newman(g, num_communities=3)

from collections import defaultdict
groups = defaultdict(list)
for node, comm in communities.items():
    groups[comm].append(node)

print(f"Communities: {dict(groups)}")
```

## Advantages

- Intuitive - removes bridge edges
- Provides hierarchical structure
- Can visualize community structure
- Good understanding of why communities form

## Disadvantages

- Very slow - O(V·E²)
- Only suitable for small graphs (< 1000 nodes)
- Requires specifying number of communities
- Must recalculate betweenness at each step

## When to Use

- Small to medium graphs
- When understanding community structure is important
- Hierarchical clustering desired
- Research and analysis (not production)

## Variants

- **Fast Girvan-Newman**: Approximations for speed
- **Edge Betweenness**: Can use other edge centralities
- **Hierarchical Clustering**: Generate dendrograms

## Comparison

| Algorithm         | Speed     | Quality   | Parameters          |
|-------------------|-----------|-----------|---------------------|
| Girvan-Newman     | Slow      | Excellent | k (num communities) |
| Label Propagation | Very Fast | Good      | max_iters           |
| Louvain           | Fast      | Excellent | resolution          |
| Spectral          | Medium    | Good      | k (num communities) |
