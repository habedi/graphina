# `parallel` Semantics

Non-obvious semantics, return-type choices, and edge-case behavior for this module. Crate-wide rules live in the
root AGENTS.md; every function here is gated behind the module's feature flag unless the module is `core`.

Rayon-backed counterparts that mirror sequential algorithms over `core` and require `A: Sync` and `W: Sync`.
All return collections (`HashMap`/`Vec`) rather than `Result`, except `pagerank_parallel`, which returns `Result` and rejects an `nstart`
that sums to zero like the sequential `pagerank`. Results are independent of thread count.

- `bfs_parallel(graph, starts)` and `shortest_paths_parallel(graph, sources)` run one search per source and return results in input order (a source that is not in the graph yields an empty result at its position); shortest
  paths are unweighted (hop counts).
- `degrees_parallel`, `clustering_coefficients_parallel`, `triangles_parallel`, `connected_components_parallel` (and its `_list` variant;
  currently a sequential BFS kept for API parity, since component discovery is inherently ordered),
  `pagerank_parallel` (weight aware and stopping on the same L1 change rule as the sequential `pagerank`; takes
  `nstart: Option<&HashMap<NodeId, f64>>`),
  `closeness_centrality_parallel`, and `all_pairs_shortest_path_length_parallel`
  return per-node maps or path results.
