# Centrality Algorithms

Centrality measures identify the most important nodes in a graph based on various criteria.

## Overview

PyGraphina provides multiple centrality algorithms, each capturing different notions of "importance":

- **Degree**: Based on number of connections
- **Betweenness**: Based on being on shortest paths
- **Closeness**: Based on distance to other nodes
- **Eigenvector**: Based on connections to important nodes
- **PageRank**: Based on random walk probability
- **Katz**: Based on weighted path counts
- **Harmonic**: Based on reciprocal distances
- **Reaching**: Based on reachability

All centrality functions are available under the `pg.centrality` module.

## Available Functions

| Function | Time Complexity | Best For |
|----------|----------------|----------|
| `degree()` | O(V) | Quick importance metric |
| `betweenness()` | O(V·E) | Finding bridges/connectors |
| `closeness()` | O(V·E) | Finding central nodes |
| `eigenvector()` | O(V·E·k) | Influence-based importance |
| `pagerank()` | O(V·E·k) | Web ranking, influence |
| `katz()` | O(V·E·k) | Weighted influence |
| `harmonic()` | O(V·E) | Alternative to closeness |
| `local_reaching()` | O(V·E) | Local reachability |
| `global_reaching()` | O(V·E) | Global reachability |

Where:
- V = number of nodes
- E = number of edges
- k = number of iterations (for iterative algorithms)

## Common Usage Pattern

```python
import pygraphina as pg

# Create or load a graph
g = pg.PyGraph()
# ... add nodes and edges ...

# Calculate centrality
scores = pg.centrality.pagerank(g, damping=0.85, max_iters=100, tol=1e-6)

# Find the most important node
most_important = max(scores, key=scores.get)
print(f"Most important node: {most_important} (score: {scores[most_important]:.4f})")

# Sort nodes by importance
ranked = sorted(scores.items(), key=lambda x: x[1], reverse=True)
print("Top 5 nodes:")
for node, score in ranked[:5]:
    print(f"  Node {node}: {score:.4f}")
```

## Quick Reference

### Degree Centrality

```python
scores = pg.centrality.degree(g)
```

Simple count of connections. Fast but doesn't consider graph structure.

### Betweenness Centrality

```python
scores = pg.centrality.betweenness(g)
```

Measures how often a node appears on shortest paths. Good for finding bridges.

### Closeness Centrality

```python
scores = pg.centrality.closeness(g)
```

Based on average distance to all other nodes. Good for finding central hubs.

### Eigenvector Centrality

```python
scores = pg.centrality.eigenvector(g, max_iters=100, tol=1e-6)
```

Connections to important nodes matter more. Similar to PageRank.

### PageRank

```python
scores = pg.centrality.pagerank(g, damping=0.85, max_iters=100, tol=1e-6)
```

The algorithm behind Google Search. Models random surfing behavior.

### Katz Centrality

```python
scores = pg.centrality.katz(g, alpha=0.1, beta=1.0, max_iters=100, tol=1e-6)
```

Weighted sum of all paths, with exponential decay for longer paths.

### Harmonic Centrality

```python
scores = pg.centrality.harmonic(g)
```

Based on reciprocal distances. Handles disconnected graphs better than closeness.

## Choosing the Right Centrality

### For Social Networks

- **Influencers**: PageRank, Eigenvector
- **Connectors**: Betweenness
- **Popular users**: Degree

### For Transportation Networks

- **Central hubs**: Closeness, Harmonic
- **Critical junctions**: Betweenness
- **Well-connected stations**: Degree

### For Citation Networks

- **Important papers**: PageRank, Eigenvector
- **Foundational work**: Betweenness
- **Highly cited**: Degree

### For Biological Networks

- **Critical proteins**: Betweenness
- **Regulatory hubs**: Degree
- **Central pathways**: Closeness

## See Individual Pages

- [Degree Centrality](degree.md)
- [Betweenness Centrality](betweenness.md)
- [Closeness Centrality](closeness.md)
- [Eigenvector Centrality](eigenvector.md)
- [PageRank](pagerank.md)
- [Katz Centrality](katz.md)
- [Harmonic Centrality](harmonic.md)
- [Reaching Centrality](reaching.md)

## Example: Comparing Centrality Measures

```python
import pygraphina as pg

# Create a small social network
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(7)]

# Create a star topology with a hub
hub = nodes[0]
for node in nodes[1:]:
    g.add_edge(hub, node, 1.0)

# Add some peripheral connections
g.add_edge(nodes[1], nodes[2], 1.0)
g.add_edge(nodes[3], nodes[4], 1.0)

# Calculate different centrality measures
degree = pg.centrality.degree(g)
betweenness = pg.centrality.betweenness(g)
pagerank = pg.centrality.pagerank(g, 0.85, 100, 1e-6)

print("Centrality Comparison:")
print(f"{'Node':<6} {'Degree':<10} {'Betweenness':<15} {'PageRank':<10}")
for node in nodes:
    print(f"{node:<6} {degree[node]:<10} {betweenness[node]:<15.4f} {pagerank[node]:<10.4f}")

# Output shows hub (node 0) has highest scores in all measures
```

## Performance Tips

1. **Use parallel implementations** for large graphs (see [Parallel module](../parallel/index.md))
2. **Approximate algorithms** are faster for very large graphs
3. **Degree centrality** is fastest for quick analysis
4. **Adjust iteration limits** to balance speed vs accuracy

## Related

- [Community Detection](../community/index.md): Find groups of related nodes
- [Link Prediction](../links/index.md): Predict future connections
- [Metrics](../metrics/graph.md): Graph-level statistics
