# Graph Builders

Graphina provides builder patterns to construct graphs programmatically with validation and preset configurations.

## AdvancedGraphBuilder

For more control over graph properties (capacity, self-loops, etc.), use the `AdvancedGraphBuilder`.

### Basic Usage

```rust
use graphina::core::builders::UndirectedGraphBuilder;

fn main() {
    let graph = UndirectedGraphBuilder::<i32, f64>::undirected()
        .with_capacity(100, 200)       // Pre-allocate memory for nodes and edges
        .allow_self_loops(false)       // Disallow self-loops
        .allow_parallel_edges(false)   // Disallow parallel edges
        .add_node(1)
        .add_node(2)
        .add_edge(0, 1, 1.0)
        .build()
        .unwrap();

    println!("Graph created with {} nodes", graph.node_count());
}
```

### Type Aliases

*   `DirectedGraphBuilder`: Alias for `AdvancedGraphBuilder<A, W, Directed>`
*   `UndirectedGraphBuilder`: Alias for `AdvancedGraphBuilder<A, W, Undirected>`

### Validation

The `build()` method validates the configuration:

*   Checks if edge indices are within bounds.
*   Enforces `allow_self_loops` and `allow_parallel_edges` constraints.

## TopologyBuilder

The `TopologyBuilder` provides convenience methods for creating standard graph structures.

### Supported Topologies

*   Complete Graph: Every node is connected to every other node.
*   Cycle Graph: Nodes connected in a closed loop.
*   Path Graph: Nodes connected in a linear chain.
*   Star Graph: One central node connected to all other peripheral nodes.
*   Grid Graph: Nodes arranged in a grid lattice.

### Examples

```rust
use graphina::core::builders::TopologyBuilder;

// Create a complete graph with 5 nodes
// Node attributes are () (unit), Edge weights are 1.0
let complete = TopologyBuilder::complete(5, (), 1.0);

// Create a cycle graph with 6 nodes
let cycle = TopologyBuilder::cycle(6, "Node", 1.0);

// Create a star graph with 10 nodes (1 center + 9 leaves)
let star = TopologyBuilder::star(10, (), 1.0);

// Create a 3x4 grid graph
let grid = TopologyBuilder::grid(3, 4, (), 1.0);
```
