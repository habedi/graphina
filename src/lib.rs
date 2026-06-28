#![cfg_attr(docsrs, feature(doc_cfg))]
/*!
# Graphina Crate

A graph data science library that provides common graph types, algorithms, and data structures.

## Module Overview

* `core` – Always enabled: basic graph types, builders, IO, serialization, paths, validation.
* `centrality` *(feature: centrality)* – Node/edge importance measures (Result-based APIs).
* `community` *(feature: community)* – Community detection and clustering (Result-based APIs).
* `links` *(feature: links)* – Link prediction algorithms.
* `metrics` *(feature: metrics)* – Graph and node metrics (diameter, radius, clustering, etc.).
* `mst` *(feature: mst)* – Minimum spanning tree algorithms.
* `traversal` *(feature: traversal)* – BFS/DFS and related traversal strategies.
* `approximation` *(feature: approximation)* – Heuristics for NP-hard problems.
* `parallel` *(feature: parallel)* – Parallel implementations for selected algorithms.
* `subgraphs` *(feature: subgraphs)* – Induced subgraph and ego network utilities.
* `core::pool` *(feature: pool)* – Experimental memory pooling utilities.

## API Conventions

Algorithms return `Result<_, graphina::core::error::GraphinaError>` for error handling.
Selector-style helpers that pick nodes (like `voterank`) may return plain collections.

Enable only required features to minimize size and compile time.

## Stability Notes

The `pool` feature is experimental; API subject to change. Gate usage with `cfg(feature = "pool")`.
*/

/// Approximation algorithms for NP-hard problems.
#[cfg(feature = "approximation")]
pub mod approximation;
#[cfg(feature = "centrality")]
pub mod centrality;
/// Community detection and clustering algorithms.
#[cfg(feature = "community")]
pub mod community;
/// Core graph types and utilities.
pub mod core;
/// Link prediction algorithms.
#[cfg(feature = "links")]
pub mod links;
/// Graph metrics and metrics-based algorithms.
#[cfg(feature = "metrics")]
pub mod metrics;
/// Minimum spanning tree algorithms.
#[cfg(feature = "mst")]
pub mod mst;
/// Parallel implementations of algorithms.
#[cfg(feature = "parallel")]
pub mod parallel;
/// Logging configuration.
#[cfg(feature = "logging")]
mod settings;
/// Induced subgraph and ego network utilities.
#[cfg(feature = "subgraphs")]
pub mod subgraphs;
/// Graph traversal algorithms.
#[cfg(feature = "traversal")]
pub mod traversal;
