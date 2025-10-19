# Graphina Architecture

## Design Principles

### 1. Module Decoupling

Graphina is organized into high-level modules that must remain decoupled:

- **Core Module** (`src/core/`): Foundation providing graph data structures, I/O, traversal, and basic algorithms
- **Centrality Module** (`src/centrality/`): Node importance metrics (degree, betweenness, PageRank, etc.)
- **Community Module** (`src/community/`): Community detection algorithms (Louvain, label propagation, etc.)
- **Links Module** (`src/links/`): Link prediction algorithms
- **Approximation Module** (`src/approximation/`): Approximation algorithms for NP-hard problems

**Architectural Constraint**: High-level modules (centrality, community, links, approximation) must ONLY depend on the
core module. They must NOT depend on each other.

### 2. Error Handling Strategy

Graphina uses a custom error hierarchy defined in `core::exceptions`:

- `GraphinaException`: General-purpose error
- `NodeNotFound`: Node doesn't exist in graph
- `GraphinaNoPath`: No path exists between nodes
- `PowerIterationFailedConvergence`: Iterative algorithms failed to converge
- Additional specialized errors for specific scenarios

All public algorithms should return `Result<T, GraphinaException>` or a more specific error type when appropriate.

### 3. Type Safety

- `NodeId` and `EdgeId` are opaque wrappers around petgraph indices
- This prevents accidental mixing of node/edge indices from different graphs
- Generic types `A` (node attributes) and `W` (edge weights) provide flexibility

### 4. API Consistency

Graphina provides two API styles:

1. **Standard API**: Returns `Option<T>` or simple types
   ```rust
   graph.degree(node) -> Option<usize>
   graph.update_node(node, attr) -> bool
   ```

2. **Try API**: Returns `Result<T, Error>` for better error propagation
   ```rust
   graph.try_update_node(node, attr) -> Result<(), NodeNotFound>
   graph.try_remove_node(node) -> Result<A, NodeNotFound>
   ```

### 5. Performance Considerations

- Uses `StableGraph` from petgraph to prevent node index reuse
- Parallel algorithms available via the `parallel` module
- Bulk operations (`add_nodes_bulk`, `add_edges_bulk`) for efficiency
- Pre-allocated data structures where possible

## Module Organization

```
src/
├── lib.rs                 # Feature-gated module exports
├── core/                  # Core graph infrastructure
│   ├── types.rs          # Graph types, NodeId, EdgeId
│   ├── exceptions.rs     # Error types
│   ├── io.rs            # Graph I/O (edge lists, adjacency lists)
│   ├── generators.rs    # Graph generators (random, structured)
│   ├── paths.rs         # Shortest path algorithms
│   ├── mst.rs           # Minimum spanning tree
│   ├── traversal.rs     # BFS, DFS, etc.
│   ├── metrics.rs       # Graph metrics (diameter, clustering, etc.)
│   ├── validation.rs    # Graph validation utilities
│   ├── serialization.rs # JSON/binary serialization
│   ├── subgraphs.rs     # Subgraph extraction
│   ├── parallel.rs      # Parallel algorithms
│   └── visualization.rs # Graph visualization
├── centrality/           # Centrality algorithms
├── community/            # Community detection
├── links/                # Link prediction
└── approximation/        # Approximation algorithms
```

## Testing Strategy

### Unit Tests

- Located within each module file using `#[cfg(test)]` modules
- Test individual functions and edge cases
- Should cover error conditions

### Integration Tests

- Located in `tests/` directory
- Test cross-module functionality
- Test end-to-end workflows
- Test real-world datasets

### Property-Based Tests

- Use `proptest` for randomized testing
- Verify invariants hold across random inputs
- Located in `tests/test_property_based.rs`

## Future Improvements

1. **Unified Error Type**: Consider using an enum-based error type with `thiserror` for better ergonomics
2. **Trait-based Graph API**: Consider defining traits for graph operations to support multiple backends
3. **Builder Patterns**: Expand use of builder patterns for complex configurations
4. **Memory Pooling**: For performance-critical operations with many temporary allocations
5. **GPU Acceleration**: For large-scale graph algorithms
