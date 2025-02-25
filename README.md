<div align="center">
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="logo.png">
    <source media="(prefers-color-scheme: dark)" srcset="logo.png">
    <img alt="template-rust-project logo" src="logo.png" height="50%" width="50%">
  </picture>
</div>
<br>

## Graphina

[![Tests](https://img.shields.io/github/actions/workflow/status/habedi/graphina/tests.yml?label=tests&style=popout-square&logo=github)](https://github.com/habedi/graphina/actions/workflows/tests.yml)
[![Lint](https://img.shields.io/github/actions/workflow/status/habedi/graphina/lint.yml?label=lints&style=popout-square&logo=github)](https://github.com/habedi/graphina/actions/workflows/lint.yml)
[![Code Coverage](https://img.shields.io/codecov/c/github/habedi/graphina?style=popout-square&logo=codecov)](https://codecov.io/gh/habedi/graphina)
[![CodeFactor](https://img.shields.io/codefactor/grade/github/habedi/graphina?style=popout-square&logo=codefactor)](https://www.codefactor.io/repository/github/habedi/graphina)
[![Crates.io](https://img.shields.io/crates/v/graphina.svg?style=popout-square&color=fc8d62&logo=rust)](https://crates.io/crates/graphina)
[![Docs.rs](https://img.shields.io/badge/docs.rs-graphina-66c2a5?style=popout-square&logo=docs.rs)](https://docs.rs/graphina)
[![Downloads](https://img.shields.io/crates/d/graphina?style=popout-square&logo=rust)](https://crates.io/crates/graphina)
[![MSRV](https://img.shields.io/badge/MSRV-1.83.0-orange?style=popout-square&logo=rust&label=msrv)](https://github.com/rust-lang/rust/releases/tag/1.83.0)
[![Docs](https://img.shields.io/badge/docs-latest-3776ab?style=flat&logo=readthedocs)](docs)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-007ec6?style=flat&logo=open-source-initiative)](https://github.com/habedi/graphina)

Graphina is a graph data science library for Rust.
It provides the common data structures and algorithms used for analyzing the graphs of real-world networks such as
social, transportation, and biological networks.

Compared to other Rust graph libraries like [petgraph](https://github.com/petgraph/petgraph)
and [rustworkx](https://www.rustworkx.org/), Graphina aims to provide a more high-level API and a wide range of
ready-to-use algorithms for network analysis and graph mining tasks.
The main goal is to make Graphina as feature-rich as [NetworkX](https://networkx.org/),
but with the performance of Rust.

Additionally, [PyGraphina](https://pypi.org/project/pygraphina/) Python library allows users to use Graphina in Python.
Check out [pygraphina](pygraphina/) directory for more details.

> [!IMPORTANT]
> Graphina is in the early stages of development, so breaking changes may occur.
> Bugs and API inconsistencies are also expected, and the algorithms may not yet be optimized for performance.

### Structure

Graphina consists of two main parts: a *core library* and *extensions*.
The core library provides the basic data structures and algorithms for working with graphs.
The extensions are modules outside the core library that contain more advanced algorithms for specific tasks like
community detection, link prediction, and calculating node and edge centrality scores.

The extensions are independent of each other. However, they depend on the core library for the basic graph operations.

#### Graphina Core

| Module                                   | Feature/Algorithm                                                                                                                                                                             | Status | Notes                                             |
|------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------|---------------------------------------------------|
| [**Types**](src/core/types.rs)           | <ul><li>Directed and undirected graphs</li><li>Weighted and unweighted graphs</li></ul>                                                                                                       | Tested | Graph types that Graphina supports                |                                 
| [**Exceptions**](src/core/exceptions.rs) | <ul><li>List of exceptions</li></ul>                                                                                                                                                          | Tested | Custom error types for Graphina                   |
| [**IO**](src/core/io.rs)                 | <ul><li>Edge list</li><li>Adjacency list</li></ul>                                                                                                                                            | Tested | I/O routines for reading/writing graph data       |
| [**Generators**](src/core/generators.rs) | <ul><li>Erdős–Rényi graph</li><li>Watts–Strogatz graph</li><li>Barabási–Albert graph</li><li>Complete graph</li><li>Bipartite graph</li><li>Star graph</li><li>Cycle graph</li></ul>          | Tested | Graph generators for random and structured graphs |
| [**Paths**](src/core/paths.rs)           | <ul><li>Dijkstra’s algorithm</li><li>Bellman–Ford algorithm</li><li>Floyd–Warshall algorithm</li><li>Johnson’s algorithm</li><li>A* search algorithm</li><li>Iterative deepening A*</li></ul> | Tested | Shortest paths algorithms                         |
| [**MST**](src/core/mst.rs)               | <ul><li>Prim’s algorithm</li><li>Kruskal’s algorithm</li><li>Borůvka’s algorithm</li></ul>                                                                                                    | Tested | Minimum spanning tree algorithms                  |
| [**Traversal**](src/core/traversal.rs)   | <ul><li>Breadth-first search (BFS)</li><li>Depth-first search (DFS)</li><li>Iterative deepening DFS</li><li>Bidirectional search</li></ul>                                                    | Tested | Graph traversal algorithms                        |

#### Extensions

| Module                                               | Feature/Algorithm                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Status | Notes                                                     |
|------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------|-----------------------------------------------------------|
| [**Centrality**](src/centrality/algorithms.rs)       | <ul><li>Degree centrality</li><li>Closeness centrality</li><li>Betweenness centrality</li><li>Eigenvector centrality</li><li>PageRank centrality</li><li>Katz centrality</li><li>Harmonic centrality</li><li>Local/global reaching centrality</li><li>Voterank centrality</li><li>Laplacian centrality</li></ul>                                                                                                                                                                                                                                                                                                                     |        | Centrality measures                                       |
| [**Links**](src/links/algorithms.rs)                 | <ul><li>Resource allocation index</li><li>Jaccard coefficient</li><li>Adamic–Adar index</li><li>Preferential attachment</li><li>CN Soundarajan–Hopcroft</li><li>RA index Soundarajan–Hopcroft</li><li>Within–inter-cluster ratio</li><li>Common neighbor centrality</li></ul>                                                                                                                                                                                                                                                                                                                                                        |        | Link prediction algorithms                                |
| [**Community**](src/community/algorithms.rs)         | <ul><li>Label propagation</li><li>Louvain method</li><li>Girvan–Newman algorithm</li><li>Spectral clustering</li><li>Personalized PageRank</li><li>Infomap</li><li>Connected components</li></ul>                                                                                                                                                                                                                                                                                                                                                                                                                                    |        | Community detection and clustering algorithms             |
| [**Approximation**](src/approximation/algorithms.rs) | <ul><li>Local node connectivity (BFS-based)</li><li>Maximum independent set (greedy with neighbor caching)</li><li>Maximum clique (greedy heuristic)</li><li>Clique removal</li><li>Large clique size</li><li>Average clustering coefficient</li><li>Densest subgraph (greedy peeling)</li><li>Diameter lower bound</li><li>Minimum weighted vertex cover (greedy re‑evaluated)</li><li>Minimum maximal matching (greedy)</li><li>Approximate Ramsey R2</li><li>TSP approximations (greedy, simulated annealing, threshold accepting, Christofides placeholder)</li><li>Treewidth decompositions (min degree, min fill-in)</li></ul> |        | Approximations and heuristic methods for NP‑hard problems |

> [!NOTE]
> Status shows whether the module is `Tested` and `Benchmarked`.
> Empty status means the module is implemented but not tested and benchmarked yet.

### Installation

```
cargo add graphina
```

*Graphina requires Rust 1.83 or later.*

### Documentation

See the [docs](docs/README.md) for the latest documentation.

Check out the [docs.rs/graphina](https://docs.rs/graphina) for the latest API documentation.

#### Simple Example

```rust
use graphina::core::types::Graph;

fn main() {
    // Create a new undirected graph
    let mut graph = Graph::new();

    // Add nodes and edges to the graph
    let n0 = graph.add_node(1);
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    graph.add_edge(n0, n1, 1.0);
    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n2, n3, 1.0);

    // Get the neighbors of node 1
    for neighbor in graph.neighbors(n1) {
        println!("Node 1 has neighbor: {}", neighbor.index());
    }
}
```

See the [tests](tests/) directory for more usage examples.

### Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to make a contribution.

### Logo

The mascot is named "Graphina the Dinosaur".
As the name implies, she's a dinosaur, however, she herself thinks she's a dragon.
The logo was created using Gimp, ComfyUI, and a Flux Schnell v2 model.

### Licensing

Graphina is licensed under either of these:

* MIT License ([LICENSE-MIT](LICENSE-MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

PyGraphina is licensed under the MIT License ([LICENSE](pygraphina/LICENSE)).
