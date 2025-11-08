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
    /// Validates that weight is finite (not NaN or Inf).
    pub fn add_edge_impl(&mut self, source: usize, target: usize, weight: f64) -> PyResult<usize> {
        // Validate weight is finite
        if !weight.is_finite() {
            return Err(PyValueError::new_err(format!(
                "Edge weight must be finite, got: {}",
                weight
            )));
        }

        let src_id = *self
            .py_to_internal
            .get(&source)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid source node id: {}", source)))?;
        let tgt_id = *self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid target node id: {}", target)))?;
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
            .ok_or_else(|| PyValueError::new_err(format!("Invalid node id: {}", py_node)))?;
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
            .ok_or_else(|| PyValueError::new_err(format!("Invalid source node id: {}", source)))?;
        let tgt_id = *self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid target node id: {}", target)))?;
        Ok(self.graph.contains_edge(src_id, tgt_id))
    }

    /// Get all node IDs.
    pub fn nodes_impl(&self) -> Vec<usize> {
        self.graph
            .nodes()
            .filter_map(|(nid, _)| self.internal_to_py.get(&nid).copied())
            .collect()
    }

    /// Get the neighbors of a node.
    pub fn neighbors_impl(&self, py_node: usize) -> PyResult<Vec<usize>> {
        let internal_id = *self
            .py_to_internal
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid node id: {}", py_node)))?;
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

    /// Remove an edge between two nodes. Returns true if edge was removed, false if not found.
    pub fn remove_edge_impl(&mut self, source: usize, target: usize) -> PyResult<bool> {
        let src_id = *self
            .py_to_internal
            .get(&source)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid source node id: {}", source)))?;
        let tgt_id = *self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid target node id: {}", target)))?;

        // Find the edge first
        if let Some(edge_id) = self.graph.find_edge(src_id, tgt_id) {
            self.graph.remove_edge(edge_id);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Try to remove an edge. Raises ValueError if edge doesn't exist.
    pub fn try_remove_edge_impl(&mut self, source: usize, target: usize) -> PyResult<()> {
        let src_id = *self
            .py_to_internal
            .get(&source)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid source node id: {}", source)))?;
        let tgt_id = *self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid target node id: {}", target)))?;

        // Find the edge first
        let edge_id = self.graph.find_edge(src_id, tgt_id).ok_or_else(|| {
            PyValueError::new_err(format!("Edge not found between {} and {}", source, target))
        })?;

        self.graph
            .try_remove_edge(edge_id)
            .map_err(|e| PyValueError::new_err(format!("{}", e)))?;
        Ok(())
    }

    /// Get the weight of an edge between two nodes.
    pub fn get_edge_weight_impl(&self, source: usize, target: usize) -> PyResult<Option<f64>> {
        let src_id = *self
            .py_to_internal
            .get(&source)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid source node id: {}", source)))?;
        let tgt_id = *self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid target node id: {}", target)))?;

        // Find the edge first
        if let Some(edge_id) = self.graph.find_edge(src_id, tgt_id) {
            Ok(self.graph.edge_weight(edge_id).copied())
        } else {
            Ok(None)
        }
    }

    /// Update the weight of an existing edge. Returns true if updated, false if edge not found.
    pub fn update_edge_weight_impl(
        &mut self,
        source: usize,
        target: usize,
        new_weight: f64,
    ) -> PyResult<bool> {
        // Validate weight is finite
        if !new_weight.is_finite() {
            return Err(PyValueError::new_err(format!(
                "Edge weight must be finite, got: {}",
                new_weight
            )));
        }

        let src_id = *self
            .py_to_internal
            .get(&source)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid source node id: {}", source)))?;
        let tgt_id = *self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid target node id: {}", target)))?;

        // Find the edge first
        if let Some(edge_id) = self.graph.find_edge(src_id, tgt_id) {
            if let Some(weight_ref) = self.graph.edge_weight_mut(edge_id) {
                *weight_ref = new_weight;
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    /// Try to update edge weight. Raises ValueError if edge doesn't exist.
    pub fn try_update_edge_weight_impl(
        &mut self,
        source: usize,
        target: usize,
        new_weight: f64,
    ) -> PyResult<()> {
        // Validate weight is finite
        if !new_weight.is_finite() {
            return Err(PyValueError::new_err(format!(
                "Edge weight must be finite, got: {}",
                new_weight
            )));
        }

        let src_id = *self
            .py_to_internal
            .get(&source)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid source node id: {}", source)))?;
        let tgt_id = *self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid target node id: {}", target)))?;

        // Find the edge first
        let edge_id = self.graph.find_edge(src_id, tgt_id).ok_or_else(|| {
            PyValueError::new_err(format!("Edge not found between {} and {}", source, target))
        })?;

        if let Some(weight_ref) = self.graph.edge_weight_mut(edge_id) {
            *weight_ref = new_weight;
            Ok(())
        } else {
            Err(PyValueError::new_err(format!(
                "Failed to update edge weight between {} and {}",
                source, target
            )))
        }
    }
}
