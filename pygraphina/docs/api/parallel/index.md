# Parallel Algorithms

Parallel algorithms use multi-threading to speed up computations on large graphs.

## Overview

PyGraphina provides parallel implementations of common graph algorithms that leverage multiple CPU cores. These
algorithms are useful when working with large graphs where the overhead of parallelization is offset by the performance
gain.

All parallel functions are available under the `pg.parallel` module.

## Available Functions

| Function                          | Sequential Equivalent    | Time Complexity    | Best For              |
|-----------------------------------|--------------------------|--------------------|-----------------------|
| `bfs_parallel()`                  | Multiple BFS calls       | O(V + E) per start | Multiple traversals   |
| `degrees_parallel()`              | `degree()` for each node | O(V + E)           | Computing all degrees |
| `connected_components_parallel()` | `connected_components()` | O(V + E)           | Large graphs          |

Where:

- V = number of nodes
- E = number of edges

## When to Use Parallel Algorithms

Parallel algorithms provide benefits when:

- Graph has many nodes (typically > 10,000)
- System has multiple CPU cores available
- Running multiple independent operations (like BFS from multiple sources)

For small graphs, sequential algorithms may be faster due to threading overhead.

## Function Reference

### bfs_parallel

```python
pg.parallel.bfs_parallel(
    graph: Union[PyGraph, PyDiGraph],
starts: List[int]
) -> List[List[int]]
```

Perform breadth-first search from multiple starting nodes in parallel.

**Parameters:**

- `graph`: The graph to traverse
- `starts`: List of starting node IDs

**Returns:**

List of node lists, where each inner list contains nodes visited from the corresponding start node.

**Example:**

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Add edges
for i in range(9):
    g.add_edge(nodes[i], nodes[i + 1], 1.0)

# Run BFS from multiple sources in parallel
start_nodes = [nodes[0], nodes[5]]
results = pg.parallel.bfs_parallel(g, start_nodes)

for i, visited in enumerate(results):
    print(f"BFS from {start_nodes[i]}: {visited}")
```

**Use Case:** Finding reachable nodes from multiple sources simultaneously (like influence spread analysis).

### degrees_parallel

```python
pg.parallel.degrees_parallel(
    graph: Union[PyGraph, PyDiGraph]
) -> Dict[int, int]
```

Compute the degree of all nodes in parallel.

**Parameters:**

- `graph`: The graph to analyze

**Returns:**

Dictionary mapping node IDs to their degrees.

**Example:**

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(100)]

# Add random edges
for i in range(200):
    u, v = i % 100, (i * 7) % 100
    if u != v:
        g.add_edge(nodes[u], nodes[v], 1.0)

# Compute all degrees in parallel
degrees = pg.parallel.degrees_parallel(g)

# Find highest degree node
max_node = max(degrees, key=degrees.get)
print(f"Node {max_node} has degree {degrees[max_node]}")
```

**Use Case:** Efficiently computing degree distribution for large graphs.

### connected_components_parallel

```python
pg.parallel.connected_components_parallel(
    graph: PyGraph
) -> Dict[int, int]
```

Find connected components using parallel processing.

**Parameters:**

- `graph`: The undirected graph to analyze

**Returns:**

Dictionary mapping node IDs to component IDs. Nodes with the same component ID belong to the same connected component.

**Example:**

```python
import pygraphina as pg

# Create a graph with multiple components
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Component 1: nodes 0-3
g.add_edge(nodes[0], nodes[1], 1.0)
g.add_edge(nodes[1], nodes[2], 1.0)
g.add_edge(nodes[2], nodes[3], 1.0)

# Component 2: nodes 4-6
g.add_edge(nodes[4], nodes[5], 1.0)
g.add_edge(nodes[5], nodes[6], 1.0)

# Component 3: nodes 7-9
g.add_edge(nodes[7], nodes[8], 1.0)
g.add_edge(nodes[8], nodes[9], 1.0)

# Find components in parallel
components = pg.parallel.connected_components_parallel(g)

# Group nodes by component
from collections import defaultdict

comp_groups = defaultdict(list)
for node, comp_id in components.items():
    comp_groups[comp_id].append(node)

print(f"Found {len(comp_groups)} components:")
for comp_id, nodes_list in comp_groups.items():
    print(f"  Component {comp_id}: {nodes_list}")
```

**Use Case:** Analyzing network structure in social networks, finding disconnected subgraphs.

## Performance Considerations

### Speedup Factors

Typical speedup on multi-core systems:

- **bfs_parallel**: Near-linear speedup with number of start nodes (up to core count)
- **degrees_parallel**: 2-4x speedup on 4+ core systems
- **connected_components_parallel**: 2-6x speedup depending on graph structure

### When Sequential is Better

Use sequential algorithms when:

- Graph has fewer than 1,000 nodes
- Running on single-core system
- Only performing one operation
- Memory is constrained

## Example: Performance Comparison

```python
import pygraphina as pg
import time

# Create a large graph
g = pg.core.barabasi_albert_graph(10000, 5, seed=42)

# Sequential degree computation
start = time.time()
seq_degrees = {node: g.degree(node) for node in g.nodes()}
seq_time = time.time() - start

# Parallel degree computation
start = time.time()
par_degrees = pg.parallel.degrees_parallel(g)
par_time = time.time() - start

print(f"Sequential: {seq_time:.4f}s")
print(f"Parallel: {par_time:.4f}s")
print(f"Speedup: {seq_time / par_time:.2f}x")
```

## Combining Parallel Operations

```python
import pygraphina as pg

# Large social network
g = pg.core.erdos_renyi_graph(5000, 0.01, seed=42)

# Parallel operations
degrees = pg.parallel.degrees_parallel(g)
components = pg.parallel.connected_components_parallel(g)

# Analyze degree distribution per component
from collections import defaultdict

comp_degrees = defaultdict(list)

for node in g.nodes():
    comp_id = components[node]
    degree = degrees[node]
    comp_degrees[comp_id].append(degree)

# Report statistics
for comp_id, deg_list in comp_degrees.items():
    avg_deg = sum(deg_list) / len(deg_list)
    print(f"Component {comp_id}: {len(deg_list)} nodes, avg degree {avg_deg:.2f}")
```
