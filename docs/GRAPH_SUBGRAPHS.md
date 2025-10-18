# Graph Views & Subgraphs Feature

**Date:** October 18, 2025  
**Version:** 0.4.0  
**Status:** ✅ Implemented and Tested

## Overview

The Graph Views & Subgraphs feature provides efficient ways to work with subsets of graphs without manually copying nodes and edges. Essential for analysis workflows like community detection, ego network analysis, and graph filtering.

## Features Implemented

### Subgraph Extraction
- `subgraph()` - Extract nodes and their connections
- `induced_subgraph()` - Create induced subgraph
- `component_subgraph()` - Extract connected component

### Network Analysis
- `ego_graph()` - Ego network with radius
- `k_hop_neighbors()` - K-hop neighborhood
- `connected_component()` - Component membership

### Filtering
- `filter_nodes()` - Filter by node attributes
- `filter_edges()` - Filter by edge weights

---

## API Reference

### `subgraph(nodes: &[NodeId]) -> Result<Graph>`

Extracts a subgraph containing only specified nodes and edges between them.

```rust
use graphina::core::types::Graph;

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
let n3 = g.add_node(3);
let n4 = g.add_node(4);

g.add_edge(n1, n2, 1.0);
g.add_edge(n2, n3, 2.0);
g.add_edge(n3, n4, 3.0);

// Extract subgraph with nodes 1, 2, 3
let sub = g.subgraph(&[n1, n2, n3]).unwrap();
assert_eq!(sub.node_count(), 3);
assert_eq!(sub.edge_count(), 2); // Only edges among selected nodes
```

---

### `ego_graph(center: NodeId, radius: usize) -> Result<Graph>`

Extracts an ego network centered on a node within a given radius.

```rust
use graphina::core::types::Graph;

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
let n3 = g.add_node(3);
let n4 = g.add_node(4);

g.add_edge(n1, n2, 1.0);
g.add_edge(n2, n3, 1.0);
g.add_edge(n3, n4, 1.0);

// Get ego network of n2 with radius 1
let ego = g.ego_graph(n2, 1).unwrap();
assert_eq!(ego.node_count(), 3); // n1, n2, n3 (within 1 hop)
```

**Use cases:**
- Social network analysis (friends and friends-of-friends)
- Local neighborhood exploration
- Influence network detection

---

### `filter_nodes<F>(predicate: F) -> Graph`

Creates a subgraph containing only nodes that satisfy the predicate.

```rust
use graphina::core::types::Graph;

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
let n3 = g.add_node(3);
let n4 = g.add_node(4);

g.add_edge(n1, n2, 1.0);
g.add_edge(n2, n3, 1.0);

// Keep only even-valued nodes
let filtered = g.filter_nodes(|_id, attr| *attr % 2 == 0);
assert_eq!(filtered.node_count(), 2); // nodes 2 and 4
```

---

### `filter_edges<F>(predicate: F) -> Graph`

Creates a graph with all nodes but only edges satisfying the predicate.

```rust
use graphina::core::types::Graph;

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
let n3 = g.add_node(3);

g.add_edge(n1, n2, 0.5);
g.add_edge(n2, n3, 1.5);
g.add_edge(n3, n1, 2.5);

// Keep only strong connections (weight > 1.0)
let filtered = g.filter_edges(|_src, _tgt, w| *w > 1.0);
assert_eq!(filtered.node_count(), 3);
assert_eq!(filtered.edge_count(), 2);
```

---

### `k_hop_neighbors(start: NodeId, k: usize) -> Vec<NodeId>`

Returns all nodes within k hops from the starting node.

```rust
use graphina::core::types::Graph;

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
let n3 = g.add_node(3);
let n4 = g.add_node(4);

g.add_edge(n1, n2, 1.0);
g.add_edge(n2, n3, 1.0);
g.add_edge(n3, n4, 1.0);

let neighbors = g.k_hop_neighbors(n1, 2);
assert_eq!(neighbors.len(), 3); // n1, n2, n3
```

---

### `connected_component(start: NodeId) -> Vec<NodeId>`

Returns all nodes in the same connected component as the starting node.

```rust
use graphina::core::types::Graph;

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
let n3 = g.add_node(3);
let n4 = g.add_node(4);

g.add_edge(n1, n2, 1.0);
g.add_edge(n2, n3, 1.0);
// n4 is isolated

let component = g.connected_component(n1);
assert_eq!(component.len(), 3); // n1, n2, n3
```

---

