#!/usr/bin/env python3
"""Generate a NetworkX graph-metrics oracle corpus for Graphina.

This script produces a deterministic, seeded corpus of simple, connected,
undirected, unweighted graphs together with the reference output of NetworkX for
the structural metrics Graphina implements. The corpus is committed to the
repository and replayed by a hermetic Rust test (`tests/oracle_metrics_tests.rs`);
the NetworkX dependency lives here, in the generator, and never in the Rust test
path.

Graphs are connected because several Graphina metrics (`diameter`, `radius`, and
`average_path_length`) are defined only on connected graphs and return `None`
otherwise; a connected corpus exercises the numeric path of every metric. The
graphs are unweighted, and Graphina's distance-based metrics use unweighted BFS,
so the comparison is against the unweighted NetworkX metrics.

Each metric is pinned to the convention Graphina implements:

  - diameter                       -> nx.diameter (unweighted)
  - radius                         -> nx.radius (unweighted)
  - average shortest path length   -> nx.average_shortest_path_length (unweighted)
  - transitivity                   -> nx.transitivity
  - average clustering             -> nx.average_clustering
  - clustering (per node)          -> nx.clustering
  - triangles (per node)           -> nx.triangles
  - degree assortativity           -> nx.degree_assortativity_coefficient

NetworkX returns NaN for degree assortativity when every edge endpoint has the
same degree (zero variance); Graphina returns 0.0 in that case, so the corpus
stores a null and the Rust side skips the comparison for those graphs.

Regenerate with `make oracle-fixtures`.
"""

import json
import math
import random
import sys

import networkx as nx

SEED = 0x3E_77_1C_5A
NUM_GRAPHS = 80
MIN_NODES = 3
MAX_NODES = 12
# Probability of adding each extra edge beyond the spanning tree. The spanning
# tree guarantees connectivity; the extra edges vary density, clustering, and
# diameter across the corpus.
EXTRA_EDGE_DENSITIES = [0.1, 0.25, 0.45, 0.7]


def build_connected_graph(rng, n, density):
    """Build a simple connected unweighted graph: a random spanning tree plus
    extra edges sampled at `density`."""
    g = nx.Graph()
    g.add_nodes_from(range(n))
    # Random spanning tree: attach each new node to a random earlier node.
    nodes = list(range(n))
    rng.shuffle(nodes)
    for i in range(1, n):
        u = nodes[i]
        v = nodes[rng.randint(0, i - 1)]
        g.add_edge(u, v)
    for u in range(n):
        for v in range(u + 1, n):
            if not g.has_edge(u, v) and rng.random() < density:
                g.add_edge(u, v)
    return g


def main():
    rng = random.Random(SEED)
    cases = []
    for i in range(NUM_GRAPHS):
        n = rng.randint(MIN_NODES, MAX_NODES)
        density = rng.choice(EXTRA_EDGE_DENSITIES)
        g = build_connected_graph(rng, n, density)
        edges = sorted((int(u), int(v)) for u, v in g.edges())

        assortativity = nx.degree_assortativity_coefficient(g)
        # NaN means undefined (zero degree variance); store null and skip on the
        # Rust side, where Graphina returns 0.0 by convention.
        assort = None if math.isnan(assortativity) else float(assortativity)

        clustering = nx.clustering(g)
        triangles = nx.triangles(g)

        cases.append(
            {
                "id": f"m{i:04d}",
                "n": n,
                "edges": [[u, v] for u, v in edges],
                "diameter": int(nx.diameter(g)),
                "radius": int(nx.radius(g)),
                "average_path_length": float(nx.average_shortest_path_length(g)),
                "transitivity": float(nx.transitivity(g)),
                "average_clustering": float(nx.average_clustering(g)),
                "clustering": [float(clustering[k]) for k in range(n)],
                "triangles": [int(triangles[k]) for k in range(n)],
                "assortativity": assort,
            }
        )

    corpus = {
        "meta": {
            "generator": "scripts/gen_oracle_metrics_fixtures.py",
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
