# Katz Centrality

Katz centrality measures importance using a weighted sum of node paths, with exponential decay for longer paths.

## Function Signature

```python
pg.centrality.katz(
    graph: Union[PyGraph, PyDiGraph],
    alpha: float = 0.1,
    max_iters: int = 100,
    tol: float = 1e-6
) -> Dict[int, float]
```

## Parameters

- **graph**: The graph to analyze
- **alpha**: Attenuation factor (controls path weight decay), default 0.1
- **max_iters**: Maximum iterations, default 100
- **tol**: Convergence tolerance, default 1e-6

## Returns

Dictionary mapping node IDs to Katz centrality scores.

## Description

Katz centrality sums contributions from all paths, with exponential decay:

```
K(u) = Σ α^k * (number of k-length paths from u)
```

where α is the attenuation factor (0 < α < 1).

## Time Complexity

O(k·(V + E)) where k is number of iterations

## Parameters

- **alpha**: Smaller values = less weight on longer paths
    - Typical range: 0.01 to 0.3
    - Default 0.1 works well
- **max_iters**: Usually converges in 10-50 iterations

## Example

```python
import pygraphina as pg

# Create a network
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(5)]

# Create edges
edges = [(0,1), (1,2), (2,3), (3,4), (0,4)]
for u, v in edges:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Calculate Katz centrality
katz = pg.centrality.katz(g, alpha=0.1, max_iters=100)

for node, score in sorted(katz.items()):
    print(f"Node {node}: {score:.4f}")
```

## Use Cases

- Network influence analysis
- Recommendation systems
- Citation networks (path importance)
- Information flow analysis

## Advantages

- Considers all paths (not just shortest)
- Parameter alpha allows tuning
- Works for directed and undirected
- More stable than eigenvector

## Disadvantages

- Parameter selection affects results
- More computationally expensive than simple measures
- Requires iteration for convergence

## Comparison

| Centrality  | Basis                | Best For                  |
|-------------|----------------------|---------------------------|
| Katz        | All paths (weighted) | Influence via all routes  |
| Betweenness | Shortest paths       | Bridges/bottlenecks       |
| Eigenvector | Important neighbors  | Influence via connections |
| PageRank    | Random walk          | Web ranking               |