### `component_subgraph(start: NodeId) -> Result<Graph>`

Extracts the connected component containing the node as a subgraph.

```rust
use graphina::core::types::Graph;

let mut g = Graph::<i32, f64>::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
let n3 = g.add_node(3);

g.add_edge(n1, n2, 1.0);
// n3 isolated

let sub = g.component_subgraph(n1).unwrap();
assert_eq!(sub.node_count(), 2);
assert_eq!(sub.edge_count(), 1);
```

---

## Usage Patterns

### Pattern 1: Community Analysis

```rust
use graphina::core::types::Graph;
use graphina::core::validation::count_components;

fn analyze_communities(graph: &Graph<i32, f64>) {
    // Find all communities (connected components)
    let num_components = count_components(graph);
    println!("Found {} communities", num_components);

    // Extract each community as a subgraph
    let mut analyzed = std::collections::HashSet::new();

    for node in graph.node_ids() {
        if analyzed.contains(&node) {
            continue;
        }

        let community = graph.component_subgraph(node).unwrap();
        println!("Community size: {} nodes, {} edges",
                 community.node_count(),
                 community.edge_count());

        // Mark all nodes in this community as analyzed
        for n in community.node_ids() {
            analyzed.insert(n);
        }
    }
}
```

---

### Pattern 2: Ego Network Analysis

```rust
use graphina::core::types::Graph;
use graphina::core::metrics::clustering_coefficient;

fn analyze_ego_networks(graph: &Graph<i32, f64>) {
    for node in graph.node_ids().take(10) {
        // Extract 2-hop ego network
        let ego = graph.ego_graph(node, 2).unwrap();

        println!("Node {} ego network:", node.index());
        println!("  Size: {} nodes", ego.node_count());
        println!("  Density: {:.3}", ego.density());

        // Analyze local structure
        let avg_clustering = ego.node_ids()
            .map(|n| clustering_coefficient(&ego, n))
            .sum::<f64>() / ego.node_count() as f64;

        println!("  Avg clustering: {:.3}", avg_clustering);
    }
}
```

---

### Pattern 3: Threshold-Based Filtering

```rust
use graphina::core::types::Graph;

fn filter_weak_connections(
    graph: &Graph<i32, f64>,
    threshold: f64
) -> Graph<i32, f64> {
    // Keep only strong connections
    graph.filter_edges(|_src, _tgt, weight| *weight >= threshold)
}

fn main() {
    let mut g = Graph::<i32, f64>::new();
    // ... build graph ...

    // Filter out weak edges
    let strong = filter_weak_connections(&g, 0.5);

    println!("Original: {} edges", g.edge_count());
    println!("After filtering: {} edges", strong.edge_count());
}
```

---

### Pattern 4: Multi-Level Analysis

```rust
use graphina::core::types::Graph;

fn multi_level_analysis(graph: &Graph<i32, f64>, center: NodeId) {
    println!("=== Multi-Level Network Analysis ===");

    // Level 1: Direct neighbors
    let level1 = graph.ego_graph(center, 1).unwrap();
    println!("Level 1 (direct): {} nodes", level1.node_count());

    // Level 2: Friends of friends
    let level2 = graph.ego_graph(center, 2).unwrap();
    println!("Level 2 (2-hop): {} nodes", level2.node_count());

    // Level 3: Extended network
    let level3 = graph.ego_graph(center, 3).unwrap();
    println!("Level 3 (3-hop): {} nodes", level3.node_count());

    // Analyze growth rate
    println!("Growth: 1→2: {}x, 2→3: {}x",
             level2.node_count() as f64 / level1.node_count() as f64,
             level3.node_count() as f64 / level2.node_count() as f64);
}
```

---

### Pattern 5: Attribute-Based Filtering

```rust
use graphina::core::types::Graph;

#[derive(Clone)]
struct Person {
    name: String,
    age: u32,
    city: String,
}

fn filter_by_demographics(graph: &Graph<Person, f64>) {
    // Filter by age
    let young = graph.filter_nodes(|_id, person| person.age < 30);
    println!("Young people: {}", young.node_count());

    // Filter by city
    let nyc = graph.filter_nodes(|_id, person| person.city == "NYC");
    println!("NYC residents: {}", nyc.node_count());

    // Combined filter
    let young_nyc = graph.filter_nodes(|_id, person| {
        person.age < 30 && person.city == "NYC"
    });
    println!("Young NYC residents: {}", young_nyc.node_count());
}
```

---

## Performance Characteristics

