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
