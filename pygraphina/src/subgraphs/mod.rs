use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashSet;

use crate::{PyDiGraph, PyGraph};
use graphina::subgraphs::SubgraphOps;

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

        // No type conversion needed - Graph<i64, f64> stays as Graph<i64, f64>
        let mut py_graph = PyGraph::new();
        py_graph.populate_from_internal(subgraph);
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

        // No type conversion needed - Graph<i64, f64> stays as Graph<i64, f64>
        let mut py_graph = PyGraph::new();
        py_graph.populate_from_internal(induced);
        Ok(py_graph)
    }

    /// Extract the ego graph centered at a node within a given radius.
    ///
    /// Args:
    ///     center: Center node ID
    ///     radius: Maximum distance from center
    ///
    /// Returns:
    ///     PyGraph: Ego graph
    ///
    /// Example:
    ///     >>> g = pygraphina.PyGraph()
    ///     >>> n0 = g.add_node(0)
    ///     >>> n1 = g.add_node(1)
    ///     >>> n2 = g.add_node(2)
    ///     >>> g.add_edge(n0, n1, 1.0)
    ///     >>> g.add_edge(n1, n2, 1.0)
    ///     >>> ego = g.ego_graph(n0, 1)
    pub(crate) fn ego_graph_impl(&self, center: usize, radius: usize) -> PyResult<PyGraph> {
        let center_id = *self
            .py_to_internal
            .get(&center)
            .ok_or_else(|| PyValueError::new_err("Invalid center node id"))?;

        let ego = self
            .graph
            .ego_graph(center_id, radius)
            .map_err(|e| PyValueError::new_err(format!("Ego graph failed: {}", e)))?;

        // No type conversion needed - Graph<i64, f64> stays as Graph<i64, f64>
        let mut py_graph = PyGraph::new();
        py_graph.populate_from_internal(ego);
        Ok(py_graph)
    }

    /// Get all nodes within k hops of the start node.
    ///
    /// Args:
    ///     start: Starting node ID
    ///     k: Number of hops
    ///
    /// Returns:
    ///     list: Node IDs within k hops
    ///
    /// Example:
    ///     >>> g = pygraphina.PyGraph()
    ///     >>> n0 = g.add_node(0)
    ///     >>> n1 = g.add_node(1)
    ///     >>> g.add_edge(n0, n1, 1.0)
    ///     >>> neighbors = g.k_hop_neighbors(n0, 1)
    pub(crate) fn k_hop_neighbors_impl(&self, start: usize, k: usize) -> PyResult<Vec<usize>> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err("Invalid start node id"))?;

        let neighbors = self.graph.k_hop_neighbors(start_id, k);

        Ok(neighbors
            .into_iter()
            .filter_map(|nid| self.internal_to_py.get(&nid).copied())
            .collect())
    }

    /// Get the connected component containing the start node.
    ///
    /// Args:
    ///     start: Starting node ID
    ///
    /// Returns:
    ///     list: Node IDs in the connected component
    ///
    /// Example:
    ///     >>> g = pygraphina.PyGraph()
    ///     >>> n0 = g.add_node(0)
    ///     >>> n1 = g.add_node(1)
    ///     >>> g.add_edge(n0, n1, 1.0)
    ///     >>> component = g.connected_component(n0)
    pub(crate) fn connected_component_impl(&self, start: usize) -> PyResult<Vec<usize>> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err("Invalid start node id"))?;

        let component = self.graph.connected_component(start_id);

        Ok(component
            .into_iter()
            .filter_map(|nid| self.internal_to_py.get(&nid).copied())
            .collect())
    }

    /// Extract the subgraph for the connected component containing the start node.
    ///
    /// Args:
    ///     start: Starting node ID
    ///
    /// Returns:
    ///     PyGraph: Connected component subgraph
    ///
    /// Example:
    ///     >>> g = pygraphina.PyGraph()
    ///     >>> n0 = g.add_node(0)
    ///     >>> n1 = g.add_node(1)
    ///     >>> g.add_edge(n0, n1, 1.0)
    ///     >>> comp_graph = g.component_subgraph(n0)
    pub(crate) fn component_subgraph_impl(&self, start: usize) -> PyResult<PyGraph> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err("Invalid start node id"))?;

        let comp_subgraph = self
            .graph
            .component_subgraph(start_id)
            .map_err(|e| PyValueError::new_err(format!("Component subgraph failed: {}", e)))?;

        // No type conversion needed - Graph<i64, f64> stays as Graph<i64, f64>
        let mut py_graph = PyGraph::new();
        py_graph.populate_from_internal(comp_subgraph);
        Ok(py_graph)
    }

    /// Filter nodes using a Python predicate: fn(node_id:int, attr:int) -> bool
    pub(crate) fn filter_nodes_py_impl(
        &self,
        predicate: &pyo3::prelude::Bound<'_, pyo3::PyAny>,
    ) -> PyResult<PyGraph> {
        let filtered = self.graph.filter_nodes(|nid, attr| {
            // Map NodeId to Python id for the predicate
            if let Some(py_id) = self.internal_to_py.get(&nid).copied() {
                if let Ok(result) = predicate.call1((py_id, *attr)) {
                    if let Ok(py_bool) = result.cast::<pyo3::types::PyBool>() {
                        return py_bool.is_true();
                    }
                }
            }
            false
        });

        let mut py_graph = PyGraph::new();
        py_graph.populate_from_internal(filtered);
        Ok(py_graph)
    }

    /// Filter edges using a Python predicate: fn(u:int, v:int, w:float) -> bool
    pub(crate) fn filter_edges_py_impl(
        &self,
        predicate: &pyo3::prelude::Bound<'_, pyo3::PyAny>,
    ) -> PyResult<PyGraph> {
        use pyo3::types::PyBool;

        let filtered = self.graph.filter_edges(|u, v, w| {
            let py_u = self.internal_to_py.get(&u).copied();
            let py_v = self.internal_to_py.get(&v).copied();
            match (py_u, py_v) {
                (Some(pu), Some(pv)) => {
                    if let Ok(result) = predicate.call1((pu, pv, *w)) {
                        if let Ok(py_bool) = result.cast::<PyBool>() {
                            return py_bool.is_true();
                        }
                    }
                    false
                }
                _ => false,
            }
        });

        let mut py_graph = PyGraph::new();
        py_graph.populate_from_internal(filtered);
        Ok(py_graph)
    }
}

