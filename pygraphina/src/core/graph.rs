//! PyGraph - Python-accessible undirected graph wrapper.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::core::views::degree::DegreeView;
use crate::core::views::edge::EdgeView;
use crate::core::views::node::NodeView;
use graphina::core::types::{BaseGraph, NodeId, Undirected};

/// A Python-accessible Graph class wrapping Graphina's core undirected graph.
///
/// This class uses `i64` as the node attribute type and `f64` as the edge weight type.
/// Internally, it maintains a mapping from Python-assigned node IDs (simple `usize` values)
/// to the Graphina `NodeId`s.
#[pyclass]
pub struct PyGraph {
    pub(crate) graph: BaseGraph<i64, f64, Undirected>,
    pub(crate) mapper: crate::core::id_map::IdMapper,
}

#[pymethods]
impl PyGraph {
    /// Creates a new, empty graph.
    #[new]
    pub fn new() -> Self {
        PyGraph {
            graph: BaseGraph::new(),
            mapper: crate::core::id_map::IdMapper::new(),
        }
    }

    // Basic operations
    pub fn add_node(&mut self, attr: i64) -> usize {
        self.add_node_impl(attr)
    }
    pub fn update_node(&mut self, py_node: usize, new_attr: i64) -> PyResult<bool> {
        self.update_node_impl(py_node, new_attr)
    }
    pub fn try_update_node(&mut self, py_node: usize, new_attr: i64) -> PyResult<()> {
        self.try_update_node_impl(py_node, new_attr)
    }
    pub fn add_edge(&mut self, source: usize, target: usize, weight: f64) -> PyResult<usize> {
        self.add_edge_impl(source, target, weight)
    }
    pub fn remove_node(&mut self, py_node: usize) -> PyResult<Option<i64>> {
        self.remove_node_impl(py_node)
    }
    pub fn try_remove_node(&mut self, py_node: usize) -> PyResult<i64> {
        self.try_remove_node_impl(py_node)
    }
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
    pub fn is_directed(&self) -> bool {
        self.graph.is_directed()
    }
    pub fn density(&self) -> f64 {
        self.graph.density()
    }
    pub fn contains_node(&self, py_node: usize) -> bool {
        self.contains_node_impl(py_node)
    }
    pub fn contains_edge(&self, source: usize, target: usize) -> PyResult<bool> {
        self.contains_edge_impl(source, target)
    }
    #[getter]
    pub fn nodes(slf: PyRef<'_, Self>) -> PyResult<NodeView> {
        let py = slf.py();
        Ok(NodeView::new(slf.into_pyobject(py)?.into_any().unbind()))
    }

    pub fn neighbors(&self, py_node: usize) -> PyResult<Vec<usize>> {
        self.neighbors_impl(py_node)
    }

    #[pyo3(name = "_degree")]
    pub fn _degree(&self, py_node: usize) -> Option<usize> {
        self.degree_impl(py_node)
    }

    #[getter]
    pub fn degree(slf: PyRef<'_, Self>) -> PyResult<DegreeView> {
        let py = slf.py();
        Ok(DegreeView::new(slf.into_pyobject(py)?.into_any().unbind()))
    }
    pub fn get_node_attr(&self, py_node: usize) -> Option<i64> {
        self.get_node_attr_impl(py_node)
    }
    pub fn clear(&mut self) {
        self.clear_impl()
    }

    // Edge operations
    pub fn remove_edge(&mut self, source: usize, target: usize) -> PyResult<bool> {
        self.remove_edge_impl(source, target)
    }
    pub fn try_remove_edge(&mut self, source: usize, target: usize) -> PyResult<()> {
        self.try_remove_edge_impl(source, target)
    }
    pub fn get_edge_weight(&self, source: usize, target: usize) -> PyResult<Option<f64>> {
        self.get_edge_weight_impl(source, target)
    }
    pub fn update_edge_weight(
        &mut self,
        source: usize,
        target: usize,
        new_weight: f64,
    ) -> PyResult<bool> {
        self.update_edge_weight_impl(source, target, new_weight)
    }
    pub fn try_update_edge_weight(
        &mut self,
        source: usize,
        target: usize,
        new_weight: f64,
    ) -> PyResult<()> {
        self.try_update_edge_weight_impl(source, target, new_weight)
    }

    // Traversal
    pub fn bfs(&self, start: usize) -> PyResult<Vec<usize>> {
        self.bfs_impl(start)
    }
    pub fn dfs(&self, start: usize) -> PyResult<Vec<usize>> {
        self.dfs_impl(start)
    }
    pub fn iddfs(
        &self,
        start: usize,
        target: usize,
        max_depth: usize,
    ) -> PyResult<Option<Vec<usize>>> {
        self.iddfs_impl(start, target, max_depth)
    }
    pub fn try_iddfs(&self, start: usize, target: usize, max_depth: usize) -> PyResult<Vec<usize>> {
        self.try_iddfs_impl(start, target, max_depth)
    }
    pub fn bidirectional_search(
        &self,
        start: usize,
        target: usize,
    ) -> PyResult<Option<Vec<usize>>> {
        self.bidirectional_search_impl(start, target)
    }
    pub fn try_bidirectional_search(&self, start: usize, target: usize) -> PyResult<Vec<usize>> {
        self.try_bidirectional_search_impl(start, target)
    }

