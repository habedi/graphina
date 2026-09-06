# `subgraphs` Semantics

Non-obvious semantics, return-type choices, and edge-case behavior for this module. Crate-wide rules live in the
root AGENTS.md; every function here is gated behind the module's feature flag unless the module is `core`.

The `SubgraphOps` trait is implemented for `BaseGraph`. Extraction methods that build a new graph (`subgraph`, `induced_subgraph`, `ego_graph`,
`component_subgraph`) return `Result` and remap `NodeId`s in the result; `filter_nodes` and `filter_edges` also remap but return the graph directly.
Query methods (`k_hop_neighbors`, `connected_component`) return `Vec<NodeId>` over the original ids, with `radius`/`k` of 0 returning just the start
node.
