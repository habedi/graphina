# Ramsey R(2,2) Approximation

Find a clique or independent set using Ramsey theory.

## Function Signature

```python
pg.approximation.ramsey_r2(
    graph: PyGraph
) -> Tuple[List[int], List[int]]
```

## Parameters

- **graph**: Input undirected graph

## Returns

A tuple `(clique, independent_set)` containing:
- **clique**: List of node IDs forming a clique (all pairs connected)
- **independent_set**: List of node IDs forming an independent set (no pairs connected)

## Description

Based on Ramsey theory, this algorithm guarantees to find either a clique or an independent set of size at least
⌈log₂(n)/2⌉ in any graph with n vertices. This is a constructive proof of Ramsey's theorem for R(2,2).

### Ramsey Theory Background

Ramsey's theorem states that for any graph, you can always find either:
- A clique of a certain size (all nodes connected to each other), or
- An independent set of a certain size (no nodes connected to each other)

The R(2,2) case is the simplest non-trivial instance of this theorem.

### Algorithm

The algorithm uses a greedy approach:

1. Start with an arbitrary vertex
2. Partition remaining vertices into neighbors and non-neighbors
3. Recursively apply to the larger partition
4. Build either a clique or independent set based on the partitioning

## Time Complexity

**Time:** O(V²)  
**Space:** O(V)

## Example

### Basic Usage

```python
import pygraphina as pg

# Create a random graph
g = pg.core.erdos_renyi(50, 0.5, 42)

# Find clique or independent set
clique, independent_set = pg.approximation.ramsey_r2(g)

print(f"Found clique of size {len(clique)}")
print(f"Found independent set of size {len(independent_set)}")

# Verify clique (all pairs should be connected)
if len(clique) > 1:
    edges_in_clique = 0
    for i, u in enumerate(clique):
        for v in clique[i+1:]:
            if g.contains_edge(u, v):
                edges_in_clique += 1
    expected = len(clique) * (len(clique) - 1) // 2
    print(f"Clique verification: {edges_in_clique}/{expected} edges present")

# Verify independent set (no pairs should be connected)
if len(independent_set) > 1:
    edges_in_indset = 0
    for i, u in enumerate(independent_set):
        for v in independent_set[i+1:]:
            if g.contains_edge(u, v):
                edges_in_indset += 1
    print(f"Independent set verification: {edges_in_indset} edges (should be 0)")
```

### Analyzing Different Graph Types

```python
import pygraphina as pg

# Test on different graph types
graphs = {
    "Dense": pg.core.erdos_renyi(100, 0.8, 42),
    "Sparse": pg.core.erdos_renyi(100, 0.2, 42),
    "Scale-free": pg.core.barabasi_albert(100, 3, 42),
    "Complete": pg.core.complete_graph(50)
}

for name, g in graphs.items():
    clique, indset = pg.approximation.ramsey_r2(g)
    print(f"{name:12s}: clique={len(clique):3d}, indset={len(indset):3d}")
```

Expected output patterns:
- Dense graphs: Larger cliques, smaller independent sets
- Sparse graphs: Smaller cliques, larger independent sets
- Complete graphs: Maximum clique, single-node independent set

## Use Cases

- **Graph Coloring**: Finding independent sets helps with vertex coloring
- **Social Network Analysis**: Identifying groups that are fully connected or completely disconnected
- **Theoretical Computer Science**: Constructive proofs and algorithm design
- **Network Robustness**: Finding dense or sparse regions
- **Bioinformatics**: Identifying protein complexes (cliques) or non-interacting groups

## Theoretical Guarantees

For any graph with n vertices, this algorithm guarantees to find either:
- A clique of size at least ⌈log₂(n)/2⌉, OR
- An independent set of size at least ⌈log₂(n)/2⌉

This means at least one of the returned sets will have logarithmic size relative to the graph.

## Comparison with Related Algorithms

| Algorithm         | Time Complexity | Guarantee         | Output              |
|-------------------|-----------------|-------------------|---------------------|
| Ramsey R(2,2)     | O(V²)           | log(n)/2 size     | Clique OR indset    |
| Max Clique        | Exponential     | Optimal clique    | Clique only         |
| Max Independent   | Exponential     | Optimal indset    | Independent set only|
| Large Clique Size | O(V²)           | Approximation     | Clique only         |

## Practical Considerations

- The algorithm always succeeds (never fails to find a solution)
- One of the two sets will typically be much larger than the other
- Dense graphs tend to have larger cliques
- Sparse graphs tend to have larger independent sets
- The algorithm is deterministic (same graph always produces same result)

## Example: Finding Coherent vs. Diverse Groups

```python
import pygraphina as pg

# Model a social network where edges represent conflict
conflict_network = pg.core.erdos_renyi(80, 0.3, 42)

clique, independent = pg.approximation.ramsey_r2(conflict_network)

print("Network Analysis:")
print(f"  Conflicted group (all in conflict): {len(clique)} people")
print(f"  Harmonious group (no conflicts): {len(independent)} people")

if len(independent) > len(clique):
    print(f"\nThis network has a large harmonious group of {len(independent)} people")
    print("They can work together without conflicts!")
else:
    print(f"\nThis network is highly conflicted - {len(clique)} people")
    print("are all in conflict with each other")
```

## References

- Ramsey, F. P. (1930). "On a Problem of Formal Logic"
- Erdős, P., & Szekeres, G. (1935). "A combinatorial problem in geometry"

## See Also

- [Max Clique](clique.md) - Finding maximum cliques
- [Independent Set](independent_set.md) - Finding maximum independent sets
- [Vertex Cover](vertex_cover.md) - Complement of independent set