    // Subgraph operations
    pub fn subgraph(&self, nodes: Vec<usize>) -> PyResult<PyGraph> {
        self.subgraph_impl(nodes)
    }
    pub fn induced_subgraph(&self, nodes: Vec<usize>) -> PyResult<PyGraph> {
        self.induced_subgraph_impl(nodes)
    }
    pub fn ego_graph(&self, center: usize, radius: usize) -> PyResult<PyGraph> {
        self.ego_graph_impl(center, radius)
    }
    pub fn k_hop_neighbors(&self, start: usize, k: usize) -> PyResult<Vec<usize>> {
        self.k_hop_neighbors_impl(start, k)
    }
    pub fn connected_component(&self, start: usize) -> PyResult<Vec<usize>> {
        self.connected_component_impl(start)
    }
    pub fn component_subgraph(&self, start: usize) -> PyResult<PyGraph> {
        self.component_subgraph_impl(start)
    }

    // Filter operations
    pub fn filter_nodes(
        &self,
        predicate: &pyo3::prelude::Bound<'_, pyo3::PyAny>,
    ) -> PyResult<PyGraph> {
        self.filter_nodes_py_impl(predicate)
    }
    pub fn filter_edges(
        &self,
        predicate: &pyo3::prelude::Bound<'_, pyo3::PyAny>,
    ) -> PyResult<PyGraph> {
        self.filter_edges_py_impl(predicate)
    }

    // Metrics
    pub fn diameter(&self) -> Option<usize> {
        self.diameter_impl()
    }
    pub fn radius(&self) -> Option<usize> {
        self.radius_impl()
    }
    pub fn average_clustering(&self) -> f64 {
        self.average_clustering_impl()
    }
    pub fn clustering_of(&self, py_node: usize) -> PyResult<f64> {
        self.clustering_of_impl(py_node)
    }
    pub fn transitivity(&self) -> f64 {
        self.transitivity_impl()
    }
    pub fn triangles_of(&self, py_node: usize) -> PyResult<usize> {
        self.triangles_of_impl(py_node)
    }
    pub fn average_path_length(&self) -> Option<f64> {
        self.average_path_length_impl()
    }
    pub fn assortativity(&self) -> f64 {
        self.assortativity_impl()
    }

    // I/O operations
    #[pyo3(signature = (path, sep = " "))]
    pub fn load_edge_list(&mut self, path: &str, sep: &str) -> PyResult<(usize, usize)> {
        self.load_edge_list_impl(path, sep)
    }
    #[pyo3(signature = (path, sep = " "))]
    pub fn save_edge_list(&self, path: &str, sep: &str) -> PyResult<()> {
        self.save_edge_list_impl(path, sep)
    }
    pub fn save_json(&self, path: &str) -> PyResult<()> {
        self.save_json_impl(path)
    }
    pub fn load_json(&mut self, path: &str) -> PyResult<()> {
        self.load_json_impl(path)
    }
    pub fn save_binary(&self, path: &str) -> PyResult<()> {
        self.save_binary_impl(path)
    }
    pub fn load_binary(&mut self, path: &str) -> PyResult<()> {
        self.load_binary_impl(path)
    }
    pub fn save_graphml(&self, path: &str) -> PyResult<()> {
        self.save_graphml_impl(path)
    }

    // Paths
    #[pyo3(signature = (start, cutoff=None))]
    pub fn dijkstra(
        &self,
        start: usize,
        cutoff: Option<f64>,
    ) -> PyResult<std::collections::HashMap<usize, Option<f64>>> {
        let start_id = self
            .mapper
            .get_internal(start)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid start node id: {}", start)))?;
        let (costs, _trace) =
            graphina::core::paths::dijkstra_path_f64(&self.graph, start_id, cutoff)
                .map_err(|e| PyValueError::new_err(format!("Dijkstra error: {}", e)))?;
        let mut out = std::collections::HashMap::new();
        for (nid, dist) in costs.into_iter() {
            if let Some(pyid) = self.mapper.get_py(nid) {
                out.insert(pyid, dist);
            }
        }
        Ok(out)
    }
    pub fn shortest_path(
        &self,
        start: usize,
        target: usize,
    ) -> PyResult<Option<(f64, Vec<usize>)>> {
        self.shortest_path_impl(start, target)
    }
    pub fn bellman_ford(&self, start: usize) -> PyResult<Option<HashMap<usize, Option<f64>>>> {
        self.bellman_ford_impl(start)
    }
    pub fn floyd_warshall(&self) -> Option<HashMap<usize, HashMap<usize, Option<f64>>>> {
        let all_pairs = graphina::core::paths::floyd_warshall(&self.graph);
        all_pairs.map(|m| {
            m.into_iter()
                .filter_map(|(u, inner)| {
                    self.mapper.get_py(u).map(|pu| {
                        let inner_map: HashMap<usize, Option<f64>> = inner
                            .into_iter()
                            .filter_map(|(v, d)| self.mapper.get_py(v).map(|pv| (pv, d)))
                            .collect();
                        (pu, inner_map)
                    })
                })
                .collect()
        })
    }

