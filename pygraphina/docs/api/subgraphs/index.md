# Subgraph Operations

Subgraph operations extract portions of a graph based on node sets or connectivity criteria.

## Overview

!!! note "API Location"
    All subgraph operations are available as instance methods on graph objects:
    ```python
    # Instance methods (recommended)
    sub = g.subgraph(nodes)
    induced = g.induced_subgraph(nodes)
    ego = g.ego_graph(center_node, radius)
    component = g.component_subgraph(node)
    neighbors = g.k_hop_neighbors(node, k)
    ```

PyGraphina provides several methods to extract subgraphs:

- Subgraph: Extract nodes and edges between them
- Induced Subgraph: Extract nodes and all edges between them from the original graph
- Ego Graph: Extract neighborhood around a central node
- Connected Component: Extract a single connected component
- K-hop Neighbors: Find all nodes within k hops of a starting node

All subgraph operations are methods on `PyGraph` and `PyDiGraph` objects.

## Function Reference

### subgraph

```python
graph.subgraph(nodes: List[int]) -> PyGraph
```

Extract a subgraph containing only the specified nodes and edges between them.

Parameters:

- `nodes`: List of node IDs to include

Returns:

New graph containing specified nodes and their connecting edges.

Example:

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(6)]

# Add edges
edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 0), (1, 4)]
for u, v in edges:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Extract subgraph with nodes 0, 1, 2
sub = g.subgraph([nodes[0], nodes[1], nodes[2]])

print(f"Subgraph: {sub.node_count()} nodes, {sub.edge_count()} edges")
```

### induced_subgraph

```python
graph.induced_subgraph(nodes: List[int]) -> PyGraph
```

Create an induced subgraph from a set of nodes.

Parameters:

- `nodes`: List of node IDs to include

Returns:

Induced subgraph containing specified nodes and all edges between them from the original graph.

Note: This is typically the same as `subgraph()` but explicitly guarantee all edges between the selected nodes are
included.

Example:

```python
import pygraphina as pg

g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(4)]

# Create a square
g.add_edge(nodes[0], nodes[1], 1.0)
g.add_edge(nodes[1], nodes[2], 1.0)
g.add_edge(nodes[2], nodes[3], 1.0)
g.add_edge(nodes[3], nodes[0], 1.0)

# Induced subgraph of opposite corners
induced = g.induced_subgraph([nodes[0], nodes[2]])
print(f"Induced subgraph: {induced.node_count()} nodes, {induced.edge_count()} edges")
```

### ego_graph

```python
graph.ego_graph(center: int, radius: int) -> PyGraph
```

Extract the ego graph centered at a node within a given radius.

Parameters:

- `center`: Center node ID
- `radius`: Maximum distance from center

Returns:

Subgraph containing the center node and all nodes within the specified radius.

Example:

```python
import pygraphina as pg

# Create a linear graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

for i in range(9):
    g.add_edge(nodes[i], nodes[i + 1], 1.0)

# Get ego graph around node 5 with radius 2
ego = g.ego_graph(nodes[5], 2)

print(f"Ego graph: {ego.node_count()} nodes")
# Output: Ego graph: 5 nodes (nodes 3, 4, 5, 6, 7)
```

Use Case: Analyzing local network structure around important nodes.

### k_hop_neighbors

```python
graph.k_hop_neighbors(start: int, k: int) -> List[int]
```

Find all nodes within k hops of a starting node.

Parameters:

- `start`: Starting node ID
- `k`: Maximum number of hops

Returns:

List of node IDs within k hops of the start node.

Example:

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(7)]

# Star topology: node 0 connected to all others
for i in range(1, 7):
    g.add_edge(nodes[0], nodes[i], 1.0)

# 1-hop neighbors of node 0
neighbors_1 = g.k_hop_neighbors(nodes[0], 1)
print(f"1-hop neighbors: {len(neighbors_1)}")  # 6 nodes

# 2-hop neighbors
neighbors_2 = g.k_hop_neighbors(nodes[1], 2)
print(f"2-hop neighbors of node 1: {len(neighbors_2)}")  # node 0 and all others
```

Use Case: Finding influence radius, recommendation systems.

### connected_component

```python
graph.connected_component(start: int) -> List[int]
```

Get all nodes in the same connected component as the starting node.

Parameters:

- `start`: Starting node ID

Returns:

List of all node IDs in the same component.

Example:

