## Python Benchmark Comparison (rustworkx and NetworkX)

This directory contains a benchmark harness that compares PyGraphina against [rustworkx](https://www.rustworkx.org/) and [NetworkX](https://networkx.org/).
The harness builds one graph, runs the same set of algorithms through all three libraries, reports the median wall time for each, and checks that
they produce the same result.

This is the Python counterpart of the [graphina](../graphina) harness. That one compares the core Rust crates and measures the algorithm
implementations; this one goes through the Python bindings/APIs, so the numbers include the binding and interpreter overhead each library adds, which is
what a Python user actually pays.

### Running the Harness

```bash
make bench-pygraphina

# Or directly, once PyGraphina is built into the active environment (make develop-py):
uv run --with rustworkx --with networkx python comparisons/pygraphina/compare.py

# Against the real-world datasets (run `make testdata` first to download them):
make bench-pygraphina-datasets

# Or one dataset directly:
PYGRAPHINA_COMPARE_DATASET=tests/testdata/graphina-graphs/wikipedia_chameleon.txt \
    uv run --with rustworkx --with networkx python comparisons/pygraphina/compare.py
```

The harness imports PyGraphina from the active environment, so `make bench-pygraphina` runs `make develop-py` first to build and install the current
PyGraphina with maturin. rustworkx and networkx are pulled only for this run with `--with`, so they never become project dependencies.

The runs can be configured with these environment variables:

- `PYGRAPHINA_COMPARE_NODES`: node count (default: 1000)
- `PYGRAPHINA_COMPARE_EDGES`: edge count, distinct unordered pairs with no self-loops (default: 5000)
- `PYGRAPHINA_COMPARE_REPS`: timed repetitions per algorithm, median reported (default: 5)
- `PYGRAPHINA_COMPARE_WARMUPS`: untimed warmup runs per algorithm (default: 1)
- `PYGRAPHINA_COMPARE_SKEW`: `uniform` (default) or `zipf` for a power-law degree distribution with hub nodes; the skewed graph contains far more
  two-paths and triangles, so join-heavy algorithms get slower
- `PYGRAPHINA_COMPARE_SWEEP`: set to `1` to run the workload at base/5, base, and base*5 sizes and print per-algorithm scaling ratios between
  consecutive sizes
- `PYGRAPHINA_COMPARE_BUDGET_SECS`: time budget per algorithm per library (default: 20s); repetitions stop early when the budget is spent, and a
  trailing `*` in the table shows the median taken from fewer than the requested repetitions
- `PYGRAPHINA_COMPARE_DATASET`: path to an edge-list file to load instead of generating a synthetic graph; when set, the synthetic knobs (nodes,
  edges, skew, sweep) are ignored
- `PYGRAPHINA_COMPARE_MAX_DENSE_NODES`: in dataset mode, the node-count ceiling above which the superlinear algorithms are skipped (default: 4000);
  synthetic runs are never gated
- `PYGRAPHINA_COMPARE_MAX_NETWORKX_NODES`: node-count ceiling above which all NetworkX algorithms are skipped (default: 5000) to prevent long runs or hangs
- `PYGRAPHINA_COMPARE_MAX_NETWORKX_DENSE_NODES`: node-count ceiling above which NetworkX superlinear algorithms (betweenness, closeness, eigenvector) are skipped (default: 1500)
- `PYGRAPHINA_COMPARE_CSV`: path of a CSV file the per-algorithm timings are written to, one line per algorithm and library; the
  `make bench-pygraphina` and `make bench-pygraphina-datasets` targets set it so results land in `comparisons/results/`, and `make bench-plots`
  renders charts from them

The defaults are smaller than the [Rust harness](../graphina)'s 2000 nodes because the slowest algorithms here (eigenvector's dense eigendecomposition
and the per-node closeness) run through the Python binding and scale steeply. A default run takes a few minutes, dominated by those two; raise
`PYGRAPHINA_COMPARE_NODES` for a heavier comparison.

### Data