    // Validation
    pub fn is_empty(&self) -> bool {
        self.is_empty_impl()
    }
    pub fn is_connected(&self) -> bool {
        self.is_connected_impl()
    }
    pub fn has_negative_weights(&self) -> bool {
        self.has_negative_weights_impl()
    }
    pub fn has_self_loops(&self) -> bool {
        self.has_self_loops_impl()
    }
    pub fn is_bipartite(&self) -> bool {
        self.is_bipartite_impl()
    }
    pub fn count_components(&self) -> usize {
        self.count_components_impl()
    }

    // Bulk operations
    pub fn add_nodes_from(&mut self, attrs: Vec<i64>) -> Vec<usize> {
        let mut ids = Vec::with_capacity(attrs.len());
        for a in attrs.into_iter() {
            let nid = self.graph.add_node(a);
            let py_id = self.mapper.add(nid);
            ids.push(py_id);
        }
        ids
    }
    pub fn add_edges_from(
        &mut self,
        edges: Vec<(usize, usize, Option<f64>)>,
    ) -> PyResult<Vec<usize>> {
        let mut ids = Vec::with_capacity(edges.len());
        for (u, v, wopt) in edges.into_iter() {
            let w = wopt.unwrap_or(1.0);
            if !w.is_finite() {
                return Err(PyValueError::new_err(format!(
                    "Edge weight must be finite, got: {}",
                    w
                )));
            }
            let src = self
                .mapper
                .get_internal(u)
                .ok_or_else(|| PyValueError::new_err(format!("Invalid source node id: {}", u)))?;
            let dst = self
                .mapper
                .get_internal(v)
                .ok_or_else(|| PyValueError::new_err(format!("Invalid target node id: {}", v)))?;
            ids.push(self.graph.add_edge(src, dst, w).index());
        }
        Ok(ids)
    }

    // Edges/nodes view helpers expected by tests
    // Edges/nodes view helpers expected by tests
    #[getter]
    pub fn edges(slf: PyRef<'_, Self>) -> PyResult<EdgeView> {
        let py = slf.py();
        Ok(EdgeView::new(slf.into_pyobject(py)?.into_any().unbind()))
    }

    #[pyo3(name = "_edges_with_weights")]
    pub fn edges_with_weights(&self) -> Vec<(usize, usize, f64)> {
        self.graph
            .edges()
            .filter_map(|(u, v, &w)| {
                let pu = self.mapper.get_py(u);
                let pv = self.mapper.get_py(v);
                match (pu, pv) {
                    (Some(a), Some(b)) => Some((a, b, w)),
                    _ => None,
                }
            })
            .collect()
    }

    pub fn nodes_with_attrs(&self) -> Vec<(usize, i64)> {
        self.graph
            .nodes()
            .filter_map(|(nid, &attr)| self.mapper.get_py(nid).map(|py| (py, attr)))
            .collect()
    }

    // Pythonic extras
    fn __len__(&self) -> usize {
        self.graph.node_count()
    }
    fn __contains__(&self, py_node: usize) -> bool {
        self.mapper.contains_py(py_node)
    }
    fn __repr__(&self) -> String {
        format!(
            "PyGraph(nodes={}, edges={})",
            self.graph.node_count(),
            self.graph.edge_count()
        )
    }
    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<PyAny>> {
        // Returns a proper Python iterator over node IDs
        let nodes = slf.nodes_impl();
        let py = slf.py();
        let py_list = pyo3::types::PyList::new(py, &nodes)?;
        let builtins = pyo3::types::PyModule::import(py, "builtins")?;
        let iter_obj = builtins.getattr("iter")?.call1((py_list,))?;
        Ok(iter_obj.into_pyobject(py)?.unbind())
    }
}

// Internal helper implementation for PyGraph (not exposed to Python)
impl PyGraph {
    pub(crate) fn populate_from_internal(&mut self, graph: graphina::core::types::Graph<i64, f64>) {
        self.clear_impl();
        let mut node_map: HashMap<NodeId, NodeId> = HashMap::new();
        // Nodes
        for (nid, &attr) in graph.nodes() {
            let new_id = self.graph.add_node(attr);
            let _ = self.mapper.add(new_id);
            node_map.insert(nid, new_id);
        }
        // Edges
        for (u, v, &w) in graph.edges() {
            let iu = node_map[&u];
            let iv = node_map[&v];
            self.graph.add_edge(iu, iv, w);
        }
    }
}
