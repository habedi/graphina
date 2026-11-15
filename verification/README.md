## Graphina Verification

> [!WARNING]
> Verifying Graphina's algorithms is in early stage and very experimental for now.

This directory is a separate Lean 4 project within the Graphina repository.
It reuses the Lake library, test driver, and pinned Lean and mathlib versions
from [template-lean-project](https://github.com/habedi/template-lean-project).
The dependency manifest pins the transitive dependencies as well.

### Current Scope

The project imports mathlib's simple graph connectivity definitions.
`Tests.lean` checks that the graph vocabulary is available by proving two examples about reflexive and transitive reachability.
These are build checks, not proofs about Graphina's algorithms.

Mathlib's `SimpleGraph` models undirected graphs without self-loops or parallel edges.
It is not yet a model of Graphina's directed and undirected graph types.
The first algorithm pilot is BFS reachability.
The [extraction report](extraction/README.md) records its contract, pinned tools, reproduction commands, and observed blockers.
Charon extracted the Rust entry point, but Aeneas failed to translate it to Lean.
There is no BFS correctness proof yet.

### Build and Test

Install [elan](https://github.com/leanprover/elan), and ensure `lake` is on your `PATH`.
Elan selects the Lean version in `lean-toolchain`.
From the repository root, run the following commands.

```sh
cd verification
lake exe cache get Mathlib.Combinatorics.SimpleGraph.Connectivity.Connected
lake --wfail build
lake --wfail test
```

The cache command downloads compiled mathlib dependencies to avoid building them from source.
Commit `lake-manifest.json` when updating dependencies.
Keep the Lean toolchain and mathlib versions compatible.

After the dependencies are available, `make verify` from the repository root builds the library and runs the Lean test driver.
The separate verification workflow runs these checks in GitHub Actions.
Rust builds do not require Lean.
Warnings are treated as errors, including warnings for incomplete proofs using `sorry`.

### Proof Boundaries

A theorem about a handwritten Lean model does not establish correctness of the Rust implementation.
Each future algorithm proof must document its Rust source, assumptions, graph semantics, and any unproved connection to Rust.
The extraction experiment is separate from the Lean build and does not establish Rust equivalence.
