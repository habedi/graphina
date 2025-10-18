use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use graphina::core::io::{read_edge_list, write_edge_list};
use graphina::core::types::NodeId;

use crate::PyGraph;

impl PyGraph {
    /// Load an edge list file into this graph. This resets the current graph contents.
    pub fn load_edge_list_impl(&mut self, path: &str, sep: &str) -> PyResult<(usize, usize)> {
        let sep_char = sep.chars().next().ok_or_else(|| {
            PyValueError::new_err("Separator must be a non-empty string (first char used)")
        })?;

        // Read into a temporary Graph<i32, f64> using core I/O utilities
        let mut tmp: graphina::core::types::Graph<i32, f64> = graphina::core::types::Graph::new();
        read_edge_list(path, &mut tmp, sep_char)
            .map_err(|e| PyValueError::new_err(format!("Failed to read edge list: {}", e)))?;

        // Reset current graph and mappings
        self.clear_impl();

        // Map temporary node ids to new internal ids in self.graph
        let mut tmp_to_internal: HashMap<NodeId, NodeId> = HashMap::new();
        for (nid, &attr) in tmp.nodes() {
            let new_internal = self.graph.add_node(attr as i64);
            let py_id = self.next_id;
            self.py_to_internal.insert(py_id, new_internal);
            self.internal_to_py.insert(new_internal, py_id);
            self.next_id += 1;
            tmp_to_internal.insert(nid, new_internal);
        }
        for (u, v, &w) in tmp.edges() {
            let iu = *tmp_to_internal.get(&u).unwrap();
            let iv = *tmp_to_internal.get(&v).unwrap();
            self.graph.add_edge(iu, iv, w as f64);
        }

        Ok((self.graph.node_count(), self.graph.edge_count()))
    }

    /// Save the current graph as an edge list file using the provided separator.
    pub fn save_edge_list_impl(&self, path: &str, sep: &str) -> PyResult<()> {
        let sep_char = sep.chars().next().ok_or_else(|| {
            PyValueError::new_err("Separator must be a non-empty string (first char used)")
        })?;
        // Convert to a temporary Graph<i32, f32> for writing
        let mut tmp: graphina::core::types::Graph<i32, f32> = graphina::core::types::Graph::new();
        let mut map: HashMap<NodeId, NodeId> = HashMap::new();
        for (nid, &attr) in self.graph.nodes() {
            let cast_attr: i32 = attr as i32;
            let new_id = tmp.add_node(cast_attr);
            map.insert(nid, new_id);
        }
        for (u, v, &w) in self.graph.edges() {
            let iu = *map.get(&u).unwrap();
            let iv = *map.get(&v).unwrap();
            let cast_w: f32 = w as f32;
            tmp.add_edge(iu, iv, cast_w);
        }
        write_edge_list(path, &tmp, sep_char)
            .map_err(|e| PyValueError::new_err(format!("Failed to write edge list: {}", e)))
    }

    /// Save graph as JSON (nodes: i64, edges: f64).
    pub fn save_json_impl(&self, path: &str) -> PyResult<()> {
        self.graph
            .save_json(path)
            .map_err(|e| PyValueError::new_err(format!("{}", e)))
    }

    /// Load graph from JSON (resets current graph; expects nodes i64 and edges f64).
    pub fn load_json_impl(&mut self, path: &str) -> PyResult<()> {
        let loaded = graphina::core::types::Graph::<i64, f64>::load_json(path)
            .map_err(|e| PyValueError::new_err(format!("{}", e)))?;
        self.clear_impl();
        // rebuild mappings
        let mut map: HashMap<NodeId, NodeId> = HashMap::new();
        for (nid, &attr) in loaded.nodes() {
            let new_id = self.graph.add_node(attr);
            let py_id = self.next_id;
            self.next_id += 1;
            self.py_to_internal.insert(py_id, new_id);
            self.internal_to_py.insert(new_id, py_id);
            map.insert(nid, new_id);
        }
        for (u, v, &w) in loaded.edges() {
            let iu = map[&u];
            let iv = map[&v];
            self.graph.add_edge(iu, iv, w);
        }
        Ok(())
    }

    /// Save graph as binary file.
    pub fn save_binary_impl(&self, path: &str) -> PyResult<()> {
        self.graph
            .save_binary(path)
            .map_err(|e| PyValueError::new_err(format!("{}", e)))
    }

    /// Load graph from binary file (resets current graph; expects nodes i64 and edges f64).
    pub fn load_binary_impl(&mut self, path: &str) -> PyResult<()> {
        let loaded = graphina::core::types::Graph::<i64, f64>::load_binary(path)
            .map_err(|e| PyValueError::new_err(format!("{}", e)))?;
        self.clear_impl();
        let mut map: HashMap<NodeId, NodeId> = HashMap::new();
        for (nid, &attr) in loaded.nodes() {
            let new_id = self.graph.add_node(attr);
            let py_id = self.next_id;
            self.next_id += 1;
            self.py_to_internal.insert(py_id, new_id);
            self.internal_to_py.insert(new_id, py_id);
            map.insert(nid, new_id);
        }
        for (u, v, &w) in loaded.edges() {
            let iu = map[&u];
            let iv = map[&v];
            self.graph.add_edge(iu, iv, w);
        }
        Ok(())
    }

    /// Save graph in GraphML format.
    pub fn save_graphml_impl(&self, path: &str) -> PyResult<()> {
        self.graph
            .save_graphml(path)
            .map_err(|e| PyValueError::new_err(format!("{}", e)))
    }
}