The graph is an undirected simple graph (no self-loops, no parallel edges) with unit edge weights. It is generated with a fixed-seed 64-bit LCG (the
same generator and constants as the Rust harness) so runs are reproducible. Edge endpoints are sampled uniformly by default or from a Zipf
distribution (exponent 0.8) with `PYGRAPHINA_COMPARE_SKEW=zipf`, which produces hub nodes as in real graphs and stresses the join-heavy algorithms.
The single-source traversals start from the highest-degree node so the traversal is non-trivial under both distributions. Both libraries receive nodes
in the same order, so node ids `0..n` align and a result keyed by node id compares without a remap.

### Real-World Datasets

With `PYGRAPHINA_COMPARE_DATASET` set (or via `make bench-pygraphina-datasets`), the harness loads a real-world graph from an edge-list file instead
of generating one. The loader accepts comma or whitespace-separated edges, one per line, skips a leading header and `#` comments, remaps node ids to a
contiguous range, treats the graph as undirected, drops self-loops, and deduplicates parallel edges.
The [graphina-graphs](https://huggingface.co/datasets/habedi/graphina-graphs) datasets downloaded by `make testdata` are in this format.

Real graphs are far larger and more skewed than the synthetic default, so the superlinear algorithms (betweenness, closeness, and eigenvector) are
skipped above `PYGRAPHINA_COMPARE_MAX_DENSE_NODES` nodes (default 4000) and reported as `skipped`. Only the near-linear algorithms (single-source
shortest paths, connected components, degree centrality, and PageRank) run on every dataset. The smallest dataset (`wikipedia_chameleon`, about 2300
nodes) runs the full suite; the larger ones run the near-linear subset.
`make bench-pygraphina-datasets` covers the undirected datasets; the large directed graphs (`stanford_web_graph`, `dblp_citation_network`) are
excluded by default but can be run by pointing `PYGRAPHINA_COMPARE_DATASET` at them.

### Algorithms

Each algorithm is run on a PyGraphina graph, an equivalent rustworkx graph, and an equivalent NetworkX graph. The result is normalized to a canonical, library-independent form and compared before timing.
The workload covers the PyGraphina algorithms that have directly comparable counterparts:

- Single-source shortest path lengths (`dijkstra`)
- Connected components
- Degree centrality
- Betweenness centrality (unnormalized)
- Closeness centrality
- Eigenvector centrality
- PageRank

The differential check runs before timing: medians for an algorithm the libraries disagree on are meaningless, since a library doing the wrong amount
of work can look faster.
A divergent algorithm is reported as `DIFF` (or `DIFF (networkx)` if only NetworkX disagrees) and not timed; an algorithm that raises is reported as `ERR` and not timed.

### Fairness Notes

- The graph carries unit edge weights, so weighted shortest paths equal unweighted hop counts. rustworkx betweenness and closeness are structural (
  unweighted) while PyGraphina's are weighted; unit weights make them directly comparable.
- rustworkx betweenness and closeness parallelize above a node-count threshold. The harness passes a very large `parallel_threshold` to force the
  sequential path, so both libraries are measured single-threaded.
- Eigenvector centrality uses different scaling and sign conventions across the libraries, so its vector is L2-normalized and sign-fixed (
  largest-magnitude component made positive) before comparison.
- Degree centrality is raw degree counts in PyGraphina but divided by `n - 1` in rustworkx and NetworkX, so the PyGraphina side is scaled by `1 / (n - 1)` before
  comparison.
- rustworkx PageRank takes a directed graph only, so the rustworkx side runs on a bidirected copy of the same edges (each undirected edge becomes a
  pair of opposing directed edges), which matches PyGraphina's undirected PageRank to within numerical tolerance.
- Floating-point results are compared within a small tolerance rather than by exact equality, since summation order differs between the two
  implementations.
- The single-source distance maps are compared over reachable targets only (PyGraphina returns every node with `None` for unreachable and `0` for the
  source; rustworkx returns reachable targets, excluding the source).

> [!NOTE]
> The harness covers only algorithms that both libraries expose through their Python bindings with directly comparable semantics.
> Algorithms exclusive to one library (PyGraphina's community detection and link prediction; rustworkx's isomorphism, planarity, coloring, and
> matching) are out of scope for a like-for-like timing comparison.

> [!NOTE]
> These numbers measure the full Python stack (binding plus algorithm plus interpreter overhead), not the Rust implementations in isolation.
> For a Rust-to-Rust comparison of the algorithm implementations, use [graphina](../graphina).