impl PyDiGraph {
    /// Extract a subgraph containing only the specified nodes.
    ///
    /// Args:
    ///     nodes: List of node IDs to include in subgraph
    ///
    /// Returns:
    ///     PyDiGraph: New graph containing only specified nodes and edges between them
    ///
    /// Example:
    ///     >>> g = pygraphina.PyDiGraph()
    ///     >>> n0 = g.add_node(0)
    ///     >>> n1 = g.add_node(1)
    ///     >>> n2 = g.add_node(2)
    ///     >>> g.add_edge(n0, n1, 1.0)
    ///     >>> g.add_edge(n1, n2, 2.0)
    ///     >>> sub = g.subgraph([n0, n1])
    pub(crate) fn subgraph_impl(&self, nodes: Vec<usize>) -> PyResult<PyDiGraph> {
        let internal_nodes: Vec<_> = nodes
            .iter()
            .filter_map(|&py_id| self.py_to_internal.get(&py_id).copied())
            .collect();

        if internal_nodes.len() != nodes.len() {
            return Err(PyValueError::new_err("Invalid node IDs"));
        }

        let sub = self
            .graph
            .subgraph(&internal_nodes)
            .map_err(|e| PyValueError::new_err(format!("Subgraph extraction failed: {}", e)))?;

        let mut out = PyDiGraph::new();
        out.populate_from_internal(sub);
        Ok(out)
    }

    /// Create an induced subgraph from a set of nodes.
    ///
    /// Args:
    ///     nodes: List of node IDs to include
    ///
    /// Returns:
    ///     PyDiGraph: Induced subgraph
    ///
    /// Example:
    ///     >>> g = pygraphina.PyDiGraph()
    ///     >>> n0 = g.add_node(0)
    ///     >>> n1 = g.add_node(1)
    ///     >>> g.add_edge(n0, n1, 1.0)
    ///     >>> induced = g.induced_subgraph([n0, n1])
    pub(crate) fn induced_subgraph_impl(&self, nodes: Vec<usize>) -> PyResult<PyDiGraph> {
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

        let mut out = PyDiGraph::new();
        out.populate_from_internal(induced);
        Ok(out)
    }

    /// Extract the ego graph centered at a node within a given radius.
    ///
    /// Args:
    ///     center: Center node ID
    ///     radius: Maximum distance from center
    ///
    /// Returns:
    ///     PyDiGraph: Ego graph
    ///
    /// Example:
    ///     >>> g = pygraphina.PyDiGraph()
    ///     >>> n0 = g.add_node(0)
    ///     >>> n1 = g.add_node(1)
    ///     >>> n2 = g.add_node(2)
    ///     >>> g.add_edge(n0, n1, 1.0)
    ///     >>> g.add_edge(n1, n2, 1.0)
    ///     >>> ego = g.ego_graph(n0, 1)
    pub(crate) fn ego_graph_impl(&self, center: usize, radius: usize) -> PyResult<PyDiGraph> {
        let center_id = *self
            .py_to_internal
            .get(&center)
            .ok_or_else(|| PyValueError::new_err("Invalid center node id"))?;

        let ego = self
            .graph
            .ego_graph(center_id, radius)
            .map_err(|e| PyValueError::new_err(format!("Ego graph failed: {}", e)))?;

        let mut out = PyDiGraph::new();
        out.populate_from_internal(ego);
        Ok(out)
    }

