import networkx as nx

G = nx.DiGraph()

for i in range(5):
    G.add_node(i)

edges = [
    (0, 1, 1.0),
    (0, 2, 2.0),
    (1, 3, 1.0),
]

for src, dst, weight in edges:
    G.add_edge(src, dst, weight=weight)

params = [
    (0.1, 1.0, 1000, None, True),
    (0.1, 1.0, 1000, None, False),
    (0.01, 0.5, 1000, "weight", False),
    (0.01, 0.5, 1000, "weight", True),
]

for alpha, beta, max_iter, weight, normalized in params:
    centrality = nx.katz_centrality(
        G,
        alpha=alpha,
        beta=beta,
        max_iter=max_iter,
        weight=weight,
        normalized=normalized,
    )

    print(f"alpha: {alpha:.3f}, beta: {beta:.3f}, max iter: {max_iter:>5}, ", end="")
    print(f"weighted: {weight=='weight'}, normalized: {normalized}".lower())
    for k, v in centrality.items():
        print(f">> {k} : {v:>.5f}")
    print()
