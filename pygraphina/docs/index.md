# PyGraphina Documentation

Welcome to PyGraphina, the Python bindings for the [Graphina](https://github.com/habedi/graphina) graph data science library!

[![Python version](https://img.shields.io/badge/python-%3E=3.10-blue)](https://github.com/habedi/graphina)
[![PyPI version](https://badge.fury.io/py/pygraphina.svg)](https://badge.fury.io/py/pygraphina)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/habedi/graphina)

## What is PyGraphina?

PyGraphina provides Python access to Graphina, a high-performance graph data science library written in Rust. It combines the ease of use of Python with the speed and efficiency of Rust, making it ideal for analyzing large-scale networks.

## Key Features

- **Fast Performance**: Rust-powered backend for high-performance graph algorithms
- **Rich Algorithm Library**: Comprehensive suite of graph algorithms including:
    - **Centrality measures**: PageRank, betweenness, closeness, eigenvector, Katz, and more
    - **Community detection**: Louvain, label propagation, Girvan-Newman, spectral clustering
    - **Link prediction**: Jaccard coefficient, Adamic-Adar, resource allocation, preferential attachment
    - **Path algorithms**: Dijkstra, Bellman-Ford, A*, Floyd-Warshall, Johnson's algorithm
    - **Graph metrics**: Clustering coefficient, transitivity, diameter, assortativity
    - **Approximation algorithms**: For cliques, vertex cover, independent sets, TSP, and more
    - **Minimum spanning trees**: Prim's, Kruskal's, and Borůvka's algorithms
- **Easy to Use**: Pythonic API that's familiar and intuitive
- **Graph Generators**: Create random and structured graphs (Erdős-Rényi, Barabási-Albert, Watts-Strogatz, etc.)
- **I/O Support**: Read and write graphs in multiple formats (edge lists, adjacency lists, JSON, GraphML)
- **Parallel Processing**: Multi-threaded implementations for compute-intensive operations

## Quick Example

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()

# Add nodes and edges
a, b, c, d = [g.add_node(i) for i in range(4)]
g.add_edge(a, b, 1.0)
g.add_edge(b, c, 1.0)
g.add_edge(c, d, 1.0)
g.add_edge(d, a, 1.0)

# Calculate PageRank scores
pagerank = pg.centrality.pagerank(g, 0.85, 100, 1e-6)
print(f"PageRank scores: {pagerank}")

# Detect communities
communities = pg.community.label_propagation(g, 100)
print(f"Communities: {communities}")

# Predict links
jaccard = pg.links.jaccard_coefficient(g)
print(f"Jaccard coefficients: {jaccard}")
```

## Performance

PyGraphina leverages Rust's performance to handle large-scale graphs efficiently. It's designed to be:

- **Fast**: Rust implementation provides near-native performance
- **Memory efficient**: Optimized data structures minimize memory footprint
- **Scalable**: Parallel algorithms for multi-core processors
- **Production-ready**: Battle-tested algorithms with comprehensive test coverage

## When to Use PyGraphina

PyGraphina is ideal for:

- **Network analysis**: Social networks, collaboration networks, citation networks
- **Link prediction**: Recommender systems, knowledge graph completion
- **Community detection**: Finding clusters and groups in networks
- **Graph metrics**: Analyzing graph properties and structure
- **Algorithm research**: Fast prototyping of graph algorithms
- **Data science pipelines**: Integrating graph analysis into Python workflows

## Comparison with NetworkX

While NetworkX is a popular Python graph library, PyGraphina offers significant performance advantages:

| Feature | PyGraphina | NetworkX |
|---------|-----------|----------|
| Language | Rust + Python bindings | Pure Python |
| Performance | High (compiled, parallel) | Moderate (interpreted) |
| Memory Usage | Low (optimized structures) | Higher |
| API Style | Pythonic | Pythonic |
| Algorithm Coverage | Growing | Extensive |
| Maturity | Early development | Mature |

!!! note "Project Status"
    PyGraphina is in early development. While it's functional and tested, you may encounter bugs or breaking changes. Please report issues on our [GitHub issue tracker](https://github.com/habedi/graphina/issues).

## Next Steps

- [Installation Guide](getting-started/installation.md): Get PyGraphina installed
- [Quick Start Tutorial](getting-started/quickstart.md): Your first PyGraphina program
- [Basic Concepts](getting-started/concepts.md): Understand graphs in PyGraphina
- [API Reference](api/graph.md): Detailed API documentation
- [Examples](examples/basic.md): Learn from practical examples

## Community and Support

- **GitHub**: [https://github.com/habedi/graphina](https://github.com/habedi/graphina)
- **Issues**: [Report bugs or request features](https://github.com/habedi/graphina/issues)
- **Contributing**: See our [Contributing Guide](contributing.md)

## License

PyGraphina is licensed under the MIT License. See the [LICENSE](https://github.com/habedi/graphina/blob/main/LICENSE-MIT) file for details.
