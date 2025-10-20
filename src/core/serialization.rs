/*!
# Graph Serialization Module

This module provides serialization and deserialization support for graphs in various formats:
- JSON (human-readable, debugging)
- Binary (fast, compact)
- GraphML (interoperability with other tools)
- Edge list (simple text format)
*/

use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use bincode;
use serde::{Deserialize, Serialize};

use crate::core::error::GraphinaError;
use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use petgraph::EdgeType;

/// Serializable representation of a graph for JSON/binary formats.
///
/// This intermediate format allows serialization of graphs with any node/edge attributes
/// that implement Serialize + Deserialize.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableGraph<A, W> {
    /// Whether the graph is directed
    pub directed: bool,
    /// Node attributes indexed by their position
    pub nodes: Vec<A>,
    /// Edges as (source_index, target_index, weight) tuples
    pub edges: Vec<(usize, usize, W)>,
}

impl<A, W, Ty> BaseGraph<A, W, Ty>
where
    A: Clone + Serialize,
    W: Clone + Serialize,
    Ty: GraphConstructor<A, W> + EdgeType,
{
    /// Converts the graph to a serializable format.
    ///
    /// # Example
    ///
    /// ```rust
    /// use graphina::core::types::Graph;
    /// use graphina::core::serialization::SerializableGraph;
    ///
    /// let mut g = Graph::<i32, f64>::new();
    /// let n1 = g.add_node(1);
    /// let n2 = g.add_node(2);
    /// g.add_edge(n1, n2, 1.5);
    ///
    /// let serializable = g.to_serializable();
    /// assert_eq!(serializable.nodes.len(), 2);
    /// assert_eq!(serializable.edges.len(), 1);
    /// ```
    pub fn to_serializable(&self) -> SerializableGraph<A, W> {
        // Collect nodes and build index mapping
        let nodes: Vec<(NodeId, A)> = self.nodes().map(|(id, attr)| (id, attr.clone())).collect();
        let node_to_index: std::collections::HashMap<NodeId, usize> = nodes
            .iter()
            .enumerate()
            .map(|(idx, (node_id, _))| (*node_id, idx))
            .collect();

        let node_attrs: Vec<A> = nodes.into_iter().map(|(_, attr)| attr).collect();

        // Collect edges
        let edges: Vec<(usize, usize, W)> = self
            .edges()
            .map(|(src, tgt, weight)| {
                let src_idx = node_to_index[&src];
                let tgt_idx = node_to_index[&tgt];
                (src_idx, tgt_idx, weight.clone())
            })
            .collect();

        SerializableGraph {
            directed: self.is_directed(),
            nodes: node_attrs,
            edges,
        }
    }

    /// Creates a graph from a serializable representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use graphina::core::types::Graph;
    /// use graphina::core::serialization::SerializableGraph;
    ///
    /// let serializable = SerializableGraph {
    ///     directed: false,
    ///     nodes: vec![1, 2, 3],
    ///     edges: vec![(0, 1, 1.0), (1, 2, 2.0)],
    /// };
    ///
    /// let graph = Graph::<i32, f64>::from_serializable(&serializable);
    /// assert_eq!(graph.node_count(), 3);
    /// assert_eq!(graph.edge_count(), 2);
    /// ```
    pub fn from_serializable(data: &SerializableGraph<A, W>) -> Self {
        let mut graph = Self::with_capacity(data.nodes.len(), data.edges.len());

        // Add nodes
        let node_ids: Vec<NodeId> = data
            .nodes
            .iter()
            .map(|attr| graph.add_node(attr.clone()))
            .collect();

        // Add edges
        for (src_idx, tgt_idx, weight) in &data.edges {
            graph.add_edge(node_ids[*src_idx], node_ids[*tgt_idx], weight.clone());
        }

        graph
    }

    /// Creates a graph from a serializable representation, validating directedness.
    ///
    /// Returns an error if the `SerializableGraph.directed` flag does not match the
    /// target graph type `Ty` (Directed vs Undirected).
    pub fn try_from_serializable(data: &SerializableGraph<A, W>) -> Result<Self, GraphinaError> {
        if <Ty as GraphConstructor<A, W>>::is_directed() != data.directed {
            return Err(GraphinaError::InvalidGraph(
                "Directedness mismatch between SerializableGraph and target graph type".into(),
            ));
        }
        let mut graph = Self::with_capacity(data.nodes.len(), data.edges.len());
        let node_ids: Vec<NodeId> = data
            .nodes
            .iter()
            .map(|attr| graph.add_node(attr.clone()))
            .collect();
        for (src_idx, tgt_idx, weight) in &data.edges {
            graph.add_edge(node_ids[*src_idx], node_ids[*tgt_idx], weight.clone());
        }
        Ok(graph)
    }

    /// Saves the graph to a JSON file.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use graphina::core::types::Graph;
    ///
    /// let mut g = Graph::<i32, f64>::new();
    /// let n1 = g.add_node(1);
    /// let n2 = g.add_node(2);
    /// g.add_edge(n1, n2, 1.5);
    ///
    /// g.save_json("graph.json").expect("Failed to save");
    /// ```
    pub fn save_json<P: AsRef<Path>>(&self, path: P) -> Result<(), GraphinaError> {
        let serializable = self.to_serializable();
        let file = File::create(path).map_err(GraphinaError::from)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &serializable).map_err(GraphinaError::from)?;

        Ok(())
    }

    /// Loads a graph from a JSON file.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use graphina::core::types::Graph;
    ///
    /// let graph = Graph::<i32, f64>::load_json("graph.json")
    ///     .expect("Failed to load");
    /// ```
    pub fn load_json<P: AsRef<Path>>(path: P) -> Result<Self, GraphinaError>
    where
        A: for<'de> Deserialize<'de>,
        W: for<'de> Deserialize<'de>,
    {
        let file = File::open(path).map_err(GraphinaError::from)?;
        let reader = BufReader::new(file);

        let serializable: SerializableGraph<A, W> =
            serde_json::from_reader(reader).map_err(GraphinaError::from)?;

        Ok(Self::from_serializable(&serializable))
    }

    /// Loads a graph from a JSON file, validating directedness.
    pub fn load_json_strict<P: AsRef<Path>>(path: P) -> Result<Self, GraphinaError>
    where
        A: for<'de> Deserialize<'de>,
        W: for<'de> Deserialize<'de>,
    {
        let file = File::open(path).map_err(GraphinaError::from)?;
        let reader = BufReader::new(file);
        let serializable: SerializableGraph<A, W> =
            serde_json::from_reader(reader).map_err(GraphinaError::from)?;
        Self::try_from_serializable(&serializable)
    }

    /// Saves the graph to a binary file (using bincode).
    ///
    /// Binary format is much faster and more compact than JSON.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use graphina::core::types::Graph;
    ///
    /// let mut g = Graph::<i32, f64>::new();
    /// let n1 = g.add_node(1);
    /// let n2 = g.add_node(2);
    /// g.add_edge(n1, n2, 1.5);
    ///
    /// g.save_binary("graph.bin").expect("Failed to save");
    /// ```
    pub fn save_binary<P: AsRef<Path>>(&self, path: P) -> Result<(), GraphinaError> {
        let serializable = self.to_serializable();
        let file = File::create(path).map_err(GraphinaError::from)?;
        let mut writer = BufWriter::new(file);

        let encoded = bincode::serde::encode_to_vec(&serializable, bincode::config::standard())
            .map_err(GraphinaError::from)?;

        std::io::Write::write_all(&mut writer, &encoded).map_err(GraphinaError::from)?;

        Ok(())
    }

    /// Loads a graph from a binary file.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use graphina::core::types::Graph;
    ///
    /// let graph = Graph::<i32, f64>::load_binary("graph.bin")
    ///     .expect("Failed to load");
    /// ```
    pub fn load_binary<P: AsRef<Path>>(path: P) -> Result<Self, GraphinaError>
    where
        A: for<'de> Deserialize<'de>,
        W: for<'de> Deserialize<'de>,
    {
        let file = File::open(path).map_err(GraphinaError::from)?;
        let mut reader = BufReader::new(file);

        let mut buffer = Vec::new();
        std::io::Read::read_to_end(&mut reader, &mut buffer).map_err(GraphinaError::from)?;

        let (serializable, _): (SerializableGraph<A, W>, usize) =
            bincode::serde::decode_from_slice(&buffer, bincode::config::standard())
                .map_err(GraphinaError::from)?;

        Ok(Self::from_serializable(&serializable))
    }

    /// Loads a graph from a binary file, validating directedness.
    pub fn load_binary_strict<P: AsRef<Path>>(path: P) -> Result<Self, GraphinaError>
    where
        A: for<'de> Deserialize<'de>,
        W: for<'de> Deserialize<'de>,
    {
        let file = File::open(path).map_err(GraphinaError::from)?;
        let mut reader = BufReader::new(file);

        let mut buffer = Vec::new();
        std::io::Read::read_to_end(&mut reader, &mut buffer).map_err(GraphinaError::from)?;

        let (serializable, _): (SerializableGraph<A, W>, usize) =
            bincode::serde::decode_from_slice(&buffer, bincode::config::standard())
                .map_err(GraphinaError::from)?;

        Self::try_from_serializable(&serializable)
    }

    /// Saves the graph in GraphML format.
    ///
    /// GraphML is an XML-based format widely supported by graph visualization tools
    /// like Gephi, Cytoscape, and yEd.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use graphina::core::types::Graph;
    ///
    /// let mut g = Graph::<i32, f64>::new();
    /// let n1 = g.add_node(1);
    /// let n2 = g.add_node(2);
    /// g.add_edge(n1, n2, 1.5);
    ///
    /// g.save_graphml("graph.graphml").expect("Failed to save");
    /// ```
    pub fn save_graphml<P: AsRef<Path>>(&self, path: P) -> Result<(), GraphinaError>
    where
        A: std::fmt::Display,
        W: std::fmt::Display,
    {
        let file = File::create(path).map_err(GraphinaError::from)?;
        let mut writer = BufWriter::new(file);

        // Write GraphML header
        writeln!(writer, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")
            .map_err(GraphinaError::from)?;
        writeln!(
            writer,
            "<graphml xmlns=\"http://graphml.graphdrawing.org/xmlns\""
        )
        .map_err(GraphinaError::from)?;
        writeln!(
            writer,
            "         xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\""
        )
        .map_err(GraphinaError::from)?;
        writeln!(
            writer,
            "         xsi:schemaLocation=\"http://graphml.graphdrawing.org/xmlns"
        )
        .map_err(GraphinaError::from)?;
        writeln!(
            writer,
            "         http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd\">"
        )
        .map_err(GraphinaError::from)?;

        // Define attributes
        writeln!(
            writer,
            "  <key id=\"d0\" for=\"node\" attr.name=\"value\" attr.type=\"string\"/>"
        )
        .map_err(GraphinaError::from)?;
        writeln!(
            writer,
            "  <key id=\"d1\" for=\"edge\" attr.name=\"weight\" attr.type=\"double\"/>"
        )
        .map_err(GraphinaError::from)?;

        // Start graph
        let graph_type = if self.is_directed() {
            "directed"
        } else {
            "undirected"
        };
        writeln!(writer, "  <graph id=\"G\" edgedefault=\"{}\">", graph_type)
            .map_err(GraphinaError::from)?;

        // Write nodes
        let nodes: Vec<(NodeId, &A)> = self.nodes().collect();
        for (node_id, attr) in &nodes {
            writeln!(writer, "    <node id=\"n{}\">", node_id.index())
                .map_err(GraphinaError::from)?;
            writeln!(writer, "      <data key=\"d0\">{}</data>", attr)
                .map_err(GraphinaError::from)?;
            writeln!(writer, "    </node>").map_err(GraphinaError::from)?;
        }

        // Write edges
        for (edge_count, (src, tgt, weight)) in self.edges().enumerate() {
            writeln!(
                writer,
                "    <edge id=\"e{}\" source=\"n{}\" target=\"n{}\">",
                edge_count,
                src.index(),
                tgt.index()
            )
            .map_err(GraphinaError::from)?;
            writeln!(writer, "      <data key=\"d1\">{}</data>", weight)
                .map_err(GraphinaError::from)?;
            writeln!(writer, "    </edge>").map_err(GraphinaError::from)?;
        }

        // Close graph and graphml
        writeln!(writer, "  </graph>").map_err(GraphinaError::from)?;
        writeln!(writer, "</graphml>").map_err(GraphinaError::from)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{Digraph, Graph};
    use std::fs;

    #[test]
    fn test_to_serializable() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(n1, n2, 1.5);
        g.add_edge(n2, n3, 2.5);

        let serializable = g.to_serializable();
        assert_eq!(serializable.nodes.len(), 3);
        assert_eq!(serializable.edges.len(), 2);
        assert!(!serializable.directed);
    }

    #[test]
    fn test_from_serializable() {
        let serializable = SerializableGraph {
            directed: false,
            nodes: vec![10, 20, 30],
            edges: vec![(0, 1, 1.0), (1, 2, 2.0), (2, 0, 3.0)],
        };

        let graph = Graph::<i32, f64>::from_serializable(&serializable);
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 3);
    }

    #[test]
    fn test_json_roundtrip() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(100);
        let n2 = g.add_node(200);
        let n3 = g.add_node(300);
        g.add_edge(n1, n2, 1.5);
        g.add_edge(n2, n3, 2.5);

        let path = "test_graph.json";
        g.save_json(path).expect("Failed to save JSON");

        let loaded = Graph::<i32, f64>::load_json(path).expect("Failed to load JSON");
        assert_eq!(loaded.node_count(), 3);
        assert_eq!(loaded.edge_count(), 2);

        fs::remove_file(path).ok();
    }

    #[test]
    fn test_binary_roundtrip() {
        let mut g = Digraph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(n1, n2, 10.0);
        g.add_edge(n2, n3, 20.0);
        g.add_edge(n3, n1, 30.0);

        let path = "test_graph.bin";
        g.save_binary(path).expect("Failed to save binary");

        let loaded = Digraph::<i32, f64>::load_binary(path).expect("Failed to load binary");
        assert_eq!(loaded.node_count(), 3);
        assert_eq!(loaded.edge_count(), 3);
        assert!(loaded.is_directed());

        fs::remove_file(path).ok();
    }

    #[test]
    fn test_graphml_export() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n1, n2, 5.0);

        let path = "test_graph.graphml";
        g.save_graphml(path).expect("Failed to save GraphML");

        // Verify file was created and contains expected content
        let content = fs::read_to_string(path).expect("Failed to read file");
        assert!(content.contains("<?xml version"));
        assert!(content.contains("<graphml"));
        assert!(content.contains("edgedefault=\"undirected\""));
        assert!(content.contains("<node id="));
        assert!(content.contains("<edge"));

        fs::remove_file(path).ok();
    }

    #[test]
    fn test_large_graph_serialization() {
        let mut g = Graph::<i32, f64>::new();

        // Create graph with 100 nodes and 200 edges
        let nodes: Vec<_> = (0..100).map(|i| g.add_node(i)).collect();
        for i in 0..200 {
            let src = nodes[i % 100];
            let tgt = nodes[(i * 7) % 100];
            if src != tgt {
                g.add_edge(src, tgt, i as f64);
            }
        }

        // Test binary serialization (fast)
        let path = "large_graph.bin";
        g.save_binary(path).expect("Failed to save large graph");
        let loaded = Graph::<i32, f64>::load_binary(path).expect("Failed to load");
        assert_eq!(loaded.node_count(), g.node_count());

        fs::remove_file(path).ok();
    }

    #[test]
    fn test_directedness_mismatch_strict_load() {
        // Build a directed serializable graph
        let serializable = SerializableGraph {
            directed: true,
            nodes: vec![1, 2],
            edges: vec![(0, 1, 1.0)],
        };
        // Undirected Graph should error in strict mode
        type UGraph = crate::core::types::Graph<i32, f64>;
        let err =
            UGraph::try_from_serializable(&serializable).expect_err("expected mismatch error");
        assert!(format!("{}", err).to_lowercase().contains("mismatch"));

        // Directed graph should succeed
        type DGraph = crate::core::types::Digraph<i32, f64>;
        let g = DGraph::try_from_serializable(&serializable).expect("directed should load");
        assert_eq!(g.node_count(), 2);
        assert_eq!(g.edge_count(), 1);
    }
}
