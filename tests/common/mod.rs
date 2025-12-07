//! Shared test utilities for Graphina integration tests.
//!
//! This module provides common helpers, macros, and constants used across
//! multiple test files to avoid duplication while keeping tests decoupled.

#![allow(dead_code)]

use graphina::core::io::read_edge_list;
use graphina::core::types::{Digraph, Graph};
use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::path::Path;

/// Dataset metadata for end-to-end testing.
#[derive(Debug, Clone)]
pub struct DatasetInfo {
    pub name: &'static str,
    pub file: &'static str,
    pub is_directed: bool,
    pub min_nodes: usize,
    pub min_edges: usize,
}

/// Available datasets for testing.
pub const DATASETS: &[DatasetInfo] = &[
    DatasetInfo {
        name: "Wikipedia Chameleon",
        file: "wikipedia_chameleon.txt",
        is_directed: false,
        min_nodes: 2000,
        min_edges: 30000,
    },
    DatasetInfo {
        name: "Wikipedia Squirrel",
        file: "wikipedia_squirrel.txt",
        is_directed: false,
        min_nodes: 5000,
        min_edges: 190000,
    },
    DatasetInfo {
        name: "Wikipedia Crocodile",
        file: "wikipedia_crocodile.txt",
        is_directed: false,
        min_nodes: 11000,
        min_edges: 170000,
    },
    DatasetInfo {
        name: "Facebook Page-Page",
        file: "facebook_page_page.txt",
        is_directed: false,
        min_nodes: 22000,
        min_edges: 170000,
    },
    DatasetInfo {
        name: "Stanford Web Graph",
        file: "stanford_web_graph.txt",
        is_directed: true,
        min_nodes: 280000,
        min_edges: 2300000,
    },
    DatasetInfo {
        name: "DBLP Citation Network",
        file: "dblp_citation_network.txt",
        is_directed: false,
        min_nodes: 317000,
        min_edges: 1049000,
    },
];

/// Path to the test datasets directory.
pub const DATASETS_DIR: &str = "tests/testdata/graphina-graphs";

/// Check if the test datasets are available.
pub fn datasets_available() -> bool {
    Path::new(DATASETS_DIR).exists()
}

/// Macro to skip a test if datasets are not available.
/// Prints a helpful message about how to download the datasets.
macro_rules! skip_if_no_datasets {
    () => {
        if !common::datasets_available() {
            eprintln!("Skipping test: datasets not found in {}", common::DATASETS_DIR);
            eprintln!("   Run: huggingface-cli download habedi/graphina-graphs --repo-type dataset --local-dir tests/testdata/graphina-graphs");
            return;
        }
    };
}

pub(crate) use skip_if_no_datasets;

/// Load an undirected graph with f32 edge weights from a dataset file.
pub fn load_undirected_graph_f32(filename: &str) -> Result<Graph<i32, f32>, std::io::Error> {
    let path = format!("{}/{}", DATASETS_DIR, filename);
    let mut graph = Graph::new();
    read_edge_list(&path, &mut graph, ' ')?;
    Ok(graph)
}

/// Load an undirected graph with OrderedFloat<f64> edge weights from a dataset file.
pub fn load_undirected_graph_f64(
    filename: &str,
) -> Result<Graph<i32, OrderedFloat<f64>>, std::io::Error> {
    let graph_f32 = load_undirected_graph_f32(filename)?;
    Ok(convert_graph_to_ordered_f64(&graph_f32))
}

/// Load an undirected graph with f64 edge weights from a dataset file.
pub fn load_undirected_graph_plain_f64(filename: &str) -> Result<Graph<i32, f64>, std::io::Error> {
    let graph_f32 = load_undirected_graph_f32(filename)?;
    Ok(convert_graph_to_f64(&graph_f32))
}

/// Load a directed graph with f32 edge weights from a dataset file.
pub fn load_directed_graph_f32(filename: &str) -> Result<Digraph<i32, f32>, std::io::Error> {
    let path = format!("{}/{}", DATASETS_DIR, filename);
    let mut graph = Digraph::new();
    read_edge_list(&path, &mut graph, ' ')?;
    Ok(graph)
}

/// Convert a Graph<i32, f32> to Graph<i32, OrderedFloat<f64>>.
pub fn convert_graph_to_ordered_f64(graph_f32: &Graph<i32, f32>) -> Graph<i32, OrderedFloat<f64>> {
    let mut graph = Graph::new();
    let mut node_map = HashMap::new();

    for (node, attr) in graph_f32.nodes() {
        let new_node = graph.add_node(*attr);
        node_map.insert(node, new_node);
    }

    for (u, v, &w) in graph_f32.edges() {
        graph.add_edge(node_map[&u], node_map[&v], OrderedFloat(w as f64));
    }

    graph
}

/// Convert a Graph<i32, f32> to Graph<i32, f64>.
pub fn convert_graph_to_f64(graph_f32: &Graph<i32, f32>) -> Graph<i32, f64> {
    let mut graph = Graph::new();
    let mut node_map = HashMap::new();

    for (node, attr) in graph_f32.nodes() {
        let new_node = graph.add_node(*attr);
        node_map.insert(node, new_node);
    }

    for (u, v, &w) in graph_f32.edges() {
        graph.add_edge(node_map[&u], node_map[&v], w as f64);
    }

    graph
}
