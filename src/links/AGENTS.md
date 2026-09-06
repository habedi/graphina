# `links` Semantics

Non-obvious semantics, return-type choices, and edge-case behavior for this module. Crate-wide rules live in the
root AGENTS.md; every function here is gated behind the module's feature flag unless the module is `core`.

All link-prediction scorers take an optional `ebunch: Option<&[(NodeId, NodeId)]>` (defaulting to all unordered node pairs), operate on `f64`-weighted
graphs, treat pairs as undirected, and return a plain `Vec<((NodeId, NodeId), f64)>` (never a `Result`).

- `resource_allocation_index`, `adamic_adar_index`: sum over common neighbors; Adamic-Adar skips neighbors of degree `<= 1` (avoids `ln(1) = 0`). No
  common neighbors yields `0.0`.
- `jaccard_coefficient`: intersection over union of neighbor sets; `0.0` when the union is empty.
- `preferential_attachment`: `degree(u) * degree(v)`.
- `common_neighbor_centrality(graph, ebunch, alpha)`: `|N(u) ∩ N(v)|^alpha`.
- `common_neighbors(graph, u, v) -> usize`: plain count, not a scorer.
- Community-aware variants (`ra_index_soundarajan_hopcroft`, `cn_soundarajan_hopcroft`, `within_inter_cluster`) take a `community: Fn(NodeId) -> C`
  closure; `within_inter_cluster` scores `0.0` for a pair in different communities and otherwise `within / (inter + delta)` as in NetworkX,
  where the positive `delta` keeps the score finite when there are no inter-cluster common neighbors.
