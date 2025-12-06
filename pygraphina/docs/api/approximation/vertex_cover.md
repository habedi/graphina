# Vertex Cover Approximation

A vertex cover is a set of nodes such that every edge has at least one endpoint in the set. The minimum vertex cover problem is NP-hard.

## Function Signature

```python
pg.approximation.min_weighted_vertex_cover(graph: PyGraph) -> Set[int]
```

## Parameters

- **graph**: The graph to analyze

## Returns

Set of node IDs that form the vertex cover.

## Description

This implements a 2-approximation algorithm for the minimum vertex cover problem:
- Solution is guaranteed to be within 2x the optimal
- Runs in polynomial time

## Algorithm

The algorithm uses a greedy approach based on finding maximum matchings:
1. While uncovered edges exist:
2. Pick an arbitrary uncovered edge
3. Add both its endpoints to the cover
4. Remove all edges incident to these nodes

## Time Complexity

O(V + E)

## Space Complexity

O(V)

## Example

```python
import pygraphina as pg

# Create a small graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(6)]

# Add edges
edges = [(0,1), (1,2), (2,3), (3,4), (4,5), (5,0)]
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

This is a 2-approximation algorithm, meaning:
- Size of returned cover ≤ 2 × (size of optimal cover)
- No polynomial algorithm is known to do better (unless P=NP)

## Use Cases

- Network reliability analysis
- Sensor placement
- Dominating set approximation
- Resource allocation

## Comparison

| Aspect | Exact | Approximation |
|--------|-------|--------------|
| Optimality | Guaranteed | 2x guarantee |
| Time | Exponential | Polynomial |
| Scalability | Small graphs | Large graphs |
