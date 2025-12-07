# Graph Builders

Graph builders provide methods to construct graphs programmatically.

## Basic Node and Edge Addition

```python
import pygraphina as pg

# Create empty graph
g = pg.PyGraph()

# Add single nodes
node_a = g.add_node(100)  # attribute = 100
node_b = g.add_node(200)

# Add single edge
g.add_edge(node_a, node_b, 1.5)  # weight = 1.5
```

## Bulk Operations

### Add Multiple Nodes

```python
# Add multiple nodes at once
attributes = [10, 20, 30, 40, 50]
node_ids = g.add_nodes_from(attributes)
# Returns list of node IDs
```

### Add Multiple Edges

```python
# Add multiple edges at once
edges = [
    (0, 1, 1.0),
    (1, 2, 2.0),
    (2, 3, 1.5),
]
edge_ids = g.add_edges_from(edges)
# Returns list of edge IDs
```

## Node Management

### Update Node Attributes

```python
# Update node attribute
success = g.update_node(node_a, 999)  # Change attribute to 999

# Get node attribute
attr = g.get_node_attr(node_a)
```

### Remove Nodes

```python
# Remove single node (removes incident edges too)
removed_attr = g.remove_node(node_a)

# Check if node exists
exists = g.contains_node(node_a)
```

## Edge Management

### Update Edge Weights

```python
# Update edge weight
success = g.update_edge_weight(node_a, node_b, 2.5)

# Get edge weight
weight = g.get_edge_weight(node_a, node_b)
```

### Remove Edges

```python
# Remove edge
removed = g.remove_edge(node_a, node_b)

# Check if edge exists
exists = g.contains_edge(node_a, node_b)
```

## Practical Example

```python
import pygraphina as pg

# Build a social network
network = pg.PyGraph()

# Add users (nodes)
users = {
    "Alice": network.add_node(1),
    "Bob": network.add_node(2),
    "Charlie": network.add_node(3),
    "David": network.add_node(4),
}

# Add friendships (edges)
friendships = [
    ("Alice", "Bob", 1.0),
    ("Bob", "Charlie", 1.0),
    ("Charlie", "David", 1.0),
    ("Alice", "David", 0.5),
]

for person1, person2, strength in friendships:
    network.add_edge(users[person1], users[person2], strength)

# Network is ready to use
print(f"Network size: {network.node_count()} users, {network.edge_count()} friendships")
```

## Efficiency Tips

1. **Bulk operations faster than single operations**
    - Use `add_nodes_from()` instead of multiple `add_node()` calls
    - Use `add_edges_from()` instead of multiple `add_edge()` calls

2. **Pre-allocate if possible**
    - Building graph incrementally is fine
    - Batch similar operations together

3. **Avoid repeated lookups**
    - Store node IDs returned from `add_node()`
    - Don't re-add same nodes

## Related Methods

- `g.clear()` - Remove all nodes and edges
- `g.filter_nodes()` - Create subgraph with filtered nodes
- `g.filter_edges()` - Create subgraph with filtered edges
- `g.nodes` - Get all node IDs
- `g.edges` - Get all edges
