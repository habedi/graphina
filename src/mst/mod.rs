//! Minimum Spanning Tree algorithms module.
//!
//! This module provides various algorithms for computing Minimum Spanning Trees (MST).
//! All algorithms depend only on the core module for basic graph operations.

pub mod algorithms;

// Re-export all public items
pub use algorithms::{MstEdge, boruvka_mst, kruskal_mst, prim_mst};
