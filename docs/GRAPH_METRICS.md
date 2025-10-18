# Graph Metrics Feature

**Date:** October 18, 2025  
**Version:** 0.4.0  
**Status:** ✅ Implemented and Tested

## Overview

The Graph Metrics module provides essential statistical measures and metrics for network analysis. These metrics help
characterize graph structure, identify important patterns, and compare different networks.

## Features Implemented

### Global Metrics (Entire Graph)

- **Diameter** - Longest shortest path
- **Radius** - Minimum eccentricity
- **Average Clustering Coefficient** - Overall clustering tendency
- **Transitivity** - Global clustering coefficient
- **Average Path Length** - Mean distance between node pairs
- **Assortativity** - Degree correlation measure

### Local Metrics (Per Node)

- **Clustering Coefficient** - Local clustering tendency
- **Triangles** - Number of triangles containing a node

---

## API Reference

### `diameter(graph) -> Option<usize>`

Computes the longest shortest path in the graph.

```rust
use graphina::core::{types::Graph, metrics::diameter};

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
let n3 = g.add_node(3);
g.add_edge(n1, n2, 1.0);
g.add_edge(n2, n3, 1.0);

assert_eq!(diameter(&g), Some(2));
```

**Returns:** `None` for disconnected graphs  
**Time Complexity:** O(V * (V + E))

---

### `radius(graph) -> Option<usize>`

Computes the minimum eccentricity (smallest maximum distance from a node).

```rust
use graphina::core::{types::Graph, metrics::radius};

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
let n3 = g.add_node(3);
g.add_edge(n1, n2, 1.0);
g.add_edge(n2, n3, 1.0);

assert_eq!(radius(&g), Some(1)); // Center node n2
```

**Time Complexity:** O(V * (V + E))

---

### `average_clustering_coefficient(graph) -> f64`

Computes the average of all local clustering coefficients.

```rust
use graphina::core::{types::Graph, metrics::average_clustering_coefficient};

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
let n3 = g.add_node(3);
g.add_edge(n1, n2, 1.0);
g.add_edge(n2, n3, 1.0);
g.add_edge(n3, n1, 1.0);

assert!((average_clustering_coefficient(&g) - 1.0).abs() < 0.001);
```

**Returns:** Value between 0.0 and 1.0  
**Time Complexity:** O(V * d²) where d is average degree

---

### `clustering_coefficient(graph, node) -> f64`

Computes the local clustering coefficient for a specific node.

```rust
use graphina::core::{types::Graph, metrics::clustering_coefficient};

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
let n3 = g.add_node(3);
g.add_edge(n1, n2, 1.0);
g.add_edge(n2, n3, 1.0);
g.add_edge(n3, n1, 1.0);

assert_eq!(clustering_coefficient(&g, n1), 1.0);
```

**Returns:** Ratio of actual to possible edges among neighbors  
**Time Complexity:** O(d²) where d is node degree

---

### `transitivity(graph) -> f64`

Computes the global clustering coefficient (ratio of triangles to triples).

```rust
use graphina::core::{types::Graph, metrics::transitivity};

let mut g = Graph::<i32, f64>::new();
// Build triangle...
g.add_edge(n1, n2, 1.0);
g.add_edge(n2, n3, 1.0);
g.add_edge(n3, n1, 1.0);

assert_eq!(transitivity(&g), 1.0);
```

**Time Complexity:** O(V * d²)

---

### `triangles(graph, node) -> usize`

Counts triangles containing a specific node.

```rust
use graphina::core::{types::Graph, metrics::triangles};

assert_eq!(triangles(&g, n1), 1);
```

**Time Complexity:** O(d²)

---

### `average_path_length(graph) -> Option<f64>`

Computes the mean shortest path length between all node pairs.

```rust
use graphina::core::{types::Graph, metrics::average_path_length};

let avg = average_path_length(&g).unwrap();
println!("Average path length: {:.2}", avg);
```

**Returns:** `None` for disconnected graphs  
**Time Complexity:** O(V * (V + E))

---

### `assortativity(graph) -> f64`

Measures the tendency of nodes to connect to similar-degree nodes.

```rust
use graphina::core::{types::Graph, metrics::assortativity};

let assort = assortativity(&g);
// assort > 0: assortative (high-degree connects to high-degree)
// assort < 0: disassortative (high-degree connects to low-degree)
```

**Returns:** Value between -1.0 and 1.0  
**Time Complexity:** O(E)

---

## Complete Example: Network Analysis

