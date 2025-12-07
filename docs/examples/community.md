# Community Detection Examples

This guide demonstrates how to use Graphina's community detection algorithms to find clusters in your data.

## Detecting Communities with Louvain Method

The Louvain method is excellent for finding modular communities in large networks.

```rust
use graphina::core::types::Graph;
use graphina::community::louvain;
use std::collections::HashMap;

fn main() {
    // 1. Create a graph (e.g., a small social circle)
    let mut graph = Graph::<&str, f64>::new();
    let n0 = graph.add_node("Alice");
    let n1 = graph.add_node("Bob");
    let n2 = graph.add_node("Charlie"); // Cluster 1

    let n3 = graph.add_node("Dave");
    let n4 = graph.add_node("Eve");     // Cluster 2

    // Connect Cluster 1
    graph.add_edge(n0, n1, 1.0);
    graph.add_edge(n1, n2, 1.0);
    graph.add_edge(n0, n2, 1.0);

    // Connect Cluster 2
    graph.add_edge(n3, n4, 1.0);

    // Weak connection between clusters
    graph.add_edge(n2, n3, 0.2);

    // 2. Run Louvain
    let communities = louvain(&graph);

    // 3. Print results
    // Organize by community ID
    let mut groups: HashMap<usize, Vec<&str>> = HashMap::new();
    for (node_id, comm_id) in communities {
        let label = graph.node_weight(node_id).unwrap();
        groups.entry(comm_id).or_default().push(label);
    }

    for (id, members) in groups {
        println!("Community {}: {:?}", id, members);
    }
}
```

## Girvan-Newman for Hierarchical Clustering

If you need to understand the hierarchy of splits (e.g., separating a graph into exactly 2 communities), Girvan-Newman is useful.

```rust
use graphina::core::types::Graph;
use graphina::community::girvan_newman::girvan_newman;

fn main() {
    let mut graph = Graph::<&str, f64>::new();
    // ... setup graph ...

    // Split into exactly 2 communities
    match girvan_newman(&graph, 2) {
        Ok(communities) => {
            for (i, comm) in communities.iter().enumerate() {
                println!("Community {}: {} members", i, comm.len());
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
```

## Using Infomap for Flow-Based Clustering

Infomap is ideal when the flow of information on the network is more important than structural density.

```rust
use graphina::core::types::Graph;
use graphina::community::infomap::infomap;

fn main() {
    let mut graph = Graph::<&str, f64>::new();
    // ... setup graph ...

    let communities = infomap(&graph, 100, None).unwrap();
    // communities is a Vec<usize> mapping NodeIndex -> ModuleID
}
```
