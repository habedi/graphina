# Parallel Algorithms

Parallel implementations of graph algorithms for large-scale graphs.

## Overview

For very large graphs (millions of nodes, tens of millions of edges), PyGraphina provides parallel processing to speed up computations. The parallel module offers multi-threaded implementations of popular algorithms to maximize performance on multi-core systems.

## Available Algorithms

| Algorithm | Description |
|-----------|-------------|
| PageRank | Multi-threaded power iteration |
| BFS | Breadth-first search from multiple starting points |
| Connected Components | Parallel union-find for connected component detection |
| Degrees | Parallel degree calculation for all nodes |
| Triangles | Parallel triangle counting |
| Clustering Coefficients | Parallel clustering coefficient calculation |
| Shortest Paths | Parallel shortest path computation |

## Usage

### Basic Usage

```python
import pygraphina.parallel as pgp
import pygraphina as pg

g = pg.PyGraph()
# ... add nodes and edges ...

# Parallel PageRank
scores = pgp.pagerank_parallel(g, damping=0.85, max_iterations=100, tolerance=1e-6)

# Parallel BFS from multiple starting points
results = pgp.bfs_parallel(g, starts=[0, 10, 20])

# Parallel degree computation
degrees = pgp.degrees_parallel(g)
```


### Processing Multiple Graphs

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

### Graph Construction

Build graphs efficiently:

```python
# Recommended: Batch operations
g = pg.PyGraph()
node_ids = [g.add_node(attr) for attr in node_attributes]
for src, tgt, weight in edge_list:
    g.add_edge(src, tgt, weight)

# Slower: Many small operations
for attr in node_attributes:
    node_id = g.add_node(attr)
    # Do something with node_id immediately
```

## API Reference

### pagerank_parallel()

Compute PageRank scores using parallel processing.

```python
import pygraphina.parallel as pgp

scores = pgp.pagerank_parallel(
    graph,
    damping=0.85,
    max_iterations=100,
    tolerance=1e-6,
    nstart=None
)
```


### bfs_parallel()

Perform breadth-first search from multiple starting nodes.

```python
import pygraphina.parallel as pgp

results = pgp.bfs_parallel(
    graph,
    starts=[0, 10, 20]
)
```

### connected_components_parallel()

Find connected components using parallel union-find.

```python
import pygraphina.parallel as pgp

components = pgp.connected_components_parallel(graph)
```


### degrees_parallel()

Compute node degrees in parallel.

```python
degrees = pgp.degrees_parallel(graph)
```

### triangles_parallel()

Count triangles in parallel.

```python
triangle_counts = pgp.triangles_parallel(graph)
```

### clustering_coefficients_parallel()

Calculate clustering coefficients in parallel.

```python
clustering = pgp.clustering_coefficients_parallel(graph)
```

### shortest_paths_parallel()

Compute shortest paths in parallel.

```python
paths = pgp.shortest_paths_parallel(graph, sources=[0, 1, 2])
```



## Contributing

Interested in parallel algorithms or optimizations? We welcome contributions!

1. Check [open issues](https://github.com/habedi/graphina/issues) for parallel algorithm tasks
2. See [CONTRIBUTING.md](../../contributing.md) for guidelines
3. Parallel implementations should maintain API compatibility

## See Also

- [Centrality Algorithms](../centrality/index.md) - Sequential centrality measure implementations
- [Community Detection](../community/index.md) - Community structure detection algorithms
- [GitHub Roadmap](https://github.com/habedi/graphina/blob/main/ROADMAP.md) - Development plans
