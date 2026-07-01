/*!
# Parallel Algorithms Extension

This extension provides parallel implementations of computationally intensive graph algorithms
using Rayon for multi-threading. These implementations can provide 4-8x speedup on multi-core machines.

All parallel functions have the `_parallel` suffix to distinguish them from sequential versions.

Independent of other extensions; depends only on core.
*/

pub mod bfs;
pub mod closeness;
pub mod clustering;
pub mod components;
pub mod degrees;
pub mod pagerank;
pub mod paths;
pub mod triangles;

// Re-export main functions for convenience
pub use bfs::bfs_parallel;
pub use closeness::closeness_centrality_parallel;
pub use clustering::clustering_coefficients_parallel;
pub use components::connected_components_parallel;
pub use degrees::degrees_parallel;
pub use pagerank::pagerank_parallel;
pub use paths::{all_pairs_shortest_path_length_parallel, shortest_paths_parallel};
pub use triangles::triangles_parallel;
