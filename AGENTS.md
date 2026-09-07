# AGENTS.md

This file provides guidance to coding agents collaborating on this repository.

## Mission

Graphina is a graph data science library for Rust.
It provides graph data structures and a wide range of ready-to-use algorithms for analyzing real-world networks, such as social, transportation, and
biological networks.
The goal is to be as feature-rich as NetworkX while keeping the speed and performance of Rust, and to offer a higher-level API than libraries like
petgraph and rustworkx.
A companion Python library, PyGraphina, exposes Graphina to Python through maturin.
Priorities, in order:

1. Correct, well-tested implementations of graph algorithms.
2. Clean, idiomatic Rust with safe abstractions and a high-level, ergonomic API.
3. Clear separation between the core library and the optional, feature-gated extensions.
4. Maintainable code with consistent error handling and documentation.

## Core Rules

- Use English for code, comments, docs, and tests.
- Never use `.unwrap()` or `.expect()` in non-test code (enforced by `make lint` via `clippy::unwrap_used` and `clippy::expect_used`). Production code
  should never panic.
- Algorithms return `Result<_, graphina::core::error::GraphinaError>`. Selector-style helpers that pick nodes (like `voterank`) may return plain
  collections.
- Top-level extension modules may depend only on `core`, never on each other (enforced by `make check-module-deps`).
- Gate every extension behind its feature flag with `#[cfg(feature = "...")]`. Enable only the required features to minimize size and compile time.
- Prefer small, focused changes over large refactoring.
- Add comments only when they clarify non-obvious behavior.
- Do not add features, error handling, or abstractions beyond what is needed for the current task.
- Add tests for every bug fix and new feature to prevent regression.
- Follow red-green TDD: write a failing test first, then the code to pass it (see Test-Driven Development).

Quick examples:

- Good: add a `local_reaching_centrality` function in `src/centrality/other.rs` that returns `Result<NodeMap<f64>, GraphinaError>`, gated behind
  `#[cfg(feature = "centrality")]`, with unit tests for the empty-graph and disconnected-graph cases.
- Good: add a property-based invariant to `tests/property_based_tests.rs` asserting that `connected_components` partitions every node into exactly one
  component.
- Bad: write `use crate::metrics::triangles;` inside `src/community/` to reuse a triangle count. Extensions may import only from `core`; move the
  shared helper into `core` or duplicate the small piece.
- Bad: call `.unwrap()` on a `Result` in non-test code because "the graph is obviously non-empty". Production code must never panic; return a
  `GraphinaError` instead.
- Bad: add a new algorithm without a feature gate, so it compiles into the `default = []` build.

## Writing Style

- Write in simple, plain English. Use short sentences and everyday words.
- Use Oxford commas in inline lists: "a, b, and c" not "a, b, c".
- Do not use em dashes. Restructure the sentence, or use a colon or semicolon instead.
- Avoid colorful adjectives and adverbs. Write "graph generator" not "powerful graph generator".
- Prefer using noun phrases for checklist items, not imperative verbs. Write "negative weight detection" not "detect negative weights".
- Headings in Markdown files must be in title case: "Build from Source" not "Build from source". Minor words
  (a, an, the, and, but, or, for, in, on, at, to, by, of, with, from) stay lowercase unless they are the first word.
- Write correct and complete sentences.
- Avoid made-up words, abbreviations, and colons in the middle of sentences.
- Don't use pretentious language and made-up words.

## Architecture

The crate is split into a core library and a set of independent extensions.

- `core` is always compiled and contains everything the extensions build on: graph `Types` (directed and undirected, weighted and unweighted, with
  `NodeId`/`EdgeId` wrappers and `NodeMap`/`EdgeMap` aliases), `Builders`, `IO` (edge and adjacency lists), `Serialization` (JSON, binary, and
  GraphML), `Paths` (Dijkstra, Bellman-Ford, Floyd-Warshall, Johnson, A*, and IDA*), `Generators`, and `Validation`.
- Extensions are feature-gated modules outside `core` for higher-level tasks: centrality, community detection, link prediction, metrics, minimum
  spanning trees, traversal, approximation of NP-hard problems, parallel algorithms, and subgraph extraction.

