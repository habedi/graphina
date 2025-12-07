# Installation

Add `graphina` to your `Cargo.toml`:

```toml
[dependencies]
graphina = "0.3.0-alpha.4"
```

## Feature Flags

Graphina is modular. Enable only what you need:

*   `centrality`: Centrality measures (PageRank, Betweenness, etc.)
*   `community`: Community detection (Louvain, Label Propagation)
*   `parallel`: Parallel algorithm implementations (requires `rayon`)
*   `visualization`: Visualization helpers

```toml
[dependencies]
graphina = { version = "0.3.0-alpha.4", features = ["centrality", "parallel"] }
```
