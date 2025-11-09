//! Centrality algorithms facade.
//!
//! Convention: centrality routines in this crate generally return `Result<_, crate::core::error::GraphinaError>`
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
pub mod personalized;
pub use crate::community::personalized_pagerank::personalized_page_rank as personalized_pagerank_vec;
pub mod community_wrappers;
pub use community_wrappers::{infomap_map, label_propagation_map};
