## Comparison with rustworkx

This directory contains a benchmark harness that compares graphina's algorithm implementations against
[rustworkx-core](https://crates.io/crates/rustworkx-core), the pure-Rust algorithm crate behind
[rustworkx](https://www.rustworkx.org/). The harness builds one synthetic graph, runs the same set of
algorithms through both libraries, reports the median wall time for each, and checks that the two libraries
produce the same result.

The comparison is library-to-library in Rust, not through the rustworkx Python bindings, so the numbers
measure the algorithm implementations rather than the binding or interpreter overhead.

### Running the Harness

```bash
make bench-rustworkx

# Or directly (from this directory, so the local toolchain pin applies):
cd benchmarks/rustworkx-compare && cargo run --release
```

The harness is a detached crate: it is excluded from the graphina workspace and pulls rustworkx-core (and
its dependency tree) only when built here, so it never becomes part of `make build`, `make test`, or
`make lint`.

The runs can be configured with these environment variables:

- `RUSTWORKX_COMPARE_NODES`: node count (default: 2000)
- `RUSTWORKX_COMPARE_EDGES`: edge count, distinct unordered pairs with no self-loops (default: 10000)
- `RUSTWORKX_COMPARE_REPS`: timed repetitions per algorithm, median reported (default: 10)
- `RUSTWORKX_COMPARE_WARMUPS`: untimed warmup runs per algorithm (default: 3)
- `RUSTWORKX_COMPARE_SKEW`: `uniform` (default) or `zipf` for a power-law degree distribution with hub
  nodes; the skewed graph contains far more two-paths and triangles, so join-heavy algorithms get slower
- `RUSTWORKX_COMPARE_SWEEP`: set to `1` to run the workload at base/5, base, and base*5 sizes and print
  per-algorithm scaling ratios between consecutive sizes; ratios above the 5x dataset growth indicate
  superlinear behavior
- `RUSTWORKX_COMPARE_BUDGET_SECS`: time budget per algorithm per library (default: 30s); repetitions stop
  early when the budget is spent, and a trailing `*` in the table shows the median taken from fewer than the
  requested repetitions

### Data

The graph is an undirected simple graph (no self-loops, no parallel edges) with unit edge weights. It is
generated with a fixed random seed so runs are reproducible. Edge endpoints are sampled uniformly by default
or from a Zipf distribution (exponent 0.8) with `RUSTWORKX_COMPARE_SKEW=zipf`, which produces hub nodes as
in real graphs and stresses the join-heavy algorithms. The single-source traversals start from the
highest-degree node so the traversal is non-trivial under both distributions.

### Algorithms

Each algorithm is run on a graphina graph and on an equivalent petgraph graph, and the result is normalized
to a canonical, library-independent form and compared before timing. The workload covers every graphina
algorithm that has a directly comparable rustworkx-core counterpart:

- Single-source shortest path (`dijkstra`, `bellman_ford`)
- Point-to-point shortest path (`a_star`, zero heuristic)
- All-pairs shortest path (graphina `johnson` against rustworkx `distance_matrix`)
- Breadth-first and depth-first reachability (`bfs`, `dfs`)
- Connected components
- Degree centrality
- Betweenness and edge betweenness centrality (unnormalized)
- Closeness centrality
- Eigenvector centrality
- Katz centrality
- Transitivity (global clustering coefficient)

The differential check runs before timing: medians for an algorithm the libraries disagree on are
meaningless, since a library doing the wrong amount of work can look faster. A divergent algorithm is
reported as `DIFF` and not timed.

### Fairness Notes

- The graph carries unit edge weights, so weighted shortest paths equal unweighted hop counts. rustworkx
  betweenness and closeness are structural (unweighted) while graphina's are weighted; unit weights make
  the two directly comparable.
- rustworkx betweenness and closeness parallelize above a node-count threshold. The harness passes
  `usize::MAX` for that threshold to force the sequential path, so both libraries are measured
  single-threaded. graphina's sequential centrality modules are used (not the `parallel` feature), so the
  comparison is single-thread against single-thread.
- Eigenvector and Katz centrality use different scaling and sign conventions in the two libraries (graphina
  does not normalize, rustworkx L2-normalizes), so their vectors are L2-normalized and sign-fixed
  (largest-magnitude component made positive) before comparison.
- Degree centrality is raw degree counts in graphina but divided by `n-1` in rustworkx, so the graphina side
  is scaled by `1/(n-1)` before comparison.
- Results whose value is otherwise unused are wrapped in `std::hint::black_box` during timing so the
  optimizer cannot elide the work and report a near-zero time.
- Floating-point results are compared within a small tolerance rather than by exact equality, since
  summation order differs between the two implementations.
- The graph weight type differs by algorithm because of the graphina signatures: `betweenness_centrality`
  and `closeness_centrality` take `OrderedFloat<f64>`, the generic `dijkstra` takes an integer weight, and
  the rest take `f64`. The harness builds one graph per weight type from the same edge list.

> [!NOTE]
> The harness covers only algorithms that both libraries implement over their core Rust APIs with directly
> comparable semantics. Algorithms exclusive to one library (graphina's community detection, link
> prediction, and approximation modules; rustworkx's isomorphism, planarity, coloring, and matching) are
> out of scope for a like-for-like timing comparison.
