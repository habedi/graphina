//! Centrality algorithms facade.
//!
//! Convention: centrality routines in this crate generally return `Result<_, crate::core::exceptions::GraphinaException>`
//! to surface convergence issues, empty-graph cases, or invalid inputs in a structured way for
//! better observability and error propagation. Selector-style routines that do not produce a
//! numeric map (for example, community seed pickers) may return plain values instead.
//!

pub mod betweenness;
pub mod closeness;
pub mod degree;
pub mod eigenvector;
pub mod harmonic;
pub mod katz;
pub mod other;
pub mod pagerank;
