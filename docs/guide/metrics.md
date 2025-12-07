# Graph Metrics

Graph metrics provide quantitative measures of the structure and properties of a graph.

## Distance Metrics

### Diameter

The diameter is the longest shortest path between any two nodes in the graph.

```rust
use graphina::metrics::diameter;

let d = diameter(&graph);
// Returns Option<usize>, None if disconnected
```

### Radius

The radius is the minimum eccentricity of any node in the graph. The eccentricity of a node is the maximum distance from it to any other node.

```rust
use graphina::metrics::radius;

let r = radius(&graph);
```

### Average Path Length

The average length of shortest paths between all pairs of nodes.

```rust
use graphina::metrics::average_path_length;

let avg_len = average_path_length(&graph);
```

## Clustering and Mixing

### Clustering Coefficient

Measures the degree to which nodes tend to cluster together.

```rust
use graphina::metrics::average_clustering_coefficient;

let clustering = average_clustering_coefficient(&graph);
```

### Transitivity

Also known as the global clustering coefficient, it measures the ratio of triangles to connected triples.

```rust
use graphina::metrics::transitivity;

let t = transitivity(&graph);
```

### Assortativity

Measures the tendency of nodes to connect to others with similar degrees. Returns a value between -1 (disassortative) and 1 (assortative).

```rust
use graphina::metrics::assortativity;

let r = assortativity(&graph);
```