### Key Design Decisions

- Module independence: an extension module may import from `core` but not from another extension. This keeps features composable and is checked by
  `make check-module-deps`.
- Unified error handling: a single `GraphinaError` type and `Result` alias live in `core::error`, and algorithms return `Result` rather than
  panicking.
- Feature gating: each extension is optional. `default = []` enables only `core`; downstream users opt in to what they need, and `all` turns
  everything on for development and testing.
- Public re-exports and facades give consistent entry points (for example, the personalized PageRank vector and `NodeMap` facade APIs).
- PyGraphina is a thin binding layer over the core crate, built as a separate workspace member so the Rust library has no Python dependency.

### Dependency Boundaries

Graphina is a single crate, so the boundary is enforced by `make check-module-deps` (a `use crate::<module>` grep) rather than by the compiler. Keep
the dependency direction acyclic and hub-and-spoke:

0. `core` sits at the bottom. It depends on no other Graphina module.
1. Each extension (`approximation`, `centrality`, `community`, `links`, `metrics`, `mst`, `parallel`, `subgraphs`, `traversal`) may depend on `core`
   only.
2. No extension may depend on another extension, not through a `use crate::<other>` import and not through a fully-qualified `crate::<other>::` path.
   If two extensions need the same helper, move it into `core` or duplicate the small piece.
3. `parallel` is not exempt: a parallel algorithm reimplements over `core` rather than calling the sequential version in another extension.
4. PyGraphina depends on the Graphina crate, never the reverse. Keep Python concerns out of `core`.

### Encapsulation Rule

`core` re-exports its submodules with broad `pub mod`, so almost everything in `core` is technically reachable.
The deliberate public contract is narrower: the graph types and aliases (`BaseGraph`, `NodeId`,
`EdgeId`, `Graph`, `Digraph`, `NodeMap`, `EdgeMap`, `Directed`, `Undirected`), the `core::error` types (`GraphinaError`, `Result`), the builders, the
IO and serialization entry points, the path algorithms, the generators, and the validation helpers. Treat the rest as internal: the `pub(crate)` inner
fields of `NodeId` and `EdgeId`. Do not add a re-export "just for now" to reach an internal item from an extension; promote it to the contract
deliberately or keep it private.

### Cross-Cutting Invariants

- NodeId stability: `BaseGraph` wraps petgraph's `StableGraph`, so a `NodeId` stays valid across node removal and indices are never recycled.
  Algorithms may hold `NodeId`s across mutation.
- Subgraph extraction remaps: every `SubgraphOps` method that returns a new `BaseGraph` (`subgraph`, `induced_subgraph`, `ego_graph`,
  `component_subgraph`, `filter_nodes`, `filter_edges`) assigns fresh, sequential `NodeId`s in the returned graph. Do not assume a returned node
  matches its source `NodeId`.
- Return-type convention: algorithms return `Result<_, GraphinaError>`. Selector-style and partition-style helpers that cannot fail return plain
  collections instead: `voterank` (`Vec<NodeId>`), `connected_components`, `weakly_connected_components`, `strongly_connected_components` (
  `Vec<Vec<NodeId>>`), and `connected_components_map` (`NodeMap<usize>`). Some metrics return `Option` (`diameter`, `radius`, `average_path_length`
  return `None` for an empty or disconnected graph).
- Weight totality: the `mst` family (`kruskal_mst`, `prim_mst`, `boruvka_mst`) needs only `W: PartialOrd`, so plain `f64` weights work; the
  weights must still be totally ordered in practice, and an unordered (`NaN`) weight is rejected with `InvalidArgument`. The `centrality` and `approximation` functions, by contrast, all accept a plain
  `f64`-weighted graph: the BFS-based ones (`betweenness_centrality`, `edge_betweenness_centrality`, `local_node_connectivity`) ignore weights, and
  the
  path-based ones (`harmonic_centrality`, `closeness_centrality`, `greedy_tsp`) order distances internally.
