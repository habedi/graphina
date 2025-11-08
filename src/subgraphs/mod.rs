//! Subgraph operations module.
//!
//! This module provides operations for extracting and working with subgraphs.
//! All operations depend only on the core module for basic graph operations.

pub mod operations;

// Re-export subgraph operations as extension methods
pub use operations::SubgraphOps;
