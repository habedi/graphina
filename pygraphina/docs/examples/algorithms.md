# Graph Algorithms Examples

This page demonstrates various graph algorithms available in PyGraphina.

## Shortest Path Algorithms

### Example 1: Dijkstra's Algorithm

Find the shortest path between two nodes in a weighted graph.

```python
import pygraphina as pg

# Create a weighted road network
g = pg.PyGraph()
cities = [g.add_node(i) for i in range(6)]

# Add roads with distances
roads = [
    (0, 1, 7.0),   # A to B
    (0, 2, 9.0),   # A to C
    (0, 5, 14.0),  # A to F
    (1, 2, 10.0),  # B to C
    (1, 3, 15.0),  # B to D
    (2, 3, 11.0),  # C to D
    (2, 5, 2.0),   # C to F
    (3, 4, 6.0),   # D to E
    (4, 5, 9.0),   # E to F
]

for u, v, weight in roads:
    g.add_edge(cities[u], cities[v], weight)

# Find shortest paths from city 0
distances = g.dijkstra(cities[0])

print("Distances from city 0:")
for city, dist in sorted(distances.items()):
    if dist is not None:
        print(f"  City {city}: {dist}")

# Find specific path
result = g.shortest_path(cities[0], cities[4])
if result:
    distance, path = result
    print(f"\nShortest path from 0 to 4:")
    print(f"  Distance: {distance}")
    print(f"  Path: {' -> '.join(map(str, path))}")
```

### Example 2: Bellman-Ford Algorithm

Handle graphs with negative weights (but no negative cycles).

```python
import pygraphina as pg

# Create a graph with negative weights
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(5)]

# Add edges including negative weights
edges = [
    (0, 1, 4.0),
    (0, 2, 2.0),
    (1, 2, -3.0),  # Negative weight
    (1, 3, 2.0),
    (2, 3, 4.0),
    (2, 4, 5.0),
    (3, 4, -1.0),  # Negative weight
]

for u, v, w in edges:
    g.add_edge(nodes[u], nodes[v], w)

# Run Bellman-Ford
distances = g.bellman_ford(nodes[0])

if distances:
    print("Bellman-Ford distances from node 0:")
    for node, dist in sorted(distances.items()):
        if dist is not None:
            print(f"  Node {node}: {dist}")
else:
    print("Negative cycle detected!")
```

### Example 3: Floyd-Warshall Algorithm

Compute all-pairs shortest paths.

```python
import pygraphina as pg

# Create a small graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(4)]

# Add edges
edges = [(0, 1, 3.0), (0, 3, 7.0), (1, 2, 1.0),
         (1, 3, 2.0), (2, 3, 4.0)]

for u, v, w in edges:
    g.add_edge(nodes[u], nodes[v], w)

# Compute all-pairs shortest paths
all_paths = g.floyd_warshall()

if all_paths:
    print("All-pairs shortest distances:")
    for src in sorted(all_paths.keys()):
        for dst in sorted(all_paths[src].keys()):
            dist = all_paths[src][dst]
            if dist is not None:
                print(f"  {src} -> {dst}: {dist}")
```

## Traversal Algorithms

### Example 4: Breadth-First Search (BFS)

Explore nodes level by level.

```python
import pygraphina as pg

# Create a tree structure
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Build a binary tree
edges = [
    (0, 1), (0, 2),      # Level 1
    (1, 3), (1, 4),      # Level 2 left
    (2, 5), (2, 6),      # Level 2 right
    (3, 7), (3, 8),      # Level 3
    (4, 9),              # Level 3
]

for u, v in edges:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Perform BFS from root
bfs_order = g.bfs(nodes[0])
print(f"BFS traversal: {bfs_order}")

# BFS gives level-order traversal
# Output: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
```

### Example 5: Depth-First Search (DFS)

Explore as far as possible before backtracking.

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(7)]

# Create a more complex structure
edges = [
    (0, 1), (0, 2),
    (1, 3), (1, 4),
    (2, 5), (2, 6),
]

for u, v in edges:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Perform DFS from root
dfs_order = g.dfs(nodes[0])
print(f"DFS traversal: {dfs_order}")

