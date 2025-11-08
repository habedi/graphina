## PyGraphina

[![Python version](https://img.shields.io/badge/python-%3E=3.10-blue)](https://github.com/habedi/graphina)
[![PyPI version](https://badge.fury.io/py/pygraphina.svg)](https://badge.fury.io/py/pygraphina)
[![Documentation](https://img.shields.io/badge/docs-read-blue.svg)](docs)
[![Examples](https://img.shields.io/badge/examples-view-orange.svg)](examples)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

PyGraphina Python library allows users to use [Graphina](https://github.com/habedi/graphina) in Python.

> [!IMPORTANT]
> PyGraphina is in early development, so bugs and breaking changes are expected.
> Please use the [issues page](https://github.com/habedi/graphina/issues) to report bugs or request features.

### Installation

```bash
pip install --pre pygraphina
```

### Quick Start

```python
import pygraphina as pg

# Make a small graph
g = pg.PyGraph()
a, b, c = [g.add_node(i) for i in range(3)]
g.add_edge(a, b, 1.0)
g.add_edge(b, c, 1.0)

# Get the PageRank centrality scores of the nodes
pr = pg.centrality.pagerank(g, 0.85, 100, 1e-6)

# Get the size of the largest clique in the graph
size = pg.approximation.large_clique_size(g)

# Get the connected components of the graph
comps = pg.community.connected_components(g)

# Get the Jaccard coefficient of the links
jc = pg.links.jaccard_coefficient(g)
```

### License

PyGraphina is licensed under the [MIT License](LICENSE).
