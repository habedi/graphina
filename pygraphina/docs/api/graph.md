# PyGraph API Reference

The `PyGraph` class is the main undirected graph data structure in PyGraphina.

## Class: PyGraph

```python
pg.PyGraph()
```

An undirected graph with integer node attributes and floating-point edge weights.

### Creating a Graph

```python
import pygraphina as pg

# Create an empty undirected graph
g = pg.PyGraph()
```

## Node Operations

### add_node

```python
add_node(attr: int) -> int
```

Add a node with an integer attribute to the graph.

**Parameters:**

- `attr` (int): The attribute value for the node

**Returns:**

- `int`: The node ID (a non-negative integer)

**Example:**

```python
g = pg.PyGraph()
node_a = g.add_node(100)  # Returns 0
node_b = g.add_node(200)  # Returns 1
```

### update_node

```python
update_node(node: int, new_attr: int) -> bool
```

Update the attribute of an existing node.

**Parameters:**

- `node` (int): The node ID
- `new_attr` (int): The new attribute value

**Returns:**

- `bool`: True if the node exists and was updated, False otherwise

**Example:**

```python
g = pg.PyGraph()
node = g.add_node(100)
success = g.update_node(node, 200)  # Returns True
```

### try_update_node

```python
try_update_node(node: int, new_attr: int) -> None
```

Update the attribute of an existing node. Raises an error if the node doesn't exist.

**Parameters:**

- `node` (int): The node ID
- `new_attr` (int): The new attribute value

**Raises:**

- `ValueError`: If the node doesn't exist

**Example:**

```python
g = pg.PyGraph()
node = g.add_node(100)
g.try_update_node(node, 200)  # Updates successfully
# g.try_update_node(999, 300)  # Would raise ValueError
```

### remove_node

```python
remove_node(node: int) -> Optional[int]
```

Remove a node from the graph. Also removes all edges incident to this node.

**Parameters:**

- `node` (int): The node ID to remove

**Returns:**

- `Optional[int]`: The node's attribute if it existed, None otherwise

**Example:**

```python
g = pg.PyGraph()
node = g.add_node(100)
attr = g.remove_node(node)  # Returns 100
```

### try_remove_node

```python
try_remove_node(node: int) -> int
```

Remove a node from the graph. Raises an error if the node doesn't exist.

**Parameters:**

- `node` (int): The node ID to remove

**Returns:**

- `int`: The node's attribute

**Raises:**

- `ValueError`: If the node doesn't exist

### get_node_attr

```python
get_node_attr(node: int) -> Optional[int]
```

Get the attribute of a node.

**Parameters:**

- `node` (int): The node ID

**Returns:**

- `Optional[int]`: The node's attribute if it exists, None otherwise

**Example:**

```python
g = pg.PyGraph()
node = g.add_node(100)
attr = g.get_node_attr(node)  # Returns 100
```

### contains_node

```python
contains_node(node: int) -> bool
```

Check if a node exists in the graph.

**Parameters:**

- `node` (int): The node ID

**Returns:**

- `bool`: True if the node exists, False otherwise

**Example:**

```python
g = pg.PyGraph()
node = g.add_node(100)
print(g.contains_node(node))  # True
print(g.contains_node(999))   # False
```

### nodes

```python
nodes() -> List[int]
```

Get a list of all node IDs in the graph.

**Returns:**

- `List[int]`: List of node IDs

**Example:**

```python
g = pg.PyGraph()
a, b, c = [g.add_node(i) for i in range(3)]
print(g.nodes())  # [0, 1, 2]
```

### node_count

```python
node_count() -> int
```

Get the number of nodes in the graph.

**Returns:**

- `int`: The number of nodes

**Example:**

```python
g = pg.PyGraph()
g.add_node(1)
g.add_node(2)
print(g.node_count())  # 2
```

## Edge Operations

### add_edge

```python
add_edge(source: int, target: int, weight: float) -> int
```

Add a weighted edge between two nodes.

**Parameters:**

- `source` (int): The source node ID
- `target` (int): The target node ID
- `weight` (float): The edge weight

**Returns:**

- `int`: Edge ID (implementation detail)

**Raises:**

- `ValueError`: If either node doesn't exist

**Example:**

```python
g = pg.PyGraph()
a, b = g.add_node(1), g.add_node(2)
g.add_edge(a, b, 2.5)
```

### remove_edge

