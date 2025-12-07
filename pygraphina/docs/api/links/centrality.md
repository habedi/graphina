# Common Neighbor Centrality Link Prediction

Common neighbor centrality is a link prediction metric based on centrality-weighted common neighbors.

## Overview

Common neighbor centrality combines the common neighbor principle with node centrality weights. The hypothesis: nodes with high centrality that have common neighbors are more likely to form connections.

## Function Signature

```python
pg.links.common_neighbor_centrality(
    graph: Union[PyGraph, PyDiGraph],
    alpha: float,
    ebunch: Optional[List[Tuple[int, int]]] = None
) -> Dict[Tuple[int, int], float]
```

## Parameters

- **graph**: The graph to analyze
- **alpha**: Weighting parameter for centrality contribution (typically 0.0 to 1.0)
- **ebunch**: Optional list of edges to evaluate. If None, evaluates all non-existing edges

## Returns

Dictionary mapping edge tuples (u, v) to common neighbor centrality scores.

## Description

The common neighbor centrality score is calculated as:

```
CNC(u, v) = |N(u) ∩ N(v)| * (C(u) + C(v))^alpha
```

Where:
- N(u), N(v) are neighbor sets
- C(u), C(v) are centrality values
- alpha controls how much centrality weights the score

## Time Complexity

O(k·d²) where k is the number of edges to evaluate and d is average degree

## Space Complexity

O(V + E)

## Example

```python
import pygraphina as pg

# Create a collaboration network
g = pg.PyGraph()
researchers = [g.add_node(i) for i in range(15)]

# Add collaboration edges
collaborations = [
    (0,1), (0,2), (1,2), (1,3), (2,3), (3,4), (4,5), (5,6),
    (6,7), (7,8), (8,9), (9,10), (10,11), (11,12), (12,13)
]
for u, v in collaborations:
    g.add_edge(researchers[u], researchers[v], 1.0)

# Predict new collaborations using different alpha values
alpha_low = pg.links.common_neighbor_centrality(g, alpha=0.1)
alpha_high = pg.links.common_neighbor_centrality(g, alpha=0.9)

# Get top predictions with high centrality weighting
top_high = sorted(alpha_high.items(), key=lambda x: x[1], reverse=True)[:5]

print("Top 5 predictions (high centrality weighting):")
for (u, v), score in top_high:
    if not g.contains_edge(u, v):
        print(f"  {u}-{v}: {score:.4f}")

# Evaluate on specific edges
candidates = [(0, 4), (2, 5), (5, 11)]
scores = pg.links.common_neighbor_centrality(g, alpha=0.5, ebunch=candidates)
print(f"\nScores for candidate edges: {scores}")
```

## Advantages

- Combines local (common neighbors) and global (centrality) information
- Tunable parameter (alpha) for different use cases
- Fast computation
- Works well for networks with hub nodes
- Can focus on specific edges via ebunch parameter

## Disadvantages

- Requires parameter tuning (alpha)
- May overweight central nodes
- Assumes centrality is predictive
- Less effective in homogeneous networks

## When to Use

- Influence networks where centrality matters
- Networks with clear hub structure
- When combining local and global information
- Expert identification in collaboration networks
- Recommendation systems using centrality

## Parameter Tuning

- **alpha = 0.0**: Pure common neighbor count (ignores centrality)
- **alpha = 0.5**: Balanced common neighbor and centrality
- **alpha = 1.0**: Strong centrality weighting
- **alpha > 1.0**: Very strong centrality emphasis

## Comparison with Other Methods

| Method | Speed | Accuracy | Global Info |
|--------|-------|----------|-------------|
| Common Neighbors | ⚡⚡⚡ | Good | No |
| Jaccard | ⚡⚡⚡ | Good | No |
| Common Neighbor Centrality | ⚡⚡ | Better | Yes |
| Centrality Product | ⚡ | Good | Yes |