```rust
use graphina::core::{
    types::Graph,
    metrics::*,
};

fn analyze_network(graph: &Graph<i32, f64>) {
    println!("=== Network Analysis ===");

    // Global metrics
    if let Some(diam) = diameter(graph) {
        println!("Diameter: {}", diam);
    }

    if let Some(rad) = radius(graph) {
        println!("Radius: {}", rad);
    }

    if let Some(avg_path) = average_path_length(graph) {
        println!("Average path length: {:.3}", avg_path);
    }

    println!("Average clustering: {:.3}",
             average_clustering_coefficient(graph));
    println!("Transitivity: {:.3}", transitivity(graph));
    println!("Assortativity: {:.3}", assortativity(graph));

    // Node-level analysis
    println!("\n=== Node Analysis ===");
    for node in graph.node_ids().take(5) {
        println!("Node {}: clustering={:.3}, triangles={}",
                 node.index(),
                 clustering_coefficient(graph, node),
                 triangles(graph, node));
    }
}

// Example: Analyze Karate Club network
fn main() {
    let mut karate = Graph::<i32, f64>::new();

    // Add 34 members
    let nodes: Vec<_> = (0..34).map(|i| karate.add_node(i)).collect();

    // Add friendships (simplified example)
    karate.add_edge(nodes[0], nodes[1], 1.0);
    karate.add_edge(nodes[0], nodes[2], 1.0);
    // ... more edges

    analyze_network(&karate);
}
```

---

## Real-World Applications

### Social Network Analysis

```rust
// Identify tightly-knit communities
for node in graph.node_ids() {
    let cc = clustering_coefficient(graph, node);
    if cc > 0.7 {
        println!("Node {} is in a tight community", node.index());
    }
}
```

### Network Robustness

```rust
// Check if network maintains small-world property
if let Some(avg_path) = average_path_length(graph) {
    let avg_clustering = average_clustering_coefficient(graph);

    if avg_path < 10.0 && avg_clustering > 0.3 {
        println!("Network exhibits small-world properties");
    }
}
```

### Degree Correlation Analysis

```rust
let assort = assortativity(graph);

if assort > 0.3 {
    println!("Assortative network: hubs connect to hubs");
    println!("Common in social networks");
} else if assort < -0.3 {
    println!("Disassortative network: hubs avoid each other");
    println!("Common in technological networks");
}
```

---

## Performance Characteristics

| Metric          | Time Complexity | Space | Best For                         |
|-----------------|-----------------|-------|----------------------------------|
| Diameter        | O(V²)           | O(V)  | Small-medium graphs (<10K nodes) |
| Radius          | O(V²)           | O(V)  | Small-medium graphs              |
| Avg Clustering  | O(V·d²)         | O(1)  | All graph sizes                  |
| Transitivity    | O(V·d²)         | O(1)  | All graph sizes                  |
| Triangles       | O(d²)           | O(1)  | Per-node queries                 |
| Avg Path Length | O(V²)           | O(V)  | Small-medium graphs              |
| Assortativity   | O(E)            | O(1)  | All graph sizes ✓                |

---

## Interpreting Metrics

### Diameter & Radius

- **Small diameter** (< log V): Efficient information spread
- **Large diameter** (> V/2): Slow communication, bottlenecks
- **Radius ≈ Diameter/2**: Well-balanced network

### Clustering Coefficient

- **High (> 0.6)**: Strong local structure, communities
- **Medium (0.3-0.6)**: Balanced structure
- **Low (< 0.3)**: Random-like, tree-like structure

### Average Path Length

- **Small (< 6)**: "Small world" property
- **Medium (6-10)**: Typical for real networks
- **Large (> 10)**: Poor connectivity

### Assortativity

- **Positive**: Social networks, collaboration networks
- **Near zero**: Random networks
- **Negative**: Internet, protein interaction networks

---

## Testing

All metrics functions are tested with:

- ✅ Triangle graphs (perfect clustering)
- ✅ Path graphs (minimal clustering)
- ✅ Disconnected graphs
- ✅ Single-node graphs
- ✅ Empty graphs
- ✅ Star graphs
- ✅ Complete graphs

**Test count:** 9 comprehensive tests  
**All tests passing:** ✅

---

## Future Enhancements

1. **Weighted metrics** - Support for weighted clustering, etc.
2. **Approximate metrics** - Faster approximate diameter/radius
3. **Centrality metrics** - Integrate with centrality module
4. **Efficiency metrics** - Global/local efficiency
5. **Modularity** - Community structure quality
6. **Rich club coefficient** - Elite node connectivity

---

## Related Documentation

- [Batch Operations](BATCH_OPERATIONS.md)
- [Graph Validation](GRAPH_VALIDATION.md)
- [API Improvements](API_IMPROVEMENTS.md)
