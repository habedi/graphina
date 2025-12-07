# PyDiGraph API Reference

The `PyDiGraph` class is the main directed graph data structure in PyGraphina.

## Class: PyDiGraph

```python
pg.PyDiGraph()
```

A directed graph with integer node attributes and floating-point edge weights.

### Creating a Graph

```python
import pygraphina as pg

# Create an empty directed graph
dg = pg.PyDiGraph()
```

## Differences from PyGraph

`PyDiGraph` has all the same methods as `PyGraph`, but with directed graph semantics:

- **Edges are directional**: An edge from A to B is different from an edge from B to A
- **Degree operations**: Supports `in_degree()` and `out_degree()` in addition to `degree()`
- **Neighbors**: `neighbors()` returns outgoing neighbors, `predecessors()` returns incoming neighbors

## Additional Methods for Directed Graphs

### in_degree

```python
in_degree(node: int) -> Optional[int]
```

Get the in-degree (number of incoming edges) of a node.

**Parameters:**

- `node` (int): The node ID

**Returns:**

- `Optional[int]`: The in-degree if the node exists, None otherwise

**Example:**

```python
dg = pg.PyDiGraph()
a, b, c = [dg.add_node(i) for i in range(3)]
dg.add_edge(a, b, 1.0)  # a → b
dg.add_edge(c, b, 1.0)  # c → b

print(dg.in_degree(b))   # 2 (edges from a and c)
print(dg.in_degree(a))   # 0 (no incoming edges)
```

### out_degree

```python
out_degree(node: int) -> Optional[int]
```

Get the out-degree (number of outgoing edges) of a node.

**Parameters:**

- `node` (int): The node ID

**Returns:**

- `Optional[int]`: The out-degree if the node exists, None otherwise

**Example:**

```python
dg = pg.PyDiGraph()
a, b, c = [dg.add_node(i) for i in range(3)]
dg.add_edge(a, b, 1.0)  # a → b
dg.add_edge(a, c, 1.0)  # a → c

print(dg.out_degree(a))  # 2 (edges to b and c)
print(dg.out_degree(b))  # 0 (no outgoing edges)
```

### predecessors

```python
predecessors(node: int) -> List[int]
```

Get the predecessor nodes (nodes with edges pointing to this node).

**Parameters:**

- `node` (int): The node ID

**Returns:**

- `List[int]`: List of predecessor node IDs

**Raises:**

- `ValueError`: If the node doesn't exist

**Example:**

```python
dg = pg.PyDiGraph()
a, b, c = [dg.add_node(i) for i in range(3)]
dg.add_edge(a, c, 1.0)  # a → c
dg.add_edge(b, c, 1.0)  # b → c

print(dg.predecessors(c))  # [0, 1] (a and b)
```

### successors

```python
successors(node: int) -> List[int]
```

Get the successor nodes (nodes that this node has edges pointing to). Equivalent to `neighbors()` for directed graphs.

**Parameters:**

- `node` (int): The node ID

**Returns:**

- `List[int]`: List of successor node IDs

**Raises:**

- `ValueError`: If the node doesn't exist

**Example:**

```python
dg = pg.PyDiGraph()
a, b, c = [dg.add_node(i) for i in range(3)]
dg.add_edge(a, b, 1.0)  # a → b
dg.add_edge(a, c, 1.0)  # a → c

print(dg.successors(a))  # [1, 2] (b and c)
```

## Inherited Methods

All methods from `PyGraph` are available with the same signatures:

- `add_node()`, `remove_node()`, `update_node()`
- `add_edge()`, `remove_edge()`, `contains_edge()`
- `nodes()`, `node_count()`, `edge_count()`
- `is_directed()` (returns True for PyDiGraph)
- `density()`, `degree()`, `neighbors()`
- `clear()`

See the [PyGraph documentation](graph.md) for details.

## Complete Example

```python
import pygraphina as pg

# Create a directed graph representing a workflow
workflow = pg.PyDiGraph()

# Add tasks (nodes)
start = workflow.add_node(1)
fetch_data = workflow.add_node(2)
process = workflow.add_node(3)
validate = workflow.add_node(4)
finish = workflow.add_node(5)

# Add dependencies (directed edges)
workflow.add_edge(start, fetch_data, 1.0)
workflow.add_edge(fetch_data, process, 1.0)
workflow.add_edge(process, validate, 1.0)
workflow.add_edge(validate, finish, 1.0)
workflow.add_edge(fetch_data, validate, 0.5)  # Optional skip

# Analyze the workflow
print(f"Total tasks: {workflow.node_count()}")
print(f"Dependencies: {workflow.edge_count()}")

# Check task dependencies
print(f"Process depends on: {workflow.predecessors(process)}")
print(f"Fetch data leads to: {workflow.successors(fetch_data)}")

# Find tasks with no dependencies (starting points)
start_tasks = [n for n in workflow.nodes() if workflow.in_degree(n) == 0]
print(f"Start tasks: {start_tasks}")

# Find tasks with no dependents (end points)
end_tasks = [n for n in workflow.nodes() if workflow.out_degree(n) == 0]
print(f"End tasks: {end_tasks}")
```

## Directed vs Undirected: When to Use

### Use PyDiGraph when:

- Relationships have direction (like "follows", "depends on", "links to")
- Order matters (like workflows, dependencies, hierarchies)
- Asymmetric relationships (A relates to B doesn't mean B relates to A)

**Examples:**

- Web page links
- Citation networks
- Social media follows
- Task dependencies
- Food chains

### Use PyGraph when:

- Relationships are symmetric (like "is friends with", "is connected to")
- Order doesn't matter
- Bidirectional connections

**Examples:**

- Friendships
- Road networks (bidirectional roads)
- Molecular structures
- Collaboration networks

## Converting Between Graph Types

There's no direct conversion method, but you can manually create one type from another:

```python
# Convert undirected to directed (create both directions)
undirected = pg.PyGraph()
a, b = undirected.add_node(1), undirected.add_node(2)
undirected.add_edge(a, b, 1.0)

directed = pg.PyDiGraph()
for node in undirected.nodes():
    attr = undirected.get_node_attr(node)
    directed.add_node(attr)

# For undirected edges, add both directions
# (This is conceptual - actual edge iteration not shown)
directed.add_edge(a, b, 1.0)
directed.add_edge(b, a, 1.0)
```

## See Also

- [PyGraph](graph.md): Undirected graph API
- [Core Operations](core/builders.md): Graph builders
- [Algorithms](centrality/index.md): Graph algorithms
