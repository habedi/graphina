#![cfg_attr(docsrs, feature(doc_cfg))]
/*!
# Graphina

A graph data science library for Rust with a high-level API.

## Extension (or Module) Overview

* `core` – Core graph types, builders, IO, serialization, etc. (`core` is always enabled.)
* `centrality` *(feature: centrality)* – Node and edge importance measures (Result-based APIs).
* `community` *(feature: community)* – Community detection and clustering (Result-based APIs).
* `links` *(feature: links)* – Link prediction algorithms.
* `metrics` *(feature: metrics)* – Graph and node metrics (like diameter, radius, etc.).
* `mst` *(feature: mst)* – Minimum spanning tree algorithms.
* `traversal` *(feature: traversal)* – BFS, DFS, and related traversal algorithms.
* `approximation` *(feature: approximation)* – Algorithms for computationally hard graph problems.
* `parallel` *(feature: parallel)* – Parallel implementations for a subset of algorithms.
* `subgraphs` *(feature: subgraphs)* – Algorithms for induced subgraph and ego network.

## API Conventions

Most algorithms return `Result<_, graphina::core::error::GraphinaError>` for error handling.

Enable only required features to minimize binary size and compile time.
*/

/// Algorithms for computationally hard graph problems.
#[cfg(feature = "approximation")]
pub mod approximation;
/// Node and edge centrality algorithms.
#[cfg(feature = "centrality")]
pub mod centrality;
/// Community detection and graph clustering algorithms.
#[cfg(feature = "community")]
pub mod community;
/// Core graph data types and utilities.
pub mod core;
/// Link prediction algorithms.
#[cfg(feature = "links")]
pub mod links;
/// Graph metrics and related algorithms.
#[cfg(feature = "metrics")]
pub mod metrics;
/// Minimum spanning tree algorithms.
#[cfg(feature = "mst")]
pub mod mst;
/// Parallel implementations of some of the algorithms.
#[cfg(feature = "parallel")]
pub mod parallel;
/// Algorithms related to induced subgraph and ego network.
#[cfg(feature = "subgraphs")]
pub mod subgraphs;
/// Graph traversal algorithms.
#[cfg(feature = "traversal")]
pub mod traversal;
