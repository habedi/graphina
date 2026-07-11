# Vertex Cover Approximation

A vertex cover is a set of nodes such that every edge has at least one endpoint in the set. The minimum vertex cover
problem is NP-hard.

## Function Signature

```python
pg.approximation.min_weighted_vertex_cover(graph: PyGraph) -> Set[int]
```

## Parameters

- graph: The graph to analyze

## Returns

Set of node IDs that form the vertex cover.

## Description

This implements a greedy maximum-degree heuristic for the minimum vertex cover problem:

- Edge weights are ignored
- Runs in polynomial time

## Algorithm

The algorithm greedily picks high-degree nodes:

1. While uncovered edges exist:
2. Pick the node covering the most still-uncovered edges
3. Add it to the cover
4. Mark all its incident edges as covered

## Time Complexity

O((V + E) log V)

## Space Complexity

O(V)

## Example

```python
import pygraphina as pg

# Create a small graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(6)]

# Add edges
edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 0)]
for u, v in edges:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Find vertex cover
cover = pg.approximation.min_weighted_vertex_cover(g)
print(f"Vertex cover size: {len(cover)}")
print(f"Cover nodes: {sorted(cover)}")

# Verify all edges are covered
for u, v in edges:
    assert nodes[u] in cover or nodes[v] in cover, f"Edge {u}-{v} not covered!"
print("All edges covered!")
```

## Approximation Guarantee

The maximum-degree heuristic carries a logarithmic guarantee:

- Size of returned cover ≤ O(log V) × (size of optimal cover) in the worst case
- In practice it often produces covers close to the optimum

## Use Cases

- Network reliability analysis
- Sensor placement
- Dominating set approximation
- Resource allocation

## Comparison

| Aspect      | Exact        | Approximation |
|-------------|--------------|---------------|
| Optimality  | Guaranteed   | 2x guarantee  |
| Time        | Exponential  | Polynomial    |
| Scalability | Small graphs | Large graphs  |
