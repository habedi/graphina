# PyGraphina Documentation

<p align="center">
  <img src="https://media.githubusercontent.com/media/habedi/graphina/refs/heads/main/logo.png" alt="Graphina Logo" width="300" />
</p>

Welcome to PyGraphina documentation!

[PyGraphina](https://github.com/habedi/graphina/tree/main/pygraphina) 🐍 allows users to use
[Graphina](https://github.com/habedi/graphina) 🦀 graph data science library from Python.

## Features

- All algorithms and data structures are implemented in Rust
- A **large collection of graph algorithms** including:
    - **Centrality measures**: PageRank, betweenness, closeness, eigenvector, Katz, and more
    - **Community detection**: Louvain, label propagation, Girvan-Newman, spectral clustering
    - **Link prediction**: Jaccard coefficient, Adamic-Adar, resource allocation, preferential attachment
    - **Path algorithms**: Dijkstra, Bellman-Ford, A*, Floyd-Warshall, Johnson's algorithm
    - **Graph metrics**: Clustering coefficient, transitivity, diameter, assortativity
    - **Approximation algorithms**: For cliques, vertex cover, independent sets, TSP, and more
    - **Minimum spanning trees**: Prim's, Kruskal's, and Borůvka's algorithms
- **Pythonic API**
- Create **random and structured graphs** (Erdős-Rényi, Barabási-Albert, Watts-Strogatz, etc.)
- **Read and write graphs in multiple formats** (edge lists, adjacency lists, JSON, and GraphML)
- **Multi-threaded implementations** of popular graph algorithms like PageRank

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

## When to Use PyGraphina

Typical PyGraphina use cases include:

- **Network analysis**: Social networks, collaboration networks, citation networks
- **Link prediction**: Recommender systems, knowledge graph completion
- **Community detection**: Finding clusters and groups in networks
- **Graph metrics**: Analyzing graph properties and structure
- **Algorithm research**: Fast prototyping of graph algorithms
- **Data science pipelines**: Integrating graph analysis into typical data science workflows that use Python

## Comparison with NetworkX

[NetworkX](https://networkx.org/en/) is probably the most popular Python graph data science and network science library.
NetworkX is relatively mature and has a large collection of graph algorithms, however, it's written in pure Python.
As a result, it can be slow specially when it comes to large graphs.
PyGraphina aims to be a drop-in replacement for NetworkX with providing a similar API, but with much better performance and lower memory usage.

| Feature            | PyGraphina                | NetworkX    |
|--------------------|---------------------------|-------------|
| Language           | Rust plus Python bindings | Pure Python |
| Performance        | High                      | Moderate    |
| Memory Usage       | Low                       | Higher      |
| API Style          | Pythonic                  | Pythonic    |
| Algorithm Coverage | Growing                   | Extensive   |
| Maturity           | Early development         | Mature      |

!!! note "Project Status"
    PyGraphina is in early development. While it's functional and tested, you may encounter bugs or breaking changes. Please
    report issues on our [GitHub issue tracker](https://github.com/habedi/graphina/issues).

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

PyGraphina is licensed under the MIT License. See
the [LICENSE](https://github.com/habedi/graphina/blob/main/LICENSE-MIT) file for details.
