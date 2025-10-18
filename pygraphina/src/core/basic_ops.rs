use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::PyGraph;

impl PyGraph {
    /// Add a node with the given attribute and return its Python ID.
    pub fn add_node_impl(&mut self, attr: i64) -> usize {
        let internal_id = self.graph.add_node(attr);
        let py_id = self.next_id;
        self.py_to_internal.insert(py_id, internal_id);
        self.internal_to_py.insert(internal_id, py_id);
        self.next_id += 1;
        py_id
    }

    /// Update a node's attribute. Returns true if updated, false if node doesn't exist.
    pub fn update_node_impl(&mut self, py_node: usize, new_attr: i64) -> PyResult<bool> {
        if let Some(&internal_id) = self.py_to_internal.get(&py_node) {
            Ok(self.graph.update_node(internal_id, new_attr))
        } else {
            Ok(false)
        }
    }

    /// Try to update a node's attribute. Raises ValueError if node doesn't exist.
    pub fn try_update_node_impl(&mut self, py_node: usize, new_attr: i64) -> PyResult<()> {
        let internal_id = *self
            .py_to_internal
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err("Invalid node id"))?;
        self.graph
            .try_update_node(internal_id, new_attr)
            .map_err(|e| PyValueError::new_err(format!("{}", e)))
    }

    /// Add an edge between two nodes with the given weight.
    pub fn add_edge_impl(&mut self, source: usize, target: usize, weight: f64) -> PyResult<usize> {
        let src_id = *self
            .py_to_internal
            .get(&source)
            .ok_or_else(|| PyValueError::new_err("Invalid source node id"))?;
        let tgt_id = *self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err("Invalid target node id"))?;
        let edge_id = self.graph.add_edge(src_id, tgt_id, weight);
        Ok(edge_id.index())
    }

    /// Remove a node and return its attribute if it exists.
    pub fn remove_node_impl(&mut self, py_node: usize) -> PyResult<Option<i64>> {
        if let Some(&internal_id) = self.py_to_internal.get(&py_node) {
            let attr = self.graph.remove_node(internal_id);
            if attr.is_some() {
                self.py_to_internal.remove(&py_node);
                self.internal_to_py.remove(&internal_id);
            }
            Ok(attr)
        } else {
            Ok(None)
        }
    }

    /// Try to remove a node. Raises ValueError if node doesn't exist.
    pub fn try_remove_node_impl(&mut self, py_node: usize) -> PyResult<i64> {
        let internal_id = *self
            .py_to_internal
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err("Invalid node id"))?;
        let attr = self
            .graph
            .try_remove_node(internal_id)
            .map_err(|e| PyValueError::new_err(format!("{}", e)))?;
        self.py_to_internal.remove(&py_node);
        self.internal_to_py.remove(&internal_id);
        Ok(attr)
    }

    /// Check if a node exists.
    pub fn contains_node_impl(&self, py_node: usize) -> bool {
        if let Some(&internal_id) = self.py_to_internal.get(&py_node) {
            self.graph.contains_node(internal_id)
        } else {
            false
        }
    }

    /// Check if an edge exists between two nodes.
    pub fn contains_edge_impl(&self, source: usize, target: usize) -> PyResult<bool> {
        let src_id = *self
            .py_to_internal
            .get(&source)
            .ok_or_else(|| PyValueError::new_err("Invalid source node id"))?;
        let tgt_id = *self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err("Invalid target node id"))?;
        Ok(self.graph.contains_edge(src_id, tgt_id))
    }

    /// Get all node IDs.
    pub fn nodes_impl(&self) -> Vec<usize> {
        self.graph
            .nodes()
            .map(|(nid, _)| self.internal_to_py.get(&nid).copied())
            .flatten()
            .collect()
    }

    /// Get the neighbors of a node.
    pub fn neighbors_impl(&self, py_node: usize) -> PyResult<Vec<usize>> {
        let internal_id = *self
            .py_to_internal
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err("Invalid node id"))?;
        let neighbors = self.graph.neighbors(internal_id);
        Ok(neighbors
            .filter_map(|nid| self.internal_to_py.get(&nid).copied())
            .collect())
    }

    /// Get the degree of a node.
    pub fn degree_impl(&self, py_node: usize) -> Option<usize> {
        let internal_id = *self.py_to_internal.get(&py_node)?;
        self.graph.degree(internal_id)
    }

    /// Get a node's attribute.
    pub fn get_node_attr_impl(&self, py_node: usize) -> Option<i64> {
        let internal_id = *self.py_to_internal.get(&py_node)?;
        self.graph.node_attr(internal_id).copied()
    }

    /// Clear the graph.
    pub fn clear_impl(&mut self) {
        self.graph.clear();
        self.py_to_internal.clear();
        self.internal_to_py.clear();
        self.next_id = 0;
    }
}
