# PyGraphina Documentation

This directory contains the documentation for PyGraphina, built using MkDocs with the Material theme.

## Building the Documentation

### Prerequisites

Install the required Python packages:

```bash
pip install mkdocs mkdocs-material mkdocstrings[python]
```

### Build and Serve Locally

To build and serve the documentation locally:

```bash
cd pygraphina
mkdocs serve
```

Then open your browser to `http://127.0.0.1:8000`

### Build Static Site

To build the static HTML documentation:

```bash
cd pygraphina
mkdocs build
```

The generated documentation will be in the `site/` directory.

## Documentation Structure

```
docs/
├── index.md                  # Home page
├── getting-started/          # Getting started guides
│   ├── installation.md       # Installation instructions
│   ├── quickstart.md        # Quick start tutorial
│   └── concepts.md          # Basic concepts
├── api/                     # API reference documentation
│   ├── graph.md            # PyGraph API
│   ├── digraph.md          # PyDiGraph API
│   ├── centrality/         # Centrality algorithms
│   ├── community/          # Community detection
│   ├── links/              # Link prediction
│   ├── approximation/      # Approximation algorithms
│   ├── metrics/            # Graph metrics
│   ├── mst/                # Minimum spanning trees
│   ├── traversal/          # Traversal algorithms
│   ├── subgraphs/          # Subgraph operations
│   ├── parallel/           # Parallel algorithms
│   └── core/               # Core operations
├── examples/               # Example code and tutorials
│   ├── basic.md           # Basic usage examples
│   ├── centrality.md      # Centrality examples
│   ├── community.md       # Community detection examples
│   ├── links.md           # Link prediction examples
│   └── algorithms.md      # Algorithm examples
└── contributing.md        # Contribution guidelines
```

## Writing Documentation

### Style Guide

- Use clear, concise language
- Minimize adjectives and adverbs
- Avoid emojis
- Include code examples with expected output
- Use informational admonitions (note, tip, warning) sparingly

### Code Examples

All code examples should be tested and runnable. Include imports and complete context:

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(3)]
g.add_edge(nodes[0], nodes[1], 1.0)

# Run algorithm
result = pg.centrality.pagerank(g, 0.85, 100, 1e-6)
print(result)
```

### API Documentation Format

Each API page should include:

1. Function signature with type hints
2. Description of what the function does
3. Parameter descriptions
4. Return value description
5. Time/space complexity (if relevant)
6. Example usage
7. Common use cases

## Publishing

The documentation can be deployed to GitHub Pages:

```bash
cd pygraphina
mkdocs gh-deploy
```

This builds the documentation and pushes it to the `gh-pages` branch.
