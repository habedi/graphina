//! Subgraph operations module.
//!
//! Extract and manipulate subgraphs.
//! All operations depend only on the core module for basic graph operations.

pub mod operations;

// Re-export subgraph operations as extension methods
pub use operations::SubgraphOps;
