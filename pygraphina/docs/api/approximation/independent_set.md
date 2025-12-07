# Independent Set Approximation

An independent set is a subset of nodes with no edges between them. Maximum independent set is NP-hard.

## Overview

Finding maximum independent sets is computationally hard, but approximation algorithms provide good solutions quickly.
PyGraphina provides a greedy approximation algorithm that produces reasonable results for most graphs.

## Function Signature

```python
pg.approximation.maximum_independent_set(graph: PyGraph) -> List[int]
```

## Parameters

- **graph**: The undirected graph to analyze

## Returns

List of node IDs forming an independent set (no edges between any two nodes in the set).

## Algorithm

Uses a greedy approximation:
1. Start with empty set
2. Repeatedly add the vertex with minimum degree (not connected to current set)
3. Continue until no vertices can be added

**Time Complexity**: O(V² + E)  
**Space Complexity**: O(V)  
**Approximation Ratio**: O(log V) in worst case, often much better in practice

## Example

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Add edges - creating some conflicts
edges = [
    (0,1), (1,2), (3,4), (4,5), (6,7), (7,8),
    (0,3), (2,5), (5,8)
]
for u, v in edges:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Find maximum independent set (approximation)
indep_set = pg.approximation.maximum_independent_set(g)

print(f"Independent set size: {len(indep_set)}")
print(f"Nodes in independent set: {indep_set}")

# Verify no edges within the set
has_edges = False
for i in range(len(indep_set)):
    for j in range(i+1, len(indep_set)):
        if g.contains_edge(indep_set[i], indep_set[j]):
            has_edges = True
            break

print(f"Valid independent set (no internal edges): {not has_edges}")

# Use case: task scheduling
# Each node = task, edge = dependency/conflict
# Find maximum tasks that can run in parallel
tasks = list(range(10))
conflicts = edges

g_tasks = pg.PyGraph()
task_nodes = [g_tasks.add_node(t) for t in tasks]

for u, v in conflicts:
    g_tasks.add_edge(task_nodes[u], task_nodes[v], 1.0)

parallel_tasks = pg.approximation.maximum_independent_set(g_tasks)
print(f"\nTasks that can run in parallel: {parallel_tasks}")
print(f"Maximum parallelism level: {len(parallel_tasks)}")
```

## Use Cases

- **Task Scheduling**: Find non-conflicting tasks that can run in parallel
- **Register Allocation**: Allocate registers to variables with non-overlapping lifetimes
- **Frequency Assignment**: Assign frequencies to stations without interference
- **Set Packing**: Maximize selection of non-overlapping sets
- **Maximal Matching**: Lower bound for maximum matching
- **Graph Coloring**: Related to chromatic number

## Quality Assessment

The greedy algorithm produces:
- **Optimal solution**: For trees and bipartite graphs
- **Good approximation**: For sparse graphs
- **Reasonable bound**: For dense graphs (always within O(log n) of optimal)

## Complexity Analysis

| Aspect | Value |
|--------|-------|
| Time | O(V² + E) |
| Space | O(V) |
| Approximation Ratio | O(log V) worst case |
| Practical Performance | Often much better than worst case |

## Advantages

- Fast computation
- Simple greedy approach
- Works for any graph
- Guaranteed quality bound
- Memory efficient

## Disadvantages

- Not always optimal (NP-hard problem)
- Greedy choices can be suboptimal
- Performance varies by graph structure
- No way to verify optimality easily

## Relationship to Other Problems

- **Vertex Cover**: Complement of Independent Set
- **Maximum Independent Set**: |IS| + |VC| = |V| (for bipartite graphs)
- **Clique**: Maximum clique in complement graph
- **Coloring**: Independent set partitions for proper coloring

## Tips for Better Results

1. Preprocess graph to remove degree-1 vertices
2. Use kernelization techniques
3. Compare with other approximation heuristics
4. For small graphs (< 20 nodes), try multiple approaches
5. Consider graph structure - exploits sparse regions better
