use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use graphina::metrics::{
    assortativity, average_clustering_coefficient, average_path_length, clustering_coefficient,
    diameter, radius, transitivity, triangles,
};

use crate::{PyDiGraph, PyGraph};

impl PyGraph {
    /// Diameter (longest shortest path). None if graph is empty or disconnected.
    pub fn diameter_impl(&self) -> Option<usize> {
        diameter(&self.graph)
    }

    /// Radius (minimum eccentricity). None if graph is empty or disconnected.
    pub fn radius_impl(&self) -> Option<usize> {
        radius(&self.graph)
    }

    /// Average clustering coefficient over all nodes.
    pub fn average_clustering_impl(&self) -> f64 {
        average_clustering_coefficient(&self.graph)
    }

    /// Local clustering coefficient for a given node id.
    pub fn clustering_of_impl(&self, py_node: usize) -> PyResult<f64> {
        let nid = *self
            .py_to_internal
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err("Invalid node id"))?;
        Ok(clustering_coefficient(&self.graph, nid))
    }

    /// Global transitivity (global clustering coefficient).
    pub fn transitivity_impl(&self) -> f64 {
        transitivity(&self.graph)
    }

    /// Number of triangles containing the given node.
    pub fn triangles_of_impl(&self, py_node: usize) -> PyResult<usize> {
        let nid = *self
            .py_to_internal
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err("Invalid node id"))?;
        Ok(triangles(&self.graph, nid))
    }

    /// Average shortest path length. None if disconnected or empty.
    pub fn average_path_length_impl(&self) -> Option<f64> {
        average_path_length(&self.graph)
    }

    /// Degree assortativity coefficient.
    pub fn assortativity_impl(&self) -> f64 {
        assortativity(&self.graph)
    }
}

impl PyDiGraph {
    /// Diameter (longest shortest path). None if graph is empty or disconnected.
    pub fn diameter_impl(&self) -> Option<usize> {
        diameter(&self.graph)
    }

    /// Radius (minimum eccentricity). None if graph is empty or disconnected.
    pub fn radius_impl(&self) -> Option<usize> {
        radius(&self.graph)
    }

    /// Average clustering coefficient over all nodes.
    pub fn average_clustering_impl(&self) -> f64 {
        average_clustering_coefficient(&self.graph)
    }

    /// Local clustering coefficient for a given node id.
    pub fn clustering_of_impl(&self, py_node: usize) -> PyResult<f64> {
        let nid = *self
            .py_to_internal
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err("Invalid node id"))?;
        Ok(clustering_coefficient(&self.graph, nid))
    }

    /// Global transitivity (global clustering coefficient).
    pub fn transitivity_impl(&self) -> f64 {
        transitivity(&self.graph)
    }

    /// Number of triangles containing the given node.
    pub fn triangles_of_impl(&self, py_node: usize) -> PyResult<usize> {
        let nid = *self
            .py_to_internal
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err("Invalid node id"))?;
        Ok(triangles(&self.graph, nid))
    }

    /// Average shortest path length. None if disconnected or empty.
    pub fn average_path_length_impl(&self) -> Option<f64> {
        average_path_length(&self.graph)
    }

    /// Degree assortativity coefficient.
    pub fn assortativity_impl(&self) -> f64 {
        assortativity(&self.graph)
    }
}

pub fn register_metrics(_m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Metrics methods are exposed as PyGraph methods via #[pymethods] in lib.rs
    // This function is here for consistency and future standalone functions
    Ok(())
}
