# `centrality` Semantics

Non-obvious semantics, return-type choices, and edge-case behavior for this module. Crate-wide rules live in the
root AGENTS.md; every function here is gated behind the module's feature flag unless the module is `core`.

Most functions return `Result<NodeMap<f64>>`. Iterative methods take explicit `max_iter` and `tolerance` and return `ConvergenceFailed` rather than
looping forever.

- `degree_centrality`, `in_degree_centrality`, `out_degree_centrality`: raw counts, not normalized. In undirected graphs, total degree, in-degree, and
  out-degree all count self-loops as 2. In directed graphs, a self-loop counts as 1 for in-degree and 1 for out-degree (summing to 2 for total
  degree). Succeed on an empty graph with an empty map.
- `betweenness_centrality` and `edge_betweenness_centrality`: take a `normalized: bool` and an `f64`-weighted graph; Brandes' algorithm over BFS, so
  edge weights are ignored; error on an empty graph. Edge betweenness stores both `(u, v)` and `(v, u)` for undirected graphs.
- `closeness_centrality`: Wasserman-Faust correction for disconnected graphs; a node with no reachable neighbors scores `0.0`.
- `eigenvector_centrality`: power iteration on `A + I` for both directed and undirected graphs (the left eigenvector when directed);
  returns the unit-L2-norm vector as NetworkX does; a graph with no edges yields the uniform unit vector (`1/sqrt(n)` per node).
- `pagerank`: takes `damping`, `max_iter`, `tolerance`, and optional `nstart`; stops when the L1 change of the rank vector is below
  `tolerance * n`, as NetworkX does (the same rule applies to `personalized_page_rank` and `pagerank_parallel`); result sums to `1.0`; dangling nodes redistribute uniformly; a single
  node scores `1.0`.
- `personalized_page_rank` takes `personalization: Option<Vec<f64>>`, `damping`, `tol`, and `max_iter`, returning a raw `Vec<f64>` aligned to internal
  node order. It is re-exported as `personalized_pagerank_vec`; `personalized_pagerank` is the `NodeMap` facade over it. Both require `damping` in
  `(0, 1)` and `max_iter > 0`, and a `personalization` vector must have exactly one entry per node (`InvalidArgument` otherwise).
- `katz_centrality`: takes `alpha`, an optional per-node `beta` closure, `max_iter`, and `tolerance`; returns `Result<NodeMap<f64>, GraphinaError>` to
  handle convergence issues.
- `voterank(graph, num_seeds) -> Vec<NodeId>`: selector-style, returns a plain vector, never a `Result`; stops early when no node has positive votes.
  In directed graphs a node votes for its in-neighbors and the decay rate is the average out-degree, matching NetworkX.
- `local_reaching_centrality`, `global_reaching_centrality`, `laplacian_centrality`: `Result<NodeMap<f64>>`. Local reaching is the
  proportion of the other nodes reachable within `distance` hops (Mones et al., the NetworkX definition for unweighted graphs);
  global reaching is the same measure with no hop limit.
