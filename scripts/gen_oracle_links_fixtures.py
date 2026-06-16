#!/usr/bin/env python3
"""Generate a NetworkX link-prediction oracle corpus for Graphina.

This script produces a deterministic, seeded corpus of simple undirected graphs
together with the reference output of NetworkX for the link-prediction scores
whose convention Graphina matches exactly. The corpus is committed to the
repository and replayed by a hermetic Rust test (`tests/oracle_links_tests.rs`);
the NetworkX dependency lives here, in the generator, and never in the Rust test
path.

Each score is a function of a node pair (u, v). Graphina exposes one scalar per
pair; NetworkX exposes an `ebunch` iterator yielding `(u, v, score)`. To pin the
comparison, the generator enumerates every unordered pair (u, v) with u < v
(both edges and non-edges) and stores the NetworkX score for each, so the Rust
side can replay the exact same pairs:

  - common neighbors          -> len(list(nx.common_neighbors(g, u, v)))
    Graphina `common_neighbors` returns the raw intersection count.

  - Jaccard coefficient       -> nx.jaccard_coefficient(g, [(u, v)])
    |N(u) ∩ N(v)| / |N(u) ∪ N(v)|, 0 when the union is empty.

  - Adamic-Adar index         -> nx.adamic_adar_index(g, [(u, v)])
    Σ 1 / ln(deg(w)) over common neighbors w; degree-1 neighbors contribute 0.

  - preferential attachment   -> nx.preferential_attachment(g, [(u, v)])
    deg(u) * deg(v).

  - resource allocation index -> nx.resource_allocation_index(g, [(u, v)])
    Σ 1 / deg(w) over common neighbors w.

The graphs are simple (no self-loops and no parallel edges) and undirected, so
the neighbor-set conventions match exactly. Graphs are unweighted because every
link-prediction score above is a function of the graph structure alone.

Regenerate with `make oracle-fixtures`.
"""

import json
import random
import sys

import networkx as nx

SEED = 0x11_4B_5C_0E
NUM_GRAPHS = 80
MIN_NODES = 3
MAX_NODES = 12
# Densities avoid extremes so most graphs have a mix of edges and non-edges,
# exercising both classes of pair; isolated nodes (empty neighbor sets) can
# still occur and exercise the empty-union edge case.
DENSITIES = [0.15, 0.3, 0.5, 0.7]


def build_graph(rng, n, density):
    """Build a simple unweighted undirected graph."""
    g = nx.Graph()
    g.add_nodes_from(range(n))
    for u in range(n):
        for v in range(u + 1, n):
            if rng.random() < density:
                g.add_edge(u, v)
    return g


def main():
    rng = random.Random(SEED)
    cases = []
    for i in range(NUM_GRAPHS):
        n = rng.randint(MIN_NODES, MAX_NODES)
        density = rng.choice(DENSITIES)
        g = build_graph(rng, n, density)
        edges = sorted((int(u), int(v)) for u, v in g.edges())

        pairs = [(u, v) for u in range(n) for v in range(u + 1, n)]

        def score_map(predictor):
            return {(u, v): float(s) for u, v, s in predictor(g, pairs)}

        jaccard = score_map(nx.jaccard_coefficient)
        adamic = score_map(nx.adamic_adar_index)
        pref = score_map(nx.preferential_attachment)
        resource = score_map(nx.resource_allocation_index)
        common = {
            (u, v): len(list(nx.common_neighbors(g, u, v))) for u, v in pairs
        }

        cases.append(
            {
                "id": f"l{i:04d}",
                "n": n,
                "edges": [[u, v] for u, v in edges],
                "pairs": [[u, v] for u, v in pairs],
                "common_neighbors": [common[(u, v)] for u, v in pairs],
                "jaccard": [jaccard[(u, v)] for u, v in pairs],
                "adamic_adar": [adamic[(u, v)] for u, v in pairs],
                "preferential_attachment": [pref[(u, v)] for u, v in pairs],
                "resource_allocation": [resource[(u, v)] for u, v in pairs],
            }
        )

    corpus = {
        "meta": {
            "generator": "scripts/gen_oracle_links_fixtures.py",
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
