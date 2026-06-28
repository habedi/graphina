# AGENTS.md

This file provides guidance to coding agents collaborating on this repository.

## Mission

Graphina is a graph data science library for Rust.
It provides graph data structures and a wide range of ready-to-use algorithms for analyzing real-world
networks, such as social, transportation, and biological networks.
The goal is to be as feature-rich as NetworkX while keeping the speed and performance of Rust, and to offer
a higher-level API than libraries like petgraph and rustworkx.
A companion Python library, PyGraphina, exposes Graphina to Python through maturin.
Priorities, in order:

1. Correct, well-tested implementations of graph algorithms.
2. Clean, idiomatic Rust with safe abstractions and a high-level, ergonomic API.
3. Clear separation between the core library and the optional, feature-gated extensions.
4. Maintainable code with consistent error handling and documentation.

## Core Rules

- Use English for code, comments, docs, and tests.
- Never use `.unwrap()` or `.expect()` in non-test code (enforced by `make lint` via `clippy::unwrap_used` and
  `clippy::expect_used`). Production code should never panic.
- Algorithms return `Result<_, graphina::core::error::GraphinaError>`. Selector-style helpers that pick nodes
  (like `voterank`) may return plain collections.
- Top-level extension modules may depend only on `core`, never on each other (enforced by
  `make check-module-deps`).
- Gate every extension behind its feature flag with `#[cfg(feature = "...")]`. Enable only the required
  features to minimize size and compile time.
- Prefer small, focused changes over large refactoring.
- Add comments only when they clarify non-obvious behavior.
- Do not add features, error handling, or abstractions beyond what is needed for the current task.
- Add tests for every bug fix and new feature to prevent regression.
- Follow red-green TDD: write a failing test first, then the code to pass it (see Test-Driven Development).

Quick examples:

- Good: add a `local_reaching_centrality` function in `src/centrality/other.rs` that returns
  `Result<NodeMap<f64>, GraphinaError>`, gated behind `#[cfg(feature = "centrality")]`, with unit tests for
  the empty-graph and disconnected-graph cases.
- Good: add a property-based invariant to `tests/property_based_tests.rs` asserting that
  `connected_components` partitions every node into exactly one component.
- Bad: write `use crate::metrics::triangles;` inside `src/community/` to reuse a triangle count. Extensions
  may import only from `core`; move the shared helper into `core` or duplicate the small piece.
- Bad: call `.unwrap()` on a `Result` in non-test code because "the graph is obviously non-empty". Production
  code must never panic; return a `GraphinaError` instead.
- Bad: add a new algorithm without a feature gate, so it compiles into the `default = []` build.

## Writing Style

- Use Oxford commas in inline lists: "a, b, and c" not "a, b, c".
- Do not use em dashes. Restructure the sentence, or use a colon or semicolon instead.
- Avoid colorful adjectives and adverbs. Write "graph generator" not "powerful graph generator".
- Use noun phrases for checklist items, not imperative verbs. Write "negative weight detection" not
  "detect negative weights".
- Headings in Markdown files must be in title case: "Build from Source" not "Build from source". Minor words
  (a, an, the, and, but, or, for, in, on, at, to, by, of) stay lowercase unless they are the first word.
- Write correct and complete sentences.
- Avoid made-up words, abbreviations, and colons in the middle of sentences.

## Repository Layout

- `src/core/`: Always-enabled core library. Basic graph types, builders, IO, serialization, shortest paths,
  validation, generators, and experimental memory pooling.
- `src/centrality/`, `src/community/`, `src/links/`, `src/metrics/`, `src/mst/`, `src/traversal/`,
  `src/approximation/`, `src/parallel/`, `src/subgraphs/`, `src/visualization/`: Optional extensions, each
  behind a Cargo feature of the same name. The `all` feature enables them together.
