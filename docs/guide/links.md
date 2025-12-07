# Link Prediction

Link prediction algorithms estimate the likelihood of a connection between two nodes based on their structural properties.

## Similarity Measures

### Jaccard Coefficient

The Jaccard coefficient measures similarity between two nodes' interaction sets.

$$ J(u, v) = \frac{|N(u) \cap N(v)|}{|N(u) \cup N(v)|} $$

```rust
use graphina::links::similarity::jaccard_coefficient;

// Calculate for all pairs
let predictions = jaccard_coefficient(&graph, None);

// Or for specific pairs
let pairs = vec![(n1, n2)];
let predictions = jaccard_coefficient(&graph, Some(&pairs));
```

### Adamic-Adar Index

The Adamic-Adar index is a measure that assigns more weight to neighbors with lower degrees.

$$ A(u, v) = \sum_{w \in N(u) \cap N(v)} \frac{1}{\log |N(w)|} $$

```rust
use graphina::links::similarity::adamic_adar_index;

let predictions = adamic_adar_index(&graph, None);
```

### Common Neighbors

Simply counts the number of shared neighbors between two nodes.

```rust
use graphina::links::similarity::common_neighbors;

let count = common_neighbors(&graph, n1, n2);
```

## Resource Allocation

### Resource Allocation Index

Penalizes contributions from high-degree common neighbors.

```rust
use graphina::links::allocation::resource_allocation_index;

let predictions = resource_allocation_index(&graph, None);
```

### RA Soundarajan-Hopcroft

Resource allocation index incorporating community information.

```rust
use graphina::links::allocation::ra_index_soundarajan_hopcroft;

// Define a community closure
let community_map = |node_id| 0; // dummy
let predictions = ra_index_soundarajan_hopcroft(&graph, None, community_map);
```

## Preferential Attachment

Measures the likelihood of a link based on the product of node degrees (rich get richer).

```rust
use graphina::links::attachment::preferential_attachment;

let predictions = preferential_attachment(&graph, None);
```

### Common Neighbor Centrality

A parameterized centrality measure based on common neighbors.

```rust
use graphina::links::centrality::common_neighbor_centrality;

let alpha = 0.8;
let predictions = common_neighbor_centrality(&graph, None, alpha);
```

## Community-Based

### CN Soundarajan-Hopcroft

Counts common neighbors that belong to the same community as the source and target.

```rust
use graphina::links::soundarajan_hopcroft::cn_soundarajan_hopcroft;

let predictions = cn_soundarajan_hopcroft(&graph, None, |n| /* return community ID */ 0);
```

### Within-Inter Cluster

Ratio of within-cluster common neighbors to inter-cluster common neighbors.

```rust
use graphina::links::cluster::within_inter_cluster;

let predictions = within_inter_cluster(&graph, None, |n| 0, 0.01);
```
