# Graph Validation

Graphina provides a set of validation functions in the `core::validation` module. These functions check preconditions on graph properties before running algorithms.

## Precondition Verification

Many graph algorithms require specific graph properties. For example, Dijkstra's algorithm assumes non-negative edge weights, and topological sorting requires a Directed Acyclic Graph (DAG). 

Using the validation functions helps ensure the input graph meets these requirements.

## Boolean Predicates

Predicate functions return `bool` values:

*   `is_empty(&graph)`: Returns `true` if the graph contains no nodes.
*   `is_connected(&graph)`: Returns `true` if the graph is connected (or weakly connected for directed graphs).
*   `has_negative_weights(&graph)`: Returns `true` if any edge has a weight less than `0.0`.
*   `has_self_loops(&graph)`: Returns `true` if there are edges connecting a node to itself.
*   `is_dag(&graph)`: Returns `true` if the graph is a directed acyclic graph.
*   `is_bipartite(&graph)`: Returns `true` if the graph can be partitioned into two independent sets.
*   `count_components(&graph)`: Returns the number of connected components in the graph.

```rust
use graphina::core::validation::{is_connected, is_dag};

if is_connected(&graph) && is_dag(&graph) {
    // Run algorithm
}
```

## Precondition Validators

Validator functions return `Result<(), GraphinaError>` and yield an error if the condition is not met. These are prefixed with `require_`:

*   `require_non_empty(&graph)`
*   `require_connected(&graph)`
*   `require_directed(&graph)`
*   `require_undirected(&graph)`
*   `require_no_negative_weights(&graph)`
*   `require_no_self_loops(&graph)`
*   `require_dag(&graph)`
*   `require_bipartite(&graph)`

```rust
use graphina::core::validation::require_dag;

fn run_custom_algorithm(graph: &MyGraph) -> Result<()> {
    require_dag(graph)?;
    // Implementation
    Ok(())
}
```