- `src/lib.rs`: Crate root with module declarations, crate-level docs, and API conventions.
- `src/settings.rs`: Runtime settings (such as the `DEBUG_GRAPHINA` toggle).
- `pygraphina/`: PyGraphina, the Python bindings crate built with maturin and published to PyPI as
  `pygraphina`. Contains its own `Cargo.toml`, `src/`, `tests/`, type stubs (`pygraphina.pyi`), and docs.
- `benches/`: Criterion benchmarks (`graph_benchmarks`, `algorithm_benchmarks`, `project_benchmarks`).
- `tests/`: Workspace integration, end-to-end, regression, property-based, and visualization tests, plus
  `tests/testdata/` (downloaded via `make testdata`).
- `docs/`, `mkdocs.yml`: MkDocs documentation site.
- `Makefile`: GNU Make wrapper around `cargo`, maturin, and tooling commands.
- `rust-toolchain.toml`: Pinned Rust toolchain (1.85.0 as MSRV) with `rustfmt` and `clippy`.

## Architecture

The crate is split into a core library and a set of independent extensions.

- `core` is always compiled and contains everything the extensions build on: graph `Types` (directed and
  undirected, weighted and unweighted, with `NodeId`/`EdgeId` wrappers and `NodeMap`/`EdgeMap` aliases),
  `Builders`, `IO` (edge and adjacency lists), `Serialization` (JSON, binary, and GraphML), `Paths`
  (Dijkstra, Bellman-Ford, Floyd-Warshall, Johnson, A*, and IDA*), `Generators`, `Validation`, and an
  experimental `pool` module.
- Extensions are feature-gated modules outside `core` for higher-level tasks: centrality, community
  detection, link prediction, metrics, minimum spanning trees, traversal, approximation of NP-hard problems,
  parallel algorithms, subgraph extraction, and visualization.
- Graphina builds on `petgraph` for the underlying graph storage and uses `nalgebra`, `sprs`, and `rayon`
  for numerical and parallel work, and `plotters` (optional) for visualization rendering.

### Key Design Decisions

- Module independence: an extension module may import from `core` but not from another extension. This keeps
  features composable and is checked by `make check-module-deps`.
- Unified error handling: a single `GraphinaError` type and `Result` alias live in `core::error`, and
  algorithms return `Result` rather than panicking.
- Feature gating: each extension is optional. `default = []` enables only `core`; downstream users opt in to
  what they need, and `all` turns everything on for development and testing.
- The `pool` feature is experimental and its API may change. Gate usage with `cfg(feature = "pool")`.
- Public re-exports and facades give consistent entry points (for example, the personalized PageRank vector
  and `NodeMap` facade APIs).
- PyGraphina is a thin binding layer over the core crate, built as a separate workspace member so the Rust
  library has no Python dependency.

### Dependency Boundaries

Graphina is a single crate, so the boundary is enforced by `make check-module-deps` (a `use crate::<module>`
grep) rather than by the compiler. Keep the dependency direction acyclic and hub-and-spoke:

0. `core` sits at the bottom. It depends on no other Graphina module.
1. Each extension (`approximation`, `centrality`, `community`, `links`, `metrics`, `mst`, `parallel`,
   `subgraphs`, `traversal`, `visualization`) may depend on `core` only.
2. No extension may depend on another extension, not through a `use crate::<other>` import and not through a
   fully-qualified `crate::<other>::` path. If two extensions need the same helper, move it into `core` or
   duplicate the small piece.
3. `parallel` is not exempt: a parallel algorithm reimplements over `core` rather than calling the sequential
   version in another extension.
4. PyGraphina depends on the Graphina crate, never the reverse. Keep Python concerns out of `core`.

### Encapsulation Rule

