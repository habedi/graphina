# Installation

This guide will help you install PyGraphina on your system.

## Requirements

- Python 3.10 or later
- pip (Python package installer)

## Installing from PyPI

The easiest way to install PyGraphina is using pip:

```bash
pip install --pre pygraphina
```

!!! note "Pre-release Version"
The `--pre` flag is currently required because PyGraphina is in early development and hasn't reached a stable 1.0
release yet.

## Installing from Source

If you want to use the latest development version or contribute to PyGraphina, you can install from source:

### Prerequisites

You'll need:

- Rust 1.86 or later ([Install Rust](https://rustup.rs/))
- Python 3.10 or later
- [Maturin](https://github.com/PyO3/maturin) (Python package builder)

### Steps

1. Clone the repository:

```bash
git clone https://github.com/habedi/graphina.git
cd graphina/pygraphina
```

2. Create a virtual environment (recommended):

```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
```

3. Install dependencies:

```bash
pip install maturin[zig]
```

4. Build and install PyGraphina:

```bash
maturin develop --release
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
pip install --pre pygraphina --force-reinstall
```

### Rust Compiler Not Found

If you're building from source and get a Rust compiler error, install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then restart your terminal and try again.

### Python Version Issues

PyGraphina requires Python 3.10 or later. Check your Python version:

```bash
python --version
```

If you have multiple Python versions, you may need to use `python3.10` or later explicitly:

```bash
python3.10 -m pip install --pre pygraphina
```

## Development Installation

For development work, install with development dependencies:

```bash
cd graphina/pygraphina
pip install -e ".[dev]"
```

This installs PyGraphina in editable mode along with testing and linting tools.

## Updating PyGraphina

To update to the latest version:

```bash
pip install --pre --upgrade pygraphina
```

## Next Steps

- [Quick Start Tutorial](quickstart.md): Build your first graph
- [Basic Concepts](concepts.md): Understand PyGraphina's graph model
- [API Reference](../api/graph.md): Explore the API
