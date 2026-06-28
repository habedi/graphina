# Graph I/O

Graphina supports multiple formats for saving and loading graphs, facilitating data interchange and persistence.

## Supported Formats

| Format | Read | Write | Best For |
| :--- | :--- | :--- | :--- |
| **Edge List** | Yes | Yes | Simple, text-based data exchange. |
| **Adjacency List** | Yes | Yes | Compact text representation. |
| **JSON** | Yes | Yes | Web applications and detailed attribute storage. |
| **Binary** | Yes | Yes | Compact storage for large graphs. |
| **GraphML** | No | Yes | Interoperability with Gephi, Cytoscape, etc. |

## Text Formats

### Edge List

Reads/Writes a list of edges, one per line.

```rust
use graphina::core::types::Graph;
use graphina::core::io::{read_edge_list, write_edge_list};

// Save
write_edge_list(&graph, "graph.txt", " ").unwrap();

// Load (Node type i32, Edge weight f64)
let g: Graph<i32, f64> = read_edge_list("graph.txt", " ").unwrap();
```

### Adjacency List

Reads/Writes an adjacency list format (Node Neighbor1 Neighbor2 ...).

```rust
use graphina::core::io::{read_adjacency_list, write_adjacency_list};

write_adjacency_list(&graph, "adj.txt").unwrap();
let g: Graph<i32, f64> = read_adjacency_list("adj.txt").unwrap();
```

## Serialization

Graphina uses `serde` for serialization.

### JSON

```rust
use graphina::core::serialization::{save_json,load_json};

// Save
save_json(&graph, "graph.json").unwrap();

// Load
let g: Graph<String, f64> = load_json("graph.json").unwrap();
```

### Binary (bincode)

Compact and fast binary format.

```rust
use graphina::core::serialization::{save_binary, load_binary};

save_binary(&graph, "graph.bin").unwrap();
let g: Graph<i32, f64> = load_binary("graph.bin").unwrap();
```

### GraphML (Export Only)

GraphML is standard for graph visualization tools.

```rust
use graphina::core::serialization::save_graphml;

save_graphml(&graph, "graph.graphml").unwrap();
```
