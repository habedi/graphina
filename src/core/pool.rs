/*!
# Memory Pool

This module provides memory pooling utilities for performance-critical graph operations.
Memory pools reduce allocation overhead by reusing pre-allocated memory, which is especially
beneficial for algorithms that create many temporary data structures.

## Use Cases

- BFS/DFS visited sets
- Shortest path distance maps
- Community detection temporary structures
- Centrality computation intermediate results
*/

use crate::core::types::NodeId;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};

/// A pool for reusable HashSet<NodeId> instances.
///
/// This is useful for algorithms that frequently need temporary sets of nodes,
/// such as BFS/DFS traversals.
pub struct NodeSetPool {
    pool: RefCell<Vec<HashSet<NodeId>>>,
    max_size: usize,
}

impl NodeSetPool {
    /// Creates a new pool with the specified maximum size.
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: RefCell::new(Vec::with_capacity(max_size)),
            max_size,
        }
    }

    /// Gets a HashSet from the pool, or creates a new one if the pool is empty.
    pub fn acquire(&self) -> PooledNodeSet<'_> {
        let set = self.pool.borrow_mut().pop().unwrap_or_default();
        PooledNodeSet { set, pool: self }
    }

    /// Returns a HashSet to the pool for reuse.
    fn release(&self, mut set: HashSet<NodeId>) {
        set.clear();
        let mut pool = self.pool.borrow_mut();
        if pool.len() < self.max_size {
            pool.push(set);
        }
    }

    /// Returns the current number of sets in the pool.
    pub fn available(&self) -> usize {
        self.pool.borrow().len()
    }
}

/// A HashSet<NodeId> that returns itself to the pool when dropped.
pub struct PooledNodeSet<'a> {
    set: HashSet<NodeId>,
    pool: &'a NodeSetPool,
}

impl std::ops::Deref for PooledNodeSet<'_> {
    type Target = HashSet<NodeId>;

    fn deref(&self) -> &Self::Target {
        &self.set
    }
}

impl std::ops::DerefMut for PooledNodeSet<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.set
    }
}

impl Drop for PooledNodeSet<'_> {
    fn drop(&mut self) {
        let set = std::mem::take(&mut self.set);
        self.pool.release(set);
    }
}

/// A pool for reusable HashMap<NodeId, T> instances.
///
/// Useful for algorithms that need temporary node-indexed data structures.
pub struct NodeMapPool<T> {
    pool: RefCell<Vec<HashMap<NodeId, T>>>,
    max_size: usize,
}

impl<T> NodeMapPool<T> {
    /// Creates a new pool with the specified maximum size.
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: RefCell::new(Vec::with_capacity(max_size)),
            max_size,
        }
    }

    /// Gets a HashMap from the pool, or creates a new one if the pool is empty.
    pub fn acquire(&self) -> PooledNodeMap<'_, T> {
        let map = self.pool.borrow_mut().pop().unwrap_or_default();
        PooledNodeMap { map, pool: self }
    }

    /// Returns a HashMap to the pool for reuse.
    fn release(&self, mut map: HashMap<NodeId, T>) {
        map.clear();
        let mut pool = self.pool.borrow_mut();
        if pool.len() < self.max_size {
            pool.push(map);
        }
    }

    /// Returns the current number of maps in the pool.
    pub fn available(&self) -> usize {
        self.pool.borrow().len()
    }
}

/// A HashMap<NodeId, T> that returns itself to the pool when dropped.
pub struct PooledNodeMap<'a, T> {
    map: HashMap<NodeId, T>,
    pool: &'a NodeMapPool<T>,
}

impl<T> std::ops::Deref for PooledNodeMap<'_, T> {
    type Target = HashMap<NodeId, T>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<T> std::ops::DerefMut for PooledNodeMap<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<T> Drop for PooledNodeMap<'_, T> {
    fn drop(&mut self) {
        let map = std::mem::take(&mut self.map);
        self.pool.release(map);
    }
}

/// A pool for reusable VecDeque<NodeId> instances.
///
/// Useful for BFS queues and other queue-based algorithms.
pub struct NodeQueuePool {
    pool: RefCell<Vec<VecDeque<NodeId>>>,
    max_size: usize,
}

impl NodeQueuePool {
    /// Creates a new pool with the specified maximum size.
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: RefCell::new(Vec::with_capacity(max_size)),
            max_size,
        }
    }

    /// Gets a VecDeque from the pool, or creates a new one if the pool is empty.
    pub fn acquire(&self) -> PooledNodeQueue<'_> {
        let queue = self.pool.borrow_mut().pop().unwrap_or_default();
        PooledNodeQueue { queue, pool: self }
    }

    /// Returns a VecDeque to the pool for reuse.
    fn release(&self, mut queue: VecDeque<NodeId>) {
        queue.clear();
        let mut pool = self.pool.borrow_mut();
        if pool.len() < self.max_size {
            pool.push(queue);
        }
    }

    /// Returns the current number of queues in the pool.
    pub fn available(&self) -> usize {
        self.pool.borrow().len()
    }
}

