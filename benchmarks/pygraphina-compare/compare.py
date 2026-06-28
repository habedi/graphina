#!/usr/bin/env python3
"""Comparison harness running the same graph algorithms through pygraphina and rustworkx.

Both libraries receive the same synthetic graph, built from one deterministic edge
list into a pygraphina ``PyGraph`` and a rustworkx ``PyGraph``. Each algorithm runs
on both, and the harness reports the median wall time per library, so it doubles as a
differential correctness check: the result of each algorithm is normalized to a
canonical, library-independent form and compared before timing. Medians for an
algorithm the two libraries disagree on are meaningless (a library doing the wrong
amount of work can look faster), so a divergent algorithm is reported as ``DIFF`` and
not timed.

This is the Python counterpart of the Rust ``rustworkx-compare`` harness. That one
compares the core Rust crates and measures the algorithm implementations; this one
goes through the Python bindings, so the numbers include the binding and interpreter
overhead each library adds, which is what a Python user actually pays.

A few comparisons need care to be meaningful across the two libraries:

* The graph carries unit edge weights, so weighted shortest paths equal unweighted
  hop counts. rustworkx betweenness and closeness are structural (unweighted) while
  pygraphina's are weighted; unit weights make the two agree.
* rustworkx betweenness and closeness parallelize above a node-count threshold; the
  harness passes a very large threshold to force the sequential path, so both
  libraries are measured single-threaded.
* Eigenvector centrality uses different normalization conventions, so its vector is
  L2-normalized and sign-fixed before comparison.
* Degree centrality is raw degree counts in pygraphina but divided by ``n - 1`` in
  rustworkx, so the pygraphina side is scaled by ``1 / (n - 1)`` before comparison.

Dataset sizes, degree skew, repetition counts, and the per-algorithm time budget come
from environment variables; see ``Config.from_env`` for the knobs and their defaults.
"""

from __future__ import annotations

import math
import os
import random
import sys
import time
from dataclasses import dataclass
from typing import Callable

import pygraphina

try:
    import rustworkx
except ImportError:
    sys.exit(
        "rustworkx is not installed. Run this harness with:\n"
        "    uv run --with rustworkx python benchmarks/pygraphina-compare/compare.py\n"
        "or `make bench-pygraphina`."
    )

# Zipf exponent for the skewed degree distribution. At 0.8 the hottest node receives
# a few percent of all edge endpoints, a proper hub without saturating the
# distinct-edge constraint. Matches the Rust harness.
ZIPF_THETA = 0.8

# Each sweep step multiplies nodes and edges by this factor.
SWEEP_STEP = 5

# Forces rustworkx betweenness and closeness onto their sequential path so both
# libraries are measured single-threaded.
SEQUENTIAL_THRESHOLD = 1 << 30

# Bootstrap resamples used to estimate the confidence interval of the median.
BOOTSTRAP_RESAMPLES = 1000

U64_MASK = (1 << 64) - 1


@dataclass
class Config:
    """Run configuration, populated from environment variables."""

    nodes: int
    edges: int
    reps: int
    warmups: int
    skew: str
    sweep: bool
    budget_secs: float

    @staticmethod
    def from_env() -> "Config":
        def var(name: str, default: int) -> int:
            raw = os.environ.get(name)
            if raw is None:
                return default
            try:
                return int(raw)
            except ValueError:
                return default

        skew = os.environ.get("PYGRAPHINA_COMPARE_SKEW", "uniform")
        if skew not in ("uniform", "zipf"):
            sys.exit(f"PYGRAPHINA_COMPARE_SKEW must be 'uniform' or 'zipf', got {skew!r}")

        # Defaults are smaller than the Rust harness's because the slowest algorithms
        # here (eigenvector's dense eigendecomposition, the per-node closeness) run
        # through the Python binding and scale steeply; 1000 nodes keeps a full run to
        # a few minutes. Raise PYGRAPHINA_COMPARE_NODES for a heavier comparison.
        nodes = var("PYGRAPHINA_COMPARE_NODES", 1_000)
        edges = var("PYGRAPHINA_COMPARE_EDGES", 5_000)
        reps = var("PYGRAPHINA_COMPARE_REPS", 5)
        warmups = var("PYGRAPHINA_COMPARE_WARMUPS", 1)
        sweep = var("PYGRAPHINA_COMPARE_SWEEP", 0) != 0
        budget = float(os.environ.get("PYGRAPHINA_COMPARE_BUDGET_SECS", "20"))

        if nodes < 1:
            sys.exit("PYGRAPHINA_COMPARE_NODES must be at least 1")
        if edges != 0 and nodes < 2:
            sys.exit("PYGRAPHINA_COMPARE_EDGES requires at least two nodes")
        if reps < 1:
            sys.exit("PYGRAPHINA_COMPARE_REPS must be at least 1")

        return Config(nodes, edges, reps, warmups, skew, sweep, budget)