| Operation | Time Complexity | Space Complexity | Notes |
|-----------|----------------|------------------|-------|
| `subgraph()` | O(V + E) | O(V + E) | Creates new graph |
| `ego_graph()` | O(V + E) | O(V + E) | BFS + subgraph |
| `filter_nodes()` | O(V + E) | O(V + E) | Full graph scan |
| `filter_edges()` | O(V + E) | O(V + E) | Full graph scan |
| `k_hop_neighbors()` | O(V + E) | O(V) | BFS traversal |
| `connected_component()` | O(V + E) | O(V) | DFS traversal |

**Note:** All operations create new graphs (not zero-copy views). This ensures safety but requires memory allocation.

---

## Complete Example: Social Network Analysis

```rust
use graphina::core::types::Graph;
use graphina::core::metrics::*;

fn analyze_social_network() {
    // Build social network
    let mut network = Graph::<String, f64>::new();

    let alice = network.add_node("Alice".to_string());
    let bob = network.add_node("Bob".to_string());
    let charlie = network.add_node("Charlie".to_string());
    let diana = network.add_node("Diana".to_string());
    let eve = network.add_node("Eve".to_string());

    // Add friendships with strength scores
    network.add_edge(alice, bob, 0.95);
    network.add_edge(alice, charlie, 0.88);
    network.add_edge(bob, charlie, 0.92);
    network.add_edge(charlie, diana, 0.75);
    network.add_edge(diana, eve, 0.83);

    // 1. Analyze Alice's ego network
    println!("=== Alice's Ego Network ===");
    let alice_ego = network.ego_graph(alice, 1).unwrap();
    println!("Friends: {}", alice_ego.node_count() - 1);
    println!("Connections: {}", alice_ego.edge_count());

    // 2. Find strong connections only
    println!("\n=== Strong Connections (>0.9) ===");
    let strong = network.filter_edges(|_s, _t, w| *w > 0.9);
    println!("Strong edges: {}", strong.edge_count());

    // 3. Extract Alice's community
    println!("\n=== Alice's Community ===");
    let community = network.component_subgraph(alice).unwrap();
    println!("Community size: {}", community.node_count());
    println!("Community density: {:.3}", community.density());

    // 4. Analyze 2-hop neighborhood
    println!("\n=== 2-Hop Neighborhood ===");
    let neighbors_2hop = network.k_hop_neighbors(alice, 2);
    println!("Reachable within 2 hops: {}", neighbors_2hop.len());
}
```

---

## Testing

All subgraph functions are thoroughly tested:
- ✅ Empty graphs
- ✅ Single-node graphs
- ✅ Disconnected graphs
- ✅ Large graphs
- ✅ Edge cases (invalid nodes, etc.)
- ✅ Filter predicates

**Test count:** 8 comprehensive tests  
**All tests passing:** ✅

---

## Best Practices

### 1. Use Appropriate Methods

```rust
// Good: Use ego_graph for radius-based extraction
let ego = graph.ego_graph(center, 2).unwrap();

// Less efficient: Manual BFS + subgraph
let neighbors = graph.k_hop_neighbors(center, 2);
let ego = graph.subgraph(&neighbors).unwrap();
```

### 2. Filter Early

```rust
// Good: Filter first, then analyze
let filtered = graph.filter_edges(|_,_, w| *w > threshold);
analyze(&filtered);

// Less efficient: Analyze full graph, ignore results
analyze_with_threshold(&graph, threshold);
```

### 3. Reuse Subgraphs

```rust
// Good: Extract once, analyze multiple times
let ego = graph.ego_graph(center, 2).unwrap();
let density = ego.density();
let clustering = average_clustering_coefficient(&ego);

// Less efficient: Extract multiple times
let density = graph.ego_graph(center, 2).unwrap().density();
let clustering = average_clustering_coefficient(
    &graph.ego_graph(center, 2).unwrap()
);
```

---

## Future Enhancements

Planned additions:
1. **Zero-copy views** - Read-only graph views without allocation
2. **Lazy filtering** - Deferred evaluation for chained filters
3. **Subgraph union/intersection** - Combine multiple subgraphs
4. **Temporal subgraphs** - Time-window based extraction
5. **Parallel subgraph extraction** - Multi-threaded for large graphs

---

## Related Documentation

- [Batch Operations](BATCH_OPERATIONS.md)
- [Graph Validation](GRAPH_VALIDATION.md)
- [Graph Metrics](GRAPH_METRICS.md)
- [Graph Serialization](GRAPH_SERIALIZATION.md)
