# `core` (Always Compiled) Semantics

Non-obvious semantics, return-type choices, and edge-case behavior for this module. Crate-wide rules live in the
root AGENTS.md; every function here is gated behind the module's feature flag unless the module is `core`.

- `BaseGraph<A, W, Ty>` is the central type; `A` is the node attribute, `W` the edge weight, and `Ty` the `Directed` or `Undirected` marker.
  `Graph<A, W>` and `Digraph<A, W>` are the undirected and directed aliases. `degree`, `in_degree`, and `out_degree` return `Option<usize>` (`None`
  for a missing node); for undirected graphs in-degree and out-degree both equal the total degree. A self-loop counts twice toward the degree in
  both directed and undirected graphs, as in NetworkX. `density` returns `0.0` for fewer than two nodes.
  Graphs are simple: `add_edge` updates the weight of an existing edge instead of creating a parallel edge, and `add_edge_if_absent` inserts without
  overwriting an existing weight. `add_edge`, `add_edge_if_absent`, and `find_edge` check both directions on undirected graphs.
- `GraphinaError` (in `core::error`) is the single error type, with constructor helpers (`invalid_graph`, `node_not_found`, `no_path`,
  `convergence_failed`, and so on) and `From` impls for `io::Error`, `serde_json::Error`, and the bincode codec errors. `Result<T>` aliases
  `Result<T, GraphinaError>`.
- Builders: `AdvancedGraphBuilder` (with `DirectedGraphBuilder`/`UndirectedGraphBuilder` aliases) validates on `build`, rejecting out-of-bounds edge
  endpoints and, when configured, self-loops or parallel edges. The simpler `GraphBuilder` in `core::types` skips out-of-bounds edges in `build` and
  rejects them in `try_build`. `TopologyBuilder` has constructors (`complete`, `cycle`, `path`, `star`, `grid`) that
  return the graph directly and yield an empty graph rather than erroring on degenerate sizes.
- Serialization: `save_json`/`load_json`, `save_binary`/`load_binary`, and `save_graphml` round-trip through the index-based `SerializableGraph`. The
  `_strict` loaders (`load_json_strict`, `load_binary_strict` and `try_from_serializable`) additionally validate that the serialized directedness
  matches the target type; the plain loaders do not.
- Paths: `dijkstra`/`dijkstra_path_f64` (nonnegative weights, return `Result`), `bellman_ford` (negatives, returns `Option`, `None` on negative
  cycle), `a_star` (admissible heuristic, returns `Result<Option<(W, Vec<NodeId>)>>`), `floyd_warshall`, and `johnson` (all-pairs, return `Option`,
  `None` on negative cycle). Distance maps use `None` for unreachable nodes; the source has distance `Some(0)` (or `Some(0.0)`) and no predecessor.
- Generators: `erdos_renyi_graph`, `complete_graph`, `bipartite_graph`, `star_graph`, `cycle_graph` (`n >= 1`; `n = 1` is a self-loop and `n = 2` a single edge, as in NetworkX), `watts_strogatz_graph` (`k`
  even and `< n`), and `barabasi_albert_graph` (`n >= m`). Each takes a `seed` where randomized and returns `InvalidArgument` on out-of-range
  parameters.
- Validation: boolean predicates (`is_empty`, `is_connected`, `has_negative_weights`, `has_self_loops`, `is_dag`, `is_bipartite`, `count_components`)
  and the `require_*` and `validate_*` validator families that return a `Result<(), GraphinaError>` (for use as algorithm preconditions).
