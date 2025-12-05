# Graph Generators

Graph generators create synthetic graphs with specific properties for testing and analysis.

## Random Graph Models

### Erdos-Renyi (ER)

```python
pg.core.erdos_renyi_graph(n, p, seed=None)
```

Random graph where each edge exists with probability p.
- Properties: Uniform random
- Uses: Null model, baseline comparisons
- Time: O(V²)

### Barabasi-Albert (BA)

```python
pg.core.barabasi_albert_graph(n, m, seed=None)
```

Preferential attachment model - new nodes attach to existing high-degree nodes.
- Properties: Scale-free, power-law degree distribution
- Uses: Social networks, web graphs
- Time: O(V·m)

### Watts-Strogatz (WS)

```python
pg.core.watts_strogatz_graph(n, k, p, seed=None)
```

Small-world model - regular lattice with random rewiring.
- Properties: High clustering, low diameter
- Uses: Social networks, biological networks
- Time: O(V)

## Structured Graphs

### Complete Graph

```python
pg.core.complete_graph(n)
```

All possible edges present (K_n).
- Edges: n(n-1)/2
- Density: 1.0
- Diameter: 1

### Cycle Graph

```python
pg.core.cycle_graph(n)
```

Nodes arranged in a circle.
- Edges: n
- Diameter: n/2
- Uses: Ring networks

### Path Graph

```python
pg.core.path_graph(n)
```

Linear chain of nodes.
- Edges: n-1
- Diameter: n-1
- Uses: Sequential structures

## Examples

```python
import pygraphina as pg

# Generate different types of graphs
er = pg.core.erdos_renyi_graph(100, 0.1, seed=42)
ba = pg.core.barabasi_albert_graph(100, 3, seed=42)
ws = pg.core.watts_strogatz_graph(100, 4, 0.1, seed=42)

# Analyze properties
print(f"ER density: {er.density():.3f}")
print(f"BA avg clustering: {ba.average_clustering():.3f}")
print(f"WS diameter: {ws.diameter()}")

# Use for testing
for graph in [er, ba, ws]:
    centrality = pg.centrality.pagerank(graph, 0.85, 100, 1e-6)
    print(f"Nodes: {graph.node_count()}, PageRank range: {min(centrality.values()):.4f} - {max(centrality.values()):.4f}")
```

## Use Cases

- Algorithm testing
- Benchmark creation
- Network model comparison
- Synthetic data generation
- Hypothesis testing

## Properties Comparison

| Property | ER | BA | WS |
|----------|----|----|-----|
| Clustering | Low | Low-Medium | High |
| Diameter | Small | Small-Medium | Small |
| Degree Distribution | Binomial | Power-law | Regular |
| Realism | Low | High | Medium |

## Seeding

All generators support optional seed parameter for reproducibility:

```python
# Same seed produces identical graphs
g1 = pg.core.erdos_renyi_graph(100, 0.1, seed=42)
g2 = pg.core.erdos_renyi_graph(100, 0.1, seed=42)
# g1 and g2 are identical
```
