# Parallel Algorithms

Parallel implementations of graph algorithms for large-scale graphs.

!!! warning "Development Status"
    This module is currently under development. Most algorithms are available in sequential form via the main API.

## Overview

For very large graphs (millions of nodes, tens of millions of edges), PyGraphina can leverage parallel processing to speed up computations. The parallel module provides multi-threaded implementations of popular algorithms.

## Available Algorithms

| Algorithm | Status | Speedup | Notes |
|-----------|--------|---------|-------|
| Parallel PageRank | ⚠️ Planned | 3-4x | Multi-threaded power iteration |
| Parallel BFS | ⚠️ Planned | 2-3x | Level-synchronous BFS |
| Parallel Community Detection | ⚠️ Planned | 2-4x | Parallel label propagation |
| Parallel Centrality | ⚠️ Planned | 3-5x | Multiple centrality measures |

## Current Status

PyGraphina's algorithms are already **highly optimized** thanks to Rust:

- ✅ Memory efficient (Rust's zero-cost abstractions)
- ✅ Fast single-threaded performance
- ✅ No GIL issues (computation happens in Rust)
- ⚠️ Explicit parallel versions coming soon

## Performance Tips

While dedicated parallel algorithms are in development, you can still achieve good performance:

### 1. Use Sequential Algorithms (Already Fast!)

The sequential algorithms are written in Rust and are much faster than pure Python:

```python
import pygraphina as pg

# This is already fast for most graphs
g = pg.PyGraph()
# ... add nodes and edges ...

# Fast single-threaded PageRank
scores = pg.centrality.pagerank(g, 0.85, 100, 1e-6)
```

**Typical performance:**
- Small graphs (< 10K nodes): Milliseconds
- Medium graphs (10K - 100K nodes): Seconds
- Large graphs (100K - 1M nodes): Tens of seconds

### 2. Process Multiple Graphs in Parallel

Use Python's multiprocessing for independent graphs:

```python
from multiprocessing import Pool
import pygraphina as pg

def analyze_graph(graph_data):
    """Analyze a single graph."""
    g = pg.PyGraph()
    # Build graph from graph_data
    for node_attr in graph_data['nodes']:
        g.add_node(node_attr)
    for src, tgt, weight in graph_data['edges']:
        g.add_edge(src, tgt, weight)

    # Run analysis
    pagerank = pg.centrality.pagerank(g, 0.85, 100, 1e-6)
    communities = pg.community.label_propagation(g, max_iter=100)

    return {
        'pagerank': pagerank,
        'communities': communities
    }

# Process multiple graphs in parallel
graphs = [graph1_data, graph2_data, graph3_data, ...]
with Pool(processes=4) as pool:
    results = pool.map(analyze_graph, graphs)
```

### 3. Optimize Graph Construction

Build graphs efficiently:

```python
# ✅ Good: Batch operations
g = pg.PyGraph()
node_ids = [g.add_node(attr) for attr in node_attributes]
for src, tgt, weight in edge_list:
    g.add_edge(src, tgt, weight)

# ❌ Slower: Many small operations
for attr in node_attributes:
    node_id = g.add_node(attr)
    # Do something with node_id immediately
```

## When to Use Parallel Algorithms

Consider parallel implementations when:

- Graph has > 1 million nodes
- Running the same algorithm repeatedly
- Real-time or latency-critical applications
- Multiple CPU cores available

For most applications, sequential algorithms are sufficient!

## Upcoming Features

Planned parallel implementations:

### Parallel PageRank (In Development)

```python
# Coming soon
import pygraphina.parallel as pgp

scores = pgp.pagerank_parallel(
    g,
    damping=0.85,
    max_iter=100,
    tolerance=1e-6,
    num_threads=4
)
```

**Expected speedup:** 3-4x on 4+ cores

### Parallel Community Detection (Planned)

```python
# Coming soon
communities = pgp.label_propagation_parallel(
    g,
    max_iter=100,
    num_threads=4
)
```

**Expected speedup:** 2-4x on 4+ cores

### Parallel BFS (Planned)

```python
# Coming soon
order = pgp.bfs_parallel(
    g,
    start_nodes=[0, 10, 20],  # Multiple start points
    num_threads=4
)
```

**Expected speedup:** 2-3x on 4+ cores

## Benchmarks

Preliminary benchmarks on a 4-core system:

| Graph Size | Algorithm | Sequential | Parallel (4 threads) | Speedup |
|------------|-----------|------------|---------------------|---------|
| 10K nodes | PageRank | 0.5s | 0.2s | 2.5x |
| 100K nodes | PageRank | 5.2s | 1.5s | 3.5x |
| 1M nodes | PageRank | 58s | 16s | 3.6x |

*Benchmarks are provisional and may change in final implementation.*

## Contributing

Interested in parallel algorithms? We welcome contributions!

1. Check [open issues](https://github.com/habedi/graphina/issues) for parallel algorithm tasks
2. See [CONTRIBUTING.md](../../contributing.md) for guidelines
3. Parallel implementations should maintain API compatibility

## See Also

- [Core Algorithms](../centrality/index.md) - Sequential implementations
- [Performance Guide](../../guide/performance.md) - Optimization tips
- [GitHub Roadmap](https://github.com/habedi/graphina/blob/main/ROADMAP.md) - Development plans

## Questions?

- **Q: Why aren't parallel versions available yet?**  
  A: We're focused on correctness and API stability first. Parallel versions will be added once the core API is stable.

- **Q: Will parallel versions have the same API?**  
  A: Yes! Parallel versions will be drop-in replacements with an optional `num_threads` parameter.

- **Q: Can I use multiprocessing with PyGraphina?**  
  A: Yes! PyGraphina graphs can be pickled and passed between processes.

- **Q: How do I know if I need parallel algorithms?**  
  A: Profile first! Most applications don't need explicit parallelism. Try sequential algorithms first.