`core` re-exports its submodules with broad `pub mod`, so almost everything in `core` is technically
reachable. The deliberate public contract is narrower: the graph types and aliases (`BaseGraph`, `NodeId`,
`EdgeId`, `Graph`, `Digraph`, `NodeMap`, `EdgeMap`, `Directed`, `Undirected`), the `core::error` types
(`GraphinaError`, `Result`), the builders, the IO and serialization entry points, the path algorithms, the
generators, and the validation helpers. Treat the rest as internal: the `pub(crate)` inner fields of `NodeId`
and `EdgeId`, the `pool` module (experimental, gate with `cfg(feature = "pool")`), and the deprecated
`edge_attr`/`edge_attr_mut` methods. Do not add a re-export "just for now" to reach an internal item from an
extension; promote it to the contract deliberately or keep it private.

### Cross-Cutting Invariants

- NodeId stability: `BaseGraph` wraps petgraph's `StableGraph`, so a `NodeId` stays valid across node
  removal and indices are never recycled. Algorithms may hold `NodeId`s across mutation.
- Subgraph extraction remaps: every `SubgraphOps` method that returns a new `BaseGraph` (`subgraph`,
  `induced_subgraph`, `ego_graph`, `component_subgraph`, `filter_nodes`, `filter_edges`) assigns fresh,
  sequential `NodeId`s in the returned graph. Do not assume a returned node matches its source `NodeId`.
- Return-type convention: algorithms return `Result<_, GraphinaError>`. Selector-style and partition-style
  helpers that cannot fail return plain collections instead: `voterank` (`Vec<NodeId>`),
  `connected_components`, `weakly_connected_components`, `strongly_connected_components`
  (`Vec<Vec<NodeId>>`), and `connected_components_map` (`NodeMap<usize>`). Some metrics return `Option`
  (`diameter`, `radius`, `average_path_length` return `None` for an empty or disconnected graph).
- Weight totality: algorithms that need a total order over weights take `OrderedFloat<f64>` rather than
  `f64` (for example `betweenness_centrality`, `harmonic_centrality`, the `approximation` connectivity and
  diameter functions, and `boruvka_mst`). Wrap `f64` weights in `ordered_float::OrderedFloat` for these.
- Negative weights: `dijkstra` and `a_star` return an error on a negative weight; `bellman_ford`,
  `floyd_warshall`, and `johnson` accept negatives and return `None` on a negative cycle. Pathfinding
  assumes a non-empty graph; validate with `core::validation` first.
- Fixed attribute types in IO and generators: `core::io` reads and writes graphs with `i32` node attributes
  and `f32` edge weights; `core::generators` produces `u32` node attributes and `f32` edge weights. Convert
  with `BaseGraph::convert` or `map_node_attrs`/`map_edge_weights` if you need other types.
- Feature-gated rendering: the `visualization` D3 export (`to_d3_graph`, `to_d3_json`), `to_ascii_art`, and
  `save_as_html` are available whenever the `visualization` feature is on, but `save_as_png` and
  `save_as_svg` additionally require the `plotters` backend pulled in by that same feature.

## Component APIs

Signatures are self-describing; read them from the source rather than this file. This section pins only the
non-obvious semantics, the return-type choice, and the edge-case behavior a caller cannot infer from the
type. Every function listed is gated behind its module's feature flag.

### `core` (always compiled)

- `BaseGraph<A, W, Ty>` is the central type; `A` is the node attribute, `W` the edge weight, and `Ty` the
  `Directed` or `Undirected` marker. `Graph<A, W>` and `Digraph<A, W>` are the undirected and directed
  aliases. `degree`, `in_degree`, and `out_degree` return `Option<usize>` (`None` for a missing node); for
  undirected graphs in-degree and out-degree both equal the total degree. `density` returns `0.0` for fewer
  than two nodes. `add_edge_if_absent` and `find_edge` check both directions on undirected graphs.
- `GraphinaError` (in `core::error`) is the single error type, with constructor helpers (`invalid_graph`,
  `node_not_found`, `no_path`, `convergence_failed`, and so on) and `From` impls for `io::Error`,
  `serde_json::Error`, and the bincode codec errors. `Result<T>` aliases `Result<T, GraphinaError>`.
