# Algorithm Complexity Reference

This document provides a comprehensive reference of time and space complexity for all algorithms in the Graphina library.

**Notation:**
- `V` = number of vertices (nodes)
- `E` = number of edges
- `d` = diameter of the graph
- `k` = specific parameter (varies by algorithm)
- `b` = branching factor
- `m` = number of edges to attach (Barabási-Albert)

---

## Table of Contents

1. [Graph Generators](#graph-generators)
2. [Graph Traversal](#graph-traversal)
3. [Shortest Paths](#shortest-paths)
4. [Graph Operations](#graph-operations)
5. [MST Algorithms](#mst-algorithms)

---

## Graph Generators

### Erdős-Rényi Graph
**Function:** `erdos_renyi_graph(n, p, seed)`

**Time Complexity:**
- **Directed:** O(V²) - checks all possible V² pairs
- **Undirected:** O(V²) - checks V(V-1)/2 pairs

**Space Complexity:** O(V + E)

**Best Case:** O(V) when p ≈ 0 (sparse graph)
**Worst Case:** O(V²) when p = 1 (complete graph)

**Notes:**
- Each edge is considered with probability `p`
- Time is deterministic regardless of actual edges created
- Memory usage scales with edges created: O(V + pV²)

---

### Complete Graph
**Function:** `complete_graph(n)`

**Time Complexity:**
- **Directed:** O(V²) - creates V(V-1) edges
- **Undirected:** O(V²) - creates V(V-1)/2 edges

**Space Complexity:** O(V²)

**Notes:**
- Always creates maximum possible edges
- Memory usage dominated by edge storage

---

### Barabási-Albert Graph
**Function:** `barabasi_albert_graph(n, m, seed)`

**Time Complexity:** O(V × m × attempts)
- Worst case: O(V × m²) with bounded attempts
- Each new node attaches to `m` existing nodes
- Preferential attachment requires O(m) selections per node

**Space Complexity:** O(V + E) where E ≈ V×m

**Notes:**
- Implements bounded retry (max_attempts = n × 10)
- Falls back to deterministic selection if needed
- Creates scale-free network with power-law degree distribution

**Optimization:** Could be improved to O(V×m) with better data structures

---

### Watts-Strogatz Graph
**Function:** `watts_strogatz_graph(n, k, beta, seed)`

**Time Complexity:** O(V × k + V × k × beta × attempts)
- Ring lattice creation: O(V×k)
- Rewiring phase: O(V×k×beta×attempts)
- With bounded attempts: O(V×k)

**Space Complexity:** O(V + E) where E ≈ V×k

**Notes:**
- Creates small-world network
- `beta` controls rewiring probability (0=regular, 1=random)
- Bounded attempts prevent infinite loops

---

### Bipartite Graph
**Function:** `bipartite_graph(n1, n2, p, seed)`

**Time Complexity:** O(n1 × n2)
- Checks all possible n1×n2 edges between partitions

**Space Complexity:** O(V + E) where V = n1+n2, E ≤ n1×n2

---

### Star Graph
**Function:** `star_graph(n)`

**Time Complexity:** O(V)
- Creates one center node and V-1 edges

**Space Complexity:** O(V)

---

### Cycle Graph
**Function:** `cycle_graph(n)`

**Time Complexity:** O(V)
- Creates V nodes and V edges in a ring

**Space Complexity:** O(V)

---

## Graph Traversal

### Breadth-First Search (BFS)
**Function:** `bfs(graph, start)`

**Time Complexity:** O(V + E)
- Visits each vertex once: O(V)
- Examines each edge once: O(E)

**Space Complexity:** O(V)
- Queue storage: O(V) in worst case
- Visited set: O(V)

**Best Case:** O(1) if start node is isolated
**Worst Case:** O(V + E) for connected graph

**Notes:**
- Returns nodes in level-order from start
- Optimal for finding shortest path in unweighted graphs
- Can be parallelized for very large graphs

---

### Depth-First Search (DFS)
**Function:** `dfs(graph, start)`

**Time Complexity:** O(V + E)
- Visits each vertex once: O(V)
- Examines each edge once: O(E)

**Space Complexity:** O(V)
- Recursion stack: O(V) in worst case (long path)
- Visited set: O(V)

**Best Case:** O(1) if start node is isolated
**Worst Case:** O(V + E) for connected graph

**Notes:**
- Returns nodes in depth-first order
- Recursion depth can be V in worst case (linear graph)
- Stack-based implementation can reduce memory

---

### Iterative Deepening DFS (IDDFS)
**Function:** `iddfs(graph, start, target, max_depth)`

**Time Complexity:** O(b^d)
- Where `b` is branching factor, `d` is solution depth
- Worst case: O(b^max_depth)

**Space Complexity:** O(d)
- Only stores current path
- Much better than BFS for deep searches

**Best Case:** O(b) if target at depth 1
**Worst Case:** O(b^d) where d = max_depth

**Notes:**
- Combines space efficiency of DFS with optimality of BFS
- Asymptotically optimal for exponential search spaces
- Ideal when solution depth unknown

---

### Bidirectional Search
**Function:** `bidis(graph, start, target)`

**Time Complexity:** O(b^(d/2))
- Average case: O(b^(d/2)) where d is distance
- Worst case: O(V + E) if no path or poor meeting point

**Space Complexity:** O(b^(d/2))
- Stores two frontiers
- Approximately √(space of BFS)

**Best Case:** O(1) if start == target
**Typical:** O(b^(d/2)) - square root of BFS time

**Notes:**
- Significant speedup for long paths
- Requires ability to traverse backwards (predecessor lookup)
- Fixed bug: now properly tracks frontiers for correct shortest paths

**Performance Note:** For directed graphs, backward neighbor lookup is O(E) per call, making overall complexity O(E×V) in worst case. Adding reverse edge index would restore O(V+E) complexity.

---

## Shortest Paths

### Dijkstra's Algorithm
**Function:** `dijkstra(graph, start)`

**Time Complexity:** O((V + E) log V)
- With binary heap priority queue
- V extractions from heap: O(V log V)
- E edge relaxations: O(E log V)

**Space Complexity:** O(V)
- Distance array: O(V)
- Priority queue: O(V)
- Parent tracking: O(V)

**Best Case:** O(V log V) for sparse graph (E ≈ V)
**Worst Case:** O(E log V) for dense graph (E ≈ V²)

**Notes:**
- Returns shortest distances to all reachable nodes
- Requires non-negative edge weights
- Can be optimized to O(V log V + E) with Fibonacci heap
- Does not work with negative weights (use Bellman-Ford)

---

## Graph Operations

### Add Node
**Function:** `add_node(attr)`

**Time Complexity:** O(1) amortized
- StableGraph uses internal indexing

**Space Complexity:** O(1)

---

### Add Edge
**Function:** `add_edge(source, target, weight)`

**Time Complexity:** O(1) amortized
- Direct insertion into adjacency structure

**Space Complexity:** O(1)

---

### Remove Node
**Function:** `remove_node(node)`

**Time Complexity:** O(degree(node))
- Must remove all incident edges
- Worst case O(V) for complete graph

**Space Complexity:** O(1)

**Notes:**
- StableGraph marks node as removed without reindexing
- Preserves existing node IDs

---

### Remove Edge
**Function:** `remove_edge(edge)`

**Time Complexity:** O(1)
- Direct removal by edge ID

**Space Complexity:** O(1)

---

### Find Edge
**Function:** `find_edge(source, target)`

**Time Complexity:** O(E) worst case
- Currently iterates through all edges
- Could be O(degree(source)) with better implementation

**Space Complexity:** O(1)

**Optimization Opportunity:** Use adjacency list lookup for O(degree(source))

---

### Node Degree
**Function:** `degree(node)` / `in_degree(node)` / `out_degree(node)`

**Time Complexity:**
- **Undirected:** O(degree(node))
- **Directed (out-degree):** O(degree(node))
- **Directed (in-degree):** O(E) - currently inefficient

**Space Complexity:** O(1)

**Notes:**
- In-degree for directed graphs requires full edge scan
- Could be O(1) with reverse edge index

---

### Graph Density
**Function:** `density()`

**Time Complexity:** O(1)
- Uses cached node_count and edge_count

**Space Complexity:** O(1)

**Formula:**
- Directed: E / (V × (V-1))
- Undirected: 2E / (V × (V-1))

---

### Contains Node/Edge
**Function:** `contains_node(node)` / `contains_edge(source, target)`

**Time Complexity:**
- `contains_node`: O(1)
- `contains_edge`: O(E) worst case

**Space Complexity:** O(1)

---

### Neighbors Iterator
**Function:** `neighbors(node)`

**Time Complexity:** O(degree(node)) to iterate all
**Space Complexity:** O(1) for iterator itself

---

### Clear Graph
**Function:** `clear()`

**Time Complexity:** O(V + E)
- Must deallocate all nodes and edges

**Space Complexity:** O(1)

---

## MST Algorithms

### Kruskal's MST
**Function:** `kruskal_mst(graph)`

**Time Complexity:** O(E log E)
- Sort edges: O(E log E)
- Union-find operations: O(E α(V)) ≈ O(E)
- Dominated by sorting

**Space Complexity:** O(V + E)
- Union-find structure: O(V)
- Edge list: O(E)

**Notes:**
- Optimal for sparse graphs
- Produces minimum spanning tree
- Works on undirected, connected graphs

---

### Prim's MST
**Function:** `prim_mst(graph, start)`

**Time Complexity:** O((V + E) log V)
- Similar to Dijkstra's algorithm
- With binary heap: O(E log V)

**Space Complexity:** O(V)
- Priority queue: O(V)
- MST edge list: O(V)

**Notes:**
- Optimal for dense graphs
- Can start from any node
- Similar structure to Dijkstra's

---

## Validation Utilities

### Is Connected
**Function:** `is_connected(graph)`

**Time Complexity:** O(V + E)
- Runs BFS from arbitrary node
- Checks if all nodes visited

**Space Complexity:** O(V)

---

### Has Negative Weights
**Function:** `has_negative_weights(graph)`

**Time Complexity:** O(E)
- Iterates through all edges once

**Space Complexity:** O(1)

---

## Performance Comparison Table

| Algorithm | Best Case | Average Case | Worst Case | Space |
|-----------|-----------|--------------|------------|-------|
| **BFS** | O(1) | O(V+E) | O(V+E) | O(V) |
| **DFS** | O(1) | O(V+E) | O(V+E) | O(V) |
| **IDDFS** | O(b) | O(b^d) | O(b^d) | O(d) |
| **Bidirectional** | O(1) | O(b^(d/2)) | O(V+E) | O(b^(d/2)) |
| **Dijkstra** | O(V log V) | O((V+E) log V) | O(E log V) | O(V) |
| **Erdős-Rényi** | O(V) | O(V²) | O(V²) | O(V+E) |
| **Barabási-Albert** | O(V×m) | O(V×m²) | O(V×m²) | O(V×m) |
| **Kruskal MST** | O(E log E) | O(E log E) | O(E log E) | O(V+E) |
| **Prim MST** | O(E log V) | O(E log V) | O(E log V) | O(V) |

---

## Optimization Priorities

### High Impact
1. **Reverse Edge Index for Directed Graphs**
   - Current: O(E) backward neighbor lookup
   - With index: O(degree(node))
   - Affects: Bidirectional search, in-degree calculation

2. **Adjacency List for find_edge**
   - Current: O(E)
   - With proper structure: O(degree(source))

### Medium Impact
3. **Parallel BFS/DFS** using Rayon
   - Good for very large graphs (V > 10,000)
   - Expected speedup: 2-4x on multi-core systems

4. **Fibonacci Heap for Dijkstra**
   - O((V+E) log V) → O(V log V + E)
   - Complex implementation, marginal benefit for most graphs

### Low Impact
5. **Edge List Caching**
   - Trade memory for speed in repeated queries
   - Useful for read-heavy workloads

---

## Usage Guidelines

### When to Use Each Algorithm

**BFS:**
- Shortest path in unweighted graphs
- Level-order traversal needed
- Exploring graph layer by layer

**DFS:**
- Detecting cycles
- Topological sorting
- Connected components
- When path doesn't need to be shortest

**Bidirectional Search:**
- Long shortest paths in large graphs
- When both forward and backward traversal possible
- Significant speedup over BFS for distant nodes

**Dijkstra:**
- Weighted shortest paths
- Non-negative weights
- Single-source to all destinations

**IDDFS:**
- Memory-constrained environments
- Unknown solution depth
- Combines DFS memory efficiency with BFS optimality

---

## References

1. Cormen, T. H., et al. (2009). *Introduction to Algorithms* (3rd ed.)
2. Sedgewick, R., & Wayne, K. (2011). *Algorithms* (4th ed.)
3. Barabási, A. L., & Albert, R. (1999). *Emergence of scaling in random networks*
4. Watts, D. J., & Strogatz, S. H. (1998). *Collective dynamics of small-world networks*

---

**Last Updated:** October 18, 2025  
**Library Version:** 0.4.0
