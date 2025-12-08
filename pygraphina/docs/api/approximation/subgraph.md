# Densest Subgraph Approximation

Find an approximately densest subgraph using iterative vertex removal.

## Function Signature

```python
pg.approximation.densest_subgraph(
    graph: PyGraph,
    iterations: Optional[int] = None
) -> List[int]
```

## Parameters

- **graph**: Input undirected graph
- **iterations**: Maximum number of iterations (optional, defaults to automatic convergence)

## Returns

List of node IDs forming the approximately densest subgraph.

## Description

The densest subgraph problem seeks the subgraph with maximum edge density, defined as the ratio of edges to nodes.
This is an approximation algorithm that iteratively removes low-degree vertices to find a dense core.

The algorithm works by:

1. Starting with the full graph
2. Iteratively removing the vertex with the smallest degree
3. Tracking the density at each step
4. Returning the subgraph with the highest density encountered

## Time Complexity

**Time:** O(k·(V + E)) where k is the number of iterations (typically O(V))  
**Space:** O(V)

## Example

### Basic Usage

```python
import pygraphina as pg

# Create a graph with dense regions
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(20)]

# Create a dense core (nodes 0-5)
for i in range(6):
    for j in range(i+1, 6):
        g.add_edge(nodes[i], nodes[j], 1.0)

# Add sparse connections to other nodes
for i in range(6, 20):
    g.add_edge(nodes[0], nodes[i], 1.0)

# Find densest subgraph
dense_nodes = pg.approximation.densest_subgraph(g)

print(f"Densest subgraph has {len(dense_nodes)} nodes")
print(f"Nodes: {dense_nodes}")

# Calculate density
if len(dense_nodes) > 1:
    subg = g.subgraph(dense_nodes)
    density = subg.edge_count() / subg.node_count()
    print(f"Density: {density:.2f} edges per node")
```

### Real-World Example: Finding Tightly-Connected Communities

```python
import pygraphina as pg

# Create a social network
g = pg.core.barabasi_albert(200, 5, 42)

# Find the densest subgraph (most tightly connected group)
core = pg.approximation.densest_subgraph(g, iterations=100)

print(f"Found dense core with {len(core)} members")

# Analyze the core
core_graph = g.subgraph(core)
avg_degree = sum(core_graph.degree(n) for n in core_graph.nodes) / len(core)
print(f"Average degree in core: {avg_degree:.2f}")
print(f"Core density: {core_graph.density():.3f}")
```

## Use Cases

- **Social Network Analysis**: Finding tightly-knit communities
- **Biological Networks**: Identifying functional modules in protein interaction networks
- **Web Graphs**: Finding link farms or densely connected web communities
- **Recommendation Systems**: Identifying groups with similar preferences
- **Network Core Identification**: Finding the most important connected subgroup

## Approximation Quality

This algorithm provides a 2-approximation for the densest subgraph problem, meaning the density of the returned
subgraph is at least half the density of the optimal solution.

## Comparison with Related Algorithms

| Algorithm           | Time Complexity | Approximation Ratio | Use Case                |
|---------------------|-----------------|---------------------|-------------------------|
| Densest Subgraph    | O(V²)           | 2-approx            | Dense core discovery    |
| Max Clique          | NP-hard         | Varies              | Fully connected groups  |
| K-Core Decomposition| O(V + E)        | Exact               | Core hierarchy          |

## Notes

- Works best on graphs with clear dense regions
- The `iterations` parameter can be used to limit computation time for very large graphs
- Returns nodes in the subgraph; use `graph.subgraph()` to extract the actual subgraph

## References

- Charikar, M. (2000). "Greedy approximation algorithms for finding dense components in a graph"
- Goldberg, A. V. (1984). "Finding a maximum density subgraph"

## See Also

- [Max Clique](clique.md) - Finding fully connected subgraphs
- [Connected Components](../community/connected_components.md) - Finding disconnected regions
- [Community Detection](../community/index.md) - Finding loosely connected groups