- Builders: `AdvancedGraphBuilder` (with `DirectedGraphBuilder`/`UndirectedGraphBuilder` aliases) validates
  on `build`, rejecting out-of-bounds edge endpoints and, when configured, self-loops or parallel edges.
  `TopologyBuilder` has constructors (`complete`, `cycle`, `path`, `star`, `grid`) that return the graph
  directly and yield an empty graph rather than erroring on degenerate sizes.
- Serialization: `save_json`/`load_json`, `save_binary`/`load_binary`, and `save_graphml` round-trip through
  the index-based `SerializableGraph`. The `_strict` loaders (`load_json_strict`, `load_binary_strict` and
  `try_from_serializable`) additionally validate that the serialized directedness matches the target type;
  the plain loaders do not.
- Paths: `dijkstra`/`dijkstra_path_f64` (nonnegative weights), `bellman_ford` (negatives, `None` on negative
  cycle), `a_star` (admissible heuristic, returns the path directly), `floyd_warshall`, and `johnson`
  (all-pairs, `None` on negative cycle). Distance maps use `None` for unreachable nodes; the source has
  distance `Some(0)` and no predecessor.
- Generators: `erdos_renyi_graph`, `complete_graph`, `bipartite_graph`, `star_graph`, `cycle_graph`
  (requires `n >= 3`), `watts_strogatz_graph` (`k` even and `< n`), and `barabasi_albert_graph` (`n >= m`).
  Each takes a `seed` where randomized and returns `InvalidArgument` on out-of-range parameters.
- Validation: boolean predicates (`is_connected`, `has_negative_weights`, `is_dag`, `is_bipartite`,
  `count_components`) and `require_*`/`validate_*` variants that return a `GraphinaError` for use as
  algorithm preconditions.

### `centrality`

Most functions return `Result<NodeMap<f64>>`. Iterative methods take explicit `max_iter` and `tolerance` and
return `ConvergenceFailed` rather than looping forever.

- `degree_centrality`, `in_degree_centrality`, `out_degree_centrality`: raw counts, not normalized; a
  self-loop counts as 2 on undirected graphs. Succeed on an empty graph with an empty map.
- `betweenness_centrality` and `edge_betweenness_centrality`: take a `normalized: bool` and an
  `OrderedFloat<f64>`-weighted graph; Brandes' algorithm; error on an empty graph. Edge betweenness stores
  both `(u, v)` and `(v, u)` for undirected graphs.
- `closeness_centrality`: Wasserman-Faust correction for disconnected graphs; a node with no reachable
  neighbors scores `0.0`.
- `eigenvector_centrality`: power iteration for directed graphs, symmetric eigendecomposition for undirected
  (avoids bipartite oscillation); values normalized to sum to `n`; isolated or zero-weight graphs yield a
  uniform distribution.
- `pagerank`: takes `damping`, `max_iter`, `tolerance`, and optional `nstart`; result sums to `1.0`; dangling
  nodes redistribute uniformly; a single node scores `1.0`.
- `personalized_page_rank` returns a raw `Vec<f64>` aligned to internal node order and is re-exported as
  `personalized_pagerank_vec`; `personalized_pagerank` is the `NodeMap` facade over it. Both require
  `damping` in `(0, 1)` and `max_iter > 0`.
- `katz_centrality`: takes `alpha` (below the reciprocal of the largest eigenvalue) and an optional per-node
  `beta` closure; no normalization.
- `voterank(graph, num_seeds) -> Vec<NodeId>`: selector-style, returns a plain vector, never a `Result`;
  stops early when no node has positive votes.
- `local_reaching_centrality`, `global_reaching_centrality`, `laplacian_centrality`: `Result<NodeMap<f64>>`.

### `community`

Detection functions return `Result<Vec<Vec<NodeId>>>` (communities) or `Result<Vec<usize>>` (per-node
labels in internal node order); the connected-component family returns plain collections.

