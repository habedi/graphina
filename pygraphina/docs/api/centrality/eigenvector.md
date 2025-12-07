# Eigenvector Centrality

Eigenvector centrality measures importance based on connections to other important nodes.

## Function Signature

```python
pg.centrality.eigenvector(
    graph: Union[PyGraph, PyDiGraph],
    max_iter: int = 100,
    tolerance: float = 1e-6
) -> Dict[int, float]
```

## Parameters

- **graph**: The graph to analyze (directed or undirected)
- **max_iter**: Maximum number of iterations. Default: 100
- **tolerance**: Convergence tolerance (default: 1e-6)

## Returns

Dictionary mapping node IDs to eigenvector centrality scores.

## Description

Eigenvector centrality is computed using power iteration on the adjacency matrix. A node's importance is proportional to
the sum of its neighbors' importances.

## Time Complexity

O(k·E) where k is number of iterations

## Space Complexity

O(V + E)

## Example

```python
import pygraphina as pg

# Create a network where some nodes are more important
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(6)]

# Create a network with hub structure
edges = [
    (0, 1), (0, 2), (0, 3),  # Node 0 is important
    (1, 4), (2, 4), (3, 5),  # Node 4 connects to important nodes
]

for u, v in edges:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Calculate eigenvector centrality
eigen = pg.centrality.eigenvector(g, max_iter=100, tolerance=1e-6)

for node, score in sorted(eigen.items()):
    print(f"Node {node}: {score:.4f}")
```

## Use Cases

- Influence analysis (connections to influential people)
- Network importance assessment
- Recommendation systems
- Link analysis (similar to PageRank)

## Advantages

- Captures transitive importance
- Well-defined mathematically
- Works for directed and undirected

## Disadvantages

- Computationally expensive
- May not converge for some graphs
- Sensitive to network structure

## Convergence

If iteration doesn't converge within max_iter:

- May return approximate result
- Try increasing max_iter or decreasing tolerance

## Relationship to PageRank

- Similar concept: importance via connections
- PageRank: biased random walk
- Eigenvector: equilibrium importance
