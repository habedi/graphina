# Graph Validation Feature

**Date:** October 18, 2025  
**Version:** 0.4.0  
**Status:** ✅ Implemented and Tested

## Overview

The Graph Validation feature provides comprehensive utilities for checking graph properties and validating preconditions before running algorithms. This prevents silent failures, provides clear error messages, and improves code quality.

## Benefits

- **Prevents Silent Failures**: Catches invalid inputs before algorithms run
- **Clear Error Messages**: Descriptive messages explain what went wrong
- **Centralized Validation**: Reusable checks reduce code duplication
- **Professional Quality**: Industry-standard error handling

---

## API Reference

### Property Checking Functions

These functions check graph properties and return boolean values.

#### `is_empty(graph) -> bool`

Returns true if the graph contains no nodes.

```rust
use graphina::core::{types::Graph, validation::is_empty};

let g = Graph::<i32, f64>::new();
assert!(is_empty(&g));
```

---

#### `is_connected(graph) -> bool`

Returns true if the graph is connected (undirected) or weakly connected (directed).

```rust
use graphina::core::{types::Graph, validation::is_connected};

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
g.add_edge(n1, n2, 1.0);

assert!(is_connected(&g));
```

**Algorithm:** Uses DFS to check if all nodes are reachable from a starting node.  
**Time Complexity:** O(V + E)

---

#### `has_negative_weights(graph) -> bool`

Returns true if any edge has a negative weight.

```rust
use graphina::core::{types::Graph, validation::has_negative_weights};

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
g.add_edge(n1, n2, -5.0);

assert!(has_negative_weights(&g));
```

**Time Complexity:** O(E)

---

#### `has_self_loops(graph) -> bool`

Returns true if any node has an edge to itself.

```rust
use graphina::core::{types::Graph, validation::has_self_loops};

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
g.add_edge(n1, n1, 1.0); // Self-loop

assert!(has_self_loops(&g));
```

**Time Complexity:** O(E)

---

#### `is_dag(graph) -> bool`

Returns true if the directed graph is acyclic (a DAG).

```rust
use graphina::core::{types::Digraph, validation::is_dag};

let mut g = Digraph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
let n3 = g.add_node(3);
g.add_edge(n1, n2, 1.0);
g.add_edge(n2, n3, 1.0);

assert!(is_dag(&g));

// Adding a cycle
g.add_edge(n3, n1, 1.0);
assert!(!is_dag(&g));
```

**Algorithm:** Three-color DFS cycle detection (white, gray, black)  
**Time Complexity:** O(V + E)

---

#### `is_bipartite(graph) -> bool`

Returns true if the graph can be 2-colored (is bipartite).

```rust
use graphina::core::{types::Graph, validation::is_bipartite};

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
let n3 = g.add_node(3);
let n4 = g.add_node(4);

// Bipartite: partition {1,2} and {3,4}
g.add_edge(n1, n3, 1.0);
g.add_edge(n1, n4, 1.0);
g.add_edge(n2, n3, 1.0);
g.add_edge(n2, n4, 1.0);

assert!(is_bipartite(&g));
```

**Algorithm:** BFS with 2-coloring  
**Time Complexity:** O(V + E)

---

#### `count_components(graph) -> usize`

Returns the number of connected components.

```rust
use graphina::core::{types::Graph, validation::count_components};

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
let n3 = g.add_node(3);
let n4 = g.add_node(4);

// Two components: {1,2} and {3,4}
g.add_edge(n1, n2, 1.0);
g.add_edge(n3, n4, 1.0);

assert_eq!(count_components(&g), 2);
```

**Algorithm:** Iterative DFS component counting  
**Time Complexity:** O(V + E)

---

### Validation Functions

These functions validate preconditions and return `Result<(), GraphinaException>`.

#### `require_non_empty(graph, algo_name) -> Result<(), GraphinaException>`

Validates that the graph has at least one node.

```rust
use graphina::core::{types::Graph, validation::require_non_empty};

let mut g = Graph::<i32, f64>::new();
g.add_node(1);

assert!(require_non_empty(&g, "my_algorithm").is_ok());

let empty = Graph::<i32, f64>::new();
assert!(require_non_empty(&empty, "my_algorithm").is_err());
```

---

#### `require_connected(graph, algo_name) -> Result<(), GraphinaException>`

