## Comparison with rustworkx

This directory contains a benchmark harness that compares graphina's algorithm implementations
against [rustworkx-core](https://crates.io/crates/rustworkx-core), the pure-Rust algorithm crate behind [rustworkx](https://www.rustworkx.org/).
The harness builds one synthetic graph, runs the same set of algorithms through both libraries, reports the median wall time for each, and checks that
the two libraries produce the same result.

The comparison is library-to-library in Rust, not through the rustworkx Python bindings, so the numbers measure the algorithm implementations rather
than the binding or interpreter overhead.

### Running the Harness

```bash
make bench-graphina

# Or directly (from this directory, so the local toolchain pin applies):
cd comparisons/graphina && cargo run --release

# Against the real-world datasets (run `make testdata` first to download them):
make bench-graphina-datasets

# Or one dataset directly:
cd comparisons/graphina && RUSTWORKX_COMPARE_DATASET=../../tests/testdata/graphina-graphs/wikipedia_chameleon.txt cargo run --release
```

The harness is a detached crate: it is excluded from the graphina workspace and pulls rustworkx-core (and its dependency tree) only when built here,
so it never becomes part of `make build`, `make test`, or `make lint`.

The runs can be configured with these environment variables:

- `RUSTWORKX_COMPARE_NODES`: node count (default: 2000)
- `RUSTWORKX_COMPARE_EDGES`: edge count, distinct unordered pairs with no self-loops (default: 10000)
- `RUSTWORKX_COMPARE_REPS`: timed repetitions per algorithm, median reported (default: 10)
- `RUSTWORKX_COMPARE_WARMUPS`: untimed warmup runs per algorithm (default: 3)
- `RUSTWORKX_COMPARE_SKEW`: `uniform` (default) or `zipf` for a power-law degree distribution with hub nodes; the skewed graph contains far more
  two-paths and triangles, so join-heavy algorithms get slower
- `RUSTWORKX_COMPARE_SWEEP`: set to `1` to run the workload at base/5, base, and base*5 sizes and print per-algorithm scaling ratios between
  consecutive sizes; ratios above the 5x dataset growth indicate superlinear behavior
- `RUSTWORKX_COMPARE_BUDGET_SECS`: time budget per algorithm per library (default: 30s); repetitions stop early when the budget is spent, and a
  trailing `*` in the table shows the median taken from fewer than the requested repetitions
- `RUSTWORKX_COMPARE_DATASET`: path to an edge-list file to load instead of generating a synthetic graph; when set, the synthetic knobs (nodes, edges,
  skew, sweep) are ignored
- `RUSTWORKX_COMPARE_MAX_DENSE_NODES`: in dataset mode, the node-count ceiling above which the superlinear algorithms are skipped (default: 4000);
  synthetic runs are never gated

### Data

The graph is an undirected simple graph (no self-loops, no parallel edges) with unit edge weights.
It is generated with a fixed random seed so runs are reproducible. Edge endpoints are sampled uniformly by default or from a Zipf distribution (
exponent 0.8) with `RUSTWORKX_COMPARE_SKEW=zipf`, which produces hub nodes as in real graphs and stresses the join-heavy algorithms. The single-source
traversals start from the highest-degree node so the traversal is non-trivial under both distributions.

### Real-World Datasets

With `RUSTWORKX_COMPARE_DATASET` set (or via `make bench-graphina-datasets`), the harness loads a real-world graph from an edge-list file instead of
generating one.
The loader accepts comma- or whitespace-separated edges, one per line, skips a leading header and `#` comments, remaps node ids to a contiguous range,
treats the graph as undirected, drops self-loops, and deduplicates parallel edges.
The [graphina-graphs](https://huggingface.co/datasets/habedi/graphina-graphs) datasets downloaded by `make testdata` are in this format.

Real graphs are far larger and more skewed than the synthetic default, so the superlinear algorithms (all-pairs, both betweenness variants, closeness,
eigenvector, Katz, and transitivity) are skipped above `RUSTWORKX_COMPARE_MAX_DENSE_NODES` nodes (default 4000) and reported as `skipped`. Only the
near-linear algorithms (the single-source and point-to-point shortest paths, BFS, DFS, connected components, and degree centrality) run on every
dataset. The smallest dataset (`wikipedia_chameleon`, about 2300 nodes) runs the full suite; the larger ones run the near-linear subset.
`make bench-graphina-datasets` covers the undirected datasets; the large directed graphs (`stanford_web_graph`, `dblp_citation_network`) are excluded
by default but can be run by pointing `RUSTWORKX_COMPARE_DATASET` at them.

### Algorithms

Each algorithm is run on a graphina graph and on an equivalent petgraph graph, and the result is normalized to a canonical, library-independent form
and compared before timing. The workload covers every graphina algorithm that has a directly comparable rustworkx-core counterpart:

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

The differential check runs before timing: medians for an algorithm the libraries disagree on are meaningless, since a library doing the wrong amount
of work can look faster.
A divergent algorithm is reported as `DIFF` and not timed; an algorithm that panics on one side (for example failing to converge on a real dataset) is
reported as `ERR` and not timed; an algorithm skipped because the dataset exceeds the dense-node ceiling is reported as `skipped`.

### Fairness Notes

- The graph carries unit edge weights, so weighted shortest paths equal unweighted hop counts. rustworkx betweenness and closeness are structural (
  unweighted) while graphina's are weighted; unit weights make the two directly comparable.
- rustworkx betweenness and closeness parallelize above a node-count threshold. The harness passes `usize::MAX` for that threshold to force the
  sequential path, so both libraries are measured single-threaded. graphina's sequential centrality modules are used (not the `parallel` feature), so
  the comparison is single-thread against single-thread.
- Eigenvector and Katz centrality use different scaling and sign conventions in the two libraries (graphina does not normalize, rustworkx
  L2-normalizes), so their vectors are L2-normalized and sign-fixed (largest-magnitude component made positive) before comparison.
- Katz centrality converges only for an attenuation factor below the reciprocal of the largest eigenvalue, which is much smaller on real graphs than
  on the synthetic default. The harness estimates the spectral radius by power iteration and sets `alpha` from it, using the same value for both
  libraries so the comparison stays fair.
- Degree centrality is raw degree counts in graphina but divided by `n-1` in rustworkx, so the graphina side is scaled by `1/(n-1)` before comparison.
- Results whose value is otherwise unused are wrapped in `std::hint::black_box` during timing so the optimizer cannot elide the work and report a
  near-zero time.
- Floating-point results are compared within a small tolerance rather than by exact equality, since summation order differs between the two
  implementations.
- The graph weight type differs by algorithm because of the graphina signatures: the generic `dijkstra` takes an integer weight, and the rest (
  including `betweenness_centrality` and `closeness_centrality`) take `f64`. The harness builds one graph per weight type from the same edge list.

> [!NOTE]
> The harness covers only algorithms that both libraries implement over their core Rust APIs with directly comparable semantics.
> Algorithms exclusive to one library (graphina's community detection, link prediction, and approximation modules; rustworkx's isomorphism, planarity,
> coloring, and matching) are out of scope for a like-for-like timing comparison.
