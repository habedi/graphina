## Graphina Tests (Non-unit Tests)

This directory contains the integration and end-to-end tests for the Graphina graph library.

> [!NOTE]
> Unit tests are kept in their respective modules (in src/**/*.rs) and are not included in this directory.
> This directory is to store all other types of tests.

### Test Organization

The tests are organized into seven consolidated test suites plus a shared utilities module:

#### Shared Utilities

**`common/mod.rs`** - Shared test utilities to avoid code duplication:

- Dataset metadata and constants
- Graph loading helpers (`load_undirected_graph_f32`, `load_directed_graph_f32`, etc.)
- Skip macro for tests that needs datasets (`skip_if_no_datasets!`)
- Type conversion utilities

#### 1. **algorithm_tests.rs**

Comprehensive tests for algorithms that don't have dedicated unit tests in source files.

**Coverage:**

- Approximation algorithms (`average_clustering`, `min_maximal_matching`, `ramsey_r2`, `densest_subgraph`, and `approximate_diameter`)
- Centrality algorithms (`closeness_centrality`, `harmonic_centrality`, `local_reaching_centrality`, `global_reaching_centrality`, `voterank`, and `laplacian_centrality`)
- Community detection (`infomap`, `spectral_embeddings`, and `spectral_clustering`)
- Link prediction (`resource_allocation_index`, `preferential_attachment`, `common_neighbor_centrality`, `cn_soundarajan_hopcroft`, `within_inter_cluster`, and `ra_index_soundarajan_hopcroft`)
- Directed graph comparisons
- Edge cases (empty graphs, self-loops, and parallel edges)

#### 2. **regression_tests.rs**

Bug fixes, regressions, and stability tests to ensure fixes don't reappear.

**Coverage:**

- Bug fixes validation
- Regression tests for community detection and graph generators
- Core graph operations with deleted/non-contiguous node indices
- Centrality algorithms with edge cases
- Approximation algorithms stability
- MST and path algorithm consistency

#### 3. **integration_tests.rs**

Cross-module functionality, architecture validation, and real-world graph analysis.

**Coverage:**

- Architecture and validation utilities
- Cross-module integration (traversal+metrics, paths+centrality, community+metrics)
- Real-world graph datasets and operations
- Graph generators and topology validation
- Data quality and robustness verification
- Concurrent access patterns
- Module independence tests

#### 4. **e2e_tests.rs**

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

#### 5. **property_based_tests.rs**

Property-based tests using `proptest` for algorithm correctness across diverse inputs.

**Coverage:**

- Graph generator properties
- Graph traversal properties
- Graph operation properties
- Algorithm correctness properties
- Graph invariants

### Running Tests

Run all tests:

```bash
cargo test --all-features
```

Run specific test suite:

```bash
cargo test --test algorithm_tests --all-features
cargo test --test regression_tests --all-features
cargo test --test integration_tests --all-features
cargo test --test e2e_tests --all-features
cargo test --test property_based_tests --all-features
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

Some tests require real-world graph datasets from the Graphina graphs repository.
Download them using:

```bash
huggingface-cli download habedi/graphina-graphs --repo-type dataset --local-dir tests/testdata/graphina-graphs
```

Tests that require datasets will skip gracefully if the data is not available.

### Test Statistics

| Test Suite | Test Count | Purpose |
|------------|------------|---------|
| Unit tests (src/) | 154 | Module-level unit tests |
| algorithm_tests.rs | 49 | Algorithm coverage |
| e2e_tests.rs | 12 | End-to-end pipelines |
| integration_tests.rs | 29 | Cross-module integration |
| property_based_tests.rs | 29 | Property-based verification |
| regression_tests.rs | 25 | Bug regression prevention |
| Doc tests | 44 | Documentation examples |
| **Total** | **342** | |

### Notes

- All tests support feature-gated compilation with `--all-features`
- Tests automatically skip if required datasets are not available
- Property-based tests use deterministic seeds for reproducibility
- The `common/` module provides shared utilities to reduce code duplication
