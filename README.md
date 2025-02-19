# Graphina

[![Tests](https://img.shields.io/github/actions/workflow/status/habedi/graphina/tests.yml?label=tests&style=popout-square&logo=github)](https://github.com/habedi/graphina/actions/workflows/tests.yml)
[![Code Coverage](https://img.shields.io/codecov/c/github/habedi/graphina?style=popout-square&logo=codecov)](https://codecov.io/gh/habedi/graphina)
[![CodeFactor](https://img.shields.io/codefactor/grade/github/habedi/graphina?style=popout-square&logo=codefactor)](https://www.codefactor.io/repository/github/habedi/graphina)
[![Crates.io](https://img.shields.io/crates/v/graphina.svg?style=popout-square&color=fc8d62&logo=rust)](https://crates.io/crates/graphina)
[![Docs.rs](https://img.shields.io/badge/docs.rs-graphina-66c2a5?style=popout-square&logo=docs.rs)](https://docs.rs/graphina)
[![Downloads](https://img.shields.io/crates/d/graphina?style=popout-square&logo=rust)](https://crates.io/crates/graphina)
[![MSRV](https://img.shields.io/badge/MSRV-1.83.0-orange?style=popout-square&logo=rust&label=msrv)](https://github.com/rust-lang/rust/releases/tag/1.83.0)
[![Docs](https://img.shields.io/badge/docs-latest-3776ab?style=flat&logo=readthedocs)](docs)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-007ec6?style=flat&logo=open-source-initiative)](https://github.com/habedi/graphina)

Graphina is a high-level graph data science library for Rust.
It provides the common data structures and algorithms used for analyzing the graphs of real-world networks like social,
transportation, and biological networks.

## Features

* **Graphs**:
    - [x] Directed and undirected graphs
    - [x] Weighted and unweighted graphs

* **IO**:
    - [x] Edge list (CSV)
    - [ ] Adjacency list (TXT)
    - [ ] GraphML
    - [ ] GML
    - [ ] JSON

* **Generators**:
    - [x] Erdős–Rényi Graph (Random)
    - [x] Complete Graph
    - [x] Bipartite Graph
    - [x] Star Graph
    - [x] Cycle Graph
    - [x] Watts–Strogatz Graph (Small-World)
    - [x] Barabási–Albert Graph (Scale-Free)

* **Algorithms**:
    - [ ] Graph traversal
    - [ ] Shortest paths
    - [ ] Minimum spanning tree
    - [ ] Connected components
    - [ ] Clustering, partitioning, and community detection
    - [ ] Centrality
    - [ ] Graph matching
    - [ ] Graph visualization

## Installation

```
cargo add graphina
```

*Graphina requires Rust 1.83 or later.*

## Documentation

See the [docs](docs/README.md) for the latest documentation.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to make a contribution.

## License

This project is licensed under either of these:

* MIT License ([LICENSE-MIT](LICENSE-MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
