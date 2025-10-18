# API Improvements and Architectural Fixes

**Date:** October 17, 2025  
**Version:** 0.4.0  
**Status:** Implemented and Tested

## Executive Summary

This document details comprehensive API improvements and architectural fixes implemented in the Graphina library to
address inconsistencies, improve usability, and enhance the developer experience.

## Issues Identified and Fixed

### 1. Inconsistent Naming: `edge_attr` vs `edge_weight`

**Issue:** The API used `node_attr()` for node attributes but `edge_attr()` for edge weights, creating confusion since
edges have "weights" not "attributes" in graph theory terminology.

**Fix:**

- Added new methods: `edge_weight()` and `edge_weight_mut()`
- Deprecated old methods: `edge_attr()` and `edge_attr_mut()`
- Maintained backward compatibility with deprecation warnings

**Before:**

```rust
let weight = graph.edge_attr(edge_id);
graph.edge_attr_mut(edge_id).map(|w| *w = 5.0);
```

**After:**

```rust
let weight = graph.edge_weight(edge_id);
graph.edge_weight_mut(edge_id).map(|w| *w = 5.0);
```

**Impact:** Improves API consistency and aligns with standard graph theory terminology.

---

### 2. Missing Builder Pattern

**Issue:** No fluent API for graph construction, requiring verbose imperative code.

**Fix:** Implemented `GraphBuilder` with method chaining support.

**Before:**

```rust
let mut graph = Graph::<i32, f64>::new();
let n1 = graph.add_node(1);
let n2 = graph.add_node(2);
let n3 = graph.add_node(3);
graph.add_edge(n1, n2, 1.0);
graph.add_edge(n2, n3, 2.0);
```

**After:**

```rust
let graph = Graph::<i32, f64>::builder()
    .add_node(1)
    .add_node(2)
    .add_node(3)
    .add_edge(0, 1, 1.0)
    .add_edge(1, 2, 2.0)
    .build();
```

**Features:**

- Fluent API with method chaining
- Automatic capacity pre-allocation
- Works with both directed and undirected graphs
- Edges reference nodes by their insertion order (0-indexed)

---

### 3. Missing Graph Property Queries

**Issue:** No convenient methods to check common graph properties.

**Fix:** Added property query methods:

#### `is_empty() -> bool`

Returns true if the graph contains no nodes.

```rust
let graph = Graph::<i32, f64>::new();
assert!(graph.is_empty());
```

#### `density() -> f64`

Returns the density of the graph (ratio of actual edges to possible edges).

```rust
let graph = /* ... */;
println!("Graph density: {}", graph.density());
```

**Formula:**

- Directed: `edges / (nodes * (nodes - 1))`
- Undirected: `(2 * edges) / (nodes * (nodes - 1))`

#### `contains_node(node: NodeId) -> bool`

Returns true if the node exists in the graph.

```rust
if graph.contains_node(node_id) {
    println!("Node exists");
}
```

#### `contains_edge(source: NodeId, target: NodeId) -> bool`

Returns true if an edge exists between the nodes.

```rust
if graph.contains_edge(n1, n2) {
    println!("Edge exists");
}
```

---

### 4. Missing Degree Query Methods

**Issue:** No convenient way to query node degrees without manual iteration.

**Fix:** Added comprehensive degree query methods:

#### `degree(node: NodeId) -> Option<usize>`

Returns the total degree of a node.

- For directed graphs: in-degree + out-degree
- For undirected graphs: number of incident edges

#### `in_degree(node: NodeId) -> Option<usize>`

Returns the number of incoming edges (or degree for undirected graphs).

#### `out_degree(node: NodeId) -> Option<usize>`

Returns the number of outgoing edges (or degree for undirected graphs).

**Example:**

```rust
let degree = graph.degree(node_id).unwrap_or(0);
println!("Node degree: {}", degree);
```

---

### 5. Missing Collection Methods

**Issue:** No way to clear a graph or iterate over IDs without attributes.

**Fix:** Added collection manipulation methods:

#### `clear()`

Removes all nodes and edges from the graph.

```rust
graph.clear();
assert!(graph.is_empty());
```

#### `node_ids() -> impl Iterator<Item = NodeId>`

Returns an iterator over all node IDs (without attributes).

```rust
for node_id in graph.node_ids() {
    println!("Node: {:?}", node_id);
}
```

#### `edge_ids() -> impl Iterator<Item = EdgeId>`

Returns an iterator over all edge IDs (without weights).

```rust
for edge_id in graph.edge_ids() {
    println!("Edge: {:?}", edge_id);
}
```

---

### 6. Missing Filter Methods

**Issue:** No convenient way to filter nodes or edges based on predicates.

**Fix:** Added retention methods:

#### `retain_nodes<F>(&mut self, predicate: F)`

Retains only nodes that satisfy the predicate. All edges connected to removed nodes are also removed.

```rust
// Keep only nodes with even values
graph.retain_nodes(|_id, attr| *attr % 2 == 0);
```

#### `retain_edges<F>(&mut self, predicate: F)`

Retains only edges that satisfy the predicate.

```rust
// Keep only edges with weight > 1.5
graph.retain_edges(|_src, _dst, weight| *weight > 1.5);
```

---

## Complete API Reference

### Construction Methods

| Method                        | Description                                   |
|-------------------------------|-----------------------------------------------|
| `new()`                       | Creates an empty graph                        |
| `with_capacity(nodes, edges)` | Creates a graph with pre-allocated capacity   |
| `builder()`                   | Returns a builder for fluent API construction |

### Property Queries

