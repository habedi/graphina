//! Graph metrics module.
//!
//! This module provides graph-level and node-level metrics for network analysis.
//! All metrics depend only on the core module for basic graph operations.

pub mod graph_metrics;
pub mod node_metrics;

// Re-export all public functions
pub use graph_metrics::{
    assortativity, average_clustering_coefficient, average_path_length, diameter, radius,
    transitivity,
};
pub use node_metrics::{clustering_coefficient, triangles};
