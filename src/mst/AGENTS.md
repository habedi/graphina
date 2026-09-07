# `mst` Semantics

Non-obvious semantics, return-type choices, and edge-case behavior for this module. Crate-wide rules live in the
root AGENTS.md; every function here is gated behind the module's feature flag unless the module is `core`.

`kruskal_mst`, `prim_mst`, and `boruvka_mst` each return `Result<(Vec<MstEdge<W>>, W)>` (edges plus total weight). They error only on an empty graph
and return a spanning forest (not an error) for a disconnected graph; a single node yields an empty edge set with zero weight. Weights need a total
order in practice (plain `f64` works; a `NaN` weight is an `InvalidArgument` error); `boruvka_mst` additionally requires `Send + Sync` and runs
its cheapest-edge search in parallel.