- `louvain(graph, seed)`: modularity optimization with aggregation; nonnegative `f64` weights; a graph with
  no edges puts each node in its own community.
- `label_propagation(graph, max_iter, seed)` and `infomap(graph, max_iter, seed)`: return `Result<Vec<usize>>`;
  treat the graph as undirected; error on an empty graph or `max_iter == 0`. `label_propagation_map` and
  `infomap_map` are the `NodeMap<usize>` facades.
- `connected_components`, `weakly_connected_components`, `strongly_connected_components`: plain
  `Vec<Vec<NodeId>>` (no `Result`); `connected_components_map` returns `NodeMap<usize>`. SCC uses Tarjan;
  the undirected and weak variants coincide on undirected graphs.
- `girvan_newman(graph, target_communities)`: iterative edge-betweenness removal; expensive, not for large
  graphs; errors if it cannot reach `target_communities`.
- `spectral_embeddings(graph, k)` and `spectral_clustering(graph, k, seed)`: unnormalized Laplacian; require
  `0 < k <= n`; clustering applies k-means over the embedding.

### `links`

All link-prediction scorers take an optional `ebunch: Option<&[(NodeId, NodeId)]>` (defaulting to all
unordered node pairs), operate on `f64`-weighted graphs, treat pairs as undirected, and return a plain
`Vec<((NodeId, NodeId), f64)>` (never a `Result`).

- `resource_allocation_index`, `adamic_adar_index`: sum over common neighbors; Adamic-Adar skips neighbors
  of degree `<= 1` (avoids `ln(1) = 0`). No common neighbors yields `0.0`.
- `jaccard_coefficient`: intersection over union of neighbor sets; `0.0` when the union is empty.
- `preferential_attachment`: `degree(u) * degree(v)`.
- `common_neighbor_centrality(graph, ebunch, alpha)`: `|N(u) ∩ N(v)|^alpha`.
- `common_neighbors(graph, u, v) -> usize`: plain count, not a scorer.
- Community-aware variants (`ra_index_soundarajan_hopcroft`, `cn_soundarajan_hopcroft`, `within_inter_cluster`)
  take a `community: Fn(NodeId) -> C` closure; `within_inter_cluster` also takes a `delta` smoothing constant
  that keeps the score finite when there are no inter-cluster neighbors.

### `metrics`

Distance metrics return `Option` (`None` for empty or disconnected); ratio metrics return plain `f64`
(`0.0` on degenerate input). Weights are ignored by the BFS-based metrics; only `assortativity` uses degree.

- `diameter`, `radius`, `average_path_length`: `Option<usize>`/`Option<f64>`; `None` if empty or
  disconnected; a single node gives `Some(0)`/`Some(0.0)`.
- `average_clustering_coefficient`, `transitivity`, `assortativity`: plain `f64` in a bounded range; `0.0`
  when undefined (no triangles, no triples, or a zero-variance degree sequence).
- `clustering_coefficient(graph, node) -> f64` and `triangles(graph, node) -> usize`: per-node;
  `0.0`/`0` for degree below 2.

### `mst`

`kruskal_mst`, `prim_mst`, and `boruvka_mst` each return `Result<(Vec<MstEdge<W>>, W)>` (edges plus total
weight). They error only on an empty graph and return a spanning forest (not an error) for a disconnected
graph; a single node yields an empty edge set with zero weight. Weights need a total order (use
`OrderedFloat<f64>` for floats); `boruvka_mst` additionally requires `Send + Sync` and runs its cheapest-edge
search in parallel.

### `traversal`

- `bfs(graph, start) -> Vec<NodeId>` and `dfs(graph, start) -> Vec<NodeId>`: visitation order; empty vector
  for a missing start node.