class Lcg:
    """Deterministic 64-bit LCG (Knuth MMIX constants), matching the Rust harness, so
    runs are reproducible and the generated graph is stable across invocations."""

    def __init__(self, seed: int) -> None:
        self.state = seed & U64_MASK

    def next(self) -> int:
        self.state = (self.state * 6364136223846793005 + 1442695040888963407) & U64_MASK
        return self.state >> 16

    def unit(self) -> float:
        return self.next() / float(1 << 48)


class Zipf:
    """Cumulative Zipf distribution over node indices ``0..n`` with exponent
    ``ZIPF_THETA``. Skewed sampling concentrates edge endpoints on low indices,
    producing hub nodes whose degrees follow a power law, as in real graphs."""

    def __init__(self, n: int) -> None:
        cdf = []
        acc = 0.0
        for rank in range(1, n + 1):
            acc += 1.0 / (rank**ZIPF_THETA)
            cdf.append(acc)
        self.cdf = [c / acc for c in cdf]

    def sample(self, u: float) -> int:
        # partition_point: first index whose cdf value is >= u.
        lo, hi = 0, len(self.cdf)
        while lo < hi:
            mid = (lo + hi) // 2
            if self.cdf[mid] < u:
                lo = mid + 1
            else:
                hi = mid
        return lo


@dataclass
class Dataset:
    """An undirected simple graph as a node count and a deduplicated edge list. Edges
    are stored as ordered pairs ``(a, b)`` with ``a < b``."""

    nodes: int
    edges: list[tuple[int, int]]


def generate(nodes: int, edges: int, skew: str) -> Dataset:
    rng = Lcg(0x1554_4ED1)
    zipf = Zipf(nodes) if skew == "zipf" else None
    seen: set[tuple[int, int]] = set()
    out: list[tuple[int, int]] = []
    max_attempts = max(edges * 100, 1)
    attempts = 0
    while len(out) < edges:
        attempts += 1
        if attempts > max_attempts:
            sys.exit(
                "edge sampling saturated; lower PYGRAPHINA_COMPARE_EDGES relative to "
                "PYGRAPHINA_COMPARE_NODES"
            )
        if zipf is not None:
            s, t = zipf.sample(rng.unit()), zipf.sample(rng.unit())
        else:
            s, t = rng.next() % nodes, rng.next() % nodes
        if s == t:
            continue
        a, b = (s, t) if s < t else (t, s)
        if (a, b) in seen:
            continue
        seen.add((a, b))
        out.append((a, b))
    return Dataset(nodes, out)


def hub_node(data: Dataset) -> int:
    """Highest-degree node, the source for the single-source traversals. Ties break
    toward the lowest index, so the choice is deterministic."""
    degree = [0] * data.nodes
    for a, b in data.edges:
        degree[a] += 1
        degree[b] += 1
    best, best_deg = 0, -1
    for i, d in enumerate(degree):
        if d > best_deg:
            best, best_deg = i, d
    return best


def build_pygraphina(data: Dataset) -> pygraphina.PyGraph:
    g = pygraphina.PyGraph()
    for _ in range(data.nodes):
        g.add_node(0)
    for a, b in data.edges:
        g.add_edge(a, b, 1.0)
    return g


