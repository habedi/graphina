## Graphina Tests (Non-unit Tests)

This directory contains the integration and end-to-end tests for the Graphina graph library.

### Test Organization

The tests are organized into five consolidated test suites:

#### 1. **regression_tests.rs**
Bug fixes, regressions, and stability tests to ensure fixes don't reappear.

**Coverage:**
- Bug fixes validation
- Regression tests for community detection and graph generators
- Core graph operations with deleted/non-contiguous node indices
- Centrality algorithms with edge cases
- Approximation algorithms stability
- MST and path algorithm consistency

#### 2. **integration_tests.rs**
Cross-module functionality, architecture validation, and real-world graph analysis.

**Coverage:**
- Architecture and validation utilities
- Cross-module integration (traversal+metrics, paths+centrality, community+metrics)
- Real-world graph datasets and operations
- Graph generators and topology validation
- Data quality and robustness verification
- Concurrent access patterns
- Module independence tests

#### 3. **e2e_tests.rs**
Comprehensive end-to-end tests using real-world datasets.

**Coverage:**
- Complete graph analysis pipelines
- Directed graph analysis
- All datasets validation
- Metrics consistency
- Centrality algorithms
- Community detection
- Traversal consistency
- Serialization formats
- Path algorithms
- Generator comparison
- Subgraph operations
- Stress tests (large graphs)

#### 4. **property_based_tests.rs**
Property-based tests using `proptest` for algorithm correctness across diverse inputs.

**Coverage:**
- Graph generator properties
- Graph traversal properties
- Graph operation properties
- Algorithm correctness properties
- Graph invariants

#### 5. **visualizations_tests.rs**
Tests for graph visualization functionality.

**Coverage:**
- ASCII art generation
- D3.js JSON export
- HTML generation
- Layout algorithms (force-directed, circular, hierarchical, grid, random)
- PNG and SVG generation
- Configuration customization
- Empty graph handling
- Large graph performance

### Running Tests

Run all tests:
```bash
cargo test --all-features
```

Run specific test suite:
```bash
cargo test --test regression_tests --all-features
cargo test --test integration_tests --all-features
cargo test --test e2e_tests --all-features
cargo test --test property_based_tests --all-features
cargo test --test visualizations_tests --all-features
```

Run tests with real-world datasets:
```bash
# Download datasets first:
huggingface-cli download habedi/graphina-graphs --repo-type dataset --local-dir tests/testdata/graphina-graphs

# Then run tests that use datasets:
cargo test --test integration_tests --all-features
cargo test --test e2e_tests --all-features
```

### Test Data

Some tests require real-world graph datasets from the Graphina graphs repository. Download them using:

```bash
huggingface-cli download habedi/graphina-graphs --repo-type dataset --local-dir tests/testdata/graphina-graphs
```

Tests that require datasets will skip gracefully if the data is not available.

### Notes

- All tests support feature-gated compilation with `--all-features`
- Tests automatically skip if required datasets are not available
- Property-based tests use deterministic seeds for reproducibility
