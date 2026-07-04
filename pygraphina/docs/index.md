# PyGraphina Documentation

<p align="center">
  <img src="https://media.githubusercontent.com/media/habedi/graphina/refs/heads/main/logo.png" alt="Graphina Logo" width="300" />
</p>

Welcome to PyGraphina documentation!

[PyGraphina](https://github.com/habedi/graphina/tree/main/pygraphina) 🐍 allows users to use
[Graphina](https://github.com/habedi/graphina) 🦀 graph data science library from Python.

## Key Features

- All algorithms and data structures are implemented in fast, safe Rust
- A large collection of graph algorithms for graph mining and network science:
    - Centrality measures: PageRank, betweenness, closeness, eigenvector, Katz, and more
    - Community detection: Louvain, label propagation, Girvan-Newman, spectral clustering
    - Link prediction: Jaccard coefficient, Adamic-Adar, resource allocation, preferential attachment
    - Path algorithms: Dijkstra, Bellman-Ford, A*, Floyd-Warshall, Johnson's algorithm
    - Graph metrics: Clustering coefficient, transitivity, diameter, assortativity
    - Approximation algorithms: For cliques, vertex cover, independent sets, TSP, and more
    - Minimum spanning trees: Prim's, Kruskal's, and Borůvka's algorithms
- A Pythonic API
- Create random and structured graphs (like Erdős-Rényi, Barabási-Albert, Watts-Strogatz, etc.)
- Read and write graphs in multiple formats (like edge lists, adjacency lists, JSON, and GraphML)
- Multi-threaded implementations of popular graph algorithms like PageRank

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

## Comparison with NetworkX

[NetworkX](https://networkx.org/en/) is probably the most popular Python graph data science and network science library.
NetworkX is relatively mature and has a large collection of graph algorithms, however, it's written in pure Python.
As a result, it can be slow, especially when it comes to large graphs.
PyGraphina aims to be a drop-in replacement for NetworkX by providing a similar API, but with much better performance and lower memory usage.

| Feature            | PyGraphina                  | NetworkX    |
|--------------------|-----------------------------|-------------|
| Language           | Rust (plus Python bindings) | Pure Python |
| Performance        | High                        | Moderate    |
| Memory Usage       | Low                         | Higher      |
| API Style          | Pythonic                    | Pythonic    |
| Algorithm Coverage | Growing                     | Extensive   |
| Maturity           | Developing                  | Mature      |

## Next Steps

- [Installation Guide](getting-started/installation.md): Get PyGraphina installed on your system
- [Quick Start Tutorial](getting-started/quickstart.md): Your first PyGraphina program
- [Basic Concepts](getting-started/concepts.md): Understand the core concepts of graphs and PyGraphina
- [API Reference](api/graph.md): Detailed API documentation
- [Examples](examples/basic.md): See PyGraphina in action with example programs

## Community and Support

- GitHub: [https://github.com/habedi/graphina](https://github.com/habedi/graphina)
- Issues: [Report bugs or request features](https://github.com/habedi/graphina/issues)
- Contributing: See the [Contributing Guide](contributing.md)

## License

PyGraphina is licensed under the MIT License.
See the [LICENSE](https://github.com/habedi/graphina/blob/main/LICENSE-MIT) file for details.
