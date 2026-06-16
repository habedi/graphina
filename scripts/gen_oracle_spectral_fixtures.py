#!/usr/bin/env python3
"""Generate a NetworkX oracle corpus for Graphina spectral centralities.

This script produces a deterministic, seeded corpus of simple, connected,
undirected graphs together with the reference output of NetworkX for eigenvector
and Katz centrality. The corpus is committed to the repository and replayed by a
hermetic Rust test (`tests/oracle_spectral_tests.rs`); the NetworkX dependency
lives here, in the generator, and never in the Rust test path.

Eigenvector and Katz centrality are defined only up to a positive scale factor,
and Graphina and NetworkX choose different normalizations (Graphina scales the
eigenvector to sum to n and returns Katz unnormalized; NetworkX scales both to
unit L2 norm). The corpus therefore stores the L2-normalized reference vector,
and the Rust test L2-normalizes Graphina's output before comparing, so the test
checks the centrality structure rather than the normalization convention.

  - eigenvector centrality -> nx.eigenvector_centrality, L2-normalized
  - Katz centrality        -> nx.katz_centrality(alpha=ALPHA), L2-normalized

The graphs are connected (eigenvector centrality is well defined and NetworkX's
power iteration converges) and unweighted, so the adjacency spectral radius stays
below 1 / ALPHA and Katz converges. The graphs are simple (no self-loops and no
parallel edges).

Regenerate with `make oracle-fixtures`.
"""

import json
import random
import sys

import networkx as nx

SEED = 0x59EC_42A1
NUM_GRAPHS = 60
MIN_NODES = 4
MAX_NODES = 10
DENSITIES = [0.1, 0.2, 0.4]
# Attenuation factor for Katz. The unweighted adjacency spectral radius is at
# most n - 1 <= 9 for these graphs, and 0.05 < 1 / 9, so Katz converges.
ALPHA = 0.05
NX_MAX_ITER = 10000
NX_TOL = 1e-10


def build_connected_graph(rng, n, density):
    """Build a simple connected undirected graph (unit weights)."""
    g = nx.Graph()
    g.add_nodes_from(range(n))
    # Random spanning tree: link each node to a random earlier one.
    order = list(range(n))
    rng.shuffle(order)
    for idx in range(1, n):
        a = order[idx]
        b = order[rng.randrange(idx)]
        g.add_edge(a, b, weight=1.0)
    # Add extra edges by density.
    for u in range(n):
        for v in range(u + 1, n):
            if not g.has_edge(u, v) and rng.random() < density:
                g.add_edge(u, v, weight=1.0)
    return g


def l2_normalize(values):
    norm = sum(v * v for v in values) ** 0.5
    if norm == 0.0:
        return [0.0 for _ in values]
    return [v / norm for v in values]


def main():
    rng = random.Random(SEED)
    cases = []
    i = 0
    while len(cases) < NUM_GRAPHS:
        n = rng.randint(MIN_NODES, MAX_NODES)
        density = rng.choice(DENSITIES)
        g = build_connected_graph(rng, n, density)
        try:
            eig = nx.eigenvector_centrality(g, max_iter=NX_MAX_ITER, tol=NX_TOL)
            katz = nx.katz_centrality(g, alpha=ALPHA, max_iter=NX_MAX_ITER, tol=NX_TOL)
        except nx.PowerIterationFailedConvergence:
            # Skip graphs where NetworkX's power iteration does not converge
            # (for example near-bipartite graphs); resample a different graph.
            continue
        edges = sorted((int(u), int(v)) for u, v in g.edges())
        cases.append(
            {
                "id": f"s{i:04d}",
                "n": n,
                "edges": [[u, v] for u, v in edges],
                "eigenvector": l2_normalize([eig[k] for k in range(n)]),
                "katz": l2_normalize([katz[k] for k in range(n)]),
            }
        )
        i += 1

    corpus = {
        "meta": {
            "generator": "scripts/gen_oracle_spectral_fixtures.py",
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
