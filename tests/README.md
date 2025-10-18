# Test Organization

## Overview

The Graphina test suite follows Rust best practices with proper separation between unit tests and integration tests.

## Test Structure

### Unit Tests (in `src/` modules)

Unit tests are located **inside their respective source modules** using `#[cfg(test)]` blocks. This allows:
- Testing private functions and implementation details
- Fast compilation (only compiles when testing)
- Clear association between code and tests

**Modules with unit tests (16+ files):**

**Core Module:**
- `src/core/types.rs` - Graph data structures, operations, builders
- `src/core/generators.rs` - Graph generators (Erdős-Rényi, Watts-Strogatz, Barabási-Albert, etc.)
- `src/core/paths.rs` - Shortest path algorithms (Dijkstra, Bellman-Ford, Floyd-Warshall, etc.)
- `src/core/traversal.rs` - Traversal algorithms (BFS, DFS, IDDFS, bidirectional search)
- `src/core/io.rs` - I/O operations (edge lists, adjacency lists)
- `src/core/mst.rs` - Minimum spanning tree algorithms
- `src/core/metrics.rs` - Graph metrics
- `src/core/validation.rs` - Graph validation
- `src/core/serialization.rs` - Serialization/deserialization
- `src/core/subgraphs.rs` - Subgraph operations
- `src/core/parallel.rs` - Parallel operations
- `src/core/exceptions.rs` - Error types

**Extension Modules:**
- `src/centrality/betweenness.rs` - Betweenness centrality (includes bug fix tests)
- `src/centrality/eigenvector.rs` - Eigenvector centrality
- `src/community/louvain.rs` - Louvain algorithm (includes robustness tests)
- `src/links/similarity.rs` - Link similarity measures

### Integration Tests (in `tests/` directory)

Integration tests are in the `tests/` directory and test the library as external users would. These tests:
- Test the public API
- Test cross-module interactions
- Use real-world data and scenarios
- Are compiled as separate crates

**Integration test files (3 files):**

1. **`integration_real_graphs.rs`** - Real-world dataset validation
   - Tests with Karate Club, Dolphins, Les Misérables, Football networks
   - Core operations, traversals, paths, centrality, community detection
   - Performance benchmarking
   - Automatic skipping if datasets not available
   - ~15 comprehensive end-to-end tests

2. **`test_property_based.rs`** - Property-based testing
   - Uses proptest for randomized testing
   - Tests graph invariants and properties
   - Catches edge cases through randomized inputs

3. **`test_visualization.rs`** - Visualization integration
   - Tests graph rendering and visualization
   - Validates output formats

## Running Tests

### All Tests
```bash
# Run both unit and integration tests
cargo test --all-features

# With output
cargo test --all-features -- --nocapture
```

### Unit Tests Only
```bash
# Run unit tests in a specific module
cargo test --lib core::types
cargo test --lib core::generators
cargo test --lib centrality::betweenness
cargo test --lib community::louvain

# Run all unit tests (faster than integration tests)
cargo test --lib --all-features
```

### Integration Tests Only
```bash
# Run all integration tests
cargo test --test '*' --all-features

# Run specific integration test
cargo test --test integration_real_graphs --all-features
cargo test --test test_property_based --all-features
```

### With Features
```bash
# Test centrality module (requires feature)
cargo test --lib centrality --features centrality

# Test community module (requires feature)  
cargo test --lib community --features community
```

## Real-World Dataset Integration

The `integration_real_graphs.rs` test uses real network datasets from Hugging Face.

### Setup
```bash
# Install Hugging Face CLI
pip install huggingface-hub[cli]

# Download datasets to tests/testdata/graphina-graphs/
cd tests/testdata
huggingface-cli download habedi/graphina-graphs --repo-type dataset --local-dir graphina-graphs
```

### Automatic Skipping
Tests automatically skip with helpful messages if datasets aren't available:
```
Skipping test: datasets not found in tests/testdata/graphina-graphs/
To download: huggingface-cli download habedi/graphina-graphs --repo-type dataset --local-dir tests/testdata/graphina-graphs
```

### Available Datasets
- **Karate Club** (34 nodes) - Social network
- **Dolphins** (62 nodes) - Dolphin social network  
- **Les Misérables** (77 nodes) - Character co-occurrences
- **Football** (115 nodes) - College football games

## Test Coverage Summary

- **Unit Tests:** 100+ tests across 16+ source modules
- **Integration Tests:** 15+ real-world tests + property-based tests
- **Total:** 115+ well-organized tests

## Benefits of This Organization

✅ **Rust Best Practices** - Follows official Rust guidelines
✅ **Fast Compilation** - Unit tests only compile when needed
✅ **Private Testing** - Unit tests can test private functions
✅ **Clear Separation** - Unit vs integration tests clearly separated
✅ **Maintainability** - Tests co-located with code they test
✅ **Real-World Validation** - Integration tests use actual network data

## Bug Fixes with Tests

All critical bug fixes have unit tests in their respective modules:

- ✅ Betweenness O(V·E²) fix → `src/centrality/betweenness.rs`
- ✅ Betweenness variable shadowing fix → `src/centrality/betweenness.rs`
- ✅ Louvain robustness improvements → `src/community/louvain.rs`
- ✅ Watts-Strogatz duplicate edges → `src/core/generators.rs`
- ✅ Barabási-Albert infinite loops → `src/core/generators.rs`
- ✅ Non-contiguous indices handling → `src/core/paths.rs`

## Example: Adding New Tests

### Adding Unit Tests
```rust
// In src/core/my_module.rs

pub fn my_function() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        assert!(my_function());
    }
}
```

### Adding Integration Tests
```rust
// In tests/integration_my_feature.rs

use graphina::core::types::Graph;

#[test]
fn test_end_to_end_workflow() {
    let mut graph = Graph::new();
    // Test public API as external user would
}
```

## Documentation

- **This file** - Test organization guide
- `tests/README.md` - Dataset information
- `docs/BUG_FIXES_ALPHA.md` - Bug fix documentation
- `docs/FINAL_SUMMARY.md` - Complete bug fix summary

## Next Steps

1. ✅ Unit tests in source modules
2. ✅ Integration tests in tests/ directory  
3. ✅ Real-world dataset tests with graceful skipping
4. 📋 Download datasets for comprehensive testing
5. 📋 Run full test suite regularly
6. 📋 Maintain this organization for new features

**The test suite follows Rust best practices with proper separation of unit and integration tests!**

