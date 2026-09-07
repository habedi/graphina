# `community` Semantics

Non-obvious semantics, return-type choices, and edge-case behavior for this module. Crate-wide rules live in the
root AGENTS.md; every function here is gated behind the module's feature flag unless the module is `core`.

Detection functions return `Result<Vec<Vec<NodeId>>>` (communities) or `Result<Vec<usize>>` (per-node labels in internal node order); the
connected-component family returns plain collections.

- `louvain(graph, seed)`: modularity optimization with aggregation; nonnegative `f64` weights; a graph with no edges puts each node in its own
  community.
- `label_propagation(graph, max_iter, seed)` and `infomap(graph, max_iter, seed)`: return `Result<Vec<usize>>`; treat the graph as undirected; error
  on an empty graph or `max_iter == 0`. `label_propagation_map` and `infomap_map` are the `NodeMap<usize>` facades.
- `connected_components`, `weakly_connected_components`, `strongly_connected_components`: plain `Vec<Vec<NodeId>>` (no `Result`);
  `connected_components_map` returns `NodeMap<usize>`. SCC uses Tarjan; the undirected and weak variants coincide on undirected graphs.
- `girvan_newman(graph, target_communities)`: iterative edge-betweenness removal; expensive, not for large graphs; errors if it cannot reach
  `target_communities`.
- `spectral_embeddings(graph, k)` and `spectral_clustering(graph, k, seed)`: unnormalized Laplacian; require `0 < k <= n`; clustering applies k-means
  over the embedding.
