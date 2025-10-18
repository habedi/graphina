# Graphina Integration and E2E Tests

This directory contains comprehensive integration and end-to-end tests for the Graphina library using real-world graph datasets.

## Test Files Overview

### 1. `test_integration_e2e.rs` - End-to-End Integration Tests
Complete pipeline tests covering the entire workflow from loading data to analysis and serialization.

**Test Coverage:**
-  Complete graph analysis pipeline (load → validate → analyze → serialize → reload)
-  Directed graph analysis with in/out degree metrics
-  Dataset validation across all available graphs
-  Graph metrics consistency (density, degree distribution)
-  Centrality algorithms (degree, closeness, betweenness)
-  Community detection (connected components, label propagation, Louvain)
-  Traversal algorithms (BFS, DFS) consistency
-  Serialization format compatibility (JSON, Binary, GraphML)
-  Path algorithms (Dijkstra, Bellman-Ford) consistency
-  Graph generators vs real data comparison
-  Subgraph operations
-  Stress testing on large graphs (run with `--ignored`)

**Example Run:**
```bash
# Run all E2E tests
cargo test --test test_integration_e2e --all-features -- --nocapture

# Run specific test
cargo test --test test_integration_e2e test_e2e_complete_graph_analysis_pipeline --all-features -- --nocapture

# Run stress tests
cargo test --test test_integration_e2e --all-features -- --ignored --nocapture
```

### 2. `test_cross_module_integration.rs` - Cross-Module Integration Tests
Tests verifying that different modules work together correctly.

**Test Coverage:**
-  Traversal + Metrics integration
-  Paths + Centrality integration
-  Community + Metrics integration
-  Serialization preserves algorithm results
-  Generators + Validation integration
-  Subgraphs + Community detection
-  MST + Path algorithms
-  Directed graph cross-module operations
-  Validation ensures algorithm correctness
-  Parallel centrality computation

**Example Run:**
```bash
# Run all cross-module tests
cargo test --test test_cross_module_integration --all-features -- --nocapture

# Run specific cross-module test
cargo test --test test_cross_module_integration test_cross_paths_and_centrality --all-features -- --nocapture
```

### 3. `test_data_quality_robustness.rs` - Data Quality and Robustness Tests
Tests ensuring the library handles edge cases, errors, and real-world data quirks gracefully.

**Test Coverage:**
-  Graph invariants verification (edge count, node count, symmetry)
-  High-degree node handling (hubs in real networks)
-  Edge case subgraphs (empty, single-node)
-  Algorithm stability (deterministic results)
-  Memory efficiency with large graphs
-  Numerical stability (no NaN/infinity)
-  Error recovery (graceful failure)
-  Duplicate edge handling
-  Algorithm scalability across graph sizes
-  Isolated node detection
-  Concurrent read operations
-  Complete data pipeline (load → transform → analyze → export)

**Example Run:**
```bash
# Run all data quality tests
cargo test --test test_data_quality_robustness --all-features -- --nocapture

# Run memory efficiency tests
cargo test --test test_data_quality_robustness --all-features -- --ignored --nocapture
```

## Test Datasets

The tests use real-world graph datasets from HuggingFace. These datasets are located in `tests/testdata/graphina-graphs/`.

### Available Datasets

| Dataset | Nodes | Edges | Type | Description |
|---------|-------|-------|------|-------------|
| Wikipedia Chameleon | 2,277 | 31,421 | Undirected | Wikipedia article network |
| Wikipedia Squirrel | 5,201 | 198,493 | Undirected | Wikipedia article network |
| Wikipedia Crocodile | 11,631 | 170,918 | Undirected | Wikipedia article network |
| Facebook Page-Page | 22,470 | 171,002 | Undirected | Social network |
| Stanford Web Graph | 281,903 | 2,312,497 | Directed | Web graph |
| DBLP Citation Network | 317,080 | 1,049,866 | Undirected | Citation network |

### Downloading Datasets

**Option 1: Using Hugging Face CLI (Recommended)**
```bash
# Install the Hugging Face CLI
pip install huggingface-hub

# Download datasets
huggingface-cli download habedi/graphina-graphs --repo-type dataset --local-dir tests/testdata/graphina-graphs
```

**Option 2: Using the Download Script**
```bash
cd tests/testdata
chmod +x download_datasets.sh
./download_datasets.sh
```

