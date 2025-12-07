# Minimum Spanning Tree Algorithms

Minimum Spanning Tree (MST) algorithms find a subset of edges that connect all nodes in a graph with minimum total
weight.

## Overview

PyGraphina provides three MST algorithms:

- **Prim's Algorithm**: Greedy algorithm that grows the MST one edge at a time
- **Kruskal's Algorithm**: Greedy algorithm that sorts all edges and adds them if they don't create cycles
- **Boruvka's Algorithm**: Parallel algorithm that finds MST using component-based approach

All MST functions are available under the `pg.mst` module.

## Available Functions

| Function        | Time Complexity | Space Complexity | Best For           |
|-----------------|-----------------|------------------|--------------------|
| `prim_mst()`    | O(E log V)      | O(V)             | Dense graphs       |
| `kruskal_mst()` | O(E log E)      | O(V)             | Sparse graphs      |
| `boruvka_mst()` | O(E log V)      | O(V)             | Parallel execution |

Where:

- V = number of nodes
- E = number of edges

## Requirements

All MST algorithms require:

- Undirected graph (use `PyGraph`, not `PyDiGraph`)
- Connected graph
- Non-negative edge weights

## Return Value

All MST functions return a tuple `(total_weight, edges)`:

- `total_weight` (float): Sum of all edge weights in the MST
- `edges` (list): List of tuples `(node_u, node_v, weight)` representing MST edges

## Common Usage Pattern

```python
import pygraphina as pg

# Create a weighted graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(5)]

# Add weighted edges
g.add_edge(nodes[0], nodes[1], 2.0)
g.add_edge(nodes[0], nodes[2], 3.0)
g.add_edge(nodes[1], nodes[2], 1.0)
g.add_edge(nodes[1], nodes[3], 4.0)
g.add_edge(nodes[2], nodes[3], 5.0)
g.add_edge(nodes[3], nodes[4], 6.0)

# Compute MST using any algorithm
total_weight, mst_edges = pg.mst.prim_mst(g)

print(f"MST total weight: {total_weight}")
print(f"MST edges: {len(mst_edges)}")

for u, v, weight in mst_edges:
    print(f"  {u} -- {v} (weight: {weight})")
```

## Function Reference

### prim_mst

```python
pg.mst.prim_mst(graph: PyGraph) -> Tuple[float, List[Tuple[int, int, float]]]
```

Compute the MST using Prim's algorithm.

**Algorithm:** Starts from an arbitrary node and grows the MST by repeatedly adding the minimum-weight edge that
connects a node in the MST to a node outside it.

**Best for:** Dense graphs where E is close to V².

**Example:**

```python
total, edges = pg.mst.prim_mst(g)
```

### kruskal_mst

```python
pg.mst.kruskal_mst(graph: PyGraph) -> Tuple[float, List[Tuple[int, int, float]]]
```

Compute the MST using Kruskal's algorithm.

**Algorithm:** Sorts all edges by weight and adds them to the MST if they don't create a cycle (using union-find data
structure).

**Best for:** Sparse graphs where E is much smaller than V².

**Example:**

```python
total, edges = pg.mst.kruskal_mst(g)
```

### boruvka_mst

```python
pg.mst.boruvka_mst(graph: PyGraph) -> Tuple[float, List[Tuple[int, int, float]]]
```

Compute the MST using Boruvka's algorithm (parallel).

**Algorithm:** Each component simultaneously selects its minimum-weight outgoing edge, merging components in parallel.

**Best for:** Large graphs on multi-core systems where parallelism provides speedup.

**Example:**

```python
total, edges = pg.mst.boruvka_mst(g)
```

## Example: Comparing MST Algorithms

```python
import pygraphina as pg
import time

# Create a large random graph
g = pg.core.erdos_renyi_graph(100, 0.3, seed=42)

# Add random weights
for u, v in g.edges():
    g.update_edge_weight(u, v, hash((u, v)) % 100 + 1.0)

# Compare algorithms
for name, func in [("Prim", pg.mst.prim_mst),
                   ("Kruskal", pg.mst.kruskal_mst),
                   ("Boruvka", pg.mst.boruvka_mst)]:
    start = time.time()
    total, edges = func(g)
    elapsed = time.time() - start
    print(f"{name}: total={total:.2f}, edges={len(edges)}, time={elapsed:.4f}s")
```

## Use Cases

### Network Design

Find the minimum cost to connect all locations:

```python
# Cities with distances
cities = pg.PyGraph()
city_nodes = {name: cities.add_node(i)
              for i, name in enumerate(["A", "B", "C", "D"])}

# Add roads with costs
cities.add_edge(city_nodes["A"], city_nodes["B"], 10.0)
cities.add_edge(city_nodes["A"], city_nodes["C"], 15.0)
cities.add_edge(city_nodes["B"], city_nodes["C"], 5.0)
cities.add_edge(city_nodes["B"], city_nodes["D"], 20.0)
cities.add_edge(city_nodes["C"], city_nodes["D"], 8.0)

# Find minimum cost network
cost, roads = pg.mst.prim_mst(cities)
print(f"Minimum cost to connect all cities: {cost}")
```

### Clustering

MST can be used for hierarchical clustering by removing the heaviest edges:

```python
# Build MST
total, mst_edges = pg.mst.kruskal_mst(g)

# Sort edges by weight (descending)
sorted_edges = sorted(mst_edges, key=lambda e: e[2], reverse=True)

# Remove k-1 heaviest edges to create k clusters
k = 3
edges_to_keep = sorted_edges[k - 1:]

# Build cluster graph
cluster_graph = pg.PyGraph()
for node in g.nodes():
    cluster_graph.add_node(node)
for u, v, w in edges_to_keep:
    cluster_graph.add_edge(u, v, w)

# Find connected components (clusters)
clusters = pg.community.connected_components(cluster_graph)
```
