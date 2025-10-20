//! Graph traversal algorithms module.
//!
//! This module provides various graph traversal algorithms including BFS, DFS, IDDFS, and bidirectional search.
//! All algorithms depend only on the core module for basic graph operations.

pub mod algorithms;

// Re-export commonly used functions
pub use algorithms::{bfs, bidis, dfs, iddfs, try_bidirectional_search, try_iddfs};
