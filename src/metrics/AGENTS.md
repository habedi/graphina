# `metrics` Semantics

Non-obvious semantics, return-type choices, and edge-case behavior for this module. Crate-wide rules live in the
root AGENTS.md; every function here is gated behind the module's feature flag unless the module is `core`.

Distance metrics return `Option` (`None` for empty or disconnected); ratio metrics return plain `f64` (`0.0` on degenerate input). Weights are ignored
by the BFS-based metrics; only `assortativity` uses degree. On directed graphs `assortativity` correlates the out-degree of each edge's
source with the in-degree of its target, as NetworkX does.

- `diameter`, `radius`, `average_path_length`: `Option<usize>`/`Option<f64>`; `None` if empty or disconnected; a single node gives `Some(0)`/
  `Some(0.0)`.
- `average_clustering_coefficient`, `transitivity`, `assortativity`: plain `f64` in a bounded range; `0.0` when undefined (no triangles, no triples,
  or a zero-variance degree sequence).
- `clustering_coefficient(graph, node) -> f64` and `triangles(graph, node) -> usize`: per-node; `0.0`/`0` for degree below 2. On directed
  graphs `clustering_coefficient` is Fagiolo's directed clustering and `transitivity` the successor-triad ratio, both as in NetworkX;
  `triangles` counts closed pairs among out-neighbors, which NetworkX does not define.
