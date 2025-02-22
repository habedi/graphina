/*!
# Input/Output (I/O) Routines

This module provides functions to read from and write to files containing graph representations.
Specifically, it supports:

- **Edge List I/O:**
  - Reading an edge list from a file into a graph.
  - Writing a graph's edge list to a file.

- **Adjacency List I/O:**
  - Reading an adjacency list from a file into a graph.
  - Writing a graph's adjacency list to a file.

Functions use the core graph abstraction defined in [`crate::core::types`](../core/types.rs)
and use the custom exception [`GraphinaException`](../exceptions/index.html#graphinaexception) for reporting errors.

The input files support comments (lines or inline comments beginning with `#` are ignored)
and allow for optional weight specifications. If a weight is missing, a default of `1.0` is used.
*/

use crate::core::exceptions::GraphinaException;
use crate::core::types::{BaseGraph, GraphConstructor};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Result, Write};

/// Reads an edge list from a file and populates the given graph.
///
/// Lines containing a `#` are treated as comments. Everything after the first `#` in a line is ignored.
/// Each non-comment, non-empty line must contain at least two tokens (source and target).
/// An optional third token represents the weight (default is `1.0`).
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the edge list file.
/// * `graph` - A mutable reference to the graph to be populated.
/// * `sep` - A character that separates the values in the edge list.
///
/// # Type Parameters
///
/// * `Ty` - The edge type of the graph, which must implement `GraphConstructor`.
///
/// # Returns
///
/// * `Result<()>` - An `io::Result` indicating success or failure. Failure can occur due to I/O errors,
///   or if any token fails to parse, in which case a [`GraphinaException`](../exceptions/index.html#graphinaexception)
///   is returned as part of the error message.
///
/// # Example
///
/// ```rust,no_run
/// use graphina::core::types::{Graph, NodeId};
/// use graphina::core::io::read_edge_list;
///
/// let mut graph = Graph::<i32, f32>::new();
/// // Assume "edges.txt" exists and follows the correct format.
/// read_edge_list("edges.txt", &mut graph, ',').expect("Failed to read edge list");
/// ```
pub fn read_edge_list<Ty>(path: &str, graph: &mut BaseGraph<i32, f32, Ty>, sep: char) -> Result<()>
where
    Ty: GraphConstructor<i32, f32>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut node_map = HashMap::new();
    for line in reader.lines() {
        let mut line = line?;
        // Remove comments: if '#' is present, only take text before it.
        if let Some(idx) = line.find('#') {
            line.truncate(idx);
        }
        // Skip if the line is empty after removing comments.
        if line.trim().is_empty() {
            continue;
        }
        let tokens: Vec<&str> = line.trim().split(sep).map(|s| s.trim()).collect();
        if tokens.len() < 2 {
            continue;
        }
        let src_val: i32 = tokens[0].parse().map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                GraphinaException::new(&format!(
                    "Error parsing source value '{}': {}",
                    tokens[0], e
                )),
            )
        })?;
        let tgt_val: i32 = tokens[1].parse().map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                GraphinaException::new(&format!(
                    "Error parsing target value '{}': {}",
                    tokens[1], e
                )),
            )
        })?;
        let weight: f32 = if tokens.len() >= 3 {
            tokens[2].parse().map_err(|e| {
                Error::new(
                    ErrorKind::InvalidData,
                    GraphinaException::new(&format!("Error parsing weight '{}': {}", tokens[2], e)),
                )
            })?
        } else {
            1.0
        };
        let src_node = *node_map
            .entry(src_val)
            .or_insert_with(|| graph.add_node(src_val));
        let tgt_node = *node_map
            .entry(tgt_val)
            .or_insert_with(|| graph.add_node(tgt_val));
        graph.add_edge(src_node, tgt_node, weight);
    }
    Ok(())
}