```python
import pygraphina as pg

g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(8)]

# Component 1
g.add_edge(nodes[0], nodes[1], 1.0)
g.add_edge(nodes[1], nodes[2], 1.0)

# Component 2
g.add_edge(nodes[3], nodes[4], 1.0)
g.add_edge(nodes[4], nodes[5], 1.0)

# Component 3 (isolated nodes)
# nodes[6] and nodes[7] are isolated

# Find component containing node 0
comp = g.connected_component(nodes[0])
print(f"Component size: {len(comp)}")  # 3 nodes
```

### component_subgraph

```python
graph.component_subgraph(start: int) -> PyGraph
```

Extract the subgraph of the connected component containing the starting node.

Parameters:

- `start`: Starting node ID

Returns:

Subgraph containing the entire connected component.

Example:

```python
import pygraphina as pg

g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(6)]

# Two components
g.add_edge(nodes[0], nodes[1], 1.0)
g.add_edge(nodes[1], nodes[2], 1.0)

g.add_edge(nodes[3], nodes[4], 1.0)
g.add_edge(nodes[4], nodes[5], 1.0)

# Extract first component
comp1 = g.component_subgraph(nodes[0])
print(f"Component 1: {comp1.node_count()} nodes, {comp1.edge_count()} edges")
```

## Filtering Operations

### filter_nodes

```python
graph.filter_nodes(predicate: Callable[[int, int], bool]) -> PyGraph
```

Create a subgraph containing only nodes that satisfy a predicate.

Parameters:

- `predicate`: Function taking `(node_id, node_attr)` and returning `True` to include the node

Returns:

Subgraph with filtered nodes.

Example:

```python
import pygraphina as pg

g = pg.PyGraph()
# Add nodes with different attributes
for i in range(10):
    g.add_node(i * 10)  # attributes: 0, 10, 20, ..., 90

# Add some edges
for i in range(9):
    g.add_edge(i, i + 1, 1.0)

# Keep only nodes with attribute >= 50
filtered = g.filter_nodes(lambda nid, attr: attr >= 50)
print(f"Filtered graph: {filtered.node_count()} nodes")
```

### filter_edges

```python
graph.filter_edges(predicate: Callable[[int, int, float], bool]) -> PyGraph
```

Create a subgraph containing only edges that satisfy a predicate.

Parameters:

- `predicate`: Function taking `(source_id, target_id, weight)` and returning `True` to include the edge

Returns:

Subgraph with filtered edges (all original nodes are kept).

Example:

```python
import pygraphina as pg

g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(5)]

# Add weighted edges
g.add_edge(nodes[0], nodes[1], 0.5)
g.add_edge(nodes[1], nodes[2], 1.5)
g.add_edge(nodes[2], nodes[3], 0.8)
g.add_edge(nodes[3], nodes[4], 2.0)

# Keep only edges with weight > 1.0
filtered = g.filter_edges(lambda u, v, w: w > 1.0)
print(f"Filtered: {filtered.edge_count()} edges")
```

## Use Cases

### Community Analysis

```python
import pygraphina as pg

# Social network
g = pg.core.barabasi_albert(100, 3, 42)

# Detect communities
communities = pg.community.louvain(g, seed=42)

# Extract each community as a subgraph
community_graphs = {}

for comm_id, members in enumerate(communities):
    if len(members) > 0:
        community_graphs[comm_id] = g.subgraph(members)

for comm_id, nodes in comm_nodes.items():
    community_graphs[comm_id] = g.subgraph(nodes)
    print(f"Community {comm_id}: {len(nodes)} nodes")
```

### Local Network Analysis

```python
import pygraphina as pg

# Large network
g = pg.core.barabasi_albert(1000, 3, seed=42)

# Find high-degree node
degrees = {n: g.degree[n] for n in g.nodes}
hub = max(degrees, key=degrees.get)

# Analyze hub's local network
ego = g.ego_graph(hub, radius=2)

# Compute local metrics
local_clustering = ego.average_clustering()
local_density = ego.density()

print(f"Hub {hub}: degree={degrees[hub]}")
print(f"Ego network: {ego.node_count()} nodes, density={local_density:.3f}")
```

### Path Extraction

```python
import pygraphina as pg

g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Create a path
for i in range(9):
    g.add_edge(nodes[i], nodes[i + 1], 1.0)

# Find shortest path
result = g.shortest_path(nodes[0], nodes[9])
if result:
    distance, path = result
    # Extract path as subgraph
    path_graph = g.subgraph(path)
    print(f"Path length: {distance}, nodes: {len(path)}")
```
