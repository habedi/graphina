## PyGraphina

[![Python version](https://img.shields.io/badge/python-%3E=3.10-blue)](https://github.com/habedi/graphina)
[![PyPI version](https://badge.fury.io/py/pygraphina.svg)](https://badge.fury.io/py/pygraphina)
[![Documentation](https://img.shields.io/badge/docs-read-blue.svg)](https://habedi.github.io/graphina/python)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

PyGraphina provides Python bindings for [Graphina](https://github.com/habedi/graphina).

> [!IMPORTANT]
> PyGraphina is in early development, so breaking changes and bugs are expected.
> Please report bugs on [GitHub issues](https://github.com/habedi/graphina/issues).

### Installation

```bash
pip install --pre pygraphina
```

### Quickstart

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
a, b, c = [g.add_node(i) for i in range(3)]
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
```

### Documentation

Visit PyGraphina's [documentation page](https://habedi.github.io/graphina/python) for detailed information including examples and API reference.

### License

PyGraphina is licensed under the [MIT License](LICENSE).