/// Writes the edge list of a graph to a file.
///
/// Each line in the output file will contain the source attribute, target attribute, and weight,
/// separated by the provided separator.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the output file.
/// * `graph` - A reference to the graph to be written.
/// * `sep` - A character that separates the values in the output file.
///
/// # Type Parameters
///
/// * `Ty` - The edge type of the graph, which must implement `GraphConstructor`.
///
/// # Returns
///
/// * `Result<()>` - An `io::Result` indicating success or failure. Failure occurs if a node attribute is missing
///   (triggering a [`GraphinaException`](../exceptions/index.html#graphinaexception)) or if writing to the file fails.
///
/// # Example
///
/// ```rust,no_run
/// use graphina::core::types::{Graph, NodeId};
/// use graphina::core::io::write_edge_list;
///
/// let mut graph = Graph::<i32, f32>::new();
/// let n1 = graph.add_node(1);
/// let n2 = graph.add_node(2);
/// graph.add_edge(n1, n2, 3.5);
/// write_edge_list("output_edges.txt", &graph, ',').expect("Failed to write edge list");
/// ```
pub fn write_edge_list<Ty>(path: &str, graph: &BaseGraph<i32, f32, Ty>, sep: char) -> Result<()>
where
    Ty: GraphConstructor<i32, f32>,
{
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for (src, tgt, weight) in graph.edges() {
        let src_attr = graph.node_attr(src).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                GraphinaException::new(&format!(
                    "Missing node attribute for source node: {:?}",
                    src
                )),
            )
        })?;
        let tgt_attr = graph.node_attr(tgt).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                GraphinaException::new(&format!(
                    "Missing node attribute for target node: {:?}",
                    tgt
                )),
            )
        })?;
        writeln!(writer, "{}{}{}{}{}", src_attr, sep, tgt_attr, sep, weight)?;
    }
    writer.flush()?;
    Ok(())
}

/// Reads an adjacency list from a file and populates the given graph.
///
/// Each non-empty line is expected to be in the following format:
///
/// ```text
/// <node><sep><neighbor1><sep><weight1><sep><neighbor2><sep><weight2>...
/// ```
///
/// Lines containing a `#` are treated as comments. Everything after the first `#` in a line is ignored.
/// If a weight is missing for a neighbor, a default weight of `1.0` is used.
///
/// # Arguments
///
/// * `path` - A string slice holding the path to the adjacency list file.
/// * `graph` - A mutable reference to the graph to be populated.
/// * `sep` - A character that separates the tokens in the file.
///
/// # Type Parameters
///
/// * `Ty` - The edge type of the graph, which must implement `GraphConstructor`.
///
/// # Returns
///
/// * `Result<()>` - An `io::Result` indicating success or failure. Parsing errors are returned if any token
///   fails to convert to the expected type. In such cases, a [`GraphinaException`](../exceptions/index.html#graphinaexception)
///   is used to encapsulate the error.
///
/// # Example
///
/// ```rust,no_run
/// use graphina::core::types::{Graph, NodeId};
/// use graphina::core::io::read_adjacency_list;
///
/// let mut graph = Graph::<i32, f32>::new();
/// // Assume "adj_list.txt" exists and follows the correct format.
/// read_adjacency_list("adj_list.txt", &mut graph, ' ').expect("Failed to read adjacency list");
/// ```
pub fn read_adjacency_list<Ty>(
    path: &str,
    graph: &mut BaseGraph<i32, f32, Ty>,
    sep: char,
) -> Result<()>
where
    Ty: GraphConstructor<i32, f32>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut node_map = HashMap::new();
    for line in reader.lines() {
        let mut line = line?;
        // Remove comments: if '#' is present, only take text before it.
        if let Some(idx) = line.find('#') {
            line.truncate(idx);
        }
        if line.trim().is_empty() {
            continue;
        }
        let tokens: Vec<&str> = line.trim().split(sep).map(|s| s.trim()).collect();
        if tokens.is_empty() {
            continue;
        }
        // The first token is the source node attribute.
        let src_val: i32 = tokens[0].parse().map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                GraphinaException::new(&format!(
                    "Error parsing source value '{}': {}",
                    tokens[0], e
                )),
            )
        })?;
        let src_node = *node_map
            .entry(src_val)
            .or_insert_with(|| graph.add_node(src_val));
        // Process subsequent tokens in pairs: neighbor and weight.
        let mut i = 1;
        while i < tokens.len() {
            let neighbor_val: i32 = tokens[i].parse().map_err(|e| {
                Error::new(
                    ErrorKind::InvalidData,
                    GraphinaException::new(&format!(
                        "Error parsing neighbor value '{}': {}",
                        tokens[i], e
                    )),
                )
            })?;
            let weight: f32 = if i + 1 < tokens.len() {
                tokens[i + 1].parse().map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidData,
                        GraphinaException::new(&format!(
                            "Error parsing weight '{}': {}",
                            tokens[i + 1],
                            e
                        )),
                    )
                })?
            } else {
                1.0
            };
            let neighbor_node = *node_map
                .entry(neighbor_val)
                .or_insert_with(|| graph.add_node(neighbor_val));
            graph.add_edge(src_node, neighbor_node, weight);
            i += 2;
        }
    }
    Ok(())
}

