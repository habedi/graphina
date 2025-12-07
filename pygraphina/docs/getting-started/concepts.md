# Basic Concepts

Fundamental concepts and design principles of PyGraphina.

## Graph Model

PyGraphina represents graphs using an adjacency list data structure, which provides efficient storage and fast neighbor
    While PyGraphina allows modifying the graph structure, it's generally more efficient to build the graph first and then
    analyze it. Frequent modifications (especially removals) can be slower than bulk building.
lookups for sparse graphs (common in real-world networks).

### Undirected vs. Directed Graphs

PyGraphina supports both undirected and directed graphs:

#### Undirected Graphs (`PyGraph`)

- Edges have no direction
- If node A is connected to node B, then B is also connected to A
- Used for: social networks, collaboration networks, molecular structures

```python
import pygraphina as pg

g = pg.PyGraph()
a = g.add_node(1)
b = g.add_node(2)
g.add_edge(a, b, 1.0)

# In undirected graph, both directions exist
assert g.contains_edge(a, b)
assert g.contains_edge(b, a)
```

#### Directed Graphs (`PyDiGraph`)

- Edges have direction (from source to target)
- An edge from A to B doesn't imply an edge from B to A
- Used for: web links, citation networks, workflow dependencies

```python
dg = pg.PyDiGraph()
a = dg.add_node(1)
b = dg.add_node(2)
dg.add_edge(a, b, 1.0)  # Edge from a to b

# Only the specified direction exists
assert dg.contains_edge(a, b)
assert not dg.contains_edge(b, a)  # No edge from b to a
```

## Nodes

### Node IDs

When you add a node, PyGraphina returns a unique node ID (a simple integer):

```python
g = pg.PyGraph()
node_0 = g.add_node(100)  # Returns 0
node_1 = g.add_node(200)  # Returns 1
node_2 = g.add_node(300)  # Returns 2

# Node IDs are sequential starting from 0
assert node_0 == 0
assert node_1 == 1
assert node_2 == 2
```

!!! warning "Node IDs After Deletion"
    When you remove a node, its ID is not reused. The next added node gets the next sequential ID.

### Node Attributes

Each node has an integer attribute associated with it:

```python
g = pg.PyGraph()
node_id = g.add_node(42)  # 42 is the node attribute

# Retrieve the attribute
attr = g.get_node_attr(node_id)
assert attr == 42

# Update the attribute
g.update_node(node_id, 100)
assert g.get_node_attr(node_id) == 100
```

!!! tip "Use Cases for Node Attributes"
    Node attributes can represent various properties:

    - User IDs in a social network
    - Station codes in a transportation network
    - Molecule types in a chemical structure
    - Entity IDs for mapping to external data

    Nodes are mapped to integer `NodeId`s internally. While you can use these IDs directly, the mappings (e.g., node 'A' -> ID 0)
    are managed by your application if you need to map back to original data.

## Edges

### Edge Weights

All edges in PyGraphina have a floating-point weight:

```python
g = pg.PyGraph()
a, b = g.add_node(1), g.add_node(2)

# Add edge with weight
g.add_edge(a, b, 2.5)

# Weights represent different things depending on context:
# - Distance (shorter is closer)
# - Similarity (higher is more similar)
# - Strength (higher is stronger connection)
# - Cost (higher is more expensive)
```

### Multiple Edges

PyGraphina does not support multiple edges between the same pair of nodes (no multigraph support):

```python
g = pg.PyGraph()
a, b = g.add_node(1), g.add_node(2)

g.add_edge(a, b, 1.0)
g.add_edge(a, b, 2.0)  # This will update the existing edge

# Only one edge exists between a and b
assert g.edge_count() == 1
```

### Self-Loops

Self-loops (edges from a node to itself) are supported:

```python
g = pg.PyGraph()
a = g.add_node(1)
g.add_edge(a, a, 1.0)  # Self-loop

assert g.contains_edge(a, a)
```

## Graph Operations

### Basic Queries

```python
g = pg.PyGraph()
a, b, c = [g.add_node(i) for i in range(3)]
g.add_edge(a, b, 1.0)
g.add_edge(b, c, 1.0)

# Node count
print(g.node_count())  # 3

# Edge count
print(g.edge_count())  # 2

# Check if directed
print(g.is_directed())  # False

# Graph density (ratio of existing edges to possible edges)
print(g.density())  # 0.666...

# Get all nodes
print(g.nodes)  # [0, 1, 2]

# Get neighbors of a node
print(g.neighbors(a))  # [1]
print(g.neighbors(b))  # [0, 2]

# Get node degree
print(g.degree(b))  # 2
```

### Modifying Graphs

```python
g = pg.PyGraph()

# Add nodes
a = g.add_node(1)
b = g.add_node(2)

# Add edges
g.add_edge(a, b, 1.0)

# Remove edge
g.remove_edge(a, b)

# Remove node (also removes incident edges)
g.remove_node(a)

# Clear entire graph
g.clear()
assert g.node_count() == 0
```

## Algorithm Organization

PyGraphina organizes algorithms into logical modules:

### `pg.centrality`

Measures of node importance and influence:

```python
# PageRank centrality
pagerank = pg.centrality.pagerank(g, damping=0.85, max_iter=100, tolerance=1e-6)

# Betweenness centrality
betweenness = pg.centrality.betweenness(g, True)

# Degree centrality
degree = pg.centrality.degree(g)
```

### `pg.community`

Community detection and clustering:

```python
# Label propagation
communities = pg.community.label_propagation(g, max_iter=100)

# Louvain method
communities = pg.community.louvain(g)

# Connected components
components = pg.community.connected_components(g)
```

### `pg.links`

Link prediction algorithms:

```python
# Jaccard coefficient
jaccard = pg.links.jaccard_coefficient(g)

# Adamic-Adar index
adamic_adar = pg.links.adamic_adar_index(g)

# Resource allocation
resource_alloc = pg.links.resource_allocation_index(g)
```

### `pg.approximation`

Approximation algorithms for hard problems:

```python
# Approximate maximum clique
clique_size = pg.approximation.large_clique_size(g)

# Approximate clustering coefficient
clustering = pg.approximation.approximate_clustering_coefficient(g, num_samples=1000)

# Approximate diameter
diameter = pg.approximation.approximate_diameter(g)
```

### `pg.core`

Core graph generators and utilities:

```python
# Graph generators
g = pg.core.erdos_renyi(n=100, p=0.1, seed=42)
g = pg.core.barabasi_albert(n=100, m=2, seed=42)
g = pg.core.watts_strogatz(n=100, k=4, beta=0.3, seed=42)
g = pg.core.complete_graph(n=10)

# Shortest paths use graph methods directly:
distances = g.dijkstra(source)  # Returns dict of distances
result = g.shortest_path(source, target)  # Returns (distance, path) or None
```

### `pg.metrics`

Graph and node metrics:

```python
# Graph-level metrics
diameter = pg.metrics.diameter(g)
radius = pg.metrics.radius(g)
avg_clustering = pg.metrics.average_clustering_coefficient(g)

# Node-level metrics
clustering = pg.metrics.clustering_coefficient(g, node)
triangles = pg.metrics.triangles(g, node)
```

### `pg.mst`

Minimum spanning tree algorithms:

```python
# Kruskal's algorithm
mst = pg.mst.kruskal(g)

# Prim's algorithm
mst = pg.mst.prim(g, start_node)
```

### `pg.traversal`

Graph traversal algorithms:

```python
# Breadth-first search
bfs_order = pg.traversal.bfs(g, start_node)

# Depth-first search
dfs_order = pg.traversal.dfs(g, start_node)
```

### `pg.subgraphs`

Subgraph extraction:

```python
# Extract induced subgraph
subgraph = pg.subgraphs.induced_subgraph(g, node_set)

# Extract ego graph (k-hop neighborhood)
ego = pg.subgraphs.ego_graph(g, center_node, radius=2)
```

### `pg.parallel`

Parallel implementations for large graphs:

```python
# Parallel PageRank
pagerank = pg.parallel.pagerank_parallel(g, 0.85, 100, 1e-6)

# Parallel BFS
bfs_order = pg.parallel.bfs_parallel(g, [start_node])
```

## Performance Considerations

### Graph Size

PyGraphina is designed for graphs with:

- Millions of nodes
- Tens of millions of edges
- Sparse connectivity (typical of real-world networks)

### Memory Usage

- Each node requires minimal memory (just an ID and attribute)
- Edges are stored efficiently in adjacency lists
- Memory usage grows approximately linearly with the number of edges

### Parallel Processing

Many algorithms have parallel implementations in `pg.parallel` that can leverage multiple CPU cores:

```python
# Sequential version
result = pg.centrality.pagerank(large_graph, 0.85, 100, 1e-6)

# Parallel version (faster for large graphs)
result = pg.parallel.pagerank_parallel(large_graph, 0.85, 100, 1e-6)
```

## Data Types

### Return Types

Algorithms return different data types depending on their nature:

- **Dict[int, float]**: Node-to-score mappings (centrality algorithms)
- **Dict[int, int]**: Node-to-cluster mappings (community detection)
- **Dict[tuple, float]**: Edge-to-score mappings (link prediction)
- **List[int]**: Node sequences (paths, traversals)
- **float**: Single values (density, diameter)

Example:

```python
# Returns Dict[int, float] mapping node IDs to PageRank scores
pagerank = pg.centrality.pagerank(g, 0.85, 100, 1e-6)
print(pagerank)  # {0: 0.25, 1: 0.35, 2: 0.40}

# Returns Dict[int, int] mapping node IDs to community labels
communities = pg.community.label_propagation(g, 100)
print(communities)  # {0: 0, 1: 0, 2: 1}

# Returns List[int] with node IDs in traversal order
bfs = pg.traversal.bfs(g, start_node=0)
print(bfs)  # [0, 1, 2, 3, 4]
```

## Best Practices

### 1. Use Appropriate Graph Type

Choose the right graph type for your use case:

- Use `PyGraph` for symmetric relationships
- Use `PyDiGraph` for asymmetric relationships

### 2. Batch Operations

When building large graphs, add all nodes and edges in loops rather than one at a time in separate statements.

### 3. Choose the Right Algorithm

For large graphs:

- Use parallel implementations when available
- Use approximation algorithms when exact solutions are too slow
- Consider the time complexity of algorithms

### 4. Handle Missing Data

Always check for None or empty results:

```python
result = g.shortest_path(source, target)
if result:
    distance, path = result
    print(f"Path found: {path}, distance: {distance}")
else:
    print("No path exists")
```

## Next Steps

- [Quick Start](quickstart.md): Build your first graph
- [API Reference](../api/graph.md): Detailed method documentation
- [Examples](../examples/basic.md): Learn from examples
