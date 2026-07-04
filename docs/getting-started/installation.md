# Installation

To add `graphina` to your project, run:

```bash
cargo add graphina
```

Or manually add it to your `Cargo.toml`:

```toml
[dependencies]
graphina = "0.4.0-alpha.3"
```

Replace `0.4.0-alpha.3` with the latest version or the version you want to use.

   !!! note "Note"
        `graphina` requires Rust 1.85 or newer.

## Feature Flags

Enable only what you need using the respective feature flags.

*   `centrality`: Centrality measures (PageRank, betweenness, etc.)
*   `community`: Community detection (Louvain, label propagation, etc.)
*   `links`: Link prediction algorithms (like Jaccard and Adamic-Adar indexes)
*   `approximation`: Algorithms for comutationally hard graph problems
*   `metrics`: Graph metrics (like graph diameter and clustering coefficient)
*   `mst`: Minimum Spanning Tree algorithms
*   `traversal`: Graph traversal algorithms (like BFS and DFS)
*   `subgraphs`: Subgraph extraction and filtering
*   `parallel`: Parallel implementations of a subset of algorithms

```toml
[dependencies]
graphina = { version = "0.4.0-alpha.3", features = ["centrality", "parallel"] }
```

You can also add features via command line:

```bash
cargo add graphina --features centrality,parallel
```
