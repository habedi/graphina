import networkx as nx

G = nx.Graph()

for i in range(5):
    G.add_node(i)

edges = [
    (0, 1, 1.0),
    (0, 2, 2.0),
    (1, 3, 1.0),
]

for src, dst, weight in edges:
    G.add_edge(src, dst, weight=weight)

centrality = nx.eigenvector_centrality(G, weight=None)
print("Unweighted: ")
for v, c in centrality.items():
    print(f">> {v} : {c:.5f}")
centrality = nx.eigenvector_centrality(G, weight="weight")
print("Weighted: ")
for v, c in centrality.items():
    print(f">> {v} : {c:.5f}")
