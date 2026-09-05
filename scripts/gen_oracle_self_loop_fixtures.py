#!/usr/bin/env python3
"""Generate a NetworkX oracle corpus of graphs with self-loops for Graphina.

The other oracle corpora use simple graphs, so no self-loop convention is ever
exercised by them. This script produces a deterministic, seeded corpus of
graphs in which several nodes carry a self-loop, together with the reference
output of NetworkX for the measures whose self-loop handling Graphina pins to
the NetworkX convention. The corpus is committed to the repository and
replayed by a hermetic Rust test (`tests/oracle_self_loop_tests.rs`); the
NetworkX dependency lives here, in the generator, and never in the Rust test
path.

Conventions pinned here:

  - A self-loop is a single edge. In the undirected adjacency matrix it is one
    diagonal entry, not two, so it adds its weight once to the node's strength
    in PageRank, personalized PageRank, and eigenvector centrality.

  - A self-loop does not make a node its own neighbor for clustering purposes.
    Per-node clustering, per-node triangle counts, transitivity, and average
    clustering all ignore self-loops.

  - A self-loop adds two to a node's degree (once per edge end), which feeds
    degree centrality, degree assortativity, and the VoteRank decay rate.

Undirected cases are connected (a random spanning tree plus extra edges), so
eigenvector centrality is well defined. Directed cases pin weighted PageRank
only. Every case has at least one self-loop.

Measures and the matching NetworkX call:

  - pagerank                -> nx.pagerank(alpha=0.85, weight="weight")
  - personalized_pagerank   -> nx.pagerank(personalization=..., weight="weight")
  - eigenvector             -> nx.eigenvector_centrality(weight="weight"), L2-normalized
  - transitivity            -> nx.transitivity
  - average_clustering      -> nx.average_clustering
  - clustering (per node)   -> nx.clustering
  - triangles (per node)    -> nx.triangles
  - degree                  -> nx.Graph.degree (self-loop counts two)
  - voterank                -> nx.voterank (unweighted election order)
  - assortativity           -> nx.degree_assortativity_coefficient (null when NaN)

Regenerate with `make oracle-fixtures`.
"""

import json
import math
import random
import sys

import networkx as nx

SEED = 0x5E_1F_10_0B
NUM_UNDIRECTED = 60
NUM_DIRECTED = 40
MIN_NODES = 3
MAX_NODES = 12
EXTRA_EDGE_DENSITIES = [0.1, 0.25, 0.45, 0.7]
DIRECTED_DENSITIES = [0.2, 0.35, 0.5, 0.7]
SELF_LOOP_PROBABILITY = 0.4
MIN_WEIGHT = 1
MAX_WEIGHT = 10
ALPHA = 0.85
NX_TOL = 1e-12
NX_MAX_ITER = 5000


def add_self_loops(rng, g):
    """Give each node a self-loop with fixed probability, forcing at least one."""
    n = g.number_of_nodes()
    looped = [u for u in range(n) if rng.random() < SELF_LOOP_PROBABILITY]
    if not looped:
        looped = [rng.randrange(n)]
    for u in looped:
        g.add_edge(u, u, weight=rng.randint(MIN_WEIGHT, MAX_WEIGHT))


def build_connected_graph(rng, n, density):
    """Build a connected undirected graph with positive integer weights and
    self-loops: a random spanning tree plus extra edges sampled at `density`."""
    g = nx.Graph()
    g.add_nodes_from(range(n))
    nodes = list(range(n))
    rng.shuffle(nodes)
    for i in range(1, n):
        u = nodes[i]
        v = nodes[rng.randint(0, i - 1)]
        g.add_edge(u, v, weight=rng.randint(MIN_WEIGHT, MAX_WEIGHT))
    for u in range(n):
        for v in range(u + 1, n):
            if not g.has_edge(u, v) and rng.random() < density:
                g.add_edge(u, v, weight=rng.randint(MIN_WEIGHT, MAX_WEIGHT))
    add_self_loops(rng, g)
    return g


def build_directed_graph(rng, n, density):
    """Build a directed graph with positive integer weights and self-loops."""
    g = nx.DiGraph()
    g.add_nodes_from(range(n))
    for u in range(n):
        for v in range(n):
            if u != v and rng.random() < density:
                g.add_edge(u, v, weight=rng.randint(MIN_WEIGHT, MAX_WEIGHT))
    add_self_loops(rng, g)
    return g