**Option 3: Manual Download**
Visit https://huggingface.co/datasets/habedi/graphina-graphs and download the files manually to `tests/testdata/graphina-graphs/`.

## Running Tests

### Run All Integration Tests
```bash
# With all features enabled
cargo test --tests --all-features -- --nocapture

# Specific test file
cargo test --test test_integration_e2e --all-features
```

### Run Tests Without Datasets
Tests automatically skip if datasets are not available:
```bash
cargo test --tests --all-features
```

Output will show:
```
️  Skipping test: datasets not found
   Run: huggingface-cli download habedi/graphina-graphs --repo-type dataset --local-dir tests/testdata/graphina-graphs
```

### Run Tests With Specific Features
```bash
# Only centrality tests
cargo test --test test_integration_e2e --features centrality -- --nocapture

# Only community detection tests
cargo test --test test_integration_e2e --features community -- --nocapture

# All features
cargo test --tests --all-features -- --nocapture
```

### Run Stress/Performance Tests
```bash
# Run ignored tests (large graphs, memory tests)
cargo test --tests --all-features -- --ignored --nocapture

# Run all tests including ignored
cargo test --tests --all-features -- --include-ignored --nocapture
```

### Run Tests in Parallel or Sequential
```bash
# Sequential (useful for debugging)
cargo test --tests --all-features -- --test-threads=1 --nocapture

# Parallel (default)
cargo test --tests --all-features -- --nocapture
```

## Test Output

The tests provide detailed, colorful output with emojis for easy scanning:

```
 Running Complete Graph Analysis Pipeline...

 Loaded graph: 2277 nodes, 31421 edges
 Graph structure validated
 Density: 0.012145
 Serialization successful
 Deserialization verified
 BFS visited 2277 nodes
 DFS visited 2277 nodes
 Dijkstra: 2277 reachable nodes from source

 Complete pipeline test passed!
```

## Continuous Integration

### GitHub Actions Example
```yaml
name: Integration Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Install Hugging Face CLI
        run: pip install huggingface-hub
      - name: Download test datasets
        run: |
          huggingface-cli download habedi/graphina-graphs \
            --repo-type dataset \
            --local-dir tests/testdata/graphina-graphs
      - name: Run integration tests
        run: cargo test --tests --all-features
      - name: Run stress tests
        run: cargo test --tests --all-features -- --ignored
```

## Troubleshooting

### Dataset Download Issues
```bash
# Check if datasets directory exists
ls -la tests/testdata/graphina-graphs/

# Re-download if corrupted
rm -rf tests/testdata/graphina-graphs
huggingface-cli download habedi/graphina-graphs --repo-type dataset --local-dir tests/testdata/graphina-graphs
```

### Memory Issues with Large Graphs
```bash
# Run tests with increased stack size
RUST_MIN_STACK=8388608 cargo test --tests --all-features

# Or skip large graph tests
cargo test --tests --all-features -- --skip test_stress
```

### Compilation Issues
```bash
# Clean and rebuild
cargo clean
cargo build --tests --all-features

# Check for errors
cargo check --tests --all-features
```

## Test Statistics

- **Total Test Files**: 3
- **Total Integration Tests**: ~40+
- **Lines of Test Code**: ~2,000+
- **Datasets Supported**: 6 real-world graphs
- **Feature Coverage**: Core, Centrality, Community, Paths, Generators, Serialization
- **Edge Cases Covered**: Empty graphs, single nodes, disconnected graphs, large graphs

## Contributing

When adding new integration tests:

1. **Use the dataset availability check**: Wrap tests with `skip_if_no_datasets!()` macro
2. **Provide descriptive output**: Use println! with emojis for visual feedback
3. **Test both success and failure cases**: Include edge cases and error handling
4. **Document the test**: Add comments explaining what the test validates
5. **Use appropriate assertions**: Provide meaningful error messages
6. **Consider performance**: Use `#[ignore]` for very slow tests

### Example Test Template
```rust
#[test]
fn test_my_new_feature() {
    skip_if_no_datasets!();
    
    println!("\n Testing My New Feature...\n");
    
    let graph = match load_undirected_graph("wikipedia_chameleon.txt") {
        Ok(g) => g,
        Err(_) => return,
    };
    
    // Test code here
    
    println!(" Feature validated");
    println!("\n Test passed!\n");
}
```

## License

These tests are part of the Graphina project and follow the same license (MIT OR Apache-2.0).