```python
remove_edge(source: int, target: int) -> bool
```

Remove an edge from the graph.

**Parameters:**

- `source` (int): The source node ID
- `target` (int): The target node ID

**Returns:**

- `bool`: True if the edge existed and was removed, False otherwise

**Example:**

```python
g = pg.PyGraph()
a, b = g.add_node(1), g.add_node(2)
g.add_edge(a, b, 1.0)
removed = g.remove_edge(a, b)  # True
```

### contains_edge

```python
contains_edge(source: int, target: int) -> bool
```

Check if an edge exists between two nodes.

**Parameters:**

- `source` (int): The source node ID
- `target` (int): The target node ID

**Returns:**

- `bool`: True if the edge exists, False otherwise

**Example:**

```python
g = pg.PyGraph()
a, b = g.add_node(1), g.add_node(2)
g.add_edge(a, b, 1.0)
print(g.contains_edge(a, b))  # True
print(g.contains_edge(b, a))  # True (undirected)
```

### edge_count

```python
edge_count() -> int
```

Get the number of edges in the graph.

**Returns:**

- `int`: The number of edges

**Example:**

```python
g = pg.PyGraph()
a, b, c = [g.add_node(i) for i in range(3)]
g.add_edge(a, b, 1.0)
g.add_edge(b, c, 1.0)
print(g.edge_count())  # 2
```

## Graph Properties

### is_directed

```python
is_directed() -> bool
```

Check if the graph is directed.

**Returns:**

- `bool`: False for PyGraph (always undirected)

**Example:**

```python
g = pg.PyGraph()
print(g.is_directed())  # False
```

### density

```python
density() -> float
```

Calculate the graph density (ratio of existing edges to possible edges).

**Returns:**

- `float`: Graph density (between 0 and 1)

**Formula:**

- For undirected graphs: `2 * E / (V * (V - 1))`
- Where E is the number of edges and V is the number of nodes

**Example:**

```python
g = pg.PyGraph()
a, b, c = [g.add_node(i) for i in range(3)]
g.add_edge(a, b, 1.0)
print(g.density())  # 0.333...
```

## Neighborhood Operations

### neighbors

```python
neighbors(node: int) -> List[int]
```

Get the neighbors of a node.

**Parameters:**

- `node` (int): The node ID

**Returns:**

- `List[int]`: List of neighbor node IDs

**Raises:**

- `ValueError`: If the node doesn't exist

**Example:**

```python
g = pg.PyGraph()
a, b, c = [g.add_node(i) for i in range(3)]
g.add_edge(a, b, 1.0)
g.add_edge(a, c, 1.0)
print(g.neighbors(a))  # [1, 2]
```

### degree

```python
degree(node: int) -> Optional[int]
```

Get the degree (number of neighbors) of a node.

**Parameters:**

- `node` (int): The node ID

**Returns:**

- `Optional[int]`: The degree if the node exists, None otherwise

**Example:**

```python
g = pg.PyGraph()
a, b, c = [g.add_node(i) for i in range(3)]
g.add_edge(a, b, 1.0)
g.add_edge(a, c, 1.0)
print(g.degree(a))  # 2
```

## Utility Operations

### clear

```python
clear() -> None
```

Remove all nodes and edges from the graph.

**Example:**

```python
g = pg.PyGraph()
g.add_node(1)
g.add_node(2)
g.clear()
print(g.node_count())  # 0
```

## Complete Example

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()

# Add nodes
alice = g.add_node(1)
bob = g.add_node(2)
charlie = g.add_node(3)

# Add edges
g.add_edge(alice, bob, 1.0)
g.add_edge(bob, charlie, 2.0)
g.add_edge(charlie, alice, 1.5)

# Query the graph
print(f"Nodes: {g.node_count()}")        # 3
print(f"Edges: {g.edge_count()}")        # 3
print(f"Density: {g.density():.3f}")     # 1.0 (complete triangle)
print(f"Bob's degree: {g.degree(bob)}")  # 2
print(f"Bob's neighbors: {g.neighbors(bob)}")  # [0, 2]

# Modify the graph
g.update_node(alice, 100)
g.remove_edge(alice, bob)
print(f"Edges after removal: {g.edge_count()}")  # 2
```

## See Also

- [PyDiGraph](digraph.md): Directed graph API
- [Core Operations](core/builders.md): Graph builders and generators
- [Algorithms](centrality/index.md): Graph algorithms
