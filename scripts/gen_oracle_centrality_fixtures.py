#!/usr/bin/env python3
"""Generate a NetworkX centrality oracle corpus for Graphina.

This script produces a deterministic, seeded corpus of simple undirected,
integer-weighted graphs together with the reference output of NetworkX for the
centrality measures whose convention Graphina matches exactly. The corpus is
committed to the repository and replayed by a hermetic Rust test
(`tests/oracle_centrality_tests.rs`); the NetworkX dependency lives here, in the
generator, and never in the Rust test path.

Each measure is pinned to the convention Graphina implements, verified against
the source and a worked example during development:

  - degree centrality (raw)        -> nx.Graph.degree
    Graphina `degree_centrality` returns raw incident-edge counts, not values
    normalized by n - 1, so the oracle is the plain NetworkX degree.

  - betweenness centrality         -> nx.betweenness_centrality
    Unweighted (weight=None), without endpoints. Emitted both unnormalized and
    normalized; Graphina applies the standard undirected halving in both cases.

  - closeness centrality           -> nx.closeness_centrality(distance="weight")
    Wasserman-Faust improved closeness over weighted shortest path distances,
    which is well defined on disconnected graphs.

  - harmonic centrality            -> nx.harmonic_centrality(distance="weight")
    Sum of reciprocal weighted shortest path distances over the other nodes.

  - PageRank                       -> nx.pagerank(alpha=0.85, weight="weight")
    Weighted, normalized to sum to 1. NetworkX is run to a tight tolerance so the
    reference is effectively exact; the Rust side compares with a slack tolerance
    that absorbs iteration and rounding differences.

The graphs are simple (no self-loops and no parallel edges) and undirected, so
no direction or self-loop convention difference can creep into the comparison.

Regenerate with `make oracle-fixtures`.
"""

import json
import random
import sys

import networkx as nx

SEED = 0xCE_47_5A_11
NUM_GRAPHS = 80
MIN_NODES = 3
MAX_NODES = 12
# Densities avoid 0.0 so most graphs have edges to exercise; isolated nodes can
# still occur, which exercises disconnected handling for every measure.
DENSITIES = [0.15, 0.3, 0.5, 0.7]
MIN_WEIGHT = 1
MAX_WEIGHT = 10
ALPHA = 0.85
NX_TOL = 1e-12
NX_MAX_ITER = 2000


def build_graph(rng, n, density):
    """Build a simple undirected graph with positive integer edge weights."""
    g = nx.Graph()
    g.add_nodes_from(range(n))
    for u in range(n):
        for v in range(u + 1, n):
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
        edges = sorted((int(u), int(v)) for u, v in g.edges())
        weights = [int(g[u][v]["weight"]) for u, v in edges]

        degree = [g.degree(k) for k in range(n)]
        betw_raw = nx.betweenness_centrality(
            g, normalized=False, endpoints=False, weight=None
        )
        betw_norm = nx.betweenness_centrality(
            g, normalized=True, endpoints=False, weight=None
        )
        closeness = nx.closeness_centrality(g, distance="weight")
        harmonic = nx.harmonic_centrality(g, distance="weight")
        pagerank = nx.pagerank(g, alpha=ALPHA, tol=NX_TOL, max_iter=NX_MAX_ITER, weight="weight")

        cases.append(
            {
                "id": f"c{i:04d}",
                "n": n,
                "edges": [[u, v] for u, v in edges],
                "weights": weights,
                "degree": degree,
                "betweenness_raw": [betw_raw[k] for k in range(n)],
                "betweenness_norm": [betw_norm[k] for k in range(n)],
                "closeness": [closeness[k] for k in range(n)],
                "harmonic": [harmonic[k] for k in range(n)],
                "pagerank": [pagerank[k] for k in range(n)],
            }
        )

    corpus = {
        "meta": {
            "generator": "scripts/gen_oracle_centrality_fixtures.py",
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