# DFS explores one branch completely before moving to next
```

### Example 6: Bidirectional Search

Find paths faster by searching from both ends.

```python
import pygraphina as pg

# Create a long path graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(100)]

# Create a long chain
for i in range(99):
    g.add_edge(nodes[i], nodes[i+1], 1.0)

# Bidirectional search from start to end
path = g.bidirectional_search(nodes[0], nodes[99])

if path:
    print(f"Found path with {len(path)} nodes")
    print(f"First 10 nodes: {path[:10]}")
    print(f"Last 10 nodes: {path[-10:]}")
```

## Minimum Spanning Tree

### Example 7: Comparing MST Algorithms

```python
import pygraphina as pg

# Create a weighted graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(6)]

# Add weighted edges
edges = [
    (0, 1, 4), (0, 2, 3), (1, 2, 1),
    (1, 3, 2), (2, 3, 4), (3, 4, 2),
    (2, 4, 5), (4, 5, 6), (3, 5, 1)
]

for u, v, w in edges:
    g.add_edge(nodes[u], nodes[v], float(w))

# Try all three MST algorithms
algorithms = [
    ("Prim", pg.mst.prim_mst),
    ("Kruskal", pg.mst.kruskal_mst),
    ("Boruvka", pg.mst.boruvka_mst),
]

print("MST Results:")
for name, algo in algorithms:
    total, mst_edges = algo(g)
    print(f"\n{name} Algorithm:")
    print(f"  Total weight: {total}")
    print(f"  MST edges: {len(mst_edges)}")

    # All algorithms should give same total weight
```

## Graph Metrics

### Example 8: Computing Graph Properties

```python
import pygraphina as pg

# Create a test graph
g = pg.core.erdos_renyi_graph(50, 0.15, seed=42)

# Compute various metrics
print("Graph Metrics:")
print(f"  Nodes: {g.node_count()}")
print(f"  Edges: {g.edge_count()}")
print(f"  Density: {g.density():.4f}")
print(f"  Is connected: {g.is_connected()}")

# Diameter and radius
diameter = g.diameter()
radius = g.radius()
print(f"  Diameter: {diameter}")
print(f"  Radius: {radius}")

# Clustering metrics
avg_clustering = g.average_clustering()
transitivity = g.transitivity()
print(f"  Avg clustering: {avg_clustering:.4f}")
print(f"  Transitivity: {transitivity:.4f}")

# Assortativity
assortativity = g.assortativity()
print(f"  Assortativity: {assortativity:.4f}")

# Average path length
avg_path = g.average_path_length()
if avg_path:
    print(f"  Avg path length: {avg_path:.4f}")
```

### Example 9: Node-Level Metrics

```python
import pygraphina as pg

# Create a small graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(6)]

# Create triangles and paths
edges = [
    (0, 1), (1, 2), (2, 0),  # Triangle
    (2, 3), (3, 4), (4, 5),  # Path
]

for u, v in edges:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Compute node-level metrics
print("Node-Level Metrics:")
for node in nodes:
    degree = g.degree(node)
    clustering = g.clustering_of(node)
    triangles = g.triangles_of(node)

    print(f"  Node {node}:")
    print(f"    Degree: {degree}")
    print(f"    Clustering: {clustering:.3f}")
    print(f"    Triangles: {triangles}")
```

## Approximation Algorithms

### Example 10: Approximate Clique Finding

```python
import pygraphina as pg

# Create a graph with cliques
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(15)]

# Create a clique of size 5 (nodes 0-4)
for i in range(5):
    for j in range(i+1, 5):
        g.add_edge(nodes[i], nodes[j], 1.0)

# Add some random edges
for i in range(5, 15):
    for j in range(i+1, min(i+3, 15)):
        g.add_edge(nodes[i], nodes[j], 1.0)

# Find large clique
clique_size = pg.approximation.large_clique_size(g)
print(f"Approximate max clique size: {clique_size}")

