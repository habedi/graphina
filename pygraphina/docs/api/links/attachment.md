# Preferential Attachment

Preferential attachment is a link prediction model based on node degree - high-degree nodes are more likely to connect.

## Function Signature

```python
pg.links.preferential_attachment(graph: Union[PyGraph, PyDiGraph]) -> Dict[Tuple[int, int], float]
```

## Parameters

- **graph**: The graph to analyze

## Returns

Dictionary mapping edge tuples (u, v) to attachment scores.

## Description

Preferential attachment score for edge (u, v):

```
PA(u, v) = degree(u) * degree(v)
```

High-degree nodes ("hubs") are predicted to be more likely to connect to each other and to new nodes.

## Time Complexity

O(V²) - simple degree lookup

## Space Complexity

O(V²)

## Example

```python
import pygraphina as pg

# Create a graph following preferential attachment
g = pg.core.barabasi_albert(50, 3, seed=42)

# Calculate preferential attachment
pa = pg.links.preferential_attachment(g)

# Sort by score
top_pa = sorted(pa.items(), key=lambda x: x[1], reverse=True)

print("Top predicted links by preferential attachment:")
for (u, v), score in top_pa[:10]:
    deg_u = g.degree[u]
    deg_v = g.degree[v]
    print(f"  {u}-{v}: score={score:.0f} (degrees: {deg_u}, {deg_v})")
```

## Use Cases

- Growing network analysis
- Predicting hubs in emerging networks
- Scale-free network modeling
- Power-law degree distribution networks

## Advantages

- Very fast computation (O(V²))
- No parameters
- Simple interpretation
- Good for scale-free networks
- Explanatory power (why connections form)

## Disadvantages

- Ignores neighborhood similarity
- Assumes rich-get-richer dynamic
- May overestimate hub connections
- Doesn't work well for non-scale-free networks

## Relationship to Graph Models

Preferential attachment is the basis for Barabasi-Albert model:

- New nodes connect to existing nodes with probability proportional to degree
- Creates power-law degree distributions
- Models many real-world networks (web, social, biological)

## Variant: Weighted Preferential Attachment

For weighted networks, use node strength (sum of edge weights) instead of degree.
