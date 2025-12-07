# Resource Allocation Index

Resource allocation index is a link prediction metric based on resource distribution through common neighbors.

## Function Signature

```python
pg.links.resource_allocation_index(graph: Union[PyGraph, PyDiGraph]) -> Dict[Tuple[int, int], float]
```

## Parameters

- **graph**: The graph to analyze

## Returns

Dictionary mapping edge tuples (u, v) to resource allocation scores.

## Description

Resource allocation models information flow where each common neighbor distributes resources equally among their
neighbors.

For nodes u and v:

```
RA(u, v) = Σ(1 / degree(w)) for all common neighbors w
```

## Time Complexity

O(V + E) to compute all scores

## Example

```python
import pygraphina as pg

# Create a collaboration network
g = pg.PyGraph()
researchers = [g.add_node(i) for i in range(8)]

# Add collaborations
collaborations = [
    (0, 1), (0, 2), (1, 2),  # Group 1
    (3, 4), (3, 5), (4, 5),  # Group 2
    (2, 3),  # Bridge
    (5, 6), (6, 7)  # Group 3
]

for u, v in collaborations:
    g.add_edge(researchers[u], researchers[v], 1.0)

# Predict links using resource allocation
predictions = pg.links.resource_allocation_index(g)

# Get top predictions
top_preds = sorted(predictions.items(), key=lambda x: x[1], reverse=True)
print("Top predictions (resource allocation):")
for (u, v), score in top_preds[:5]:
    if not g.contains_edge(researchers[u], researchers[v]):
        print(f"  {u}-{v}: {score:.4f}")
```

## Interpretation

Higher scores indicate:

- More common neighbors
- Low-degree common neighbors (more resources distributed)
- Higher likelihood of future connection

## Advantages

- Simple and intuitive
- Fast computation
- No parameters to tune
- Good empirical performance

## When to Use

- Researcher/collaboration networks
- Knowledge graph completion
- Information flow analysis
