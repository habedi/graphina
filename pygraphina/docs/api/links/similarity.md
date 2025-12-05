# Similarity Metrics

Similarity metrics measure how alike two nodes are based on their network connections.

## Overview

Similarity-based link prediction assumes that similar nodes are more likely to connect in the future.

## Jaccard Coefficient

The Jaccard coefficient measures the overlap of neighbor sets:

```python
pg.links.jaccard_coefficient(graph) -> Dict[Tuple, float]
```

Formula: J(u, v) = |N(u) ∩ N(v)| / |N(u) ∪ N(v)|

Ranges from 0 (no common neighbors) to 1 (identical neighborhoods).

## Common Neighbors

The simplest metric - just count shared neighbors:

```python
pg.links.common_neighbors(graph) -> Dict[Tuple, int]
```

Fast but less sophisticated than other metrics.

## Adamic-Adar Index

Weights common neighbors by their degree (low-degree neighbors count more):

```python
pg.links.adamic_adar(graph) -> Dict[Tuple, float]
```

Formula: A(u, v) = Σ(1 / log(degree(w))) for common neighbors w

Good for social networks where rare connections are more meaningful.

## Example

```python
import pygraphina as pg

# Create a network
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Add some edges
edges = [
    (0, 1), (0, 2), (1, 2),  # Triangle
    (2, 3), (3, 4), (4, 5),  # Path
]

for u, v in edges:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Compare metrics
jaccard = pg.links.jaccard_coefficient(g)
adamic = pg.links.adamic_adar(g)
common = pg.links.common_neighbors(g)

# Example: nodes 0 and 3
key = (nodes[0], nodes[3])
print(f"Jaccard (0,3): {jaccard.get(key, 'N/A')}")
print(f"Adamic-Adar (0,3): {adamic.get(key, 'N/A')}")
print(f"Common neighbors (0,3): {common.get(key, 'N/A')}")
```

## Comparison

| Metric | Complexity | Range | Sensitivity |
|--------|-----------|-------|-------------|
| Common Neighbors | O(d²) | [0, ∞) | Depends on degree |
| Jaccard | O(d²) | [0, 1] | Normalized |
| Adamic-Adar | O(d² log V) | [0, ∞) | Low-degree aware |

## Use Cases

- Friend recommendation in social networks
- Citation prediction in academic networks
- Biological network analysis
- Knowledge graph completion

## Strengths

- Simple and interpretable
- Fast computation
- No parameters to tune
- Good empirical performance

## Limitations

- Only considers local structure
- Ignores node attributes
- Equal weight to all common neighbors (except Adamic-Adar)
