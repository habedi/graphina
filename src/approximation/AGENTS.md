# `approximation` Semantics

Non-obvious semantics, return-type choices, and edge-case behavior for this module. Crate-wide rules live in the
root AGENTS.md; every function here is gated behind the module's feature flag unless the module is `core`.

Heuristics for NP-hard problems. Set/value returning functions: `min_weighted_vertex_cover`, `maximum_independent_set`, `max_clique`,
`clique_removal` (returns `Vec<HashSet<NodeId>>`), `large_clique_size` (returns `usize`), `average_clustering` (returns `f64`),
`min_maximal_matching` (returns `HashSet<(NodeId, NodeId)>`), `ramsey_r2` (returns `(HashSet, HashSet)`), `densest_subgraph`, `treewidth_min_degree`/
`treewidth_min_fill_in` (return `(usize, Vec<NodeId>)`), and `local_node_connectivity` (returns `usize`) return collections or plain values with no
`Result` (except TSP).

- TSP: `greedy_tsp(graph, start)` is a greedy nearest-neighbor heuristic over `f64` weights. The returned tour is a cycle (`tour[0] == tour[last]`).
- `min_weighted_vertex_cover` is a greedy maximum-degree heuristic; edge weights are ignored, and the guarantee is logarithmic, not a
  constant factor.
- `local_node_connectivity` takes an `f64`-weighted graph and finds vertex-disjoint paths by BFS, so edge weights are ignored.