- Negative weights: `dijkstra` and `a_star` return an error on a negative weight; `bellman_ford`, `floyd_warshall`, and `johnson` accept negatives and
  return `None` on a negative cycle. Pathfinding assumes a non-empty graph; validate with `core::validation` first. A missing or removed
  source or target node yields a `node_not_found` error from the `Result`-returning algorithms; `bellman_ford`, which returns `Option`,
  reports every live node as unreachable instead.
- Fixed attribute types in IO and generators: `core::io` reads and writes graphs with `i32` node attributes and `f32` edge weights; `core::generators`
  produces `u32` node attributes and `f32` edge weights. Convert with `BaseGraph::convert` or `map_node_attrs`/`map_edge_weights` if you need other
  types.

## Component APIs

Signatures are self-describing; read them from the source rather than this file. This section pins only the non-obvious semantics, the return-type
choice, and the edge-case behavior a caller cannot infer from the type.
Every function listed is gated behind its module's feature flag.

Per-module semantics live in `src/<module>/AGENTS.md` (for example `src/centrality/AGENTS.md`). Read the file for a module before working
under that directory.

## Required Validation

Run `make lint` and `make test` for any change. `make help` lists every target with a one-line description.

PyGraphina targets: `make develop-py` (build and install into the active environment with maturin), `make test-py` (pytest), `make wheel` /
`make wheel-manylinux` (build wheels), and `make rundoc` (test Python doc examples). The Python toolchain uses `uv`.

## Test-Driven Development

Develop with the red-green-refactor cycle. Write the test before the implementation.

1. Red: write a test that captures the desired behavior, then run it (`make test`, or `cargo test` scoped to the module) and confirm it fails for the
   expected reason. A test that passes before any code is written is not exercising the new behavior.
2. Green: write the smallest amount of code that makes the test pass. Do not add behavior the failing test does not require.
3. Refactor: clean up the implementation and tests while keeping them green, then rerun `make lint` and `make test`.

Guidelines:

- One logical behavior per cycle. Add edge cases (empty graphs, disconnected components, self-loops, negative weights) as separate red-green steps
  rather than in a single large test.
- For bug fixes, write the regression test first as the red step: it must fail on the current code and pass after the fix. A regression test is a unit
  test that happens to pin a fixed bug, so it lives in the `#[cfg(test)]` module of the source file it exercises, next to that function's other unit
  tests. Only a regression that spans several algorithms or modules (a cross-cutting consistency check) belongs in `tests/regression_tests.rs`.
- Put the test where the behavior lives: `#[cfg(test)]` modules for logic that belongs to one function or module (including single-module bug
  regressions), `tests/` for user-facing and cross-module behavior, and `property_based_tests.rs` for algorithmic invariants.

## Testing Expectations

- Unit tests live in each module's source files using `#[cfg(test)]` modules. This includes regression tests for bugs whose fix lives in a single
  function or module: co-locate them with that module's other unit tests, not in `tests/`.
- Workspace-level tests live in `tests/`: `integration_tests.rs`, `e2e_tests.rs`, `regression_tests.rs`, and `property_based_tests.rs` (proptest).
  `regression_tests.rs` holds only cross-cutting regressions, such as consistency checks across several algorithms (for example, that Kruskal, Prim,
  and Boruvka agree, or that Dijkstra and Bellman-Ford agree).
- Some integration tests reference public datasets; run `make testdata` to fetch them first.
- Property-based tests cover algorithmic invariants; add cases when changing numerical behavior.
- Regression tests exist for fixed bugs; add one for every bug fix, co-located with the code it covers unless it is cross-cutting.
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
- [ ] Docs or README updated (if API surface changed)

## Review Guidelines (P0/P1 Focus)

Review output should be concise and include only critical issues. Do not include style-only feedback or broad praise.

- `P0`: must-fix defects (incorrect algorithm result, a panic path such as `.unwrap()`/`.expect()` in non-test code, a broken build, or a broken test
  workflow).
- `P1`: high-priority defects (a cross-module dependency that breaks `make check-module-deps`, a missing feature gate, an algorithm without `Result`
  -based error handling where it can fail, a numerical change with no property-based or regression test, or wrong empty/disconnected/self-loop
  handling).

Use this review format:

1. `Severity` (`P0`/`P1`)
2. `File:line`
3. `Issue`
4. `Why it matters`
5. `Minimal fix direction`
