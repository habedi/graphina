#!/usr/bin/env python3
"""Generate a NetworkX oracle corpus for Graphina core algorithms.

This script produces a deterministic, seeded corpus of simple undirected,
integer-weighted graphs together with the reference output of NetworkX for a
fixed set of algorithms. The corpus is committed to the repository and replayed
by a hermetic Rust test (`tests/oracle_tests.rs`); the NetworkX dependency lives
here, in the generator, and never in the Rust test path.

Scope (first slice): the exact, spec-unambiguous algorithms only, where a
mismatch is unambiguously a bug rather than a convention difference:

  - connected components        -> nx.connected_components (undirected)
  - degree per node             -> nx.Graph.degree
  - weighted shortest path len  -> nx.single_source_dijkstra_path_length
  - minimum spanning tree       -> nx.minimum_spanning_tree (connected graphs)

The graphs are simple (no self-loops and no parallel edges) and undirected, so
no multigraph, self-loop, or direction convention difference can creep into the
comparison. Edge weights are small integers, so the weighted shortest path and
spanning tree totals are exact under f64 and need no tolerance.

The minimum spanning tree is compared by total weight and edge count rather than
by edge set, because the tree is not unique when edge weights tie; the total
weight and edge count are. The MST oracle is emitted only for connected graphs
(a single component), where every Graphina MST algorithm yields a tree with
exactly n - 1 edges and the same total weight.

Regenerate with `make oracle-fixtures`.
"""

import json
import random
import sys

import networkx as nx

SEED = 0x6A6F_1A11
NUM_GRAPHS = 80
MIN_NODES = 3
MAX_NODES = 12
# Edge densities sampled per graph; the spread covers sparse (disconnected),
# moderate, and dense graphs so component counts and reachability vary.
DENSITIES = [0.0, 0.1, 0.2, 0.35, 0.5, 0.7]
MIN_WEIGHT = 1
MAX_WEIGHT = 10


def build_graph(rng, n, density):
    """Build a simple undirected graph with integer edge weights."""
    g = nx.Graph()
    g.add_nodes_from(range(n))
    for u in range(n):
        for v in range(u + 1, n):
            if rng.random() < density:
                g.add_edge(u, v, weight=rng.randint(MIN_WEIGHT, MAX_WEIGHT))
    return g


def partition(component_sets):
    """Canonicalize a NetworkX component iterator to sorted node lists."""
    parts = [sorted(int(x) for x in comp) for comp in component_sets]
    parts.sort()
    return parts


def degrees(g, n):
    """Degree per node, indexed 0..n."""
    return [g.degree(i) for i in range(n)]


def dijkstra_lengths(g, n):
    """All ordered (src, dst) pairs as [src, dst, weight] using the `weight`
    edge attribute; -1.0 marks an unreachable pair."""
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


def spanning_tree(g, n):
    """Minimum spanning tree total weight and edge count, or None when the graph
    is not connected. Only the total weight and edge count are reported, since
    the chosen edge set is not unique under weight ties."""
    if n == 0 or not nx.is_connected(g):
        return None
    mst = nx.minimum_spanning_tree(g, weight="weight")
    total = sum(d["weight"] for *_, d in mst.edges(data=True))
    return {"weight": float(total), "edges": mst.number_of_edges()}


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
                "id": f"g{i:04d}",
                "n": n,
                "edges": [[u, v] for u, v in edges],
                "weights": weights,
                "components": partition(nx.connected_components(g)),
                "degree": degrees(g, n),
                "dijkstra": dijkstra_lengths(g, n),
                "mst": spanning_tree(g, n),
            }
        )

    corpus = {
        "meta": {
            "generator": "scripts/gen_oracle_fixtures.py",
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
