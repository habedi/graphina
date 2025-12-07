# Path Finding Examples

This guide provides examples of finding shortest paths in Graphina.

## Dijkstra's Algorithm

### Basic Usage with `f64` Weights

```rust
use graphina::core::types::Graph;
use graphina::core::paths::dijkstra_path_f64;

fn main() {
    let mut graph = Graph::new();
    let ids = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();

    // Add edges: (source, target, weight)
    let edges = [(0, 1, 1.0), (1, 2, 1.0), (2, 3, 2.0), (3, 4, 1.0)];
    for (s, d, w) in edges {
        graph.add_edge(ids[s], ids[d], w);
    }

    // Find path from node 0
    let (cost, trace) = dijkstra_path_f64(&graph, ids[0], None).unwrap();

    println!("Distance to node 4: {:?}", cost[&ids[4]]);
}
```

### Complex Example: Flight Network

This example demonstrates using **custom edge types** and a custom **cost evaluation function**. Imagine a flight network where edges have both a price and an aircraft type.

```rust
use graphina::core::types::Digraph;
use graphina::core::paths::dijkstra_path_impl;

fn main() {
    // Edge stores (Price, Aircraft Type)
    let mut graph: Digraph<String, (f64, String)> = Digraph::new();

    let cities = ["ATL", "PEK", "LHR", "HND", "CDG", "FRA", "HKG"];
    let ids: Vec<_> = cities.iter().map(|s| graph.add_node(s.to_string())).collect();

    // Define flights
    // (Source, Dest, (Price, Aircraft))
    let flights = [
        ("ATL", "PEK", (900.0, "boeing")),
        ("ATL", "LHR", (500.0, "airbus")),
        ("PEK", "LHR", (800.0, "boeing")),
        ("LHR", "FRA", (200.0, "boeing")),
    ];

    // Build graph
    for (source, dest, (price, model)) in flights {
        // Find NodeIds (in a real app, use a HashMap to look them up efficiently)
        let s_id = cities.iter().position(|&c| c == source).unwrap();
        let d_id = cities.iter().position(|&c| c == dest).unwrap();

        // Add edge with (Price, Aircraft)
        graph.add_edge(ids[s_id], ids[d_id], (price, model.to_string()));
    }

    // Custom Cost Function: Avoid Boeing planes!
    // Returns Some(cost) if passable, None if impassable.
    let eval_cost = |(price, manufacturer): &(f64, String)| match manufacturer.as_str() {
        "boeing" => None,         // "I'm not flying Boeing!"
        _ => Some(*price),        // "Airbus is fine."
    };

    // Run Dijkstra with custom evaluator
    // cutoff: Some(1000.0) -> Stop if cost exceeds 1000
    let (cost, _trace) = dijkstra_path_impl(&graph, ids[0], Some(1000.0), eval_cost).unwrap();

    // Result will ignore paths using Boeing planes
}
```
