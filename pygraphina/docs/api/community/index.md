# Community Detection

Community detection algorithms identify groups of densely connected nodes within a graph.

## Overview

Community detection is used to partition a graph into communities (also called clusters or modules). Communities are
subsets of nodes where connections within the community are more frequent than connections between communities.

## Available Algorithms

| Algorithm            | Time Complexity     | Parameters          | Best For                    |
|----------------------|---------------------|---------------------|-----------------------------|
| Label Propagation    | O(k·(V+E))          | max_iters           | Fast, simple detection      |
| Louvain              | O(V log V) to O(V²) | resolution          | Quality and speed balance   |
| Girvan-Newman        | O(V·E²)             | None                | Small graphs, understanding |
| Spectral Clustering  | O(V³)               | k (num communities) | Well-separated communities  |
| Connected Components | O(V+E)              | None                | Disconnected components     |

## Common Usage

```python
import pygraphina as pg

# Load or create a graph
g = pg.core.karate_club_graph()

# Detect communities using different algorithms
label_prop = pg.community.label_propagation(g, max_iter=100)
louvain = pg.community.louvain(g)
connected = pg.community.connected_components(g)

# Analyze results
from collections import Counter

print(f"Label Propagation found {len(set(label_prop.values()))} communities")
print(f"Louvain found {len(set(louvain.values()))} communities")
```

## Choosing an Algorithm

### For Speed

Use Label Propagation or Connected Components

### For Quality

Use Louvain or Spectral Clustering

### For Understanding

Use Girvan-Newman (slower but interpretable)

### For Disconnected Graphs

Use Connected Components or Label Propagation

## Metrics

After detecting communities, evaluate results using:

- **Modularity**: How well communities are separated
- **Density**: Internal edge density per community
- **Conductance**: Cut quality between communities

## References

See individual algorithm pages for specific details and citations.
