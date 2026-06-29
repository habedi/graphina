# Path Algorithms

Path algorithms find optimal or near-optimal routes through graphs.

## Available Algorithms

| Algorithm             | Type           | Time       | Negatives | Use Case                  |
|-----------------------|----------------|------------|-----------|---------------------------|
| Dijkstra              | Single-source  | O(E log V) | No        | Shortest paths            |
| Bellman-Ford          | Single-source  | O(V·E)     | Yes       | General paths             |
| Floyd-Warshall        | All-pairs      | O(V³)      | Yes       | Complete distance matrix  |
| BFS                   | Unweighted     | O(V+E)     | N/A       | Unweighted shortest paths |
| A*                    | Point-to-point | O(E)       | No        | Heuristic-guided          |
| Dijkstra (One-to-One) | Point-to-point | O(E log V) | No        | Single path               |

## Common Usage

```python
import pygraphina as pg

# Create a weighted graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(5)]

# Add weighted edges
edges = [(0, 1, 1), (0, 2, 4), (1, 2, 2), (1, 3, 5), (2, 3, 1), (3, 4, 3)]
for u, v, w in edges:
    g.add_edge(nodes[u], nodes[v], float(w))

# Single-source shortest paths
distances = g.dijkstra(nodes[0])
print(f"Distances from 0: {distances}")

# Point-to-point shortest path
result = g.shortest_path(nodes[0], nodes[4])
if result:
    distance, path = result
    print(f"Path 0->4: {path}, distance: {distance}")

# All-pairs shortest paths
all_distances = g.floyd_warshall()
print(f"Distance 0->4: {all_distances[0][4]}")
```

## shortest_path() Method

Find the shortest path between two specific nodes using Dijkstra's algorithm.

```python
result = g.shortest_path(source: int, target: int) -> Optional[Tuple[float, List[int]]]
```

### Parameters

- source (int): The starting node ID
- target (int): The destination node ID

### Returns

- Optional[Tuple[float, List[int]]]: A tuple containing:
  - The total distance/cost (float) as the first element
  - The path as a list of node IDs (in order from source to target) as the second element
  - Returns `None` if no path exists between source and target

### Raises

- ValueError: If source or target node IDs are invalid

### Requirements

- Graph must have non-negative edge weights
- Applies only to undirected graphs for PyGraph; for PyDiGraph, follows edge directions

### Example

```python
import pygraphina as pg

# Create a graph representing a network
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(6)]

# Build a network: 0-1-2-3-4-5
g.add_edge(nodes[0], nodes[1], 2.0)
g.add_edge(nodes[1], nodes[2], 3.0)
g.add_edge(nodes[2], nodes[3], 1.0)
g.add_edge(nodes[3], nodes[4], 2.0)
g.add_edge(nodes[4], nodes[5], 4.0)

# Also add a shortcut
g.add_edge(nodes[0], nodes[3], 7.0)  # Direct but longer path

# Find shortest path from 0 to 5
result = g.shortest_path(nodes[0], nodes[5])
if result:
    distance, path = result
    print(f"Shortest distance: {distance}")
    print(f"Path: {path}")
    # Output:
    # Shortest distance: 12.0
    # Path: [0, 1, 2, 3, 4, 5]
else:
    print("No path found between nodes")

# Find path via shortcut
result = g.shortest_path(nodes[0], nodes[3])
if result:
    distance, path = result
    print(f"Distance from 0 to 3: {distance}")
    print(f"Path: {path}")
    # Output:
    # Distance from 0 to 3: 6.0
    # Path: [0, 1, 2, 3]
```

### Performance Notes

- Uses Dijkstra's algorithm internally
- Time complexity: O(E log V)
- Returns as soon as target is found (early termination)
- More efficient than computing all-pairs distances for single path lookups

---

## dijkstra() Method

Compute single-source shortest paths from one node to all reachable nodes.

```python
distances = g.dijkstra(source: int, cutoff: Optional[float] = None) -> Dict[int, Optional[float]]
```

### Parameters

- source (int): The starting node ID
- cutoff (Optional[float]): Maximum distance to explore (optional). Nodes beyond this distance are not computed.

### Returns

- Dict[int, Optional[float]]: Dictionary mapping node IDs to their shortest distances from source
  - Distance is `None` if the node is unreachable

