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
    write_edge_list(&graph, "temp_graph.txt", " ")?;
    // Note: Edge list doesn't preserve complex node attributes, usually just IDs
    // So we read it back as integer IDs if we want structure, or parse specially.
    // Here we just demo writing.

    // Cleanup
    fs::remove_file("temp_graph.bin")?;
    fs::remove_file("temp_graph.json")?;
    fs::remove_file("temp_graph.txt")?;

    Ok(())
}
```
