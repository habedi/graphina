#!/usr/bin/env python3
"""Generate a NetworkX oracle corpus for Graphina directed-graph algorithms.

This is the directed companion to `gen_oracle_fixtures.py`. It produces a
deterministic, seeded corpus of simple directed, integer-weighted graphs with
the reference output of NetworkX for a fixed set of algorithms. The corpus is
committed to the repository and replayed by a hermetic Rust test
(`tests/oracle_directed_tests.rs`); the NetworkX dependency lives here, in the
generator, and never in the Rust test path.

Scope: the exact, spec-unambiguous algorithms only, where a mismatch is
unambiguously a bug rather than a convention difference:

  - in, out, and total degree      -> nx.DiGraph.in_degree / out_degree
  - directed shortest path lengths  -> nx.single_source_dijkstra_path_length
  - weakly connected components     -> nx.weakly_connected_components
  - strongly connected components   -> nx.strongly_connected_components

Edge weights are positive integers, so there are no negative cycles and the
single distance oracle (`sp_len`) is the correct reference for every shortest
path algorithm Graphina exposes on directed graphs: per-source Dijkstra and
Bellman-Ford, and all-pairs Floyd-Warshall and Johnson. The graphs are simple
(no self-loops and no parallel edges), so no multigraph or self-loop convention
difference can creep into the comparison, and the integer weight sums are exact
under f64.

Regenerate with `make oracle-fixtures`.
"""

import json
import random
import sys

import networkx as nx

SEED = 0xD1_EC_7E_D5
NUM_GRAPHS = 80
MIN_NODES = 3
MAX_NODES = 12
# Edge densities sampled per graph; the spread covers sparse (disconnected),
# moderate, and dense graphs so component counts and reachability vary.
DENSITIES = [0.0, 0.1, 0.2, 0.35, 0.5, 0.7]
MIN_WEIGHT = 1
MAX_WEIGHT = 10


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


def partition(component_sets):
    """Canonicalize a NetworkX component iterator to sorted node lists."""
    parts = [sorted(int(x) for x in comp) for comp in component_sets]
    parts.sort()
    return parts


def degrees(g, n):
    """In, out, and total degree per node (no self-loops in this corpus)."""
    return {
        "in": [g.in_degree(i) for i in range(n)],
        "out": [g.out_degree(i) for i in range(n)],
        "total": [g.in_degree(i) + g.out_degree(i) for i in range(n)],
    }


def sp_lengths(g, n):
    """All ordered (src, dst) pairs as [src, dst, weight] using the directed
    `weight` edge attribute; -1.0 marks an unreachable pair."""
    out = []
    for s in range(n):
        lengths = nx.single_source_dijkstra_path_length(g, s, weight="weight")
        for t in range(n):
            if s == t:
                continue
            if t in lengths:
                out.append([s, t, float(lengths[t])])
            else:
                out.append([s, t, -1.0])
    return out


def main():
    rng = random.Random(SEED)
    cases = []
    for i in range(NUM_GRAPHS):
        n = rng.randint(MIN_NODES, MAX_NODES)
        density = rng.choice(DENSITIES)
        g = build_graph(rng, n, density)
        edges = sorted((int(u), int(v)) for u, v in g.edges())
        weights = [int(g[u][v]["weight"]) for u, v in edges]
        cases.append(
            {
                "id": f"d{i:04d}",
                "n": n,
                "edges": [[u, v] for u, v in edges],
                "weights": weights,
                "degree": degrees(g, n),
                "sp_len": sp_lengths(g, n),
                "wcc": partition(nx.weakly_connected_components(g)),
                "scc": partition(nx.strongly_connected_components(g)),
            }
        )

    corpus = {
        "meta": {
            "generator": "scripts/gen_oracle_directed_fixtures.py",
            "networkx_version": nx.__version__,
            "seed": SEED,
            "num_graphs": NUM_GRAPHS,
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
