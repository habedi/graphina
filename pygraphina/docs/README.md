## PyGraphina Documentation

This document summarizes the Python API exposed by PyGraphina and how it maps to Graphina's Rust API. All Python function names match their Rust equivalents (no `_py` suffix).

Quick start

- Construct graphs via `PyGraph` or generators:
  - `erdos_renyi(n, p, seed)`
  - `complete_graph(n)`, `bipartite(n1, n2, p, seed)`, `star_graph(n)`, `cycle_graph(n)`, `watts_strogatz(n, k, beta, seed)`, `barabasi_albert(n, m, seed)`
- Use methods on `PyGraph` for core operations (add_node, add_edge, bfs, dijkstra, etc.).

Centrality

- `degree(g)`, `in_degree(g)`, `out_degree(g)`
- `betweenness(g, normalized)`, `edge_betweenness(g, normalized)`
- `closeness(g)`, `harmonic(g)`
- `eigenvector(g, max_iter, tolerance)`
- `pagerank(g, damping, max_iter, tolerance)`
- `katz(g, alpha, max_iter, tolerance)`

Core (selected)

- Parallel: `bfs_parallel(g, starts)`, `degrees_parallel(g)`, `connected_components_parallel(g)`
- MST: `prim_mst(g)`, `kruskal_mst(g)`, `boruvka_mst(g)` → returns `(total_weight, [(u, v, w), ...])`
- Subgraphs: `g.subgraph(nodes)`, `g.induced_subgraph(nodes)`

Approximation

- Clique: `max_clique(g) -> [node]`, `clique_removal(g) -> [[node]]`, `large_clique_size(g) -> int`
- Vertex cover: `min_weighted_vertex_cover(g) -> [node]`
- Diameter (lower bound): `diameter(g) -> float`

Community

- `connected_components(g) -> [[node]]`
- `label_propagation(g, max_iter, seed=None) -> {node: label}`
- `louvain(g, seed=None) -> [[node]]`

Links (link prediction)

- `jaccard_coefficient(g, ebunch=None) -> {(u, v): score}`
- `adamic_adar_index(g, ebunch=None) -> {(u, v): score}`
- `preferential_attachment(g, ebunch=None) -> {(u, v): score}`
- `common_neighbors(g, u, v) -> int`
- `common_neighbor_centrality(g, alpha, ebunch=None) -> {(u, v): score}`

Conventions

- All functions accept a `PyGraph` instance (`g`) as first argument where applicable.
- Node IDs are stable per `PyGraph` and map to internal IDs automatically.
- Edge weights in Python are `float` (f64). Some algorithms require ordered weights internally; PyGraphina handles conversions as needed.

Examples

- Build a graph and run centrality:
  - `g = complete_graph(5)`
  - `pr = pagerank(g, 0.85, 100, 1e-6)`
- MST:
  - `total, edges = prim_mst(g)`
- Communities:
  - `labels = label_propagation(g, 10, 42)`
  - `comms = louvain(g, 42)`
- Link prediction:
  - `jc = jaccard_coefficient(g)`

Troubleshooting

- If you previously used `_py` suffixed names, update to the names above.
- If Python can’t find a symbol, reinstall into your active environment.
