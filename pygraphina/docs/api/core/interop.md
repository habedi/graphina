# Interoperability

PyGraphina can convert graphs to and from NetworkX and pandas structures for data exchange and integration with existing Python workflows.

## NetworkX Conversion

### to_networkx

Convert a PyGraphina graph to a NetworkX graph.

```python
import pygraphina as pg

# Create a PyGraphina graph
g = pg.PyGraph()
n1 = g.add_node(10)
n2 = g.add_node(20)
g.add_edge(n1, n2, 1.5)

# Convert to NetworkX
nx_graph = pg.to_networkx(g)

# Access NetworkX features
print(nx_graph.nodes[n1]['attr'])  # 10
print(nx_graph[n1][n2]['weight'])  # 1.5
```

Parameters:

| Parameter | Type                     | Description                     |
|-----------|--------------------------|---------------------------------|
| `graph`   | `PyGraph` or `PyDiGraph` | The pygraphina graph to convert |

Returns:

| Type                                   | Description                                                      |
|----------------------------------------|------------------------------------------------------------------|
| `networkx.Graph` or `networkx.DiGraph` | A NetworkX graph with preserved node attributes and edge weights |

Notes:

- Node attributes are stored under the `'attr'` key
- Edge weights are stored under the `'weight'` key
- Requires NetworkX to be installed

---

### from_networkx

Convert a NetworkX graph to a PyGraphina graph.

```python
import networkx as nx
import pygraphina as pg

# Create a NetworkX graph
nx_g = nx.Graph()
nx_g.add_node(0, attr=100)
nx_g.add_node(1, attr=200)
nx_g.add_edge(0, 1, weight=2.5)

# Convert to PyGraphina
g = pg.from_networkx(nx_g)

print(g.node_count())  # 2
print(g.get_node_attr(0))  # 100
```

Parameters:

| Parameter  | Type                                   | Description                   |
|------------|----------------------------------------|-------------------------------|
| `nx_graph` | `networkx.Graph` or `networkx.DiGraph` | The NetworkX graph to convert |

Returns:

| Type                     | Description                                          |
|--------------------------|------------------------------------------------------|
| `PyGraph` or `PyDiGraph` | A pygraphina graph (type matches input directedness) |

Notes:

- Reads `'attr'` from node data (defaults to 0 if not present)
- Reads `'weight'` from edge data (defaults to 1.0 if not present)
- Integer node IDs are preserved when possible

---

## DataFrame Export

### to_node_dataframe

Export graph nodes to a pandas DataFrame.

```python
import pygraphina as pg

g = pg.PyGraph()
g.add_node(100)
g.add_node(200)
g.add_node(300)

df = pg.to_node_dataframe(g)
print(df)
#    node_id  attr
# 0        0   100
# 1        1   200
# 2        2   300
```

Parameters:

| Parameter | Type                     | Description         |
|-----------|--------------------------|---------------------|
| `graph`   | `PyGraph` or `PyDiGraph` | The graph to export |

Returns:

| Type               | Description                                             |
|--------------------|---------------------------------------------------------|
| `pandas.DataFrame` | DataFrame with columns `node_id` (int) and `attr` (int) |

Notes:

- Requires pandas to be installed
- Each row represents one node

---

### to_edge_dataframe

Export graph edges to a pandas DataFrame.

```python
import pygraphina as pg

g = pg.PyGraph()
n0 = g.add_node(0)
n1 = g.add_node(0)
n2 = g.add_node(0)
g.add_edge(n0, n1, 1.5)
g.add_edge(n1, n2, 2.0)

df = pg.to_edge_dataframe(g)
print(df)
#    source  target  weight
# 0       0       1     1.5
# 1       1       2     2.0
```

Parameters:

| Parameter | Type                     | Description         |
|-----------|--------------------------|---------------------|
| `graph`   | `PyGraph` or `PyDiGraph` | The graph to export |

Returns:

| Type               | Description                                                           |
|--------------------|-----------------------------------------------------------------------|
| `pandas.DataFrame` | DataFrame with columns `source`, `target` (int), and `weight` (float) |

Notes:

- Requires pandas to be installed
- For directed graphs, edge direction is preserved

---

## See Also

- [Graph I/O](io.md) - File-based serialization formats
- [Graph](../graph.md) - PyGraph API reference
- [Directed Graph](../digraph.md) - PyDiGraph API reference
