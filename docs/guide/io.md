# Graph I/O

Graphina supports multiple formats for saving and loading graphs.

## Supported Formats

| Format         | Read | Write | Best For                                                |
|:---------------|:-----|:------|:--------------------------------------------------------|
| Edge List      | Yes  | Yes   | Simple, text-based data exchange.                       |
| Adjacency List | Yes  | Yes   | Compact text representation.                            |
| JSON           | Yes  | Yes   | Web applications and detailed attribute storage.        |
| Binary         | Yes  | Yes   | Compact storage for large graphs.                       |
| GraphML        | No   | Yes   | Interoperability with tools like Gephi, Cytoscape, etc. |

## Edge List

Reads and writes a list of edges, one edge per line.

```rust
use graphina::core::types::Graph;
use graphina::core::io::{read_edge_list, write_edge_list};

// Save
write_edge_list("graph.txt", & graph, ' ').unwrap();

// Load (node attribute i32, edge weight f32)
let mut loaded_graph = Graph::<i32, f32>::new();
read_edge_list("graph.txt", & mut loaded_graph, ' ').unwrap();
```

## Adjacency List

Reads and writes an adjacency list format (each line is `Node` `Neighbor1` `Weight1` `Neighbor2` `Weight2` ...).

```rust
use graphina::core::types::Graph;
use graphina::core::io::{read_adjacency_list, write_adjacency_list};

// Save
write_adjacency_list("adj.txt", & graph, ' ').unwrap();

// Load
let mut loaded_graph = Graph::<i32, f32>::new();
read_adjacency_list("adj.txt", & mut loaded_graph, ' ').unwrap();
```

## JSON

```rust
use graphina::core::types::Graph;

// Save
graph.save_json("graph.json").unwrap();

// Load
let g = Graph::<i32, f32>::load_json("graph.json").unwrap();
```

## Binary Format

Compact and fast binary format.

```rust
use graphina::core::types::Graph;

let mut graph = Graph::<i32, f32>::new();
let a = graph.add_node(1);
let b = graph.add_node(2);
graph.add_edge(a, b, 1.0);

// Load with the same attribute and weight types that were saved.
graph.save_binary("graph.bin").unwrap();
let g = Graph::<i32, f32>::load_binary("graph.bin").unwrap();
assert_eq!(g.edge_count(), 1);
```

## GraphML (Export-only)

```rust
graph.save_graphml("graph.graphml").unwrap();
```