/// A VecDeque<NodeId> that returns itself to the pool when dropped.
pub struct PooledNodeQueue<'a> {
    queue: VecDeque<NodeId>,
    pool: &'a NodeQueuePool,
}

impl std::ops::Deref for PooledNodeQueue<'_> {
    type Target = VecDeque<NodeId>;

    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl std::ops::DerefMut for PooledNodeQueue<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queue
    }
}

impl Drop for PooledNodeQueue<'_> {
    fn drop(&mut self) {
        let queue = std::mem::take(&mut self.queue);
        self.pool.release(queue);
    }
}

thread_local! {
    static DEFAULT_SET_POOL: NodeSetPool = NodeSetPool::new(16);
    static DEFAULT_MAP_POOL: NodeMapPool<f64> = NodeMapPool::new(16);
    static DEFAULT_QUEUE_POOL: NodeQueuePool = NodeQueuePool::new(16);
}

/// Acquires a pooled set from the default thread-local pool.
///
/// Global default pools can be used throughout the library to reduce allocation overhead
/// without requiring explicit pool management.
pub fn acquire_node_set() -> PooledNodeSet<'static> {
    DEFAULT_SET_POOL.with(|pool| unsafe {
        // SAFETY: We're extending the lifetime of the pool reference to 'static.
        // This is safe because the pool is thread-local and will live as long as the thread.
        let pool_ref: &'static NodeSetPool = std::mem::transmute(pool);
        pool_ref.acquire()
    })
}

/// Acquires a pooled map from the default thread-local pool.
pub fn acquire_node_map() -> PooledNodeMap<'static, f64> {
    DEFAULT_MAP_POOL.with(|pool| unsafe {
        let pool_ref: &'static NodeMapPool<f64> = std::mem::transmute(pool);
        pool_ref.acquire()
    })
}

/// Acquires a pooled queue from the default thread-local pool.
pub fn acquire_node_queue() -> PooledNodeQueue<'static> {
    DEFAULT_QUEUE_POOL.with(|pool| unsafe {
        let pool_ref: &'static NodeQueuePool = std::mem::transmute(pool);
        pool_ref.acquire()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_set_pool() {
        let pool = NodeSetPool::new(5);
        assert_eq!(pool.available(), 0);

        {
            let mut set1 = pool.acquire();
            set1.insert(NodeId::new(petgraph::graph::NodeIndex::new(1)));
            assert_eq!(set1.len(), 1);
        }

        // After dropping, the set should be returned to the pool
        assert_eq!(pool.available(), 1);

        {
            let set2 = pool.acquire();
            // The set should be cleared when acquired again
            assert_eq!(set2.len(), 0);
        }
    }

    #[test]
    fn test_node_map_pool() {
        let pool = NodeMapPool::<i32>::new(5);

        {
            let mut map = pool.acquire();
            map.insert(NodeId::new(petgraph::graph::NodeIndex::new(1)), 42);
            assert_eq!(map.len(), 1);
        }

        assert_eq!(pool.available(), 1);

        {
            let map = pool.acquire();
            assert_eq!(map.len(), 0);
        }
    }

    #[test]
    fn test_node_queue_pool() {
        let pool = NodeQueuePool::new(5);

        {
            let mut queue = pool.acquire();
            queue.push_back(NodeId::new(petgraph::graph::NodeIndex::new(1)));
            assert_eq!(queue.len(), 1);
        }

        assert_eq!(pool.available(), 1);

        {
            let queue = pool.acquire();
            assert_eq!(queue.len(), 0);
        }
    }

    #[test]
    fn test_pool_max_size() {
        let pool = NodeSetPool::new(2);

        // Create 3 sets and keep them alive, then drop them
        {
            let _set1 = pool.acquire();
            let _set2 = pool.acquire();
            let _set3 = pool.acquire();
            // All 3 are alive here, pool is empty
            assert_eq!(pool.available(), 0);
        }
        // Now all 3 are dropped, but pool should only keep 2 (max_size)

        // Pool should only hold 2 sets (the max size), the 3rd one was discarded
        assert_eq!(pool.available(), 2);
    }

    #[test]
    fn test_multiple_concurrent_acquisitions() {
        let pool = NodeSetPool::new(5);

        let set1 = pool.acquire();
        let set2 = pool.acquire();
        let set3 = pool.acquire();

        assert_eq!(pool.available(), 0);

        drop(set1);
        assert_eq!(pool.available(), 1);

        drop(set2);
        drop(set3);
        assert_eq!(pool.available(), 3);
    }

    #[test]
    fn test_default_pools() {
        // Test that default pools work
        {
            let mut set = acquire_node_set();
            set.insert(NodeId::new(petgraph::graph::NodeIndex::new(1)));
            assert_eq!(set.len(), 1);
        }

        {
            let mut map = acquire_node_map();
            map.insert(NodeId::new(petgraph::graph::NodeIndex::new(1)), 3.14);
            assert_eq!(map.len(), 1);
        }

        {
            let mut queue = acquire_node_queue();
            queue.push_back(NodeId::new(petgraph::graph::NodeIndex::new(1)));
            assert_eq!(queue.len(), 1);
        }
    }
}