Validates that the graph is connected.

```rust
use graphina::core::{types::Graph, validation::require_connected};

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
g.add_edge(n1, n2, 1.0);

assert!(require_connected(&g, "shortest_path").is_ok());
```

---

#### `require_non_negative_weights(graph, algo_name) -> Result<(), GraphinaException>`

Validates that all edge weights are non-negative.

```rust
use graphina::core::{types::Graph, validation::require_non_negative_weights};

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
g.add_edge(n1, n2, 5.0);

assert!(require_non_negative_weights(&g, "dijkstra").is_ok());
```

---

#### `require_no_self_loops(graph, algo_name) -> Result<(), GraphinaException>`

Validates that the graph has no self-loops.

```rust
use graphina::core::{types::Graph, validation::require_no_self_loops};

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
g.add_edge(n1, n2, 1.0);

assert!(require_no_self_loops(&g, "coloring").is_ok());
```

---

#### `require_dag(graph, algo_name) -> Result<(), GraphinaException>`

Validates that the directed graph is acyclic.

```rust
use graphina::core::{types::Digraph, validation::require_dag};

let mut g = Digraph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
g.add_edge(n1, n2, 1.0);

assert!(require_dag(&g, "topological_sort").is_ok());
```

---

#### `validate_for_algorithm(graph, algo_name) -> Result<(), GraphinaException>`

Composite validation: checks non-empty, connected, and non-negative weights.

```rust
use graphina::core::{types::Graph, validation::validate_for_algorithm};

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
g.add_edge(n1, n2, 1.0);

// All checks pass
assert!(validate_for_algorithm(&g, "pagerank").is_ok());
```

---

## Usage Patterns

### Pattern 1: Algorithm Precondition Checks

```rust
use graphina::core::{
    types::Graph,
    validation::{require_connected, require_non_negative_weights},
    exceptions::GraphinaException,
};

fn my_shortest_path_algorithm(
    graph: &Graph<i32, f64>
) -> Result<Vec<f64>, GraphinaException> {
    // Validate preconditions
    require_connected(graph, "shortest_path")?;
    require_non_negative_weights(graph, "shortest_path")?;

    // Algorithm implementation
    // ...

    Ok(vec![])
}
```

---

### Pattern 2: Input Validation

```rust
use graphina::core::{
    types::Graph,
    validation::validate_for_algorithm,
};

fn process_graph(graph: &Graph<i32, f64>) {
    match validate_for_algorithm(graph, "process") {
        Ok(()) => {
            println!("Graph is valid, processing...");
            // Process the graph
        }
        Err(e) => {
            eprintln!("Invalid graph: {}", e);
            return;
        }
    }
}
```

---

### Pattern 3: Property-Based Graph Analysis

```rust
use graphina::core::{
    types::Graph,
    validation::{is_bipartite, is_connected, count_components},
};

fn analyze_graph(graph: &Graph<i32, f64>) {
    println!("Connected: {}", is_connected(graph));
    println!("Bipartite: {}", is_bipartite(graph));
    println!("Components: {}", count_components(graph));

    if is_bipartite(graph) {
        println!("Can use bipartite-specific algorithms");
    }
}
```

---

### Pattern 4: Conditional Algorithm Selection

```rust
use graphina::core::{
    types::Digraph,
    validation::{is_dag, has_negative_weights},
};

fn select_shortest_path_algorithm(graph: &Digraph<i32, f64>) {
    if is_dag(graph) {
        println!("Using DAG shortest path (O(V+E))");
        // Use topological sort-based algorithm
    } else if !has_negative_weights(graph) {
        println!("Using Dijkstra (O((V+E)logV))");
        // Use Dijkstra's algorithm
    } else {
        println!("Using Bellman-Ford (O(VE))");
        // Use Bellman-Ford algorithm
    }
}
```

---

## Error Messages

All validation functions provide descriptive error messages:

```rust
// Empty graph
// Error: "my_algorithm requires a non-empty graph"

// Disconnected graph
// Error: "shortest_path requires a connected graph"

// Negative weights
// Error: "dijkstra requires non-negative edge weights"

// Self-loops
// Error: "coloring does not support graphs with self-loops"

// Cyclic directed graph
// Error: "topological_sort requires a directed acyclic graph (DAG)"
```

