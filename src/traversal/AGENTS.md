# `traversal` Semantics

Non-obvious semantics, return-type choices, and edge-case behavior for this module. Crate-wide rules live in the
root AGENTS.md; every function here is gated behind the module's feature flag unless the module is `core`.

- `bfs(graph, start) -> Vec<NodeId>` and `dfs(graph, start) -> Vec<NodeId>`: visitation order; empty vector for a missing start node.
- `iddfs(graph, start, target, max_depth) -> Option<Vec<NodeId>>` and `bidis(graph, start, target) -> Option<Vec<NodeId>>`: return the path or `None`;
  `bidis` returns the unweighted shortest path. The `try_iddfs` and `try_bidirectional_search` variants return `Result<Vec<NodeId>>`, validating node
  existence (`node_not_found`) and distinguishing `no_path`.
