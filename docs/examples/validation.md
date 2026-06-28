# Graph Validation Examples

This page demonstrates how to use Graphina's validation utilities to check preconditions before running algorithms.

## Precondition Checking

In this example, we validate that the input graph is a Directed Acyclic Graph (DAG) and has no negative weights before executing our logic.

```rust
use graphina::core::types::Digraph;
use graphina::core::validation::{require_dag, require_no_negative_weights};
use graphina::core::error::Result;

fn process_graph(graph: &Digraph<&str, f64>) -> Result<()> {
    // 1. Verify preconditions
    require_dag(graph)?;
    require_no_negative_weights(graph)?;

    // 2. Perform graph processing
    println!("Graph is valid. Processing...");
    
    Ok(())
}

fn main() {
    let mut g = Digraph::new();
    let n1 = g.add_node("A");
    let n2 = g.add_node("B");
    g.add_edge(n1, n2, 1.0);

    match process_graph(&g) {
        Ok(_) => println!("Success"),
        Err(e) => println!("Invalid graph: {:?}", e),
    }
}
```

## Partitions and Connectivity

This example checks if the graph is bipartite and counts the number of connected components.

```rust
use graphina::core::types::Graph;
use graphina::core::validation::{is_bipartite, count_components};

fn main() {
    let mut g = Graph::<i32, f64>::new();
    let n1 = g.add_node(1);
    let n2 = g.add_node(2);
    let n3 = g.add_node(3);
    
    g.add_edge(n1, n2, 1.0);
    g.add_edge(n2, n3, 1.0);

    // A path graph of length 3 is bipartite
    if is_bipartite(&g) {
        println!("Graph is bipartite");
    }

    // Number of connected components
    let components = count_components(&g);
    println!("Number of components: {}", components);
}
```