def build_rustworkx(data: Dataset) -> "rustworkx.PyGraph":
    g = rustworkx.PyGraph(multigraph=False)
    g.add_nodes_from([0] * data.nodes)
    g.add_edges_from([(a, b, 1.0) for a, b in data.edges])
    return g


def build_rustworkx_digraph(data: Dataset) -> "rustworkx.PyDiGraph":
    """Bidirected directed graph: rustworkx PageRank takes a ``PyDiGraph`` only, so an
    undirected edge is modelled as a pair of opposing directed edges. This matches
    pygraphina's undirected PageRank to within numerical tolerance."""
    g = rustworkx.PyDiGraph()
    g.add_nodes_from([0] * data.nodes)
    g.add_edges_from([(a, b, 1.0) for a, b in data.edges])
    g.add_edges_from([(b, a, 1.0) for a, b in data.edges])
    return g


# Both libraries assign sequential integer ids 0..n-1 when nodes are added in the same
# order, so a result keyed by node id aligns between them without a remap.


def canon_map(result: object, n: int) -> list[float]:
    """Canonicalize a node->value mapping into a dense vector indexed by node id.
    Missing nodes default to 0.0. Values are rounded to absorb summation-order noise."""
    vec = [0.0] * n
    items = result.items() if hasattr(result, "items") else dict(result).items()
    for k, v in items:
        vec[int(k)] = float(v)
    return vec


def l2_sign_normalize(v: list[float]) -> list[float]:
    norm = math.sqrt(sum(x * x for x in v))
    if norm == 0.0:
        return v
    out = [x / norm for x in v]
    # Sign-fix: make the largest-magnitude component positive.
    pivot = max(range(len(out)), key=lambda i: abs(out[i]))
    if out[pivot] < 0.0:
        out = [-x for x in out]
    return out


def within_tolerance(a: list[float], b: list[float], eps: float) -> bool:
    if len(a) != len(b):
        return False
    return all(abs(x - y) <= eps + eps * max(abs(x), abs(y)) for x, y in zip(a, b))


@dataclass
class BenchStat:
    median: float
    ci_lo: float
    ci_hi: float
    truncated: bool


