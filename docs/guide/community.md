# Community Detection

Community detection (or clustering) algorithms identify groups of nodes that are more densely connected to each other
than to the rest of the network.

## Label Propagation

A fast algorithm for finding communities in large networks.
Nodes adopt the label that most of their neighbors have.

### Function Signature

```rust
pub fn label_propagation<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    max_iter: usize,
    seed: Option<u64>,
) -> Result<Vec<usize>>
```

### Example

```rust
use graphina::community::label_propagation::label_propagation;

// Returns a Result containing a Vec of community IDs, where the index matches internal node order
let communities = label_propagation(&graph, 100, None).unwrap();

// Group nodes by community ID
let mut groups: std::collections::HashMap<usize, Vec<crate::core::types::NodeId>> = std::collections::HashMap::new();
for (idx, &comm_id) in communities.iter().enumerate() {
    let node_id = graph.node_ids().nth(idx).unwrap();
    groups.entry(comm_id).or_default().push(node_id);
}
```

## Infomap

A flow-based method that minimizes the map equation to detect communities.
Efficient for understanding flow constraints in networks.

```rust
use graphina::community::infomap::infomap;

// infomap(graph, max_iterations, optional_seed)
let communities = infomap(&graph, 100, Some(42)).unwrap();
```

## Girvan-Newman

A hierarchical method that progressively removes edges with high betweenness centrality.
Good for small to medium graphs where hierarchy is important.

```rust
use graphina::community::girvan_newman::girvan_newman;

// girvan_newman(graph, target_communities)
let communities = girvan_newman(&graph, 3).unwrap();
```

## Spectral Clustering

Uses the eigenvectors of the graph Laplacian to partition the graph.

```rust
use graphina::community::spectral::spectral_clustering;

// spectral_clustering(graph, num_clusters, optional_seed)
let communities = spectral_clustering(&graph, 3, Some(42)).unwrap();
```

## Louvain Method

A heuristic method to extract communities by optimizing modularity.
It is widely considered one of the best algorithms for community detection due to its speed and quality of results.

```rust
use graphina::community::louvain::louvain;

// Returns a Result containing a Vec of communities (each a Vec of NodeIds)
let communities = louvain(&graph, None).unwrap();
```

## Connected Components

Finds isolated subgraphs where every node is reachable from every other node.

- Weakly Connected: Ignoring edge direction.
- Strongly Connected: Respecting edge direction (every node must reach every other node).

```rust
use graphina::community::connected_components::connected_components;

let components = connected_components(&graph);
println!("Found {} components", components.len());
```
