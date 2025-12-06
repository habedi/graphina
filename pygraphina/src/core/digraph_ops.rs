use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::PyDiGraph;

impl PyDiGraph {
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
            .ok_or_else(|| PyValueError::new_err(format!("Invalid node id: {}", py_node)))?;
        self.graph
            .try_update_node(internal_id, new_attr)
            .map_err(|e| PyValueError::new_err(format!("{}", e)))
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

    /// Get a node's attribute.
    pub fn get_node_attr_impl(&self, py_node: usize) -> Option<i64> {
        let internal_id = *self.py_to_internal.get(&py_node)?;
        self.graph.node_attr(internal_id).copied()
    }

    /// Check if a node exists.
    pub fn contains_node_impl(&self, py_node: usize) -> bool {
        if let Some(&internal_id) = self.py_to_internal.get(&py_node) {
            self.graph.contains_node(internal_id)
        } else {
            false
        }
    }

    /// Get the degree of a node (total: in + out).
    pub fn degree_impl(&self, py_node: usize) -> Option<usize> {
        let internal_id = *self.py_to_internal.get(&py_node)?;
        self.graph.degree(internal_id)
    }

    /// Get the in-degree of a node.
    pub fn in_degree_impl(&self, py_node: usize) -> Option<usize> {
        let internal_id = *self.py_to_internal.get(&py_node)?;
        Some(self.graph.incoming_neighbors(internal_id).count())
    }

    /// Get the out-degree of a node.
    pub fn out_degree_impl(&self, py_node: usize) -> Option<usize> {
        let internal_id = *self.py_to_internal.get(&py_node)?;
        Some(self.graph.outgoing_neighbors(internal_id).count())
    }

    /// Get outgoing neighbors of a node.
    pub fn out_neighbors_impl(&self, py_node: usize) -> PyResult<Vec<usize>> {
        let internal_id = *self
            .py_to_internal
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid node id: {}", py_node)))?;
        Ok(self
            .graph
            .outgoing_neighbors(internal_id)
            .filter_map(|nid| self.internal_to_py.get(&nid).copied())
            .collect())
    }

    /// Get incoming neighbors of a node.
    pub fn in_neighbors_impl(&self, py_node: usize) -> PyResult<Vec<usize>> {
        let internal_id = *self
            .py_to_internal
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid node id: {}", py_node)))?;
        Ok(self
            .graph
            .incoming_neighbors(internal_id)
            .filter_map(|nid| self.internal_to_py.get(&nid).copied())
            .collect())
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