def bootstrap_ci95(times: list[float]) -> tuple[float, float]:
    """95% confidence interval for the median by percentile bootstrap, with a
    fixed-seed generator so the interval is reproducible for a given set of times."""
    rng = random.Random(0xB007)
    n = len(times)
    medians = []
    for _ in range(BOOTSTRAP_RESAMPLES):
        sample = sorted(times[rng.randrange(n)] for _ in range(n))
        medians.append(sample[n // 2])
    medians.sort()
    lo = medians[int(BOOTSTRAP_RESAMPLES * 0.025)]
    hi = medians[min(int(BOOTSTRAP_RESAMPLES * 0.975), BOOTSTRAP_RESAMPLES - 1)]
    return lo, hi


def bench(warmups: int, reps: int, budget: float, f: Callable[[], object]) -> BenchStat:
    """Run untimed warmups, then timed repetitions, stopping early once the budget is
    spent (at least one timed repetition always runs)."""
    for _ in range(warmups):
        f()
    times: list[float] = []
    spent = 0.0
    truncated = False
    for i in range(reps):
        start = time.perf_counter()
        result = f()
        elapsed = time.perf_counter() - start
        # Keep a reference so the call is not optimized away.
        del result
        times.append(elapsed)
        spent += elapsed
        if spent >= budget and i + 1 < reps:
            truncated = True
            break
    times.sort()
    median = times[len(times) // 2]
    ci_lo, ci_hi = bootstrap_ci95(times)
    return BenchStat(median, ci_lo, ci_hi, truncated)


@dataclass
class Row:
    name: str
    pyg: BenchStat | None
    rwx: BenchStat | None
    status: str


def diff_and_bench(
    cfg: Config,
    name: str,
    pyg_run: Callable[[], object],
    rwx_run: Callable[[], object],
    pyg_canon: Callable[[object], list[float]],
    rwx_canon: Callable[[object], list[float]],
    eps: float,
) -> Row:
    try:
        pyg_result = pyg_run()
        rwx_result = rwx_run()
    except Exception as exc:  # noqa: BLE001 - report, do not crash the whole run
        return Row(name, None, None, f"ERR ({type(exc).__name__})")

    if not within_tolerance(pyg_canon(pyg_result), rwx_canon(rwx_result), eps):
        return Row(name, None, None, "DIFF")

    budget = cfg.budget_secs
    pyg = bench(cfg.warmups, cfg.reps, budget, pyg_run)
    rwx = bench(cfg.warmups, cfg.reps, budget, rwx_run)
    return Row(name, pyg, rwx, "ok")


def run_at(cfg: Config, nodes: int, edges: int) -> list[Row]:
    data = generate(nodes, edges, cfg.skew)
    n = data.nodes
    hub = hub_node(data)
    pyg_g = build_pygraphina(data)
    rwx_g = build_rustworkx(data)

    print(
        f"\n=== nodes={n} edges={len(data.edges)} skew={cfg.skew} "
        f"reps={cfg.reps} warmups={cfg.warmups} ==="
    )

    rows: list[Row] = []

    # Single-source shortest path lengths from the hub. pygraphina returns every node
    # (source as 0.0, unreachable as None); rustworkx returns reachable targets only,
    # excluding the source. Canonicalize both to {reachable target: distance}.
    def pyg_dijkstra_canon(result: object) -> list[float]:
        vec = [-1.0] * n
        for k, v in dict(result).items():
            if int(k) != hub and v is not None:
                vec[int(k)] = float(v)
        return vec

    def rwx_dijkstra_canon(result: object) -> list[float]:
        vec = [-1.0] * n
        for k, v in dict(result).items():
            if int(k) != hub:
                vec[int(k)] = float(v)
        return vec

    rows.append(
        diff_and_bench(
            cfg,
            "dijkstra (SSSP)",
            lambda: pyg_g.dijkstra(hub),
            lambda: rustworkx.dijkstra_shortest_path_lengths(rwx_g, hub, lambda e: e),
            pyg_dijkstra_canon,
            rwx_dijkstra_canon,
            1e-6,
        )
    )

    # Connected components: compare as a partition (set of frozensets of node ids).
    def cc_canon(components: object) -> list[float]:
        parts = sorted(tuple(sorted(int(x) for x in comp)) for comp in components)
        # Flatten to a comparable numeric signature: component id per node.
        labels = [0.0] * n
        for cid, comp in enumerate(parts):
            for node in comp:
                labels[node] = float(cid)
        return labels

    rows.append(
        diff_and_bench(
            cfg,
            "connected_components",
            lambda: pygraphina.community.connected_components(pyg_g),
            lambda: rustworkx.connected_components(rwx_g),
            cc_canon,
            cc_canon,
            0.0,
        )
    )

    # Degree centrality: pygraphina returns raw counts; rustworkx divides by n - 1.
    scale = 1.0 / (n - 1) if n > 1 else 1.0
    if hasattr(rustworkx, "degree_centrality"):
        rows.append(
            diff_and_bench(
                cfg,
                "degree_centrality",
                lambda: pygraphina.centrality.degree(pyg_g),
                lambda: rustworkx.degree_centrality(rwx_g),
                lambda r: [x * scale for x in canon_map(r, n)],
                lambda r: canon_map(r, n),
                1e-9,
            )
        )

    # Betweenness, unnormalized. Force rustworkx onto its sequential path.
    rows.append(
        diff_and_bench(
            cfg,
            "betweenness",
            lambda: pygraphina.centrality.betweenness(pyg_g, False),
            lambda: rustworkx.betweenness_centrality(
                rwx_g, normalized=False, parallel_threshold=SEQUENTIAL_THRESHOLD
            ),
            lambda r: canon_map(r, n),
            lambda r: canon_map(r, n),
            1e-6,
        )
    )

    # Closeness centrality (Wasserman-Faust on both sides).
    rows.append(
        diff_and_bench(
            cfg,
            "closeness",
            lambda: pygraphina.centrality.closeness(pyg_g),
            lambda: rustworkx.closeness_centrality(rwx_g, wf_improved=True),
            lambda r: canon_map(r, n),
            lambda r: canon_map(r, n),
            1e-6,
        )
    )

    # Eigenvector centrality: different scaling and sign conventions, so L2-normalize
    # and sign-fix both vectors before comparison.
    rows.append(
        diff_and_bench(
            cfg,
            "eigenvector",
            lambda: pygraphina.centrality.eigenvector(pyg_g, 100, 1e-6),
            lambda: rustworkx.eigenvector_centrality(rwx_g),
            lambda r: l2_sign_normalize(canon_map(r, n)),
            lambda r: l2_sign_normalize(canon_map(r, n)),
            1e-3,
        )
    )

    # PageRank: both sum to 1.0; compare the distributions within tolerance. rustworkx
    # PageRank takes a directed graph only, so the rustworkx side runs on a bidirected
    # copy of the same edges, which matches pygraphina's undirected PageRank.
    if hasattr(rustworkx, "pagerank"):
        rwx_dg = build_rustworkx_digraph(data)
        rows.append(
            diff_and_bench(
                cfg,
                "pagerank",
                lambda: pygraphina.centrality.pagerank(pyg_g),
                lambda: rustworkx.pagerank(rwx_dg, alpha=0.85),
                lambda r: canon_map(r, n),
                lambda r: canon_map(r, n),
                1e-4,
            )
        )

    print_table(rows)
    return rows


def fmt(stat: BenchStat | None) -> str:
    if stat is None:
        return "-"
    ms = stat.median * 1e3
    mark = "*" if stat.truncated else ""
    return f"{ms:9.3f} ms{mark}"


def print_table(rows: list[Row]) -> None:
    name_w = max((len(r.name) for r in rows), default=10)
    header = f"{'algorithm':<{name_w}}  {'pygraphina':>14}  {'rustworkx':>14}  {'ratio':>8}  status"
    print(header)
    print("-" * len(header))
    for r in rows:
        if r.pyg is not None and r.rwx is not None:
            ratio = r.pyg.median / r.rwx.median if r.rwx.median > 0 else float("inf")
            ratio_s = f"{ratio:7.2f}x"
        else:
            ratio_s = "-"
        print(
            f"{r.name:<{name_w}}  {fmt(r.pyg):>14}  {fmt(r.rwx):>14}  {ratio_s:>8}  {r.status}"
        )
    print(
        "\nratio > 1 means rustworkx is faster; ratio < 1 means pygraphina is faster. "
        "A trailing * marks a median taken from fewer than the requested repetitions "
        "(time budget spent)."
    )


def main() -> None:
    cfg = Config.from_env()
    print(f"pygraphina vs rustworkx {getattr(rustworkx, '__version__', '?')} comparison harness")

    if not cfg.sweep:
        run_at(cfg, cfg.nodes, cfg.edges)
        return

    sizes = [
        (max(cfg.nodes // SWEEP_STEP, 1), max(cfg.edges // SWEEP_STEP, 0)),
        (cfg.nodes, cfg.edges),
        (cfg.nodes * SWEEP_STEP, cfg.edges * SWEEP_STEP),
    ]
    all_rows = [run_at(cfg, n, e) for n, e in sizes]
    print_sweep(sizes, all_rows)


def print_sweep(sizes: list[tuple[int, int]], all_rows: list[list[Row]]) -> None:
    print(f"\n=== scaling ratios (each step is {SWEEP_STEP}x the data) ===")
    names = [r.name for r in all_rows[1]]
    print(f"{'algorithm':<22}  {'pyg s1->s2':>12}  {'pyg s2->s3':>12}")
    for name in names:

        def med(step: int, target: str = name) -> float | None:
            for r in all_rows[step]:
                if r.name == target and r.pyg is not None:
                    return r.pyg.median
            return None

        m0, m1, m2 = med(0), med(1), med(2)
        r01 = f"{m1 / m0:11.2f}x" if m0 and m1 else "-"
        r12 = f"{m2 / m1:11.2f}x" if m1 and m2 else "-"
        print(f"{name:<22}  {r01:>12}  {r12:>12}")


if __name__ == "__main__":
    main()
