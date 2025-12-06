# Centrality-Based Link Prediction

Centrality-based methods predict links using node importance measures.

## Overview

The hypothesis: nodes with high centrality are more likely to form new connections.

## Approach

1. Compute centrality for all nodes (PageRank, betweenness, etc.)
2. Combine centralities to predict edge probabilities
3. Higher combined centrality = higher prediction score

## Function Signature

```python
pg.links.centrality_based(graph: Union[PyGraph, PyDiGraph]) -> Dict[Tuple[int, int], float]
```

## Combinations Used

Common combinations:

- **PageRank Product**: PR(u) * PR(v)
- **Betweenness Sum**: BC(u) + BC(v)
- **Closeness Product**: C(u) * C(v)
- **Weighted Combinations**: Different weights per centrality

## Example

```python
import pygraphina as pg

# Create a network
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(20)]

# Add random edges
import random
random.seed(42)
for _ in range(40):
    u, v = random.sample(nodes, 2)
    if not g.contains_edge(u, v):
        g.add_edge(u, v, 1.0)

# Calculate centrality-based predictions
predictions = pg.links.centrality_based(g)

# Get top predictions
top_preds = sorted(predictions.items(), key=lambda x: x[1], reverse=True)

print("Top 10 centrality-based predictions:")
for (u, v), score in top_preds[:10]:
    if not g.contains_edge(u, v):
        print(f"  {u}-{v}: {score:.4f}")

# Compare with other methods
jaccard = pg.links.jaccard_coefficient(g)
top_jaccard = sorted(jaccard.items(), key=lambda x: x[1], reverse=True)[:10]

print(f"\nCentrality-based: {len([s for _, s in top_preds[:10] if s > 0])} positive scores")
print(f"Jaccard: {len([s for _, s in top_jaccard if s > 0])} positive scores")
```

## Advantages

- Captures global network structure
- Can use any centrality measure
- Flexible - can combine multiple centralities
- Good for influence-based networks

## Disadvantages

- More computationally expensive than local methods
- Requires centrality computation
- May emphasize hubs excessively
- Parameters for combining centralities

## When to Use

- Analyzing influence networks
- Predicting connections in social networks
- When centrality is meaningful for the problem
- Combining information sources

## Variations

- **Single Centrality**: Use one measure (PageRank, betweenness, etc.)
- **Multiple Centralities**: Combine different measures
- **Weighted Combinations**: Weight centralities by importance
- **Temporal**: Use time-varying centralities
