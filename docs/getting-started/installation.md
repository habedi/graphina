# Installation

To add `graphina` to your project, run:

```bash
cargo add graphina
```

Or manually add it to your `Cargo.toml`:

```toml
[dependencies]
graphina = "0.4.0-alpha.2"
```

## Feature Flags

Graphina is modular. Enable only what you need using the respective feature flags.

*   `centrality`: Centrality measures (PageRank, Betweenness, etc.)
*   `community`: Community detection (Louvain, Label Propagation, etc.)
*   `links`: Link prediction algorithms (like Jaccard and Adamic-Adar indexes)
*   `approximation`: Approximation algorithms for NP-hard problems
*   `metrics`: Graph metrics (like Diameter, Clustering Coefficient)
*   `mst`: Minimum Spanning Tree algorithms
*   `traversal`: Graph traversal algorithms (BFS and DFS)
*   `subgraphs`: Subgraph extraction and filtering
*   `parallel`: Parallel algorithm implementations

```toml
[dependencies]
graphina = { version = "0.4.0-alpha.2", features = ["centrality", "parallel"] }
```

You can also add features via command line:

```bash
cargo add graphina --features centrality,parallel
```
