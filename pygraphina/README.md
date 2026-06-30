## PyGraphina

[![Python version](https://img.shields.io/badge/python-%3E=3.10-3776ab?style=flat&labelColor=282c34&logo=python)](https://github.com/habedi/graphina)
[![PyPI version](https://img.shields.io/pypi/v/pygraphina?style=flat&labelColor=282c34&color=3775a9&logo=pypi)](https://badge.fury.io/py/pygraphina)
[![Documentation](https://img.shields.io/badge/docs-read-00acc1?style=flat&labelColor=282c34&logo=readthedocs)](https://habedi.github.io/graphina/python)
[![License: MIT](https://img.shields.io/badge/license-MIT-0288d1?style=flat&labelColor=282c34&logo=open-source-initiative)](LICENSE)

PyGraphina provides Python bindings for [Graphina](https://github.com/habedi/graphina).

### Installation

```bash
pip install pygraphina
```

### Quickstart

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()

# add_node(attr) stores an integer attribute and returns the new node's id
a = g.add_node(10)
b = g.add_node(20)
c = g.add_node(30)

# add_edge(source, target, weight) connects two nodes by their ids
g.add_edge(a, b, 1.0)
g.add_edge(b, c, 1.0)

# Calculate PageRank
pr = pg.centrality.pagerank(g, 0.85, 100, 1e-6)

# Find largest clique size
size = pg.approximation.large_clique_size(g)

# Find connected components
comps = pg.community.connected_components(g)

# Compute Jaccard coefficients
jc = pg.links.jaccard_coefficient(g)

print(f"PageRank: {pr}")
print(f"Clique size: {size}")
print(f"Connected components: {comps}")
print(f"Jaccard coefficients: {jc}")
```

### Documentation

Visit PyGraphina's [documentation page](https://habedi.github.io/graphina/python) for detailed information including examples and API references.

### License

PyGraphina is licensed under the [MIT License](LICENSE).
