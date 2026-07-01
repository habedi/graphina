# Graph I/O Examples

Persisting graphs is crucial for long-running applications.
Graphina supports efficient binary formats as well as standard text formats.

```rust
use graphina::core::types::Graph;
use graphina::core::io::{write_edge_list, read_edge_list};
use graphina::core::serialization::{save_binary, load_binary, save_json, load_json};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create a sample graph
    let mut graph = Graph::<String, f64>::new();
    let n1 = graph.add_node("Node A".to_string());
    let n2 = graph.add_node("Node B".to_string());
    let n3 = graph.add_node("Node C".to_string());
    graph.add_edge(n1, n2, 1.5);
    graph.add_edge(n2, n3, 2.5);
    graph.add_edge(n3, n1, 3.5);

    println!("Original: {} nodes, {} edges", graph.node_count(), graph.edge_count());

    // 2. Binary Serialization (Fastest, Compact)
    // Best for saving state between runs of the same application
    save_binary(&graph, "temp_graph.bin")?;
    let loaded_bin: Graph<String, f64> = load_binary("temp_graph.bin")?;
    println!("Loaded Binary: {} nodes", loaded_bin.node_count());

    // 3. JSON Serialization (Human readable, Portable)
    // Good for debugging or web APIs
    save_json(&graph, "temp_graph.json")?;
    let loaded_json: Graph<String, f64> = load_json("temp_graph.json")?;
    println!("Loaded JSON: {} nodes", loaded_json.node_count());

    // 4. Edge List (Universally compatible)
    // Good for exporting to tools like Gephi, NetworkX, Pandas
    // Note: Edge list functions operate on graphs with i32 node attributes and f32 edge weights.
    let mut el_graph = Graph::<i32, f32>::new();
    let e_n1 = el_graph.add_node(1);
    let e_n2 = el_graph.add_node(2);
    el_graph.add_edge(e_n1, e_n2, 1.5);

    write_edge_list("temp_graph.txt", &el_graph, ' ')?;

    let mut loaded_el_graph = Graph::<i32, f32>::new();
    read_edge_list("temp_graph.txt", &mut loaded_el_graph, ' ')?;
    println!("Loaded Edge List: {} nodes", loaded_el_graph.node_count());

    // Cleanup
    fs::remove_file("temp_graph.bin")?;
    fs::remove_file("temp_graph.json")?;
    fs::remove_file("temp_graph.txt")?;

    Ok(())
}
```
