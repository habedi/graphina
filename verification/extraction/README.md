## BFS Extraction Pilot

### Result

The pilot ran on November 15, 2026, against Graphina commit `b91230fe2ba80aae06d5ccf0e5fbd64022c47e1d`.
The production Rust code was not changed.

Charon extracted `graphina::traversal::algorithms::bfs` and its referenced definitions to LLBC successfully.
Aeneas could not translate that LLBC to Lean.
No Lean BFS definition or correctness proof was produced.
Successful Charon extraction alone does not establish correctness.
Some dependency definitions are opaque in the extracted representation.

| Experiment | Charon Exit Status | Aeneas Exit Status | Result |
| --- | --- | --- | --- |
| Existing BFS | 0 | 2 | Unsupported arrow type and loop fixed point failure. |
| BFS with `graphina::core::types` opaque | 0 | 2 | Loop fixed point failure remains. |

The second experiment deliberately hides core implementations to isolate the failure.
It does not provide verified replacements for those implementations.

### Source and Contract

The entry point is `bfs` in [algorithms.rs](../../src/traversal/algorithms.rs).
Its graph helpers are in [types.rs](../../src/core/types.rs).
The intended contract for a future proof is as follows.

- A missing or removed start node produces an empty vector.
- A live start node is the first node in the result.
- The result contains each node reachable from the start exactly once.
- Directed graphs follow outgoing edges. Undirected graphs allow traversal in either direction.
- Disconnected nodes and removed nodes are absent from the result.
- Self-loops, parallel edges, and cycles do not introduce duplicate visits.
- Nodes appear in nondecreasing unweighted distance from the start. Ties follow the existing neighbor iteration order.

The graph is borrowed immutably during BFS.
Live node indices must be below petgraph's `node_bound()`.
Neighbor iteration must terminate and yield live nodes within that bound.
These are dependency obligations for a proof, not facts established by this pilot.
Edge weights and node attributes do not affect traversal.

The implementation uses a `Vec<bool>` for visited flags, a `Vec<NodeId>` for the result, and a `VecDeque<NodeId>` for the queue.
The graph dependencies include `NodeId::index`, `BaseGraph::node_attr`, `BaseGraph::as_petgraph`, `BaseGraph::neighbors`,
and petgraph's node indices, node bounds, and neighbor iterators.

### Tool Versions

| Tool | Pinned Version |
| --- | --- |
| Aeneas release | `nightly-2026.11.15-1dc8ad3` |
| Aeneas commit | `1dc8ad31b1b062a77c50e29144a6c7bba955fa09` |
| Charon | `0.1.315` |
| Charon commit | `454586e9b770231e66d2062993e632dfc06463e2` |
| Charon Rust toolchain | `nightly-2026-09-17` |
| Aeneas Lean backend | `leanprover/lean4:v4.36.0-rc2` |

The original pilot used the Aeneas release, which bundles the matching Charon binaries.
The Nix extraction shell builds the same commits and supplies Charon's Rust toolchain directly.
Its Aeneas version output uses the commit identifier rather than the release tag.
Its Lean backend requires a different Lean/mathlib version from the existing verification project, which uses 4.35.0.
This experiment does not change that project.
The mismatch did not cause the translation errors.
Aeneas failed before producing Lean files or invoking Lean.

### Reproduction

Run the commands below from the Graphina repository root.
The extraction shell provides the pinned Aeneas and Charon binaries, the matching Rust nightly, and Lean 4.36.0-rc2.
It does not require Rustup or Elan.
The flake declares x86_64 Linux, aarch64 Linux, and aarch64 macOS environments.

```sh
nix develop .#extraction
charon version
aeneas -version
rustc --version
lean --version
mkdir -p verification/extraction/.build
```

Extract the actual Rust entry point, enabling only the traversal extension.
The remaining unconditional crate dependencies still compile.

```sh
charon cargo \
  --preset=aeneas \
  --start-from graphina::traversal::algorithms::bfs \
  --dest-file verification/extraction/.build/bfs.llbc \
  -- --package graphina --lib --no-default-features --features traversal --locked
```

Then attempt translation. This command is expected to fail with the pinned tools and source revision above.

```sh
mkdir -p verification/extraction/.build/lean
aeneas \
  -backend lean verification/extraction/.build/bfs.llbc \
  -dest verification/extraction/.build/lean \
  -split-files -namespace GraphinaBfs -warnings-as-errors -no-progress-bar
```

For the diagnostic experiment, repeat Charon extraction with `--opaque graphina::core::types` and a separate destination file,
then pass that file to Aeneas.
Charon's pretty printer can inspect which definitions are opaque.

```sh
charon pretty-print \
  verification/extraction/.build/bfs.llbc
```

### Observed Blockers

The original translation reported the following errors.

```text
Arrow types are not supported yet
Source: 'src/core/types.rs', lines 390:4-392:5
Could not compute a loop fixed point in 2 iterations
Source: 'src/traversal/algorithms.rs', lines 79:8-85:9
```

The first source location is `BaseGraph::neighbors`, which maps petgraph's neighbor iterator through the function item `NodeId::new`.
Making core types and helpers opaque removed the arrow type diagnostic but left the loop failure.
This shows that hiding the helper alone is insufficient.
It does not establish the root cause of the loop failure.

A search of this release's Lean backend and builtin mappings found no `VecDeque` model.
Petgraph operations also appear as opaque dependencies in the LLBC.
Even after translation succeeds, these dependencies need models and specifications, or their behavior remains an explicit assumption.

### Next Decision

Direct extraction of the current BFS is blocked with these pinned tools.
The next experiment should reduce the two translation failures to small examples outside production code.
This would help determine whether an upstream fix or a small change to iteration can make extraction work.
Changing the production queue or graph representation is not justified by this pilot alone.

The existing `make verify` continues to check the Lean setup.
`nix flake check` exercises the extraction tools on a small identity function, not BFS.
The failing extraction experiment is not added as a passing CI check.
A Rust equivalence claim, termination proof, and reachability proof remain outstanding.

### References

- [Aeneas Rust-to-Lean guide](https://github.com/AeneasVerif/aeneas/blob/1dc8ad31b1b062a77c50e29144a6c7bba955fa09/documentation/getting-started.md).
- [Pinned release and platform assets](https://github.com/AeneasVerif/aeneas/releases/tag/nightly-2026.11.15-1dc8ad3).
- [Matching Charon commit](https://github.com/AeneasVerif/aeneas/blob/1dc8ad31b1b062a77c50e29144a6c7bba955fa09/charon-pin).
- [Required Lean backend version](https://github.com/AeneasVerif/aeneas/blob/1dc8ad31b1b062a77c50e29144a6c7bba955fa09/backends/lean/lean-toolchain).
