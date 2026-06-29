# Clique Approximation

A clique is a complete subgraph where every pair of nodes is connected. Finding maximum cliques is NP-hard, so
PyGraphina provides approximation algorithms.

## Function Signatures

```python
pg.approximation.max_clique(graph: PyGraph) -> List[int]
pg.approximation.large_clique_size(graph: PyGraph) -> int
pg.approximation.clique_removal(graph: PyGraph) -> List[List[int]]
```

## Parameters

- graph: The graph to analyze

## Returns

- `max_clique()`: List of node IDs in the clique
- `large_clique_size()`: Size of largest found clique
- `clique_removal()`: List of cliques found iteratively

## Description

### Maximum Clique (Approximation)

Finds a large clique using a greedy approach. May not find the true maximum clique but finds good solutions quickly.

### Clique Removal

Iteratively finds and removes cliques from the graph, revealing community structure.

## Time Complexity

- `max_clique()`: O(V² + E)
- `clique_removal()`: O(iterations · (V² + E))

## Example

```python
import pygraphina as pg

# Create a graph with a known clique
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Create a clique of size 5 (nodes 0-4)
for i in range(5):
    for j in range(i + 1, 5):
        g.add_edge(nodes[i], nodes[j], 1.0)

# Add some edges from clique to other nodes
for i in range(5, 10):
    g.add_edge(nodes[0], nodes[i], 1.0)

# Find clique
clique = pg.approximation.max_clique(g)
print(f"Found clique of size {len(clique)}: {sorted(clique)}")

# Get clique size directly
size = pg.approximation.large_clique_size(g)
print(f"Largest clique size: {size}")

# Find all cliques using removal
cliques = pg.approximation.clique_removal(g)
print(f"Found {len(cliques)} cliques via removal")
```

## Use Cases

- Community structure discovery
- Dense subgraph identification
- Analyzing highly interconnected groups
- Preprocessing for other algorithms

## Notes

Results are approximations, not guaranteed to be maximum cliques.
