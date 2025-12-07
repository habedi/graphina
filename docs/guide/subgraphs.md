# Subgraphs and Views

Graphina provides a rich set of operations to extract subgraphs, filter views, and analyze local neighborhoods. These methods are available via the `SubgraphOps` trait.

## Extracting Subgraphs

### Induced Subgraph

Create a new graph containing only the specified nodes and all edges between them.

```rust
use graphina::subgraphs::SubgraphOps;

let nodes = vec![n1, n2];
let subgraph = graph.subgraph(&nodes).unwrap();
```

### Ego Graph

Extract the neighborhood around a center node up to a certain radius.

```rust
// 1-hop neighborhood around n1
let ego = graph.ego_graph(n1, 1).unwrap();
```

### Connected Components

Extract the connected component containing a specific node.

```rust
let component = graph.component_subgraph(n1).unwrap();
```

## Filtering

Create subgraphs by filtering nodes or edges based on custom predicates.

```rust
// Keep only nodes with even IDs
let even_nodes = graph.filter_nodes(|id, _attr| id.index() % 2 == 0);

// Keep only edges with weight > 10.0
let heavy_edges = graph.filter_edges(|_u, _v, w| *w > 10.0);
```

## Neighborhoods

Query local structure without creating a new graph object.

```rust
// Get all nodes within 2 hops
let neighbors = graph.k_hop_neighbors(n1, 2);
```
