# Graph Views

Views provide read-only access to graph components (nodes, edges, and degrees) with a Pythonic API similar to NetworkX.

## NodeView

Returned by `G.nodes`.
It behaves like a list of nodes or a dict of node attributes.

### Usage

```python
import pygraphina as pg
g = pg.PyGraph()
g.add_node(1, attr=100)

# Iteration
for n in g.nodes:
    print(n)

# Membership
if 1 in g.nodes:
    print("Node 1 exists")

# Attribute Access
print(g.nodes[1])  # {'attr': 100}

# Data Access
print(list(g.nodes.data("attr")))  # [(1, 100)]
```

### Methods

- **`__iter__()`**: Iterate over node IDs.
- **`__len__()`**: Number of nodes.
- **`__contains__(n)`**: Check if node `n` exists.
- **`__getitem__(n)`**: Get attributes for node `n` (returns dict).
- **`data(name=None, default=None)`**: Return a `NodeDataView` to iterate over (node, attribute_value) tuples.

---

## EdgeView

Returned by `G.edges`.
Provides access to edges and their weights.

### Usage

```python
g.add_edge(1, 2, 3.5)

# Iteration
for u, v in g.edges:
    print(u, v)

# Membership
if (1, 2) in g.edges:
    print("Edge exists")

# Attribute Access (Weights)
print(g.edges[1, 2])  # {'weight': 3.5}

# Data Access
print(list(g.edges.data("weight")))  # [(1, 2, 3.5)]
```

### Methods

- **`__iter__()`**: Iterate over edges (u, v).
- **`__len__()`**: Number of edges.
- **`__contains__((u, v))`**: Check if edge exists.
- **`__getitem__((u, v))`**: Get attributes (weight) for edge u-v.
- **`data(name="weight", default=None)`**: Return an `EdgeDataView` to iterate over (u, v, value).

---

## DegreeView

Returned by `G.degree`.
Provides access to node degrees.

### Usage

```python
# Access single node degree
print(g.degree[1])  # 2

# Iterate over (node, degree)
for n, d in g.degree:
    print(f"Node {n} has degree {d}")
```

### Methods

- **`__iter__()`**: Iterate over (node, degree) pairs.
- **`__getitem__(n)`**: Get degree of node `n`.
- **`__len__()`**: Number of nodes.
- **`__call__(nbunch=None, weight=None)`**: (Advanced) Get degree for subset of nodes or weighted degree.
