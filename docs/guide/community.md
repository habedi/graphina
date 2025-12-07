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
    max_iterations: usize
) -> HashMap<NodeId, usize>
```

### Example

```rust
use graphina::community::label_propagation;

let communities = label_propagation(&graph, 100);

// Group nodes by community ID
let mut groups: HashMap<usize, Vec<NodeId>> = HashMap::new();
for (node, comm_id) in communities {
    groups.entry(comm_id).or_default().push(node);
}
```

## Louvain Method

A heuristic method to extract communities by optimizing modularity.
It is widely considered one of the best algorithms for community detection due to its speed and quality of results.

```rust
use graphina::community::louvain;

// Returns mapping from NodeId -> Community ID
let communities = louvain(&graph);
```

## Connected Components

Finds isolated subgraphs where every node is reachable from every other node.

- **Weakly Connected**: Ignoring edge direction.
- **Strongly Connected**: Respecting edge direction (every node must reach every other node).

```rust
use graphina::community::connected_components;

let components = connected_components(&graph);
println!("Found {} components", components.len());
```