---

## Performance

All validation functions are efficient:

| Function | Time Complexity | Space Complexity |
|----------|----------------|------------------|
| `is_empty` | O(1) | O(1) |
| `is_connected` | O(V + E) | O(V) |
| `has_negative_weights` | O(E) | O(1) |
| `has_self_loops` | O(E) | O(1) |
| `is_dag` | O(V + E) | O(V) |
| `is_bipartite` | O(V + E) | O(V) |
| `count_components` | O(V + E) | O(V) |

---

## Best Practices

### 1. Always Validate Algorithm Preconditions

```rust
// Good: Validate first
fn my_algorithm(graph: &Graph<i32, f64>) -> Result<(), GraphinaException> {
    require_connected(graph, "my_algorithm")?;
    // Algorithm implementation
    Ok(())
}

// Bad: No validation
fn my_algorithm(graph: &Graph<i32, f64>) {
    // Algorithm may fail silently or panic
}
```

---

### 2. Use Composite Validators When Possible

```rust
// Good: Use composite validator
validate_for_algorithm(graph, "pagerank")?;

// Less efficient: Check individually
require_non_empty(graph, "pagerank")?;
require_connected(graph, "pagerank")?;
require_non_negative_weights(graph, "pagerank")?;
```

---

### 3. Check Properties Before Expensive Operations

```rust
// Good: Quick check first
if !is_connected(graph) {
    return Err(GraphinaException::new("Graph must be connected"));
}
// Expensive algorithm here

// Bad: No check - waste computation on invalid input
```

---

### 4. Provide Context in Error Messages

```rust
// Good: Descriptive algorithm name
require_connected(graph, "PageRank centrality")?;

// Less helpful: Generic name
require_connected(graph, "algorithm")?;
```

---

## Testing

All validation functions are thoroughly tested:

- ✅ Empty graphs
- ✅ Single-node graphs
- ✅ Connected and disconnected graphs
- ✅ Graphs with negative weights
- ✅ Graphs with self-loops
- ✅ DAGs and cyclic graphs
- ✅ Bipartite and non-bipartite graphs
- ✅ Multi-component graphs
- ✅ Error message generation

**Test count:** 9 comprehensive tests  
**All tests passing:** ✅

---

## Integration Examples

### Example 1: Dijkstra's Algorithm with Validation

```rust
use graphina::core::{
    types::Graph,
    validation::{require_non_empty, require_non_negative_weights},
    exceptions::GraphinaException,
};

fn dijkstra_with_validation(
    graph: &Graph<i32, f64>,
    start: NodeId,
) -> Result<HashMap<NodeId, f64>, GraphinaException> {
    // Validate preconditions
    require_non_empty(graph, "Dijkstra")?;
    require_non_negative_weights(graph, "Dijkstra")?;

    // Run algorithm
    // ...

    Ok(HashMap::new())
}
```

---

### Example 2: Graph Classification

```rust
use graphina::core::{types::Graph, validation::*};

fn classify_graph(graph: &Graph<i32, f64>) {
    println!("=== Graph Classification ===");
    println!("Empty: {}", is_empty(graph));
    println!("Connected: {}", is_connected(graph));
    println!("Bipartite: {}", is_bipartite(graph));
    println!("Components: {}", count_components(graph));
    println!("Has self-loops: {}", has_self_loops(graph));

    if let Ok(_) = validate_for_algorithm(graph, "test") {
        println!("✓ Suitable for most algorithms");
    } else {
        println!("✗ Requires preprocessing");
    }
}
```

---

## Future Enhancements

Potential additions for future versions:

1. **Planarity checking** - `is_planar(graph)`
2. **Tree detection** - `is_tree(graph)`
3. **Eulerian path detection** - `has_eulerian_path(graph)`
4. **Hamiltonian cycle detection** - `has_hamiltonian_cycle(graph)` (NP-complete)
5. **k-connectivity** - `vertex_connectivity(graph, k)`
6. **Strongly connected components** - `is_strongly_connected(graph)`

---

## Related Documentation

- [Batch Operations](BATCH_OPERATIONS.md)
- [API Improvements](API_IMPROVEMENTS.md)
- [Core Types](../src/core/types.rs)
- [Exceptions](../src/core/exceptions.rs)
