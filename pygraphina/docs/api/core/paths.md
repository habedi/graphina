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
edges = [(0,1,1), (0,2,4), (1,2,2), (1,3,5), (2,3,1), (3,4,3)]
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

- **No path exists**: Returns infinity or None
- **Negative cycles**: Bellman-Ford detects them
- **Self-loops**: Depend on weight (usually ignored if non-negative)
- **Large graphs**: Dijkstra preferred over Floyd-Warshall

## Complexity Comparison

For single-source shortest paths:

- Dijkstra: O(E log V) [best with binary heap]
- Bellman-Ford: O(V·E) [general but slower]
- BFS: O(V+E) [unweighted only]

For all-pairs:

- Floyd-Warshall: O(V³) [small graphs]
- Repeated Dijkstra: O(V·E log V) [sparse graphs]
