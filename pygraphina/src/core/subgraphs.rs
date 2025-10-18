use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashSet;

use crate::PyGraph;

impl PyGraph {
    /// Extract a subgraph containing only the specified nodes.
    ///
    /// Args:
    ///     nodes: List of node IDs to include in subgraph
    ///
    /// Returns:
    ///     PyGraph: New graph containing only specified nodes and edges between them
    ///
    /// Example:
    ///     >>> g = pygraphina.PyGraph()
    ///     >>> n0 = g.add_node(0)
    ///     >>> n1 = g.add_node(1)
    ///     >>> n2 = g.add_node(2)
    ///     >>> g.add_edge(n0, n1, 1.0)
    ///     >>> g.add_edge(n1, n2, 2.0)
    ///     >>> sub = g.subgraph([n0, n1])
    pub(crate) fn subgraph_impl(&self, nodes: Vec<usize>) -> PyResult<PyGraph> {
        let internal_nodes: Vec<_> = nodes
            .iter()
            .filter_map(|&py_id| self.py_to_internal.get(&py_id).copied())
            .collect();

        if internal_nodes.len() != nodes.len() {
            return Err(PyValueError::new_err("Invalid node IDs"));
        }

        let subgraph = self
            .graph
            .subgraph(&internal_nodes)
            .map_err(|e| PyValueError::new_err(format!("Subgraph extraction failed: {}", e)))?;

        // Convert Graph<i64, f64> to Graph<u32, f32> format expected by populate_from_internal
        let mut converted_graph = graphina::core::types::Graph::<u32, f32>::new();
        let mut node_map = std::collections::HashMap::new();

        for (nid, &attr) in subgraph.nodes() {
            let new_id = converted_graph.add_node(attr as u32);
            node_map.insert(nid, new_id);
        }

        for (u, v, &w) in subgraph.edges() {
            let iu = node_map[&u];
            let iv = node_map[&v];
            converted_graph.add_edge(iu, iv, w as f32);
        }

        let mut py_graph = PyGraph::new();
        py_graph.populate_from_internal(converted_graph);
        Ok(py_graph)
    }

    /// Create an induced subgraph from a set of nodes.
    ///
    /// Args:
    ///     nodes: List of node IDs to include
    ///
    /// Returns:
    ///     PyGraph: Induced subgraph
    ///
    /// Example:
    ///     >>> g = pygraphina.PyGraph()
    ///     >>> n0 = g.add_node(0)
    ///     >>> n1 = g.add_node(1)
    ///     >>> g.add_edge(n0, n1, 1.0)
    ///     >>> induced = g.induced_subgraph([n0, n1])
    pub(crate) fn induced_subgraph_impl(&self, nodes: Vec<usize>) -> PyResult<PyGraph> {
        let internal_nodes: HashSet<_> = nodes
            .iter()
            .filter_map(|&py_id| self.py_to_internal.get(&py_id).copied())
            .collect();

        if internal_nodes.len() != nodes.len() {
            return Err(PyValueError::new_err("Invalid node IDs"));
        }

        let induced = self
            .graph
            .induced_subgraph(&internal_nodes)
            .map_err(|e| PyValueError::new_err(format!("Induced subgraph failed: {}", e)))?;

        // Convert Graph<i64, f64> to Graph<u32, f32> format expected by populate_from_internal
        let mut converted_graph = graphina::core::types::Graph::<u32, f32>::new();
        let mut node_map = std::collections::HashMap::new();

        for (nid, &attr) in induced.nodes() {
            let new_id = converted_graph.add_node(attr as u32);
            node_map.insert(nid, new_id);
        }

        for (u, v, &w) in induced.edges() {
            let iu = node_map[&u];
            let iv = node_map[&v];
            converted_graph.add_edge(iu, iv, w as f32);
        }

        let mut py_graph = PyGraph::new();
        py_graph.populate_from_internal(converted_graph);
        Ok(py_graph)
    }
}

pub fn register_subgraphs(_m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Subgraph methods are added as PyGraph methods via #[pymethods] in lib.rs
    Ok(())
}
