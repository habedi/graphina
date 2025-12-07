# Installation

Install PyGraphina on your system.

## Requirements

- Python 3.10 or later
- pip (or another Python package manager like `uv` or `Poetry`)

## Installing from PyPI

The easiest way to install PyGraphina is using pip:

```bash
pip install pygraphina
```

## Verifying Installation

After installation, verify that PyGraphina is working correctly:

```python
import pygraphina as pg

# Create a simple graph
g = pg.PyGraph()
a, b = g.add_node(1), g.add_node(2)
g.add_edge(a, b, 1.0)

print(f"Graph has {g.node_count()} nodes and {g.edge_count()} edges")
# Output: Graph has 2 nodes and 1 edges
```

If you see the output without errors, congratulations! PyGraphina is installed correctly.

## Troubleshooting

### ImportError: No module named 'pygraphina'

Make sure you've installed the package:

```bash
pip list | grep pygraphina
```

If it's not listed, reinstall using:

```bash
pip install pygraphina --force-reinstall
```



### Python Version Issues

PyGraphina requires Python 3.10 or later. Check your Python version:

```bash
python --version
```

If you have multiple Python versions, you may need to use `python3.10` or later explicitly:

```bash
python3.10 -m pip install pygraphina
```

## Next Steps

- [Quick Start Tutorial](quickstart.md): Build your first graph
- [Basic Concepts](concepts.md): Understand PyGraphina's graph model
- [API Reference](../api/graph.md): Explore the API
