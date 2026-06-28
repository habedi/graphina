# Memory Pooling

Graphina provides experimental memory pooling utilities in the `core::pool` module, enabled via the `pool` feature flag.

## Purpose

Algorithms that perform traversals or search operations often allocate temporary collections, such as `HashSet` for visited nodes, `HashMap` for distance records, and `VecDeque` for queues. When run repeatedly, these allocations can cause memory fragmentation and overhead.

Memory pooling reuse existing collections to reduce allocation overhead.

## Pool Types

*   **`NodeSetPool`**: Reuses `HashSet<NodeId>` collections.
*   **`NodeMapPool`**: Reuses `HashMap<NodeId, T>` collections.
*   **`NodeQueuePool`**: Reuses `VecDeque<NodeId>` collections.

## Explicit Usage

You can instantiate pools explicitly and acquire pooled collections:

```rust
use graphina::core::pool::NodeSetPool;

// Create a set pool with a maximum capacity of 4 cached sets
let pool = NodeSetPool::new(4);

{
    // Acquire a set. It clears itself and behaves like a standard HashSet.
    let mut set = pool.acquire();
    set.insert(node_id);
} // The set is automatically returned to the pool when it goes out of scope.
```

## Thread-Local Defaults

For convenient reuse, Graphina provides thread-local default pools:

*   `with_default_set_pool`
*   `with_default_map_pool`
*   `with_default_queue_pool`

```rust
use graphina::core::pool::with_default_set_pool;

with_default_set_pool(|pool| {
    let mut set = pool.acquire();
    set.insert(node_id);
    // Use the set
});
```