/// Writes the adjacency list of a graph to a file.
///
/// Each line in the output file will be in the following format:
///
/// ```text
/// <node><sep><neighbor1>:<weight1><sep><neighbor2>:<weight2>...
/// ```
///
/// # Arguments
///
/// * `path` - A string slice holding the path to the output file.
/// * `graph` - A reference to the graph to be written.
/// * `sep` - A character that separates the tokens in the output file.
///
/// # Type Parameters
///
/// * `Ty` - The edge type of the graph, which must implement `GraphConstructor`.
///
/// # Returns
///
/// * `Result<()>` - An `io::Result` indicating success or failure. An error is returned if any node attribute
///   is missing (with a [`GraphinaException`](../exceptions/index.html#graphinaexception)) or if writing to the file fails.
///
/// # Example
///
/// ```rust,no_run
/// use graphina::core::types::{Graph, NodeId};
/// use graphina::core::io::write_adjacency_list;
///
/// let mut graph = Graph::<i32, f32>::new();
/// let n1 = graph.add_node(1);
/// let n2 = graph.add_node(2);
/// graph.add_edge(n1, n2, 2.5);
/// write_adjacency_list("output_adj.txt", &graph, ' ').expect("Failed to write adjacency list");
/// ```
pub fn write_adjacency_list<Ty>(
    path: &str,
    graph: &BaseGraph<i32, f32, Ty>,
    sep: char,
) -> Result<()>
where
    Ty: GraphConstructor<i32, f32>,
{
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    // Build a mapping from each source node (by its attribute) to its neighbors.
    let mut adj_map: HashMap<i32, Vec<(i32, f32)>> = HashMap::new();
    for (src, tgt, weight) in graph.edges() {
        let src_attr = graph.node_attr(src).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                GraphinaException::new(&format!(
                    "Missing node attribute for source node: {:?}",
                    src
                )),
            )
        })?;
        let tgt_attr = graph.node_attr(tgt).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                GraphinaException::new(&format!(
                    "Missing node attribute for target node: {:?}",
                    tgt
                )),
            )
        })?;
        adj_map
            .entry(*src_attr)
            .or_default()
            .push((*tgt_attr, *weight));
    }
    // Write each node and its neighbors.
    for (_, attr) in graph.nodes() {
        write!(writer, "{}", attr)?;
        if let Some(neighbors) = adj_map.get(attr) {
            for (nbr, weight) in neighbors {
                // Write neighbor and weight separated by a colon.
                write!(writer, "{}{}:{}", sep, nbr, weight)?;
            }
        }
        writeln!(writer)?;
    }
    writer.flush()?;
    Ok(())
}
