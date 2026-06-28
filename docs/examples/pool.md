# Memory Pooling Examples

This page demonstrates how to use memory pooling to reduce allocation overhead when executing repeated graph queries.

## Thread-Local Pool Example

We can use the default thread-local set and queue pools to implement a BFS traversal helper that avoids allocations on subsequent runs.

```rust
use graphina::core::types::Graph;
use graphina::core::pool::{with_default_set_pool, with_default_queue_pool};

fn bfs_with_pools(graph: &Graph<(), f64>, start: usize) {
    with_default_set_pool(|set_pool| {
        with_default_queue_pool(|queue_pool| {
            // Acquire clean collections from the pools
            let mut visited = set_pool.acquire();
            let mut queue = queue_pool.acquire();

            if start >= graph.node_count() {
                return;
            }

            let start_node = graph.node_ids().nth(start).unwrap();
            visited.insert(start_node);
            queue.push_back(start_node);

            while let Some(node) = queue.pop_front() {
                println!("Visiting node: {:?}", node);
                
                for neighbor in graph.neighbors(node) {
                    if visited.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
            // Collections are returned to their respective pools automatically when they drop
        });
    });
}

fn main() {
    let mut g = Graph::new();
    let n1 = g.add_node(());
    let n2 = g.add_node(());
    g.add_edge(n1, n2, 1.0);

    // Call the function multiple times to reuse the allocated sets and queues
    bfs_with_pools(&g, 0);
    bfs_with_pools(&g, 0);
}
```
