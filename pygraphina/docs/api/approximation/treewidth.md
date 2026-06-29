# Treewidth Approximation

Treewidth is a measure of how similar a graph is to a tree. It has many algorithmic applications.

## Overview

Treewidth measures how close a graph is to being a tree. Computing exact treewidth is NP-hard, but approximation
algorithms exist. PyGraphina provides two heuristic-based approximation algorithms: min-degree and min-fill-in.

## Function Signatures

```python
pg.approximation.treewidth_min_degree(graph: PyGraph) -> Tuple[int, List[int]]
pg.approximation.treewidth_min_fill_in(graph: PyGraph) -> Tuple[int, List[int]]
```

## Parameters

- graph: The undirected graph to analyze

## Returns

A tuple containing:
- int: Upper bound on treewidth
- List[int]: Elimination ordering of nodes

## Algorithms

### Min-Degree Heuristic
Iteratively removes the node with minimum degree. Fast but can produce suboptimal results.

Time Complexity: O(n³) with simple implementation  
Quality: Often reasonable, sometimes suboptimal

### Min-Fill-In Heuristic
Removes the node that creates the fewest new edges when eliminated. Slower but often produces better bounds.

Time Complexity: O(n⁴) with simple implementation  
Quality: Generally better than min-degree

## Time Complexity

- Min-Degree: O(n³)
- Min-Fill-In: O(n⁴)

Where n is the number of nodes.

## Use Cases

- Dynamic programming on graphs
- SAT/constraint satisfaction solving
- Database query optimization
- Graph coloring approximation
- Temporal reasoning in AI
- Network design and analysis

## Example

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Tree-like structure
for i in range(9):
    g.add_edge(nodes[i], nodes[i + 1], 1.0)

# Add some edges to increase treewidth
g.add_edge(nodes[0], nodes[2], 1.0)
g.add_edge(nodes[0], nodes[3], 1.0)
g.add_edge(nodes[1], nodes[3], 1.0)

# Compare algorithms
width_md, order_md = pg.approximation.treewidth_min_degree(g)
width_mfi, order_mfi = pg.approximation.treewidth_min_fill_in(g)

print(f"Min-Degree Treewidth: {width_md}")
print(f"Min-Fill-In Treewidth: {width_mfi}")
print(f"Elimination order (min-degree): {order_md}")
print(f"Elimination order (min-fill-in): {order_mfi}")

# A tree has treewidth 1
# Adding more edges increases treewidth
tree = pg.PyGraph()
tree_nodes = [tree.add_node(i) for i in range(5)]
for i in range(4):
    tree.add_edge(tree_nodes[i], tree_nodes[i+1], 1.0)

tree_width, _ = pg.approximation.treewidth_min_degree(tree)
print(f"\nTreewidth of a path (tree): {tree_width}")
```

## Interpreting Results

- Treewidth = 1: Graph is a tree or forest
- Treewidth = 2: Graph is outerplanar
- Treewidth = k: Graph has tree decomposition of width k
- Lower is better: Enables faster algorithms

## Significance

Low treewidth enables:

- Efficient exact algorithms for NP-hard problems
- Decomposition into tree-structured subproblems
- Polynomial algorithms for many hard problems on graphs with bounded treewidth
- Better understanding of graph structure

## Algorithm Selection

- Use Min-Degree: When you need fast approximation, large graphs
- Use Min-Fill-In: When you need better bounds, smaller/medium graphs
- Compare both: For important applications to pick the better result

## Applications

| Application | Why Treewidth Matters |
|-------------|----------------------|
| SAT Solving | Low treewidth → faster solving |
| Database Queries | Optimization on tree decomposition |
| Probabilistic Inference | Belief propagation on trees |
| Scheduling | Decompose into subproblems |
| Network Analysis | Understand structure complexity |