| Method                    | Return Type | Description                       |
|---------------------------|-------------|-----------------------------------|
| `is_empty()`              | `bool`      | True if graph has no nodes        |
| `is_directed()`           | `bool`      | True if graph is directed         |
| `density()`               | `f64`       | Density of the graph (0.0 to 1.0) |
| `node_count()`            | `usize`     | Number of nodes                   |
| `edge_count()`            | `usize`     | Number of edges                   |
| `contains_node(node)`     | `bool`      | True if node exists               |
| `contains_edge(src, dst)` | `bool`      | True if edge exists               |

### Degree Queries

| Method             | Return Type     | Description          |
|--------------------|-----------------|----------------------|
| `degree(node)`     | `Option<usize>` | Total degree of node |
| `in_degree(node)`  | `Option<usize>` | In-degree of node    |
| `out_degree(node)` | `Option<usize>` | Out-degree of node   |

### Node Operations

| Method                        | Return Type                | Description                         |
|-------------------------------|----------------------------|-------------------------------------|
| `add_node(attr)`              | `NodeId`                   | Adds a node with attribute          |
| `remove_node(node)`           | `Option<A>`                | Removes node and returns attribute  |
| `try_remove_node(node)`       | `Result<A, NodeNotFound>`  | Removes node or returns error       |
| `update_node(node, attr)`     | `bool`                     | Updates node attribute              |
| `try_update_node(node, attr)` | `Result<(), NodeNotFound>` | Updates or returns error            |
| `node_attr(node)`             | `Option<&A>`               | Gets reference to node attribute    |
| `node_attr_mut(node)`         | `Option<&mut A>`           | Gets mutable reference to attribute |

### Edge Operations

| Method                       | Return Type                    | Description                      |
|------------------------------|--------------------------------|----------------------------------|
| `add_edge(src, dst, weight)` | `EdgeId`                       | Adds an edge with weight         |
| `remove_edge(edge)`          | `Option<W>`                    | Removes edge and returns weight  |
| `try_remove_edge(edge)`      | `Result<W, GraphinaException>` | Removes or returns error         |
| `edge_weight(edge)`          | `Option<&W>`                   | Gets reference to edge weight    |
| `edge_weight_mut(edge)`      | `Option<&mut W>`               | Gets mutable reference to weight |
| `find_edge(src, dst)`        | `Option<EdgeId>`               | Finds edge between nodes         |

### Iteration Methods

| Method                 | Return Type                      | Description                         |
|------------------------|----------------------------------|-------------------------------------|
| `nodes()`              | `Iterator<(NodeId, &A)>`         | Iterates over nodes with attributes |
| `edges()`              | `Iterator<(NodeId, NodeId, &W)>` | Iterates over edges with weights    |
| `node_ids()`           | `Iterator<NodeId>`               | Iterates over node IDs only         |
| `edge_ids()`           | `Iterator<EdgeId>`               | Iterates over edge IDs only         |
| `neighbors(node)`      | `Iterator<NodeId>`               | Iterates over neighbors of a node   |
| `outgoing_edges(node)` | `Iterator<(NodeId, &W)>`         | Iterates over outgoing edges        |

### Collection Methods

| Method                    | Description                           |
|---------------------------|---------------------------------------|
| `clear()`                 | Removes all nodes and edges           |
| `retain_nodes(predicate)` | Keeps only nodes satisfying predicate |
| `retain_edges(predicate)` | Keeps only edges satisfying predicate |

---

## Backward Compatibility

All changes maintain backward compatibility:

1. **Deprecated methods** still work but emit warnings:
    - `edge_attr()` → Use `edge_weight()`
    - `edge_attr_mut()` → Use `edge_weight_mut()`

2. **Existing code** continues to work without modifications

3. **Migration path** is clearly documented

---

## Testing

Comprehensive test suite added in `tests/test_api_improvements.rs`:

- 25 tests covering all new functionality
- Edge cases for empty graphs, single nodes, etc.
- Both directed and undirected graph variants
- Builder pattern usage patterns
- Property query accuracy

All tests pass successfully.

---

## Performance Considerations

1. **Property queries** (`is_empty`, `density`) are O(1) or O(E) operations
2. **Degree queries** are O(E) in worst case but efficient for sparse graphs
3. **Retention methods** create temporary collections to avoid iterator invalidation
4. **Builder pattern** pre-allocates capacity for optimal performance

---

## Migration Guide

### Updating edge access code:

```rust
// Old (deprecated)
let weight = graph.edge_attr(edge);

// New (preferred)
let weight = graph.edge_weight(edge);
```

### Using the builder pattern:

```rust
// Replace imperative construction
let graph = Graph::<i32, f64>::builder()
    .add_node(1)
    .add_node(2)
    .add_edge(0, 1, 1.0)
    .build();
```

### Using new property queries:

```rust
// Check if graph is empty
if graph.is_empty() {
    return;
}

// Check graph density
if graph.density() > 0.5 {
    println!("Dense graph");
}

// Query node degrees
if let Some(deg) = graph.degree(node) {
    println!("Degree: {}", deg);
}
```

---

## Future Enhancements

Potential future improvements:

1. **Parallel iteration** methods for large graphs
2. **Graph views** (subgraph, filtered views)
3. **Incremental property updates** (cached density, etc.)
4. **More builder options** (from adjacency list, from edge list)
5. **Batch operations** (add multiple nodes/edges at once)

---

## Summary

**API Improvements:** 15+ new methods  
**Deprecated methods:** 2 (with clear migration path)  
**Tests added:** 25 comprehensive tests  
**Breaking changes:** 0  
**Backward compatibility:** 100%

These improvements significantly enhance the usability and consistency of the Graphina API while maintaining full
backward compatibility with existing code.
