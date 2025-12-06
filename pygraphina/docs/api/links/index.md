# Link Prediction

Link prediction estimates the likelihood of future connections between nodes based on network structure.

## Overview

Link prediction is used for:
- Recommender systems (suggest new connections)
- Knowledge graph completion (fill missing relationships)
- Network evolution analysis (predict growth)
- Relationship discovery (identify hidden connections)

## Available Algorithms

| Algorithm | Time | Space | Best For |
|-----------|------|-------|----------|
| Jaccard | O(V²·d) | O(V²) | General similarity |
| Adamic-Adar | O(V²·d) | O(V²) | Social networks |
| Resource Allocation | O(V²·d) | O(V²) | Information networks |
| Preferential Attachment | O(V²) | O(V²) | Scale-free networks |
| Common Neighbors | O(V²·d) | O(V²) | Quick baseline |
| Centrality-Based | O(V·E) | O(V) | Influence networks |

Where d is average degree.

## Common Usage

```python
import pygraphina as pg

# Load or create a graph
g = pg.core.karate_club_graph()

# Predict links using different methods
jaccard = pg.links.jaccard_coefficient(g)
adamic = pg.links.adamic_adar(g)
common = pg.links.common_neighbors(g)

# Get top predictions
top_jaccard = sorted(jaccard.items(), key=lambda x: x[1], reverse=True)[:10]
print("Top 10 Jaccard predictions:")
for (u, v), score in top_jaccard:
    print(f"  {u}-{v}: {score:.4f}")
```

## Evaluation

To evaluate predictions on a dataset:

1. Split edges into train/test
2. Make predictions on training graph
3. Measure how many test edges are predicted
4. Calculate precision, recall, AUC

```python
import random

# Split edges
all_edges = list(g.edges())
test_size = int(0.2 * len(all_edges))
test_edges = set(all_edges[:test_size])
train_edges = all_edges[test_size:]

# Build training graph
g_train = pg.PyGraph()
for node in g.nodes():
    g_train.add_node(node)

for u, v in train_edges:
    g_train.add_edge(u, v, 1.0)

# Make predictions
predictions = pg.links.jaccard_coefficient(g_train)

# Evaluate
hits = 0
for (u, v), score in predictions.items():
    if (u, v) in test_edges or (v, u) in test_edges:
        hits += 1

print(f"Precision: {hits / len(predictions):.3f}")
```

## Choosing an Algorithm

### For Balanced Performance
Use Jaccard or Adamic-Adar

### For Speed
Use Common Neighbors or Preferential Attachment

### For Quality
Use Resource Allocation or Centrality-Based

### For Scale-Free Networks
Use Preferential Attachment

## Metrics

- **Precision**: Fraction of predicted links that are correct
- **Recall**: Fraction of true links that are predicted
- **AUC**: Area Under Receiver Operating Characteristic curve
- **Rank**: Position of true link in sorted predictions

## Related Concepts

- **Similarity Metrics**: Measure node resemblance
- **Graph Clustering**: Find dense subgraphs
- **Community Detection**: Find natural groupings
