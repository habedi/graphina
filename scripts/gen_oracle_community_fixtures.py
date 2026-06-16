#!/usr/bin/env python3
"""Generate a NetworkX community-detection oracle corpus for Graphina.

Community-detection algorithms are randomized and have many near-optimal
solutions, so an exact partition match against NetworkX is not meaningful.
Modularity is, however, a stable scalar quality score, so this corpus pins the
quality of Graphina's Louvain output against NetworkX rather than the partition
itself.

The script produces a deterministic, seeded corpus of planted-partition graphs
(clear community structure) together with, for each graph:

  - the ground-truth block assignment (consecutive blocks of equal size),
  - the modularity of that ground-truth partition (nx.community.modularity), and
  - the modularity NetworkX's own Louvain achieves
    (nx.community.modularity over nx.community.louvain_communities).

The committed corpus is replayed by a hermetic Rust test
(`tests/oracle_community_tests.rs`): the Rust side runs Graphina's Louvain,
scores the resulting partition with the same modularity formula, and asserts the
score is within a slack tolerance of the NetworkX reference. The NetworkX
dependency lives only here, never in the Rust test path.

Graphs are unweighted and undirected. Regenerate with `make oracle-fixtures`.
"""

import json
import random
import sys

import networkx as nx

SEED = 0xC0_77_17_E5
NX_SEED = 12345
LS = [2, 3, 4]
KS = [4, 5, 6, 7]
P_INS = [0.6, 0.7, 0.85]
P_OUTS = [0.02, 0.05, 0.1]
NUM_GRAPHS = 45


def main():
    rng = random.Random(SEED)
    cases = []
    i = 0
    while len(cases) < NUM_GRAPHS:
        l = rng.choice(LS)
        k = rng.choice(KS)
        p_in = rng.choice(P_INS)
        p_out = rng.choice(P_OUTS)
        g = nx.planted_partition_graph(l, k, p_in, p_out, seed=rng.randint(0, 2**31))
        if g.number_of_edges() == 0:
            continue
        n = g.number_of_nodes()
        edges = sorted((int(u), int(v)) for u, v in g.edges())

        # Ground-truth blocks: nodes 0..k are block 0, k..2k block 1, and so on.
        ground_truth = [set(range(b * k, (b + 1) * k)) for b in range(l)]
        gt_modularity = nx.community.modularity(g, ground_truth)

        lv = nx.community.louvain_communities(g, seed=NX_SEED)
        lv_modularity = nx.community.modularity(g, lv)

        cases.append(
            {
                "id": f"q{i:04d}",
                "n": n,
                "blocks": l,
                "edges": [[u, v] for u, v in edges],
                "ground_truth_modularity": float(gt_modularity),
                "louvain_modularity": float(lv_modularity),
            }
        )
        i += 1

    corpus = {
        "meta": {
            "generator": "scripts/gen_oracle_community_fixtures.py",
            "networkx_version": nx.__version__,
            "seed": SEED,
            "num_graphs": len(cases),
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
