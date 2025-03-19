use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

// Import the core types from Graphina.
use graphina::core::types::{Graph, NodeId};

/// A Python-accessible Graph class wrapping Graphina's core undirected graph.
///
/// This class uses `i64` as the node attribute type and `f64` as the edge weight type.
/// Internally, it maintains a mapping from Python-assigned node IDs (simple `usize` values)
/// to the Graphina `NodeId`s.
#[pyclass]
struct PyGraph {
    /// The underlying Graphina graph.
    graph: Graph<i64, f64>,
    /// Mapping from Python-level node IDs to internal NodeId values.
    mapping: HashMap<usize, NodeId>,
    /// The next Python-level node ID to assign.
    next_id: usize,
}

#[pymethods]
impl PyGraph {
    /// Creates a new, empty graph.
    ///
    /// Example:
    ///     >>> g = pygraphina.PyGraph()
    #[new]
    fn new() -> Self {
        PyGraph {
            graph: Graph::new(),
            mapping: HashMap::new(),
            next_id: 0,
        }
    }

    /// Adds a node with the given integer attribute.
    ///
    /// Returns a Python-level node identifier.
    ///
    /// Example:
    ///     >>> node_id = g.add_node(42)
    fn add_node(&mut self, attr: i64) -> usize {
        let node_id = self.graph.add_node(attr);
        let py_id = self.next_id;
        self.mapping.insert(py_id, node_id);
        self.next_id += 1;
        py_id
    }

    /// Updates the attribute of an existing node.
    ///
    /// Returns True if the update was successful, or False if the node was not found.
    ///
    /// Example:
    ///     >>> success = g.update_node(0, 100)
    fn update_node(&mut self, py_node: usize, new_attr: i64) -> PyResult<bool> {
        let node_id = self
            .mapping
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err("Invalid node id"))?;
        Ok(self.graph.update_node(*node_id, new_attr))
    }

    /// Attempts to update the attribute of an existing node.
    ///
    /// Raises a ValueError on error.
    ///
    /// Example:
    ///     >>> g.try_update_node(0, 200)
    fn try_update_node(&mut self, py_node: usize, new_attr: i64) -> PyResult<()> {
        let node_id = self
            .mapping
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err("Invalid node id"))?;
        self.graph
            .try_update_node(*node_id, new_attr)
            .map_err(|e| PyValueError::new_err(format!("Error: {:?}", e)))
    }

    /// Adds an edge between two nodes with the given weight.
    ///
    /// Returns the internal edge identifier (as an integer).
    ///
    /// Example:
    ///     >>> edge_id = g.add_edge(0, 1, 3.14)
    fn add_edge(&mut self, source: usize, target: usize, weight: f64) -> PyResult<usize> {
        let s_id = self
            .mapping
            .get(&source)
            .ok_or_else(|| PyValueError::new_err("Invalid source node id"))?;
        let t_id = self
            .mapping
            .get(&target)
            .ok_or_else(|| PyValueError::new_err("Invalid target node id"))?;
        let edge = self.graph.add_edge(*s_id, *t_id, weight);
        Ok(edge.index())
    }

    /// Removes a node from the graph.
    ///
    /// Returns the attribute of the removed node, or None if the node did not exist.
    ///
    /// Example:
    ///     >>> attr = g.remove_node(0)
    fn remove_node(&mut self, py_node: usize) -> PyResult<Option<i64>> {
        let node_id = self
            .mapping
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err("Invalid node id"))?;
        let result = self.graph.remove_node(*node_id);
        self.mapping.remove(&py_node);
        Ok(result)
    }

    /// Attempts to remove a node from the graph.
    ///
    /// Raises a ValueError if the node does not exist.
    ///
    /// Example:
    ///     >>> attr = g.try_remove_node(0)
    fn try_remove_node(&mut self, py_node: usize) -> PyResult<i64> {
        let node_id = self
            .mapping
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err("Invalid node id"))?;
        let result = self
            .graph
            .try_remove_node(*node_id)
            .map_err(|e| PyValueError::new_err(format!("Error: {:?}", e)))?;
        self.mapping.remove(&py_node);
        Ok(result)
    }

    /// Returns the total number of nodes in the graph.
    ///
    /// Example:
    ///     >>> count = g.node_count()
    fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Returns the total number of edges in the graph.
    ///
    /// Example:
    ///     >>> count = g.edge_count()
    fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Returns a list of Python-level node IDs that are neighbors of the given node.
    ///
    /// Example:
    ///     >>> neighbors = g.neighbors(0)
    fn neighbors(&self, py_node: usize) -> PyResult<Vec<usize>> {
        let node_id = self
            .mapping
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err("Invalid node id"))?;
        let mut result = Vec::new();
        // Iterate over neighbors and reverse-search the mapping for their Python-level IDs.
        for neighbor in self.graph.neighbors(*node_id) {
            if let Some((&py_id, _)) = self.mapping.iter().find(|(_, &v)| v == neighbor) {
                result.push(py_id);
            }
        }
        Ok(result)
    }
}

/// The Python module declaration.
#[pymodule]
fn pygraphina(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Bound is from pyo3::prelude
    m.add_class::<PyGraph>()?;
    Ok(())
}