- `iddfs(graph, start, target, max_depth) -> Option<Vec<NodeId>>` and
  `bidis(graph, start, target) -> Option<Vec<NodeId>>`: return the path or `None`; `bidis` returns the
  unweighted shortest path. The `try_iddfs` and `try_bidirectional_search` variants return
  `Result<Vec<NodeId>>`, validating node existence (`node_not_found`) and distinguishing `no_path`.

### `approximation`

Heuristics for NP-hard problems. Set-returning functions (`min_weighted_vertex_cover`,
`maximum_independent_set`, `max_clique`, `min_maximal_matching`, `densest_subgraph`) return `HashSet`/`Vec`
with no `Result`; TSP functions return `Result<(Vec<NodeId>, f64)>`.

- TSP: `traveling_salesman_problem`, `nearest_neighbor`/`greedy_nearest_neighbor`, and `greedy_tsp` are all
  greedy nearest-neighbor; `christofides` is currently a placeholder that delegates to the greedy tour. The
  returned tour is a cycle (`tour[0] == tour[last]`).
- `min_weighted_vertex_cover` is a greedy 2-approximation; its `weight` argument is currently ignored.
- `approximate_diameter` computes the exact diameter (Dijkstra from every node) despite the name.
- `local_node_connectivity`, `approximate_diameter` take `OrderedFloat<f64>` weights.
- Several functions accept a placeholder parameter that is currently unused (`densest_subgraph`'s
  `iterations`, `min_weighted_vertex_cover`'s `weight`); do not rely on them having an effect.

### `parallel`

Rayon-backed counterparts that mirror sequential algorithms over `core` and require `A: Sync` and `W: Sync`.
All return collections (`HashMap`/`Vec`), not `Result`, and produce results independent of thread count.

- `bfs_parallel(graph, starts)` and `shortest_paths_parallel(graph, sources)` run one search per source and
  return results in input order; shortest paths are unweighted (hop counts).
- `degrees_parallel`, `clustering_coefficients_parallel`, `triangles_parallel`,
  `connected_components_parallel` (and its `_list` variant), and `pagerank_parallel` (same parameters as the
  sequential `pagerank`) return per-node maps.

### `subgraphs`

The `SubgraphOps` trait is implemented for `BaseGraph`. Extraction methods that build a new graph
(`subgraph`, `induced_subgraph`, `ego_graph`, `component_subgraph`) return `Result` and remap `NodeId`s in
the result; `filter_nodes` and `filter_edges` also remap but return the graph directly. Query methods
(`k_hop_neighbors`, `connected_component`) return `Vec<NodeId>` over the original ids, with `radius`/`k` of 0
returning just the start node.

### `visualization`

- `LayoutEngine::compute_layout(graph, algorithm, width, height) -> HashMap<NodeId, NodePosition>`: one of
  `ForceDirected`, `Circular`, `Hierarchical`, `Grid`, or `Random`; never errors.
- `to_d3_graph` (to the `D3Graph` struct), `to_d3_json` (to a JSON string), `to_ascii_art` (debug string),
  and `save_as_html` (interactive D3 file, references a D3 CDN) require only the node and edge attributes to
  be `Display`.
- `save_as_png` and `save_as_svg` render through `plotters` and write to disk; they need the `visualization`
  feature's `plotters` backend.

## Required Validation

Run `make lint` and `make test` for any change. Key targets:

| Target       | Command                  | What It Runs                                                                    |
|--------------|--------------------------|---------------------------------------------------------------------------------|
| Format       | `make format`            | `cargo fmt`                                                                     |
| Format Check | `make format-check`      | `cargo fmt --all --check` (non-mutating, used in CI)                            |
| Lint         | `make lint`              | `cargo clippy` with `-D warnings -D clippy::unwrap_used -D clippy::expect_used` |
| Test         | `make test`              | All workspace tests with `--features all --all-targets`, plus doctests          |
| Doctest      | `make doctest`           | Doc-comment code examples (`cargo test --doc --features all`)                   |
| Nextest      | `make nextest`           | Tests via `cargo nextest` with `--features all`                                 |
| Module Deps  | `make check-module-deps` | Verifies extensions depend only on `core`                                       |
| Build        | `make build`             | Release build                                                                   |
| Bench        | `make bench`             | Criterion benchmarks with `--features all`                                      |
| Coverage     | `make coverage`          | `cargo tarpaulin` with XML and HTML output                                      |
| Audit        | `make audit`             | `cargo audit` on dependencies                                                   |
| Deny         | `make deny`              | `cargo deny check` for advisories, license compliance, and bans                 |
| Careful      | `make careful`           | `cargo careful test --features all` for undefined-behavior checks               |
| Test Data    | `make testdata`          | Downloads datasets used in integration tests                                    |

PyGraphina targets: `make develop-py` (build and install into the active environment with maturin),
`make test-py` (pytest), `make wheel` / `make wheel-manylinux` (build wheels), and `make rundoc` (test Python
doc examples). The Python toolchain uses `uv`.

## Test-Driven Development

Develop with the red-green-refactor cycle. Write the test before the implementation.

1. Red: write a test that captures the desired behavior, then run it (`make test`, or `cargo test`
   scoped to the module) and confirm it fails for the expected reason. A test that passes before any
   code is written is not exercising the new behavior.
2. Green: write the smallest amount of code that makes the test pass. Do not add behavior the failing
   test does not require.
3. Refactor: clean up the implementation and tests while keeping them green, then rerun `make lint` and
   `make test`.

Guidelines:

- One logical behavior per cycle. Add edge cases (empty graphs, disconnected components, self-loops,
  negative weights) as separate red-green steps rather than in a single large test.
- For bug fixes, the regression test in `tests/regression_tests.rs` is the red step: it must fail on the
  current code and pass after the fix.
- Put the test where the behavior lives: `#[cfg(test)]` modules for unit-level logic, `tests/` for
  user-facing behavior, and `property_based_tests.rs` for algorithmic invariants.

## Testing Expectations

- Unit tests live in each module's source files using `#[cfg(test)]` modules.
- Workspace-level tests live in `tests/`: `integration_tests.rs`, `e2e_tests.rs`, `regression_tests.rs`,
  `property_based_tests.rs` (proptest), and `visualizations_tests.rs`.
- Some integration tests reference public datasets; run `make testdata` to fetch them first.
- Property-based tests cover algorithmic invariants; add cases when changing numerical behavior.
- Regression tests exist for fixed bugs; add one for every bug fix.
- No public API change is complete without a corresponding test.
- PyGraphina has its own tests under `pygraphina/tests/`, run with `make test-py`.

## Commit and PR Hygiene

- Keep commits scoped to one logical change.
- PR descriptions should include:
    1. Behavioral change summary.
    2. Tests added or updated.
    3. `make lint && make test` passes (yes/no).

Suggested PR checklist:

- [ ] Unit tests added or updated for logic changes
- [ ] Integration or regression test added for new user-facing behavior
- [ ] New algorithm gated behind the correct feature flag
- [ ] `make lint && make test` passes
- [ ] `make check-module-deps` passes
- [ ] Docs, README, or ROADMAP updated (if API surface changed)

## Review Guidelines (P0/P1 Focus)

Review output should be concise and include only critical issues. Do not include style-only feedback or
broad praise.

- `P0`: must-fix defects (incorrect algorithm result, a panic path such as `.unwrap()`/`.expect()` in
  non-test code, a broken build, or a broken test workflow).
- `P1`: high-priority defects (a cross-module dependency that breaks `make check-module-deps`, a missing
  feature gate, an algorithm without `Result`-based error handling where it can fail, a numerical change with
  no property-based or regression test, or wrong empty/disconnected/self-loop handling).

Use this review format:

1. `Severity` (`P0`/`P1`)
2. `File:line`
3. `Issue`
4. `Why it matters`
5. `Minimal fix direction`
