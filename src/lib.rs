/*!
# Graphina Crate

High-level graph data science library providing graph types, algorithms, and analytics.

## Module Overview

* `core` – always enabled: basic graph types, builders, IO, serialization, paths, validation.
* `centrality` *(feature: centrality)* – node/edge importance measures (Result-based APIs).
* `community` *(feature: community)* – community detection & clustering (Result-based APIs).
* `links` *(feature: links)* – link prediction algorithms.
* `metrics` *(feature: metrics)* – graph and node metrics (diameter, radius, clustering, etc.).
* `mst` *(feature: mst)* – minimum spanning tree algorithms.
* `traversal` *(feature: traversal)* – BFS/DFS and related traversal strategies.
* `approximation` *(feature: approximation)* – heuristics for NP-hard problems.
* `parallel` *(feature: parallel)* – parallel implementations for selected algorithms.
* `subgraphs` *(feature: subgraphs)* – induced subgraph and ego network utilities.
* `visualization` *(feature: visualization)* – layouts and render helpers (ASCII/HTML/SVG/PNG).
* `core::pool` *(feature: pool)* – experimental memory pooling utilities (subject to change).

## API Conventions

Most algorithm functions return `Result<_, graphina::core::error::GraphinaError>` for robust error handling.
Selector-style helpers that simply pick nodes (e.g. `voterank`) may return plain collections.

Enable only the features you need to keep compile times and dependency footprint low.

## Stability Notes

The `pool` feature is experimental; its public API may change. Gate usage with `cfg(feature = "pool")` if you rely on it.
*/

#[cfg(feature = "approximation")]
pub mod approximation;
#[cfg(feature = "centrality")]
pub mod centrality;
#[cfg(feature = "community")]
pub mod community;
pub mod core;
#[cfg(feature = "links")]
pub mod links;
#[cfg(feature = "metrics")]
pub mod metrics;
#[cfg(feature = "mst")]
pub mod mst;
#[cfg(feature = "parallel")]
pub mod parallel;
#[cfg(feature = "logging")]
mod settings;
#[cfg(feature = "subgraphs")]
pub mod subgraphs;
#[cfg(feature = "traversal")]
pub mod traversal;
#[cfg(feature = "visualization")]
pub mod visualization;
