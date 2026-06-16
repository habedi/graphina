#!/usr/bin/env python3
"""Generate a NetworkX centrality oracle corpus for Graphina directed graphs.

This is the directed companion to `gen_oracle_centrality_fixtures.py`. It
produces a deterministic, seeded corpus of simple directed, integer-weighted
graphs together with the reference output of NetworkX for the centrality
measures whose convention Graphina matches on directed graphs. The corpus is
committed to the repository and replayed by a hermetic Rust test
(`tests/oracle_directed_centrality_tests.rs`); the NetworkX dependency lives
here, in the generator, and never in the Rust test path.

Each measure is pinned to the convention Graphina implements:

  - degree centrality (in, out, total) -> nx.DiGraph.in_degree / out_degree
    Raw incident-edge counts; total is in + out.

  - betweenness centrality             -> nx.betweenness_centrality (directed)
    Unweighted, without endpoints, emitted unnormalized and normalized.

  - closeness centrality               -> nx.closeness_centrality(reverse)
    Graphina sums over nodes reachable FROM each node (out-distance), while
    NetworkX uses in-distance, so the reference is computed on the reversed
    graph. Wasserman-Faust improved, weighted.

  - harmonic centrality                -> nx.harmonic_centrality(reverse)
    Out-distance, matching the reversed-graph reference for the same reason.

  - PageRank                           -> nx.pagerank(alpha=0.85, weight)
    Directed, weighted, normalized to sum to 1; dangling nodes redistribute.

The graphs are simple (no self-loops and no parallel edges) with positive
integer weights, so weighted shortest path sums are exact under f64.

Regenerate with `make oracle-fixtures`.
"""

import json
import random
import sys

import networkx as nx

SEED = 0xD1_CE_47_A1
NUM_GRAPHS = 80
MIN_NODES = 3
MAX_NODES = 12
DENSITIES = [0.1, 0.2, 0.35, 0.5, 0.7]
MIN_WEIGHT = 1
MAX_WEIGHT = 10
ALPHA = 0.85
NX_TOL = 1e-12
NX_MAX_ITER = 2000


def build_graph(rng, n, density):
    """Build a simple directed graph with positive integer edge weights."""
    g = nx.DiGraph()
    g.add_nodes_from(range(n))
    for u in range(n):
        for v in range(n):
            if u == v:
                continue
            if rng.random() < density:
                g.add_edge(u, v, weight=rng.randint(MIN_WEIGHT, MAX_WEIGHT))
    return g


def main():
    rng = random.Random(SEED)
    cases = []
    for i in range(NUM_GRAPHS):
        n = rng.randint(MIN_NODES, MAX_NODES)
        density = rng.choice(DENSITIES)
        g = build_graph(rng, n, density)
        rev = g.reverse()
        edges = sorted((int(u), int(v)) for u, v in g.edges())
        weights = [int(g[u][v]["weight"]) for u, v in edges]

        betw_raw = nx.betweenness_centrality(g, normalized=False, endpoints=False, weight=None)
        betw_norm = nx.betweenness_centrality(g, normalized=True, endpoints=False, weight=None)
        # Graphina measures out-distance; NetworkX measures in-distance, so the
        # reversed graph yields Graphina's quantity.
        closeness = nx.closeness_centrality(rev, distance="weight")
        harmonic = nx.harmonic_centrality(rev, distance="weight")
        pagerank = nx.pagerank(g, alpha=ALPHA, tol=NX_TOL, max_iter=NX_MAX_ITER, weight="weight")

        cases.append(
            {
                "id": f"dc{i:04d}",
                "n": n,
                "edges": [[u, v] for u, v in edges],
                "weights": weights,
                "degree": {
                    "in": [g.in_degree(k) for k in range(n)],
                    "out": [g.out_degree(k) for k in range(n)],
                    "total": [g.in_degree(k) + g.out_degree(k) for k in range(n)],
                },
                "betweenness_raw": [betw_raw[k] for k in range(n)],
                "betweenness_norm": [betw_norm[k] for k in range(n)],
                "closeness": [closeness[k] for k in range(n)],
                "harmonic": [harmonic[k] for k in range(n)],
                "pagerank": [pagerank[k] for k in range(n)],
            }
        )

    corpus = {
        "meta": {
            "generator": "scripts/gen_oracle_directed_centrality_fixtures.py",
            "networkx_version": nx.__version__,
            "seed": SEED,
            "num_graphs": NUM_GRAPHS,
            "alpha": ALPHA,
        },
        "cases": cases,
    }

    out_path = sys.argv[1] if len(sys.argv) > 1 else "-"
    text = json.dumps(corpus, indent=2, sort_keys=True) + "\n"
    if out_path == "-":
        sys.stdout.write(text)
    else:
        with open(out_path, "w", encoding="utf-8") as f:
            f.write(text)
        print(f"Wrote {len(cases)} graphs to {out_path}", file=sys.stderr)


if __name__ == "__main__":
    main()
