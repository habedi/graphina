# Bug Fixes - 2025

This document describes critical bugs found and fixed in the Graphina project during a comprehensive code analysis in 2025.

## Critical Bugs Fixed

### 1. Force-Directed Layout Panic (Visualization Module)

**Location**: `src/visualization/layout.rs`

**Severity**: High - Could cause runtime panic

**Description**: 
The force-directed layout algorithm had potential panic points when calculating attractive forces for edges. The code used `unwrap()` calls on HashMap lookups that could fail when:
- Edges connect nodes that haven't been initialized in the displacements HashMap
- Graph has isolated nodes
- Edge endpoints don't have position entries

**Root Cause**:
```rust
// BEFORE (Bug):
let (dx_src, dy_src) = displacements.get_mut(&src).unwrap(); // Could panic!
let pos = positions.get_mut(node).unwrap(); // Could panic!
```

**Fix**:
1. Initialize displacements for all nodes at the start of each iteration
2. Use safe pattern matching (`if let Some(...)`) instead of `unwrap()`
3. Skip edges where nodes don't have positions

```rust
// AFTER (Fixed):
// Initialize all nodes
for &node in &nodes {
    displacements.insert(node, (0.0, 0.0));
}

// Safe access patterns
if let Some((dx_src, dy_src)) = displacements.get_mut(&src) {
    *dx_src += (delta_x / distance) * force;
    *dy_src += (delta_y / distance) * force;
}

if let Some(pos) = positions.get_mut(node) {
    // ... update position
}
```

**Test Coverage**: Added tests in `tests/test_bugfixes_2025.rs`:
- `test_force_directed_layout_no_panic_sparse_graph`
- `test_force_directed_layout_empty_graph`
- `test_force_directed_layout_single_node`

---

### 2. Bidirectional Search Unsafe Unwrap (Traversal Module)

**Location**: `src/traversal/algorithms.rs`

**Severity**: Medium - Defensive programming improvement

**Description**:
The bidirectional search algorithm used `unwrap()` calls when popping from queues. While the loop logic ensured these were safe (iterating exactly `queue.len()` times), this pattern is fragile and could lead to panics if the code is refactored incorrectly.

**Root Cause**:
```rust
// BEFORE (Bug):
for _ in 0..forward_level {
    let current = forward_queue.pop_front().unwrap(); // Unsafe!
    // ...
}
```

**Fix**:
Replace with safe pattern matching that explicitly handles the None case:

```rust
// AFTER (Fixed):
for _ in 0..forward_level {
    if let Some(current) = forward_queue.pop_front() {
        // ... process current
    }
}
```

**Test Coverage**: Added tests in `tests/test_bugfixes_2025.rs`:
- `test_bidirectional_search_no_panic`
- `test_bidirectional_search_disconnected`
- `test_bidirectional_search_same_node`

---

### 3. Division by Zero in Betweenness Centrality (Centrality Module)

**Location**: `src/centrality/betweenness.rs`

**Severity**: Critical - Causes NaN/Inf values

**Description**:
The normalization step in both `betweenness_centrality` and `edge_betweenness_centrality` had a division by zero bug when the graph has exactly 2 nodes. The normalization factor is calculated as:
- Directed: `1.0 / ((n - 1) * (n - 2))`
- Undirected: `2.0 / ((n - 1) * (n - 2))`

When `n = 2`:
- `(n - 1) * (n - 2) = (2 - 1) * (2 - 2) = 1 * 0 = 0`
- Division by zero results in `Inf`

**Root Cause**:
```rust
// BEFORE (Bug):
if normalized && n > 1 {
    let norm = if graph.is_directed() {
        1.0 / ((n - 1) * (n - 2)) as f64  // Division by zero when n=2!
    } else {
        2.0 / ((n - 1) * (n - 2)) as f64  // Division by zero when n=2!
    };
    for val in centrality.values_mut() {
        *val *= norm;
    }
}
```

**Fix**:
Change the condition to `n > 2` to skip normalization for graphs with 2 or fewer nodes:

```rust
// AFTER (Fixed):
if normalized && n > 2 {
    let norm = if graph.is_directed() {
        1.0 / ((n - 1) * (n - 2)) as f64
    } else {
        2.0 / ((n - 1) * (n - 2)) as f64
    };
    for val in centrality.values_mut() {
        *val *= norm;
    }
}
```

**Mathematical Justification**:
For a graph with 2 nodes, there are no meaningful "betweenness" paths since a node cannot lie "between" two other distinct nodes. The normalization factor becomes undefined (0/0). The fix correctly handles this edge case by skipping normalization.

**Test Coverage**: Added tests in `tests/test_bugfixes_2025.rs`:
- `test_betweenness_centrality_two_nodes_no_division_by_zero` (Critical test)
- `test_betweenness_centrality_three_nodes_normalized`
- `test_betweenness_centrality_directed_two_nodes`
- `test_betweenness_centrality_no_normalization`
- `test_betweenness_centrality_single_node`
- `test_betweenness_centrality_empty_graph`

---

## Minor Issues Fixed

### 4. Unused Imports and Variables in Tests

**Files**: 
- `tests/test_refactoring_2025.rs`
- `tests/test_approximation_bug_fixes.rs`

**Severity**: Low - Compilation warnings

**Description**: Several test files had unused imports and variables that caused compiler warnings.

**Fixes**:
1. Removed unused imports: `NodePosition`, `LayoutAlgorithm`
2. Prefixed unused variables with underscore: `_n1`, `_n3`, `_tw1`, `_tw2`

---

## Testing

All bug fixes have been validated with:

1. **Unit Tests**: Comprehensive test suite in `tests/test_bugfixes_2025.rs`
2. **Regression Tests**: Ensure fixes don't break existing functionality
3. **Edge Cases**: Test boundary conditions (empty graphs, single nodes, two nodes)

### Running Tests

```bash
# Run all Rust tests
make test

# Run Python tests
make test-py

# Run specific bug fix tests
cargo test --test test_bugfixes_2025
```

---

## Impact Assessment

### Breaking Changes
None. All fixes maintain backward compatibility.

### Performance Impact
Negligible. The changes add minimal overhead:
- Visualization: One HashMap initialization per iteration
- Traversal: Pattern matching instead of unwrap (zero cost)
- Centrality: One additional comparison (n > 2 vs n > 1)

### Reliability Improvement
- Eliminated 3 potential panic points
- Fixed 1 correctness bug (division by zero)
- Improved defensive programming practices

---

## Recommendations

1. **Code Review**: Add specific checks in CI/CD for:
   - Usage of `.unwrap()` in production code
   - Division operations without zero checks
   - HashMap access patterns

2. **Static Analysis**: Consider using tools like:
   - `clippy` with `unwrap_used` lint
   - `cargo-careful` for unsafe operation detection

3. **Future Development**:
   - Prefer `Result` and `Option` types over panics
   - Add property-based tests for edge cases
   - Document preconditions clearly in function docs

---

## Version Information

- **Fixed in**: Version 0.2.0-alpha3 (pending)
- **Analysis Date**: January 2025
- **Affected Versions**: All versions prior to fix

---

## References

- Betweenness Centrality: Freeman, L. C. (1977). "A set of measures of centrality based on betweenness"
- Force-Directed Layout: Fruchterman, T. M. J., & Reingold, E. M. (1991). "Graph drawing by force-directed placement"

