# Degree Centrality

Degree centrality measures node importance based on the number of connections.

## Function

```python
pg.centrality.degree(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, int]
```

Returns a dictionary mapping node IDs to their degree (number of neighbors).

## Example

```python
import pygraphina as pg
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(5)]
g.add_edge(0, 1, 1.0)
g.add_edge(0, 2, 1.0)
g.add_edge(1, 2, 1.0)
degrees = pg.centrality.degree(g)
print(degrees)  # {0: 2, 1: 2, 2: 2, 3: 0, 4: 0}
```

Simple, fast, and effective for many applications.
