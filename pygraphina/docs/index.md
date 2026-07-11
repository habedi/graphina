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
    - Centrality measures like betweenness, closeness, and eigenvector
    - Community detection like Louvain, label propagation, and Girvan-Newman
    - Link prediction like Jaccard coefficient, Adamic-Adar, and resource allocation
    - Path algorithms like Dijkstra, Bellman-Ford, A*, and Floyd-Warshall
    - Graph metrics like clustering coefficient, transitivity, diameter, and assortativity
    - Algorithms for hard problems like for cliques, vertex cover, and independent sets
    - Minimum spanning trees algorithms like Prim's and Kruskal's
- A Pythonic API
- Create random and structured graphs (like Erdős-Rényi, Watts-Strogatz, etc.)
- Read and write graphs in edge lists, adjacency lists, GraphML, etc. formats
- Multi-threaded implementations of popular graph algorithms like PageRank and shortest paths

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