def canonical(g):
    """Rebuild `g` with nodes and edges inserted in sorted order.

    The Rust replay inserts edges in this order, and VoteRank accumulates
    floating-point votes in edge order, so an exact tie between two nodes is
    broken the same way on both sides only when the summation order matches.
    """
    h = g.__class__()
    h.add_nodes_from(range(g.number_of_nodes()))
    for u, v in sorted((int(a), int(b)) for a, b in g.edges()):
        h.add_edge(u, v, weight=g[u][v]["weight"])
    return h


def edge_lists(g):
    edges = sorted((int(u), int(v)) for u, v in g.edges())
    weights = [int(g[u][v]["weight"]) for u, v in edges]
    return [[u, v] for u, v in edges], weights


def l2_normalized(values, n):
    norm = math.sqrt(sum(values[k] ** 2 for k in range(n)))
    return [values[k] / norm for k in range(n)]


def main():
    rng = random.Random(SEED)
    undirected = []
    for i in range(NUM_UNDIRECTED):
        n = rng.randint(MIN_NODES, MAX_NODES)
        density = rng.choice(EXTRA_EDGE_DENSITIES)
        g = canonical(build_connected_graph(rng, n, density))
        edges, weights = edge_lists(g)

        pagerank = nx.pagerank(g, alpha=ALPHA, tol=NX_TOL, max_iter=NX_MAX_ITER, weight="weight")
        total = sum(k + 1 for k in range(n))
        personalization = [(k + 1) / total for k in range(n)]
        ppr = nx.pagerank(
            g,
            alpha=ALPHA,
            personalization={k: personalization[k] for k in range(n)},
            tol=NX_TOL,
            max_iter=NX_MAX_ITER,
            weight="weight",
        )
        eigenvector = nx.eigenvector_centrality(g, max_iter=NX_MAX_ITER, tol=1e-10, weight="weight")
        clustering = nx.clustering(g)
        triangles = nx.triangles(g)
        assortativity = nx.degree_assortativity_coefficient(g)
        assort = None if math.isnan(assortativity) else float(assortativity)

        undirected.append(
            {
                "id": f"sl{i:04d}",
                "n": n,
                "edges": edges,
                "weights": weights,
                "pagerank": [pagerank[k] for k in range(n)],
                "personalization": personalization,
                "personalized_pagerank": [ppr[k] for k in range(n)],
                "eigenvector": l2_normalized(eigenvector, n),
                "transitivity": float(nx.transitivity(g)),
                "average_clustering": float(nx.average_clustering(g)),
                "clustering": [float(clustering[k]) for k in range(n)],
                "triangles": [int(triangles[k]) for k in range(n)],
                "degree": [int(g.degree(k)) for k in range(n)],
                "voterank": [int(x) for x in nx.voterank(g)],
                "assortativity": assort,
            }
        )

    directed = []
    for i in range(NUM_DIRECTED):
        n = rng.randint(MIN_NODES, MAX_NODES)
        density = rng.choice(DIRECTED_DENSITIES)
        g = canonical(build_directed_graph(rng, n, density))
        edges, weights = edge_lists(g)
        pagerank = nx.pagerank(g, alpha=ALPHA, tol=NX_TOL, max_iter=NX_MAX_ITER, weight="weight")
        directed.append(
            {
                "id": f"dsl{i:04d}",
                "n": n,
                "edges": edges,
                "weights": weights,
                "pagerank": [pagerank[k] for k in range(n)],
            }
        )

    corpus = {
        "meta": {
            "generator": "scripts/gen_oracle_self_loop_fixtures.py",
            "networkx_version": nx.__version__,
            "seed": SEED,
            "num_undirected": NUM_UNDIRECTED,
            "num_directed": NUM_DIRECTED,
            "alpha": ALPHA,
        },
        "undirected": undirected,
        "directed": directed,
    }

    out_path = sys.argv[1] if len(sys.argv) > 1 else "-"
    text = json.dumps(corpus, indent=2, sort_keys=True) + "\n"
    if out_path == "-":
        sys.stdout.write(text)
    else:
        with open(out_path, "w", encoding="utf-8") as f:
            f.write(text)
        print(
            f"Wrote {len(undirected)} undirected and {len(directed)} directed graphs to {out_path}",
            file=sys.stderr,
        )


if __name__ == "__main__":
    main()
