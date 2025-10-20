# Graphina Benchmarks

This directory contains comprehensive performance benchmarks for the Graphina graph library using the Criterion framework.

## Benchmark Files

### algorithm_benchmarks.rs
Benchmarks for high-level algorithms:
- **Graph Creation**: Erdős-Rényi, Barabási-Albert, Watts-Strogatz generators
- **Centrality Algorithms**: Degree, PageRank, Betweenness centrality
- **Community Detection**: Louvain, Label Propagation
- **Graph Operations**: Node/edge addition, neighbor queries, degree calculations
- **Approximation Algorithms**: Local connectivity

### graph_benchmarks.rs
Benchmarks for core graph operations:
- **Graph Generators**: Erdős-Rényi, Complete, Barabási-Albert, Watts-Strogatz
- **Traversal Algorithms**: BFS, DFS, Bidirectional Search
- **Shortest Paths**: Dijkstra's algorithm
- **Graph Operations**: Node addition, edge addition, node removal
- **Density Tests**: Sparse vs dense graph traversal
- **Comparison Tests**: Side-by-side algorithm comparisons

### project_benchmarks.rs
Integration benchmarks for end-to-end workflows and cross-module performance testing.

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench --features all

# Run specific benchmark file
cargo bench --bench algorithm_benchmarks --features all
cargo bench --bench graph_benchmarks --features all

# Run specific benchmark group
cargo bench --bench algorithm_benchmarks centrality --features all

# Generate HTML reports (in target/criterion/)
cargo bench --features all

# Compare with baseline
cargo bench --features all --bench algorithm_benchmarks -- --save-baseline my_baseline
cargo bench --features all --bench algorithm_benchmarks -- --baseline my_baseline
```

## Benchmark Methodology

- **Sample Size**: Varies by benchmark complexity (10-100 samples)
- **Warm-up Time**: 3 seconds
- **Measurement Time**: 5 seconds per benchmark
- **Graph Sizes**: Scaled appropriately for each algorithm (50-5000 nodes)
- **Reproducibility**: Fixed random seeds for generator benchmarks

## Performance Targets

| Operation | Target (per element) | Graph Size |
|-----------|---------------------|------------|
| Node Addition | < 100 ns | 1000 nodes |
| Edge Addition | < 200 ns | 1000 edges |
| BFS Traversal | < 10 µs | 1000 nodes |
| Dijkstra | < 50 µs | 500 nodes |
| PageRank (100 iter) | < 500 ms | 1000 nodes |
| Louvain | < 1s | 1000 nodes |

## Interpreting Results

Criterion generates detailed reports in `target/criterion/`:
- **report/index.html**: Main benchmark report with graphs
- **{benchmark}/report/**: Individual benchmark details
- **{benchmark}/base/**: Baseline comparison data

Look for:
- **Throughput**: Higher is better
- **Time**: Lower is better
- **Variance**: Lower is more consistent
- **Outliers**: Should be minimal (< 5%)
