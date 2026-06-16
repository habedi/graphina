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

## Writing Style

- Use Oxford commas in inline lists: "a, b, and c" not "a, b, c".
- Do not use em dashes. Restructure the sentence, or use a colon or semicolon instead.
- Avoid colorful adjectives and adverbs. Write "graph generator" not "powerful graph generator".
- Use noun phrases for checklist items, not imperative verbs. Write "negative weight detection" not
  "detect negative weights".
- Headings in Markdown files must be in title case: "Build from Source" not "Build from source". Minor words
  (a, an, the, and, but, or, for, in, on, at, to, by, of) stay lowercase unless they are the first word.

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

## Required Validation

Run `make lint` and `make test` for any change. Key targets:

| Target            | Command                  | What It Runs                                                          |
|-------------------|--------------------------|----------------------------------------------------------------------|
| Format            | `make format`            | `cargo fmt`                                                          |
| Format Check      | `make format-check`      | `cargo fmt --all --check` (non-mutating, used in CI)                |
| Lint              | `make lint`              | `cargo clippy` with `-D warnings -D clippy::unwrap_used -D clippy::expect_used` |
| Test              | `make test`              | All workspace tests with `--features all --all-targets`, plus doctests |
| Doctest           | `make doctest`           | Doc-comment code examples (`cargo test --doc --features all`)        |
| Nextest           | `make nextest`           | Tests via `cargo nextest` with `--features all`                      |
| Module Deps       | `make check-module-deps` | Verifies extensions depend only on `core`                            |
| Build             | `make build`             | Release build                                                        |
| Bench             | `make bench`             | Criterion benchmarks with `--features all`                           |
| Coverage          | `make coverage`          | `cargo tarpaulin` with XML and HTML output                           |
| Audit             | `make audit`             | `cargo audit` on dependencies                                        |
| Careful           | `make careful`           | `cargo careful` for undefined-behavior checks                        |
| Test Data         | `make testdata`          | Downloads datasets used in integration tests                         |

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