### Example

```python
import pygraphina as pg

g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(5)]
g.add_edge(nodes[0], nodes[1], 1.0)
g.add_edge(nodes[1], nodes[2], 2.0)
g.add_edge(nodes[2], nodes[3], 3.0)
g.add_edge(nodes[0], nodes[4], 10.0)

# Get all shortest distances from node 0
distances = g.dijkstra(nodes[0])
print(distances)
# {0: 0.0, 1: 1.0, 2: 3.0, 3: 6.0, 4: 10.0}

# Get distances with a cutoff
distances_cutoff = g.dijkstra(nodes[0], cutoff=5.0)
print(distances_cutoff)
# {0: 0.0, 1: 1.0, 2: 3.0, 3: None, 4: None}  # 3 and 4 exceed cutoff
```

---

## bellman_ford() Method

Compute single-source shortest paths using Bellman-Ford algorithm (handles negative weights).

```python
result = g.bellman_ford(source: int) -> Optional[Dict[int, Optional[float]]]
```

### Parameters

- source (int): The starting node ID

### Returns

- Optional[Dict[int, Optional[float]]]: Dictionary mapping node IDs to shortest distances, or `None` if a negative cycle is detected

### Handles

- Negative edge weights
- Detects negative cycles

### Example

```python
import pygraphina as pg

g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(4)]
g.add_edge(nodes[0], nodes[1], 4.0)
g.add_edge(nodes[1], nodes[2], -2.0)  # Negative weight
g.add_edge(nodes[0], nodes[2], 2.0)

distances = g.bellman_ford(nodes[0])
if distances is not None:
    print(distances)  # {0: 0.0, 1: 4.0, 2: 2.0}
else:
    print("Negative cycle detected")
```

---

## floyd_warshall() Method

Compute all-pairs shortest paths matrix.

```python
distances = g.floyd_warshall() -> Optional[Dict[int, Dict[int, Optional[float]]]]
```

### Returns

- Optional[Dict[int, Dict[int, Optional[float]]]]: A nested dictionary where `distances[u][v]` is the shortest distance from node u to node v
  - Returns `None` if a negative cycle is detected

### Use Cases

- Complete distance matrix needed
- Multiple distance lookups required
- Small to medium graphs (expensive O(V³))

### Example

```python
import pygraphina as pg

g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(4)]
g.add_edge(nodes[0], nodes[1], 1.0)
g.add_edge(nodes[1], nodes[2], 2.0)
g.add_edge(nodes[2], nodes[3], 1.0)
g.add_edge(nodes[0], nodes[3], 10.0)

# Get all-pairs shortest distances
all_distances = g.floyd_warshall()
if all_distances:
    print(f"Distance from 0 to 3: {all_distances[nodes[0]][nodes[3]]}")  # 4.0
    print(f"Distance from 1 to 3: {all_distances[nodes[1]][nodes[3]]}")  # 3.0
else:
    print("Negative cycle detected")
```

## Dijkstra's Algorithm

- Greedy algorithm for single-source shortest paths
- Requires non-negative weights
- Optimal substructure property
- O(E log V) with binary heap

## Bellman-Ford Algorithm

- Handles negative weights
- Can detect negative cycles
- Slower than Dijkstra
- O(V·E) time complexity

## Floyd-Warshall Algorithm

- Computes all-pairs shortest paths
- Handles negative weights (no negative cycles)
- Useful for small graphs
- O(V³) time and space

## Applications

- Navigation and GPS routing
- Network routing protocols
- Airline scheduling
- Game pathfinding
- Social network analysis (shortest social distance)

## Edge Cases

- No path exists: Returns infinity or None
- Negative cycles: Bellman-Ford detects them
- Self-loops: Depend on weight (usually ignored if non-negative)
- Large graphs: Dijkstra preferred over Floyd-Warshall

## Complexity Comparison

For single-source shortest paths:

- Dijkstra: O(E log V) [best with binary heap]
- Bellman-Ford: O(V·E) [general but slower]
- BFS: O(V+E) [unweighted only]

For all-pairs:

- Floyd-Warshall: O(V³) [small graphs]
- Repeated Dijkstra: O(V·E log V) [sparse graphs]
