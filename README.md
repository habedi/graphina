<div align="center">
  <picture>
    <img alt="Graphina the Dinosaur" src="logo.png" height="50%" width="50%">
  </picture>
</div>
<br>

## Graphina

[![Tests](https://img.shields.io/github/actions/workflow/status/habedi/graphina/tests.yml?label=tests&style=flat&labelColor=282c34&logo=github)](https://github.com/habedi/graphina/actions/workflows/tests.yml)
[![Code Coverage](https://img.shields.io/codecov/c/github/habedi/graphina?style=flat&labelColor=282c34&logo=codecov)](https://codecov.io/gh/habedi/graphina)
[![CodeFactor](https://img.shields.io/codefactor/grade/github/habedi/graphina?style=flat&labelColor=282c34&logo=codefactor)](https://www.codefactor.io/repository/github/habedi/graphina)
[![Crates.io](https://img.shields.io/crates/v/graphina.svg?style=flat&labelColor=282c34&color=f46623&logo=rust)](https://crates.io/crates/graphina)
[![Docs.rs](https://img.shields.io/badge/docs.rs-graphina-66c2a5?style=flat&labelColor=282c34&logo=docs.rs)](https://docs.rs/graphina)
[![Downloads](https://img.shields.io/crates/d/graphina?style=flat&labelColor=282c34&color=4caf50&logo=rust)](https://crates.io/crates/graphina)
[![MSRV](https://img.shields.io/badge/MSRV-1.86.0-007ec6?label=msrv&style=flat&labelColor=282c34&logo=rust)](https://github.com/rust-lang/rust/releases/tag/1.86.0)
[![Docs](https://img.shields.io/badge/docs-view-3776ab?style=flat&labelColor=282c34&logo=readthedocs)](docs)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-007ec6?style=flat&labelColor=282c34&logo=open-source-initiative)](https://github.com/habedi/graphina)

Graphina is a graph data science library for Rust.
It provides the common data structures and algorithms used for analyzing the graphs of real-world networks such as
social, transportation, and biological networks.

Compared to other Rust graph libraries like [petgraph](https://github.com/petgraph/petgraph)
and [rustworkx](https://www.rustworkx.org/), Graphina aims to provide a more high-level API and a wide range of
ready-to-use algorithms for network analysis and graph mining tasks.
The main goal is to make Graphina as feature-rich as [NetworkX](https://networkx.org/),
but with the performance of Rust.

Additionally, [PyGraphina](https://pypi.org/project/pygraphina/) Python library allows users to use Graphina in Python.
Check out [pygraphina](pygraphina/README.md) directory for more details.

> [!IMPORTANT]
> Graphina is in the early stages of development, so breaking changes may occur.
> Bugs and API inconsistencies are also expected, and the algorithms may not yet be optimized for performance.
> Please use the [issues page](https://github.com/habedi/graphina/issues) to report bugs or request features.

---

### Structure

Graphina consists of two main parts: a *core library* and *extensions*.
The core library provides the basic data structures and algorithms for working with graphs.
The extensions are modules outside the core library that contain more advanced algorithms for specific tasks like
community detection, link prediction, and calculating node and edge centrality scores.

The extensions are independent of each other. However, they depend on the core library for the basic graph operations.

#### Graphina Core

| Module                                         | Feature or Algorithm                                                                                                                                                                                                                              | Notes                                                      |
|------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------|
| [**Types**](src/core/types.rs)                 | <ul><li>Directed and undirected graphs</li><li>Weighted and unweighted graphs</li><li>NodeId and EdgeId wrappers</li><li>NodeMap and EdgeMap type aliases</li><li>OrderedNodeMap for deterministic iteration</li></ul>                            | Graph types that Graphina supports                         |  
| [**Error Handling**](src/core/error.rs)        | <ul><li>Unified GraphinaError for all Graphina modules</li><li>Result type alias</li><li>Error conversion helpers</li></ul>                                                                                                                       | A unified error type for the APIs                          |
| [**Builders**](src/core/builders.rs)           | <ul><li>AdvancedGraphBuilder with validation</li><li>TopologyBuilder (path, cycle, star, and complete graph builders)</li><li>Type aliases (DirectedGraphBuilder, UndirectedGraphBuilder)</li></ul>                                               | Ergonomic graph construction with fluent APIs              |
| [**IO**](src/core/io.rs)                       | <ul><li>Edge list (read and write)</li><li>Adjacency list (read and write)</li></ul>                                                                                                                                                              | I/O routines for reading and writing graph data            |
| [**Serialization**](src/core/serialization.rs) | <ul><li>JSON serialization</li><li>Binary serialization</li><li>GraphML export</li><li>SerializableGraph format</li></ul>                                                                                                                         | Multiple serialization formats for interoperability        |
| [**Generators**](src/core/generators.rs)       | <ul><li>Erdős-Rényi graph</li><li>Watts-Strogatz graph</li><li>Barabási-Albert graph</li><li>Complete graph (directed/undirected)</li><li>Bipartite graph</li><li>Star graph</li><li>Cycle graph</li><li>Path graph</li><li>Random tree</li></ul> | Graph generators for random and structured graphs          |
| [**Paths**](src/core/paths.rs)                 | <ul><li>Dijkstra's algorithm</li><li>Bellman-Ford algorithm</li><li>Floyd-Warshall algorithm</li><li>Johnson's algorithm</li><li>A* search algorithm</li><li>Iterative deepening A* (IDA*)</li></ul>                                              | Shortest paths algorithms                                  |
| [**MST**](src/core/mst.rs)                     | <ul><li>Prim's algorithm</li><li>Kruskal's algorithm</li><li>Borůvka's algorithm</li></ul>                                                                                                                                                        | Minimum spanning tree algorithms                           |
| [**Traversal**](src/core/traversal.rs)         | <ul><li>Breadth-first search (BFS)</li><li>Depth-first search (DFS)</li><li>Iterative deepening DFS (IDDFS)</li><li>Bidirectional search</li></ul>                                                                                                | Graph traversal algorithms                                 |
| [**Metrics**](src/core/metrics.rs)             | <ul><li>Diameter</li><li>Radius</li><li>Average clustering coefficient</li><li>Clustering coefficient (local)</li><li>Average path length</li><li>Transitivity</li><li>Triangles count</li><li>Assortativity coefficient</li></ul>                | Graph-level and node-level metrics                         |
| [**Subgraphs**](src/core/subgraphs.rs)         | <ul><li>Subgraph extraction</li><li>Induced subgraph</li><li>Ego graph</li><li>K-hop neighbors</li><li>Filter nodes/edges</li><li>Connected component extraction</li><li>Component subgraph</li></ul>                                             | Subgraph operations and filtering                          |
| [**Validation**](src/core/validation.rs)       | <ul><li>Graph connectivity check</li><li>DAG validation</li><li>Bipartite check</li><li>Negative weights detection</li><li>Self-loops detection</li><li>Component counting</li><li>Algorithm precondition validators</li></ul>                    | Graph property validation utilities                        |
| [**Parallel**](src/core/parallel.rs)           | <ul><li>Parallel BFS</li><li>Parallel degree computation</li><li>Parallel clustering coefficients</li><li>Parallel triangles counting</li><li>Parallel PageRank</li><li>Parallel shortest paths</li><li>Parallel connected components</li></ul>   | Parallel implementations of a few popular graph algorithms |
| [**Pool**](src/core/pool.rs)                   | <ul><li>NodeMap pool</li><li>NodeSet pool</li><li>NodeQueue pool</li><li>Thread-local pooling</li></ul>                                                                                                                                           | Memory pooling for performance optimization                |
| [**Visualization**](src/core/visualization.rs) | <ul><li>ASCII art visualization</li><li>Force-directed layout</li><li>Circular layout</li><li>Grid layout</li><li>Hierarchical layout</li><li>HTML generation</li><li>SVG export</li><li>PNG export</li><li>D3.js JSON format</li></ul>           | Graph visualization and layout algorithms                  |

#### Extensions

| Module                                  | Feature/Algorithm                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Notes                                         |
|-----------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------|
| [**Centrality**](src/centrality/)       | <ul><li>Degree centrality</li><li>Closeness centrality</li><li>Betweenness centrality (node and edge)</li><li>Eigenvector centrality</li><li>PageRank centrality</li><li>Katz centrality</li><li>Harmonic centrality</li><li>Local reaching centrality</li><li>Global reaching centrality</li><li>Voterank centrality</li><li>Laplacian centrality</li><li>Percolation centrality</li></ul>                                                                                                                                                                                                                        | Centrality measures for node importance       |
| [**Links**](src/links/)                 | <ul><li>Resource allocation index</li><li>Jaccard coefficient</li><li>Adamic-Adar index</li><li>Preferential attachment</li><li>Common neighbors</li><li>CN Soundarajan-Hopcroft</li><li>RA index Soundarajan-Hopcroft</li><li>Within-inter-cluster ratio</li><li>Common neighbor centrality</li></ul>                                                                                                                                                                                                                                                                                                             | Link prediction algorithms                    |
| [**Community**](src/community/)         | <ul><li>Label propagation</li><li>Louvain method</li><li>Girvan-Newman algorithm</li><li>Spectral clustering</li><li>Personalized PageRank</li><li>Infomap</li><li>Connected components</li></ul>                                                                                                                                                                                                                                                                                                                                                                                                                  | Community detection and clustering algorithms |
| [**Approximation**](src/approximation/) | <ul><li>Node connectivity (BFS-based)</li><li>Maximum independent set (greedy)</li><li>Maximum clique (greedy heuristic)</li><li>Clique removal</li><li>Large clique size</li><li>Average clustering coefficient (approximate)</li><li>Densest subgraph (greedy peeling)</li><li>Diameter lower bound</li><li>Minimum weighted vertex cover (greedy)</li><li>Minimum maximal matching (greedy)</li><li>Ramsey number R(2,t) approximation</li><li>TSP approximations (greedy, simulated annealing, threshold accepting, and Christofides)</li><li>Treewidth decompositions (min degree, and min fill-in)</li></ul> | Approximation algorithms for NP-hard problems |

### Installation

```shell
cargo add graphina
```

Or add this to your `Cargo.toml`:

```toml
[dependencies]
graphina = "0.4.0-a1"
```

*Graphina requires Rust 1.86 or later.*

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

#### Graph Builder API

```rust
use graphina::core::builders::UndirectedGraphBuilder;

// Use graph builder API to create an undirected graph
let g = UndirectedGraphBuilder::<i32, f64>::undirected()
.with_capacity(4, 3)
.add_node(1)
.add_node(2)
.add_node(3)
.add_edge(0, 1, 1.0)
.add_edge(1, 2, 2.0)
.build()
.unwrap();
```

#### Seeded Generators

```rust
use graphina::core::generators::{erdos_renyi_graph, barabasi_albert_graph};
use graphina::core::types::Undirected;

// Use 42 for pseudo random seed (for deterministic results)
let er = erdos_renyi_graph::<Undirected>(100, 0.2, 42).unwrap();
let ba = barabasi_albert_graph::<Undirected>(1000, 3, 42).unwrap();
```

See the [examples](examples) and [tests](tests) directories for more usage examples.

---

### Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to make a contribution.

### Logo

The mascot is named "Graphina the Dinosaur".
As the name implies, she's a dinosaur, however, she herself thinks she's a dragon.

The logo was created using GIMP, ComfyUI, and a Flux Schnell v2 model.

### Licensing

Graphina is licensed under either of these:

* MIT License ([LICENSE-MIT](LICENSE-MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

PyGraphina is licensed under the MIT License ([LICENSE](pygraphina/LICENSE)).
