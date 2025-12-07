# Basic Examples

This example demonstrates how to build a social network analysis tool using Graphina.
We will use a custom struct for node data, which is a powerful feature of Graphina's generic system.

## Social Network Analysis

We'll model a network where:

- **Nodes**: Users with names and ages.
- **Edges**: Friendships with a "strength" score.

```rust
use graphina::core::types::Graph;
use graphina::centrality::pagerank;
use std::fmt;

// Define your custom node data
#[derive(Clone, Debug, PartialEq)]
struct User {
    name: String,
    age: u32,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}

fn main() {
    // 1. Create the graph
    let mut social_graph = Graph::<User, f64>::new();

    // 2. Add users
    let alice = social_graph.add_node(User { name: "Alice".to_string(), age: 30 });
    let bob = social_graph.add_node(User { name: "Bob".to_string(), age: 25 });
    let charlie = social_graph.add_node(User { name: "Charlie".to_string(), age: 35 });
    let dave = social_graph.add_node(User { name: "Dave".to_string(), age: 28 });

    // 3. Add friendships (weights represent closeness)
    social_graph.add_edge(alice, bob, 0.9);
    social_graph.add_edge(alice, charlie, 0.5);
    social_graph.add_edge(bob, charlie, 0.8);
    social_graph.add_edge(charlie, dave, 0.3);

    // 4. Analyze: Find the "Influencer" of the group using PageRank
    let ranks = pagerank(&social_graph, 0.85, 100, 1e-6);

    println!("Social Influence Ranking:");

    // Sort results
    let mut sorted_ranks: Vec<_> = ranks.iter().collect();
    sorted_ranks.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    for (node_id, score) in sorted_ranks {
        let user = &social_graph[*node_id];
        println!("- {}: {:.4}", user, score);
    }

    // 5. Explore: Who represents the 'center' of the network?
    // (Using density as a simple metric of connectivity)
    println!("\nNetwork Density: {:.2}", social_graph.density());
}
```