# Get actual clique
max_clique = pg.approximation.max_clique(g)
print(f"Found clique: {max_clique}")
```

### Example 11: Approximate Vertex Cover

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(8)]

# Add edges
edges = [
    (0, 1), (0, 2), (1, 3), (2, 3),
    (3, 4), (4, 5), (5, 6), (6, 7)
]

for u, v in edges:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Find approximate vertex cover
vertex_cover = pg.approximation.min_weighted_vertex_cover(g)

print(f"Vertex cover size: {len(vertex_cover)}")
print(f"Vertex cover nodes: {sorted(vertex_cover)}")

# Verify it's a valid cover
covered_edges = 0
for u, v in edges:
    if nodes[u] in vertex_cover or nodes[v] in vertex_cover:
        covered_edges += 1

print(f"Covered {covered_edges}/{len(edges)} edges")
```

## Parallel Algorithms

### Example 12: Parallel BFS from Multiple Sources

```python
import pygraphina as pg

# Create a large graph
g = pg.core.barabasi_albert_graph(1000, 3, seed=42)

# Select multiple start nodes
start_nodes = [0, 100, 200, 300, 400]

# Run parallel BFS
results = pg.parallel.bfs_parallel(g, start_nodes)

print("Parallel BFS Results:")
for i, start in enumerate(start_nodes):
    reachable = len(results[i])
    print(f"  From node {start}: {reachable} reachable nodes")
```

### Example 13: Parallel Degree Computation

```python
import pygraphina as pg
import time

# Create a large graph
g = pg.core.erdos_renyi_graph(5000, 0.01, seed=42)

# Sequential degree computation
start = time.time()
seq_degrees = {node: g.degree(node) for node in g.nodes()}
seq_time = time.time() - start

# Parallel degree computation
start = time.time()
par_degrees = pg.parallel.degrees_parallel(g)
par_time = time.time() - start

print("Degree Computation:")
print(f"  Sequential: {seq_time:.4f}s")
print(f"  Parallel: {par_time:.4f}s")
print(f"  Speedup: {seq_time/par_time:.2f}x")

# Verify results match
assert seq_degrees == par_degrees
```

## Graph Generators

### Example 14: Using Built-in Graph Generators

```python
import pygraphina as pg

# Erdos-Renyi random graph
er_graph = pg.core.erdos_renyi_graph(n=100, p=0.1, seed=42)
print(f"Erdos-Renyi: {er_graph.node_count()} nodes, {er_graph.edge_count()} edges")

# Barabasi-Albert scale-free graph
ba_graph = pg.core.barabasi_albert_graph(n=100, m=3, seed=42)
print(f"Barabasi-Albert: {ba_graph.node_count()} nodes, {ba_graph.edge_count()} edges")

# Watts-Strogatz small-world graph
ws_graph = pg.core.watts_strogatz_graph(n=100, k=4, p=0.1, seed=42)
print(f"Watts-Strogatz: {ws_graph.node_count()} nodes, {ws_graph.edge_count()} edges")

# Complete graph
complete = pg.core.complete_graph(n=10)
print(f"Complete K10: {complete.node_count()} nodes, {complete.edge_count()} edges")

# Cycle graph
cycle = pg.core.cycle_graph(n=20)
print(f"Cycle: {cycle.node_count()} nodes, {cycle.edge_count()} edges")

# Path graph
path = pg.core.path_graph(n=15)
print(f"Path: {path.node_count()} nodes, {path.edge_count()} edges")
```

## I/O Operations

### Example 15: Saving and Loading Graphs

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(5)]

for i in range(4):
    g.add_edge(nodes[i], nodes[i+1], float(i+1))

# Save as edge list
g.save_edge_list("graph.txt", sep=" ")
print("Saved as edge list")

# Save as JSON
g.save_json("graph.json")
print("Saved as JSON")

# Save as binary
g.save_binary("graph.bin")
print("Saved as binary")

# Save as GraphML
g.save_graphml("graph.graphml")
print("Saved as GraphML")

# Load from edge list
g2 = pg.PyGraph()
num_nodes, num_edges = g2.load_edge_list("graph.txt", sep=" ")
print(f"Loaded: {num_nodes} nodes, {num_edges} edges")

# Load from JSON
g3 = pg.PyGraph()
g3.load_json("graph.json")
print(f"Loaded from JSON: {g3.node_count()} nodes")
```