    /// Get all nodes within k hops of the start node.
    ///
    /// Args:
    ///     start: Starting node ID
    ///     k: Number of hops
    ///
    /// Returns:
    ///     list: Node IDs within k hops
    ///
    /// Example:
    ///     >>> g = pygraphina.PyDiGraph()
    ///     >>> n0 = g.add_node(0)
    ///     >>> n1 = g.add_node(1)
    ///     >>> g.add_edge(n0, n1, 1.0)
    ///     >>> neighbors = g.k_hop_neighbors(n0, 1)
    pub(crate) fn k_hop_neighbors_impl(&self, start: usize, k: usize) -> PyResult<Vec<usize>> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err("Invalid start node id"))?;

        let neighbors = self.graph.k_hop_neighbors(start_id, k);

        Ok(neighbors
            .into_iter()
            .filter_map(|nid| self.internal_to_py.get(&nid).copied())
            .collect())
    }

    /// Get the weakly connected component containing the start node.
    ///
    /// Args:
    ///     start: Starting node ID
    ///
    /// Returns:
    ///     list: Node IDs in the connected component
    ///
    /// Example:
    ///     >>> g = pygraphina.PyDiGraph()
    ///     >>> n0 = g.add_node(0)
    ///     >>> n1 = g.add_node(1)
    ///     >>> g.add_edge(n0, n1, 1.0)
    ///     >>> component = g.connected_component(n0)
    pub(crate) fn connected_component_impl(&self, start: usize) -> PyResult<Vec<usize>> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err("Invalid start node id"))?;

        let component = self.graph.connected_component(start_id);

        Ok(component
            .into_iter()
            .filter_map(|nid| self.internal_to_py.get(&nid).copied())
            .collect())
    }

    /// Extract the subgraph for the (weakly) connected component containing the start node.
    ///
    /// Args:
    ///     start: Starting node ID
    ///
    /// Returns:
    ///     PyDiGraph: Connected component subgraph
    ///
    /// Example:
    ///     >>> g = pygraphina.PyDiGraph()
    ///     >>> n0 = g.add_node(0)
    ///     >>> n1 = g.add_node(1)
    ///     >>> g.add_edge(n0, n1, 1.0)
    ///     >>> comp_graph = g.component_subgraph(n0)
    pub(crate) fn component_subgraph_impl(&self, start: usize) -> PyResult<PyDiGraph> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err("Invalid start node id"))?;

        let comp = self
            .graph
            .component_subgraph(start_id)
            .map_err(|e| PyValueError::new_err(format!("Component subgraph failed: {}", e)))?;

        let mut out = PyDiGraph::new();
        out.populate_from_internal(comp);
        Ok(out)
    }

    /// Filter nodes using a Python predicate: fn(node_id:int, attr:int) -> bool
    pub(crate) fn filter_nodes_py_impl(
        &self,
        predicate: &pyo3::prelude::Bound<'_, pyo3::PyAny>,
    ) -> PyResult<PyDiGraph> {
        let filtered = self.graph.filter_nodes(|nid, attr| {
            // Map NodeId to Python id for the predicate
            if let Some(py_id) = self.internal_to_py.get(&nid).copied() {
                if let Ok(result) = predicate.call1((py_id, *attr)) {
                    if let Ok(py_bool) = result.cast::<pyo3::types::PyBool>() {
                        return py_bool.is_true();
                    }
                }
            }
            false
        });

        let mut out = PyDiGraph::new();
        out.populate_from_internal(filtered);
        Ok(out)
    }

    /// Filter edges using a Python predicate: fn(u:int, v:int, w:float) -> bool
    pub(crate) fn filter_edges_py_impl(
        &self,
        predicate: &pyo3::prelude::Bound<'_, pyo3::PyAny>,
    ) -> PyResult<PyDiGraph> {
        use pyo3::types::PyBool;

        let filtered = self.graph.filter_edges(|u, v, w| {
            let py_u = self.internal_to_py.get(&u).copied();
            let py_v = self.internal_to_py.get(&v).copied();
            match (py_u, py_v) {
                (Some(pu), Some(pv)) => {
                    if let Ok(result) = predicate.call1((pu, pv, *w)) {
                        if let Ok(py_bool) = result.cast::<PyBool>() {
                            return py_bool.is_true();
                        }
                    }
                    false
                }
                _ => false,
            }
        });

        let mut out = PyDiGraph::new();
        out.populate_from_internal(filtered);
        Ok(out)
    }
}

pub fn register_subgraphs(_m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Subgraph methods are exposed as PyGraph methods via #[pymethods] in lib.rs
    // This function is here for consistency and future standalone functions
    Ok(())
}
