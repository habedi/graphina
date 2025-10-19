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

Functions use the core graph abstractions defined in `graphina::core::types` and report errors using
`graphina::core::error::GraphinaError` where appropriate.

The input files support comments (lines or inline comments beginning with `#` are ignored)
and allow for optional weight specifications. If a weight is missing, a default of `1.0` is used.
*/

use crate::core::types::{BaseGraph, GraphConstructor};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Write};

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
/// * `W` - The weight type of the graph, which must implement `FromStr` and `Copy`.
/// * `Ty` - The edge type of the graph, which must implement `GraphConstructor`.
///
/// # Returns
///
/// * `Result<()>` - An `io::Result` indicating success or failure. Failure can occur due to I/O errors,
///   or if any token fails to parse; in such cases, the error message will include details suitable for debugging.
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
pub fn read_edge_list<W, Ty>(
    path: &str,
    graph: &mut BaseGraph<i32, W, Ty>,
    sep: char,
) -> std::io::Result<()>
where
    W: Copy + std::str::FromStr,
    <W as std::str::FromStr>::Err: std::fmt::Display + std::fmt::Debug,
    Ty: GraphConstructor<i32, W>,
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
                format!("Error parsing source value '{}': {}", tokens[0], e),
            )
        })?;
        let tgt_val: i32 = tokens[1].parse().map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Error parsing target value '{}': {}", tokens[1], e),
            )
        })?;
        let weight: W = if tokens.len() >= 3 {
            tokens[2].parse().map_err(|e| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Error parsing weight '{}': {}", tokens[2], e),
                )
            })?
        } else {
            "1.0".parse().unwrap()
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
///   or if writing to the file fails.
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
pub fn write_edge_list<Ty>(
    path: &str,
    graph: &BaseGraph<i32, f32, Ty>,
    sep: char,
) -> std::io::Result<()>
where
    Ty: GraphConstructor<i32, f32>,
{
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for (src, tgt, weight) in graph.edges() {
        let src_attr = graph.node_attr(src).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Missing node attribute for source node: {:?}", src),
            )
        })?;
        let tgt_attr = graph.node_attr(tgt).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Missing node attribute for target node: {:?}", tgt),
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
///   fails to convert to the expected type.
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
) -> std::io::Result<()>
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
                format!("Error parsing source value '{}': {}", tokens[0], e),
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
                    format!("Error parsing neighbor value '{}': {}", tokens[i], e),
                )
            })?;
            let weight: f32 = if i + 1 < tokens.len() {
                tokens[i + 1].parse().map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidData,
                        format!("Error parsing weight '{}': {}", tokens[i + 1], e),
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
///   is missing or if writing to the file fails.
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
) -> std::io::Result<()>
where
    Ty: GraphConstructor<i32, f32>,
{
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let mut adj_map: HashMap<i32, Vec<(i32, f32)>> = HashMap::new();
    for (src, tgt, weight) in graph.edges() {
        let src_attr = graph.node_attr(src).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Missing node attribute for source node: {:?}", src),
            )
        })?;
        let tgt_attr = graph.node_attr(tgt).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Missing node attribute for target node: {:?}", tgt),
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;
    use std::fs;
    use std::io::Read;
    #[test]
    fn test_read_edge_list() {
        let tmp_path = "tmp_edge_list.txt";
        let edge_list = "\
# This is a comment line and should be ignored
1,2,1.5
2,3,2.0
3,1,3.0  # Comment after data should be ignored
";
        fs::write(tmp_path, edge_list).expect("Unable to write temporary file");
        let mut graph = Graph::<i32, f32>::new();
        read_edge_list(tmp_path, &mut graph, ',').expect("read_edge_list failed");
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 3);
        fs::remove_file(tmp_path).expect("Failed to remove temporary file");
    }
    #[test]
    fn test_write_edge_list() {
        let mut graph = Graph::<i32, f32>::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, 1.5);
        graph.add_edge(n2, n3, 2.0);
        graph.add_edge(n3, n1, 3.0);
        let tmp_path = "tmp_edge_list_out.txt";
        write_edge_list(tmp_path, &graph, ',').expect("write_edge_list failed");
        let mut content = String::new();
        fs::File::open(tmp_path)
            .expect("Failed to open output file")
            .read_to_string(&mut content)
            .expect("Failed to read output file");
        assert!(content.contains("1,2,1.5") || content.contains("2,1,1.5"));
        assert!(content.contains("2,3,2") || content.contains("3,2,2"));
        assert!(content.contains("3,1,3") || content.contains("1,3,3"));
        fs::remove_file(tmp_path).expect("Failed to remove temporary file");
    }
    #[test]
    fn test_read_adjacency_list() {
        let tmp_path = "tmp_adj_list.txt";
        let adj_list = "\
# Adjacency list with comments
1,2,1.5,3,2.5
2,3,2.0
3
";
        fs::write(tmp_path, adj_list).expect("Unable to write temporary file");
        let mut graph = Graph::<i32, f32>::new();
        read_adjacency_list(tmp_path, &mut graph, ',').expect("read_adjacency_list failed");
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 3);
        fs::remove_file(tmp_path).expect("Failed to remove temporary file");
    }
    #[test]
    fn test_write_adjacency_list() {
        let mut graph = Graph::<i32, f32>::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, 1.5);
        graph.add_edge(n1, n3, 2.5);
        graph.add_edge(n2, n3, 2.0);
        let tmp_path = "tmp_adj_list_out.txt";
        write_adjacency_list(tmp_path, &graph, ',').expect("write_adjacency_list failed");
        let mut content = String::new();
        fs::File::open(tmp_path)
            .expect("Failed to open output file")
            .read_to_string(&mut content)
            .expect("Failed to read output file");
        assert!(!content.is_empty());
        fs::remove_file(tmp_path).expect("Failed to remove temporary file");
    }
}
