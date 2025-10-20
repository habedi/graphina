use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use graphina::core::types::{BaseGraph, Digraph, NodeId, Undirected};
use pyo3::types::PyDict;

mod approximation;
mod centrality;
mod community;
mod core;
mod digraph_ops;
mod links;
mod metrics;
mod mst;
mod parallel;
mod subgraphs;
mod traversal;

/// A Python-accessible Graph class wrapping Graphina's core undirected graph.
///
/// This class uses `i64` as the node attribute type and `f64` as the edge weight type.
/// Internally, it maintains a mapping from Python-assigned node IDs (simple `usize` values)
/// to the Graphina `NodeId`s.
#[pyclass]
pub struct PyGraph {
    pub(crate) graph: BaseGraph<i64, f64, Undirected>,
    pub(crate) py_to_internal: HashMap<usize, NodeId>,
    pub(crate) internal_to_py: HashMap<NodeId, usize>,
    pub(crate) next_id: usize,
}

#[pymethods]
impl PyGraph {
    /// Creates a new, empty graph.
    #[new]
    fn new() -> Self {
        PyGraph {
            graph: BaseGraph::new(),
            py_to_internal: HashMap::new(),
            internal_to_py: HashMap::new(),
            next_id: 0,
        }
    }

    // Basic operations
    fn add_node(&mut self, attr: i64) -> usize {
        self.add_node_impl(attr)
    }
    fn update_node(&mut self, py_node: usize, new_attr: i64) -> PyResult<bool> {
        self.update_node_impl(py_node, new_attr)
    }
    fn try_update_node(&mut self, py_node: usize, new_attr: i64) -> PyResult<()> {
        self.try_update_node_impl(py_node, new_attr)
    }
    fn add_edge(&mut self, source: usize, target: usize, weight: f64) -> PyResult<usize> {
        self.add_edge_impl(source, target, weight)
    }
    fn remove_node(&mut self, py_node: usize) -> PyResult<Option<i64>> {
        self.remove_node_impl(py_node)
    }
    fn try_remove_node(&mut self, py_node: usize) -> PyResult<i64> {
        self.try_remove_node_impl(py_node)
    }
    fn node_count(&self) -> usize {
        self.graph.node_count()
    }
    fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
    fn is_directed(&self) -> bool {
        self.graph.is_directed()
    }
    fn density(&self) -> f64 {
        self.graph.density()
    }
    fn contains_node(&self, py_node: usize) -> bool {
        self.contains_node_impl(py_node)
    }
    fn contains_edge(&self, source: usize, target: usize) -> PyResult<bool> {
        self.contains_edge_impl(source, target)
    }
    fn nodes(&self) -> Vec<usize> {
        self.nodes_impl()
    }
    fn neighbors(&self, py_node: usize) -> PyResult<Vec<usize>> {
        self.neighbors_impl(py_node)
    }
    fn degree(&self, py_node: usize) -> Option<usize> {
        self.degree_impl(py_node)
    }
    fn get_node_attr(&self, py_node: usize) -> Option<i64> {
        self.get_node_attr_impl(py_node)
    }
    fn clear(&mut self) {
        self.clear_impl()
    }

    // Edge operations
    fn remove_edge(&mut self, source: usize, target: usize) -> PyResult<bool> {
        self.remove_edge_impl(source, target)
    }
    fn try_remove_edge(&mut self, source: usize, target: usize) -> PyResult<()> {
        self.try_remove_edge_impl(source, target)
    }
    fn get_edge_weight(&self, source: usize, target: usize) -> PyResult<Option<f64>> {
        self.get_edge_weight_impl(source, target)
    }
    fn update_edge_weight(
        &mut self,
        source: usize,
        target: usize,
        new_weight: f64,
    ) -> PyResult<bool> {
        self.update_edge_weight_impl(source, target, new_weight)
    }
    fn try_update_edge_weight(
        &mut self,
        source: usize,
        target: usize,
        new_weight: f64,
    ) -> PyResult<()> {
        self.try_update_edge_weight_impl(source, target, new_weight)
    }

    // Traversal
    fn bfs(&self, start: usize) -> PyResult<Vec<usize>> {
        self.bfs_impl(start)
    }
    fn dfs(&self, start: usize) -> PyResult<Vec<usize>> {
        self.dfs_impl(start)
    }
    fn iddfs(&self, start: usize, target: usize, max_depth: usize) -> PyResult<Option<Vec<usize>>> {
        self.iddfs_impl(start, target, max_depth)
    }
    fn try_iddfs(&self, start: usize, target: usize, max_depth: usize) -> PyResult<Vec<usize>> {
        self.try_iddfs_impl(start, target, max_depth)
    }
    fn bidirectional_search(&self, start: usize, target: usize) -> PyResult<Option<Vec<usize>>> {
        self.bidirectional_search_impl(start, target)
    }
    fn try_bidirectional_search(&self, start: usize, target: usize) -> PyResult<Vec<usize>> {
        self.try_bidirectional_search_impl(start, target)
    }

    // Subgraph operations
    fn subgraph(&self, nodes: Vec<usize>) -> PyResult<PyGraph> {
        self.subgraph_impl(nodes)
    }
    fn induced_subgraph(&self, nodes: Vec<usize>) -> PyResult<PyGraph> {
        self.induced_subgraph_impl(nodes)
    }
    fn ego_graph(&self, center: usize, radius: usize) -> PyResult<PyGraph> {
        self.ego_graph_impl(center, radius)
    }
    fn k_hop_neighbors(&self, start: usize, k: usize) -> PyResult<Vec<usize>> {
        self.k_hop_neighbors_impl(start, k)
    }
    fn connected_component(&self, start: usize) -> PyResult<Vec<usize>> {
        self.connected_component_impl(start)
    }
    fn component_subgraph(&self, start: usize) -> PyResult<PyGraph> {
        self.component_subgraph_impl(start)
    }

    // Filter operations
    fn filter_nodes(&self, predicate: &pyo3::prelude::Bound<'_, pyo3::PyAny>) -> PyResult<PyGraph> {
        self.filter_nodes_py_impl(predicate)
    }
    fn filter_edges(&self, predicate: &pyo3::prelude::Bound<'_, pyo3::PyAny>) -> PyResult<PyGraph> {
        self.filter_edges_py_impl(predicate)
    }

    // Metrics
    fn diameter(&self) -> Option<usize> {
        self.diameter_impl()
    }
    fn radius(&self) -> Option<usize> {
        self.radius_impl()
    }
    fn average_clustering(&self) -> f64 {
        self.average_clustering_impl()
    }
    fn clustering_of(&self, py_node: usize) -> PyResult<f64> {
        self.clustering_of_impl(py_node)
    }
    fn transitivity(&self) -> f64 {
        self.transitivity_impl()
    }
    fn triangles_of(&self, py_node: usize) -> PyResult<usize> {
        self.triangles_of_impl(py_node)
    }
    fn average_path_length(&self) -> Option<f64> {
        self.average_path_length_impl()
    }
    fn assortativity(&self) -> f64 {
        self.assortativity_impl()
    }

    // I/O operations
    #[pyo3(signature = (path, sep = " "))]
    fn load_edge_list(&mut self, path: &str, sep: &str) -> PyResult<(usize, usize)> {
        self.load_edge_list_impl(path, sep)
    }
    #[pyo3(signature = (path, sep = " "))]
    fn save_edge_list(&self, path: &str, sep: &str) -> PyResult<()> {
        self.save_edge_list_impl(path, sep)
    }
    fn save_json(&self, path: &str) -> PyResult<()> {
        self.save_json_impl(path)
    }
    fn load_json(&mut self, path: &str) -> PyResult<()> {
        self.load_json_impl(path)
    }
    fn save_binary(&self, path: &str) -> PyResult<()> {
        self.save_binary_impl(path)
    }
    fn load_binary(&mut self, path: &str) -> PyResult<()> {
        self.load_binary_impl(path)
    }
    fn save_graphml(&self, path: &str) -> PyResult<()> {
        self.save_graphml_impl(path)
    }

    // Paths (add bellman_ford and floyd_warshall to match PyGraph API)
    #[pyo3(signature = (start, cutoff=None))]
    fn dijkstra(
        &self,
        start: usize,
        cutoff: Option<f64>,
    ) -> PyResult<std::collections::HashMap<usize, Option<f64>>> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid start node id: {}", start)))?;
        let (costs, _trace) =
            graphina::core::paths::dijkstra_path_f64(&self.graph, start_id, cutoff)
                .map_err(|e| PyValueError::new_err(format!("Dijkstra error: {}", e)))?;
        let mut out = std::collections::HashMap::new();
        for (nid, dist) in costs.into_iter() {
            if let Some(pyid) = self.internal_to_py.get(&nid) {
                out.insert(*pyid, dist);
            }
        }
        Ok(out)
    }
    fn shortest_path(&self, start: usize, target: usize) -> PyResult<Option<(f64, Vec<usize>)>> {
        self.shortest_path_impl(start, target)
    }
    fn bellman_ford(&self, start: usize) -> PyResult<Option<HashMap<usize, Option<f64>>>> {
        self.bellman_ford_impl(start)
    }
    fn floyd_warshall(&self) -> Option<HashMap<usize, HashMap<usize, Option<f64>>>> {
        let all_pairs = graphina::core::paths::floyd_warshall(&self.graph);
        all_pairs.map(|m| {
            m.into_iter()
                .filter_map(|(u, inner)| {
                    self.internal_to_py.get(&u).copied().map(|pu| {
                        let inner_map: HashMap<usize, Option<f64>> = inner
                            .into_iter()
                            .filter_map(|(v, d)| {
                                self.internal_to_py.get(&v).copied().map(|pv| (pv, d))
                            })
                            .collect();
                        (pu, inner_map)
                    })
                })
                .collect()
        })
    }

    // Validation
    fn is_empty(&self) -> bool {
        self.is_empty_impl()
    }
    fn is_connected(&self) -> bool {
        self.is_connected_impl()
    }
    fn has_negative_weights(&self) -> bool {
        self.has_negative_weights_impl()
    }
    fn has_self_loops(&self) -> bool {
        self.has_self_loops_impl()
    }
    fn is_bipartite(&self) -> bool {
        self.is_bipartite_impl()
    }
    fn count_components(&self) -> usize {
        self.count_components_impl()
    }

    // Bulk operations
    fn add_nodes_from(&mut self, attrs: Vec<i64>) -> Vec<usize> {
        let mut ids = Vec::with_capacity(attrs.len());
        for a in attrs.into_iter() {
            let nid = self.graph.add_node(a);
            let py_id = self.next_id;
            self.py_to_internal.insert(py_id, nid);
            self.internal_to_py.insert(nid, py_id);
            self.next_id += 1;
            ids.push(py_id);
        }
        ids
    }
    fn add_edges_from(&mut self, edges: Vec<(usize, usize, Option<f64>)>) -> PyResult<Vec<usize>> {
        let mut ids = Vec::with_capacity(edges.len());
        for (u, v, wopt) in edges.into_iter() {
            let w = wopt.unwrap_or(1.0);
            if !w.is_finite() {
                return Err(PyValueError::new_err(format!(
                    "Edge weight must be finite, got: {}",
                    w
                )));
            }
            let src = *self
                .py_to_internal
                .get(&u)
                .ok_or_else(|| PyValueError::new_err(format!("Invalid source node id: {}", u)))?;
            let dst = *self
                .py_to_internal
                .get(&v)
                .ok_or_else(|| PyValueError::new_err(format!("Invalid target node id: {}", v)))?;
            ids.push(self.graph.add_edge(src, dst, w).index());
        }
        Ok(ids)
    }

    // Edges/nodes view helpers expected by tests
    fn edges(&self) -> Vec<(usize, usize)> {
        self.graph
            .edges()
            .filter_map(|(u, v, _w)| {
                let pu = self.internal_to_py.get(&u).copied();
                let pv = self.internal_to_py.get(&v).copied();
                match (pu, pv) {
                    (Some(a), Some(b)) => Some((a, b)),
                    _ => None,
                }
            })
            .collect()
    }

    fn edges_with_weights(&self) -> Vec<(usize, usize, f64)> {
        self.graph
            .edges()
            .filter_map(|(u, v, &w)| {
                let pu = self.internal_to_py.get(&u).copied();
                let pv = self.internal_to_py.get(&v).copied();
                match (pu, pv) {
                    (Some(a), Some(b)) => Some((a, b, w)),
                    _ => None,
                }
            })
            .collect()
    }

    fn nodes_with_attrs(&self) -> Vec<(usize, i64)> {
        self.graph
            .nodes()
            .filter_map(|(nid, &attr)| self.internal_to_py.get(&nid).copied().map(|py| (py, attr)))
            .collect()
    }

    // Pythonic extras
    fn __len__(&self) -> usize {
        self.graph.node_count()
    }
    fn __contains__(&self, py_node: usize) -> bool {
        self.py_to_internal.get(&py_node).is_some()
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
            let py_id = self.next_id;
            self.py_to_internal.insert(py_id, new_id);
            self.internal_to_py.insert(new_id, py_id);
            self.next_id += 1;
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

/// A Python-accessible DiGraph class wrapping Graphina's core directed graph.
///
/// This class uses `i64` as the node attribute type and `f64` as the edge weight type.
/// Internally, it maintains a mapping from Python-assigned node IDs (simple `usize` values)
/// to the Graphina `NodeId`s.
#[pyclass]
pub struct PyDiGraph {
    pub(crate) graph: Digraph<i64, f64>,
    pub(crate) py_to_internal: HashMap<usize, NodeId>,
    pub(crate) internal_to_py: HashMap<NodeId, usize>,
    pub(crate) next_id: usize,
}

#[pymethods]
impl PyDiGraph {
    /// Creates a new, empty directed graph.
    #[new]
    fn new() -> Self {
        PyDiGraph {
            graph: Digraph::new(),
            py_to_internal: HashMap::new(),
            internal_to_py: HashMap::new(),
            next_id: 0,
        }
    }

    // Basic node operations
    fn add_node(&mut self, attr: i64) -> usize {
        let nid = self.graph.add_node(attr);
        let py_id = self.next_id;
        self.py_to_internal.insert(py_id, nid);
        self.internal_to_py.insert(nid, py_id);
        self.next_id += 1;
        py_id
    }
    fn update_node(&mut self, py_node: usize, new_attr: i64) -> PyResult<bool> {
        self.update_node_impl(py_node, new_attr)
    }
    fn try_update_node(&mut self, py_node: usize, new_attr: i64) -> PyResult<()> {
        self.try_update_node_impl(py_node, new_attr)
    }
    fn remove_node(&mut self, py_node: usize) -> PyResult<Option<i64>> {
        self.remove_node_impl(py_node)
    }
    fn try_remove_node(&mut self, py_node: usize) -> PyResult<i64> {
        self.try_remove_node_impl(py_node)
    }
    fn get_node_attr(&self, py_node: usize) -> Option<i64> {
        self.get_node_attr_impl(py_node)
    }
    fn contains_node(&self, py_node: usize) -> bool {
        self.contains_node_impl(py_node)
    }
    fn clear(&mut self) {
        self.clear_impl()
    }

    // Edge operations
    fn add_edge(&mut self, source: usize, target: usize, weight: f64) -> PyResult<usize> {
        // Validate weight is finite
        if !weight.is_finite() {
            return Err(PyValueError::new_err(format!(
                "Edge weight must be finite, got: {}",
                weight
            )));
        }

        let src = *self
            .py_to_internal
            .get(&source)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid source node id: {}", source)))?;
        let dst = *self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid target node id: {}", target)))?;
        Ok(self.graph.add_edge(src, dst, weight).index())
    }

    fn remove_edge(&mut self, source: usize, target: usize) -> PyResult<bool> {
        self.remove_edge_impl(source, target)
    }

    fn try_remove_edge(&mut self, source: usize, target: usize) -> PyResult<()> {
        self.try_remove_edge_impl(source, target)
    }

    fn get_edge_weight(&self, source: usize, target: usize) -> PyResult<Option<f64>> {
        self.get_edge_weight_impl(source, target)
    }

    fn update_edge_weight(
        &mut self,
        source: usize,
        target: usize,
        new_weight: f64,
    ) -> PyResult<bool> {
        self.update_edge_weight_impl(source, target, new_weight)
    }

    fn try_update_edge_weight(
        &mut self,
        source: usize,
        target: usize,
        new_weight: f64,
    ) -> PyResult<()> {
        self.try_update_edge_weight_impl(source, target, new_weight)
    }

    fn contains_edge(&self, source: usize, target: usize) -> PyResult<bool> {
        let src = *self
            .py_to_internal
            .get(&source)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid source node id: {}", source)))?;
        let dst = *self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid target node id: {}", target)))?;
        Ok(self.graph.contains_edge(src, dst))
    }

    // Node queries
    fn nodes(&self) -> Vec<usize> {
        self.graph
            .nodes()
            .filter_map(|(nid, _)| self.internal_to_py.get(&nid).copied())
            .collect()
    }

    fn neighbors(&self, py_node: usize) -> PyResult<Vec<usize>> {
        let nid = *self
            .py_to_internal
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid node id: {}", py_node)))?;
        Ok(self
            .graph
            .neighbors(nid)
            .filter_map(|n| self.internal_to_py.get(&n).copied())
            .collect())
    }

    fn out_neighbors(&self, py_node: usize) -> PyResult<Vec<usize>> {
        self.out_neighbors_impl(py_node)
    }

    fn in_neighbors(&self, py_node: usize) -> PyResult<Vec<usize>> {
        self.in_neighbors_impl(py_node)
    }

    fn degree(&self, py_node: usize) -> Option<usize> {
        self.degree_impl(py_node)
    }

    fn in_degree(&self, py_node: usize) -> Option<usize> {
        self.in_degree_impl(py_node)
    }

    fn out_degree(&self, py_node: usize) -> Option<usize> {
        self.out_degree_impl(py_node)
    }

    // Stats
    fn node_count(&self) -> usize {
        self.graph.node_count()
    }
    fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
    fn density(&self) -> f64 {
        self.graph.density()
    }
    fn is_directed(&self) -> bool {
        self.graph.is_directed()
    }
    fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }

    // Validation
    fn is_connected(&self) -> bool {
        self.is_connected_impl()
    }
    fn has_negative_weights(&self) -> bool {
        self.has_negative_weights_impl()
    }
    fn has_self_loops(&self) -> bool {
        self.has_self_loops_impl()
    }
    fn is_bipartite(&self) -> bool {
        self.is_bipartite_impl()
    }
    fn count_components(&self) -> usize {
        self.count_components_impl()
    }

    // Traversal
    fn bfs(&self, start: usize) -> PyResult<Vec<usize>> {
        self.bfs_impl(start)
    }
    fn dfs(&self, start: usize) -> PyResult<Vec<usize>> {
        self.dfs_impl(start)
    }
    fn iddfs(&self, start: usize, target: usize, max_depth: usize) -> PyResult<Option<Vec<usize>>> {
        self.iddfs_impl(start, target, max_depth)
    }
    fn try_iddfs(&self, start: usize, target: usize, max_depth: usize) -> PyResult<Vec<usize>> {
        self.try_iddfs_impl(start, target, max_depth)
    }
    fn bidirectional_search(&self, start: usize, target: usize) -> PyResult<Option<Vec<usize>>> {
        self.bidirectional_search_impl(start, target)
    }
    fn try_bidirectional_search(&self, start: usize, target: usize) -> PyResult<Vec<usize>> {
        self.try_bidirectional_search_impl(start, target)
    }

    // Subgraph operations
    fn subgraph(&self, nodes: Vec<usize>) -> PyResult<PyDiGraph> {
        self.subgraph_impl(nodes)
    }
    fn induced_subgraph(&self, nodes: Vec<usize>) -> PyResult<PyDiGraph> {
        self.induced_subgraph_impl(nodes)
    }
    fn ego_graph(&self, center: usize, radius: usize) -> PyResult<PyDiGraph> {
        self.ego_graph_impl(center, radius)
    }
    fn k_hop_neighbors(&self, start: usize, k: usize) -> PyResult<Vec<usize>> {
        self.k_hop_neighbors_impl(start, k)
    }
    fn connected_component(&self, start: usize) -> PyResult<Vec<usize>> {
        self.connected_component_impl(start)
    }
    fn component_subgraph(&self, start: usize) -> PyResult<PyDiGraph> {
        self.component_subgraph_impl(start)
    }

    // Filter operations
    fn filter_nodes(
        &self,
        predicate: &pyo3::prelude::Bound<'_, pyo3::PyAny>,
    ) -> PyResult<PyDiGraph> {
        self.filter_nodes_py_impl(predicate)
    }
    fn filter_edges(
        &self,
        predicate: &pyo3::prelude::Bound<'_, pyo3::PyAny>,
    ) -> PyResult<PyDiGraph> {
        self.filter_edges_py_impl(predicate)
    }

    // Metrics
    fn diameter(&self) -> Option<usize> {
        self.diameter_impl()
    }
    fn radius(&self) -> Option<usize> {
        self.radius_impl()
    }
    fn average_clustering(&self) -> f64 {
        self.average_clustering_impl()
    }
    fn clustering_of(&self, py_node: usize) -> PyResult<f64> {
        self.clustering_of_impl(py_node)
    }
    fn transitivity(&self) -> f64 {
        self.transitivity_impl()
    }
    fn triangles_of(&self, py_node: usize) -> PyResult<usize> {
        self.triangles_of_impl(py_node)
    }
    fn average_path_length(&self) -> Option<f64> {
        self.average_path_length_impl()
    }
    fn assortativity(&self) -> f64 {
        self.assortativity_impl()
    }

    // I/O operations
    #[pyo3(signature = (path, sep = " "))]
    fn load_edge_list(&mut self, path: &str, sep: &str) -> PyResult<(usize, usize)> {
        self.load_edge_list_impl(path, sep)
    }
    #[pyo3(signature = (path, sep = " "))]
    fn save_edge_list(&self, path: &str, sep: &str) -> PyResult<()> {
        self.save_edge_list_impl(path, sep)
    }
    fn save_json(&self, path: &str) -> PyResult<()> {
        self.save_json_impl(path)
    }
    fn load_json(&mut self, path: &str) -> PyResult<()> {
        self.load_json_impl(path)
    }
    fn save_binary(&self, path: &str) -> PyResult<()> {
        self.save_binary_impl(path)
    }
    fn load_binary(&mut self, path: &str) -> PyResult<()> {
        self.load_binary_impl(path)
    }
    fn save_graphml(&self, path: &str) -> PyResult<()> {
        self.save_graphml_impl(path)
    }

    // Paths (add bellman_ford and floyd_warshall to match PyGraph API)
    #[pyo3(signature = (start, cutoff=None))]
    fn dijkstra(
        &self,
        start: usize,
        cutoff: Option<f64>,
    ) -> PyResult<std::collections::HashMap<usize, Option<f64>>> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid start node id: {}", start)))?;
        let (costs, _trace) =
            graphina::core::paths::dijkstra_path_f64(&self.graph, start_id, cutoff)
                .map_err(|e| PyValueError::new_err(format!("Dijkstra error: {}", e)))?;
        let mut out = std::collections::HashMap::new();
        for (nid, dist) in costs.into_iter() {
            if let Some(pyid) = self.internal_to_py.get(&nid) {
                out.insert(*pyid, dist);
            }
        }
        Ok(out)
    }
    fn shortest_path(&self, start: usize, target: usize) -> PyResult<Option<(f64, Vec<usize>)>> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid start node id: {}", start)))?;
        let target_id = *self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid target node id: {}", target)))?;
        let (costs, prev) = graphina::core::paths::dijkstra_path_f64(&self.graph, start_id, None)
            .map_err(|e| PyValueError::new_err(format!("Dijkstra error: {}", e)))?;
        match costs[&target_id] {
            Some(total) => {
                let mut path = vec![target_id];
                let mut cur = target_id;
                while let Some(p) = prev[&cur] {
                    if p == cur {
                        break;
                    }
                    cur = p;
                    path.push(cur);
                    if cur == start_id {
                        break;
                    }
                }
                if *path.last().unwrap() != start_id {
                    return Ok(None);
                }
                path.reverse();
                let py_path: Vec<usize> = path
                    .into_iter()
                    .filter_map(|nid| self.internal_to_py.get(&nid).copied())
                    .collect();
                Ok(Some((total, py_path)))
            }
            None => Ok(None),
        }
    }

    fn bellman_ford(&self, start: usize) -> PyResult<Option<HashMap<usize, Option<f64>>>> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid start node id: {}", start)))?;
        let costs = graphina::core::paths::bellman_ford(&self.graph, start_id);
        Ok(costs.map(|map| {
            map.into_iter()
                .filter_map(|(nid, d)| self.internal_to_py.get(&nid).copied().map(|py| (py, d)))
                .collect()
        }))
    }

    fn floyd_warshall(&self) -> Option<HashMap<usize, HashMap<usize, Option<f64>>>> {
        let all_pairs = graphina::core::paths::floyd_warshall(&self.graph);
        all_pairs.map(|m| {
            m.into_iter()
                .filter_map(|(u, inner)| {
                    self.internal_to_py.get(&u).copied().map(|pu| {
                        let inner_map: HashMap<usize, Option<f64>> = inner
                            .into_iter()
                            .filter_map(|(v, d)| {
                                self.internal_to_py.get(&v).copied().map(|pv| (pv, d))
                            })
                            .collect();
                        (pu, inner_map)
                    })
                })
                .collect()
        })
    }

    // Bulk operations
    fn add_nodes_from(&mut self, attrs: Vec<i64>) -> Vec<usize> {
        let mut ids = Vec::with_capacity(attrs.len());
        for a in attrs.into_iter() {
            let nid = self.graph.add_node(a);
            let py_id = self.next_id;
            self.py_to_internal.insert(py_id, nid);
            self.internal_to_py.insert(nid, py_id);
            self.next_id += 1;
            ids.push(py_id);
        }
        ids
    }

    fn add_edges_from(&mut self, edges: Vec<(usize, usize, Option<f64>)>) -> PyResult<Vec<usize>> {
        let mut ids = Vec::with_capacity(edges.len());
        for (u, v, wopt) in edges.into_iter() {
            let w = wopt.unwrap_or(1.0);
            if !w.is_finite() {
                return Err(PyValueError::new_err(format!(
                    "Edge weight must be finite, got: {}",
                    w
                )));
            }
            let src = *self
                .py_to_internal
                .get(&u)
                .ok_or_else(|| PyValueError::new_err(format!("Invalid source node id: {}", u)))?;
            let dst = *self
                .py_to_internal
                .get(&v)
                .ok_or_else(|| PyValueError::new_err(format!("Invalid target node id: {}", v)))?;
            ids.push(self.graph.add_edge(src, dst, w).index());
        }
        Ok(ids)
    }

    // Pythonic extras
    fn __len__(&self) -> usize {
        self.graph.node_count()
    }
    fn __contains__(&self, py_node: usize) -> bool {
        self.py_to_internal.get(&py_node).is_some()
    }
    fn __repr__(&self) -> String {
        format!(
            "PyDiGraph(nodes={}, edges={})",
            self.graph.node_count(),
            self.graph.edge_count()
        )
    }
    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<PyAny>> {
        // Returns a proper Python iterator over node IDs
        let nodes: Vec<usize> = slf
            .graph
            .nodes()
            .filter_map(|(nid, _)| slf.internal_to_py.get(&nid).copied())
            .collect();
        let py = slf.py();
        let py_list = pyo3::types::PyList::new(py, &nodes)?;
        let builtins = pyo3::types::PyModule::import(py, "builtins")?;
        let iter_obj = builtins.getattr("iter")?.call1((py_list,))?;
        Ok(iter_obj.into_pyobject(py)?.unbind())
    }
}

impl PyDiGraph {
    /// Helper method to populate PyDiGraph from an internal Digraph<i64, f64>
    pub(crate) fn populate_from_internal(
        &mut self,
        graph: graphina::core::types::Digraph<i64, f64>,
    ) {
        self.clear_impl();
        let mut node_map: HashMap<NodeId, NodeId> = HashMap::new();

        // Add all nodes
        for (nid, &attr) in graph.nodes() {
            let new_id = self.graph.add_node(attr);
            let py_id = self.next_id;
            self.py_to_internal.insert(py_id, new_id);
            self.internal_to_py.insert(new_id, py_id);
            self.next_id += 1;
            node_map.insert(nid, new_id);
        }

        // Add all edges
        for (u, v, &w) in graph.edges() {
            let iu = node_map[&u];
            let iv = node_map[&v];
            self.graph.add_edge(iu, iv, w);
        }
    }
}

/// Top-level convenience wrappers for common functions
#[pyfunction]
fn diameter(graph: &PyGraph) -> Option<usize> {
    graph.diameter()
}
#[pyfunction]
fn radius(graph: &PyGraph) -> Option<usize> {
    graph.radius()
}
#[pyfunction]
fn transitivity(graph: &PyGraph) -> f64 {
    graph.transitivity()
}
#[pyfunction]
fn average_clustering(graph: &PyGraph) -> f64 {
    graph.average_clustering()
}

/// The Python module declaration.
#[pymodule]
fn pygraphina(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyGraph>()?;
    m.add_class::<PyDiGraph>()?;

    // Add Pythonic aliases (without "Py" prefix) for more intuitive API
    // This allows users to write: pg.Graph() instead of pg.PyGraph()
    m.add("Graph", m.getattr("PyGraph")?)?;
    m.add("DiGraph", m.getattr("PyDiGraph")?)?;

    // Register core generators at top-level for backward compatibility
    core::generators::register_generators(m)?;

    // Also expose a few commonly used functions at top-level for backward compatibility
    // Parallel algorithms
    m.add_function(wrap_pyfunction!(parallel::bfs_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(parallel::degrees_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(
        parallel::connected_components_parallel,
        m
    )?)?;

    // Approximation / links helpers
    m.add_function(wrap_pyfunction!(approximation::clique::max_clique, m)?)?;
    m.add_function(wrap_pyfunction!(approximation::clique::clique_removal, m)?)?;
    m.add_function(wrap_pyfunction!(
        approximation::clique::large_clique_size,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        approximation::vertex_cover::min_weighted_vertex_cover,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        approximation::clustering::average_clustering_approx,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(approximation::ramsey::ramsey_r2, m)?)?;

    // Metrics convenience
    m.add_function(wrap_pyfunction!(diameter, m)?)?;
    m.add_function(wrap_pyfunction!(radius, m)?)?;
    m.add_function(wrap_pyfunction!(transitivity, m)?)?;
    m.add_function(wrap_pyfunction!(average_clustering, m)?)?;

    // MST algorithms
    m.add_function(wrap_pyfunction!(mst::prim_mst, m)?)?;
    m.add_function(wrap_pyfunction!(mst::kruskal_mst, m)?)?;
    m.add_function(wrap_pyfunction!(mst::boruvka_mst, m)?)?;

    // Create namespaced submodules matching Graphina structure

    // Core submodules (kept as pygraphina.core.*)
    let core_mod = PyModule::new(m.py(), "core")?;
    core::generators::register_generators(&core_mod)?;
    m.add_submodule(&core_mod)?;

    // Extension modules (pygraphina.metrics.*, pygraphina.mst.*, etc.)
    let metrics_mod = PyModule::new(m.py(), "metrics")?;
    metrics::register_metrics(&metrics_mod)?;
    m.add_submodule(&metrics_mod)?;

    let mst_mod = PyModule::new(m.py(), "mst")?;
    mst::register_mst(&mst_mod)?;
    m.add_submodule(&mst_mod)?;

    let traversal_mod = PyModule::new(m.py(), "traversal")?;
    traversal::register_traversal(&traversal_mod)?;
    m.add_submodule(&traversal_mod)?;

    let subgraphs_mod = PyModule::new(m.py(), "subgraphs")?;
    subgraphs::register_subgraphs(&subgraphs_mod)?;
    m.add_submodule(&subgraphs_mod)?;

    let parallel_mod = PyModule::new(m.py(), "parallel")?;
    parallel::register_parallel(&parallel_mod)?;
    m.add_submodule(&parallel_mod)?;

    let centrality_mod = PyModule::new(m.py(), "centrality")?;
    centrality::register_centrality(&centrality_mod)?;
    m.add_submodule(&centrality_mod)?;

    let approximation_mod = PyModule::new(m.py(), "approximation")?;
    approximation::register_approximation(&approximation_mod)?;
    m.add_submodule(&approximation_mod)?;

    let community_mod = PyModule::new(m.py(), "community")?;
    community::register_community(&community_mod)?;
    m.add_submodule(&community_mod)?;

    let links_mod = PyModule::new(m.py(), "links")?;
    links::register_links(&links_mod)?;
    m.add_submodule(&links_mod)?;

    #[cfg(feature = "networkx")]
    {
        m.add_function(wrap_pyfunction!(to_networkx, m)?)?;
        m.add_function(wrap_pyfunction!(from_networkx, m)?)?;
    }

    Ok(())
}

#[cfg(feature = "networkx")]
#[pyfunction]
fn to_networkx(py: pyo3::Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    let nx = PyModule::import(py, "networkx")?;

    // Try PyGraph first
    if let Ok(graph) = obj.extract::<PyRef<PyGraph>>() {
        let g = nx.getattr("Graph")?.call0()?;
        // Add nodes with 'attr'
        for (nid, &attr) in graph.graph.nodes() {
            if let Some(&py_id) = graph.internal_to_py.get(&nid) {
                g.call_method("add_node", (py_id,), None)?;
                let nodes_view = g.getattr("nodes")?;
                let node_entry = nodes_view.call_method1("__getitem__", (py_id,))?;
                node_entry.call_method1("__setitem__", ("attr", attr))?;
            }
        }
        // Add edges with 'weight' (set after creation to avoid kwargs complexity)
        for (u, v, &w) in graph.graph.edges() {
            let pu = graph.internal_to_py.get(&u).copied().unwrap();
            let pv = graph.internal_to_py.get(&v).copied().unwrap();
            g.call_method("add_edge", (pu, pv), None)?;
            let adj = g.getattr("adj")?;
            let u_adj = adj.call_method1("__getitem__", (pu,))?;
            let uv = u_adj.call_method1("__getitem__", (pv,))?;
            uv.call_method1("__setitem__", ("weight", w))?;
        }
        return Ok(g.into_pyobject(py)?.unbind());
    }

    // Try PyDiGraph
    if let Ok(digraph) = obj.extract::<PyRef<PyDiGraph>>() {
        let g = nx.getattr("DiGraph")?.call0()?;
        for (nid, &attr) in digraph.graph.nodes() {
            if let Some(&py_id) = digraph.internal_to_py.get(&nid) {
                g.call_method("add_node", (py_id,), None)?;
                let nodes_view = g.getattr("nodes")?;
                let node_entry = nodes_view.call_method1("__getitem__", (py_id,))?;
                node_entry.call_method1("__setitem__", ("attr", attr))?;
            }
        }
        for (u, v, &w) in digraph.graph.edges() {
            let pu = digraph.internal_to_py.get(&u).copied().unwrap();
            let pv = digraph.internal_to_py.get(&v).copied().unwrap();
            g.call_method("add_edge", (pu, pv), None)?;
            let adj = g.getattr("adj")?;
            let u_adj = adj.call_method1("__getitem__", (pu,))?;
            let uv = u_adj.call_method1("__getitem__", (pv,))?;
            uv.call_method1("__setitem__", ("weight", w))?;
        }
        return Ok(g.into_pyobject(py)?.unbind());
    }

    Err(PyValueError::new_err(
        "to_networkx expects a PyGraph or PyDiGraph instance",
    ))
}

#[cfg(feature = "networkx")]
#[pyfunction]
fn from_networkx(py: pyo3::Python<'_>, nx_graph: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    // Determine directedness via method is_directed()
    let directed: bool = nx_graph.call_method0("is_directed")?.extract::<bool>()?;

    use pyo3::types::PyIterator;
    use std::collections::HashMap;

    if directed {
        let cell = Py::new(py, PyDiGraph::new())?;
        let mut dg = cell.borrow_mut(py);
        let mut map: HashMap<String, usize> = HashMap::new();

        // nodes(data=True)
        let kwargs = PyDict::new(py);
        kwargs.set_item("data", true)?;
        let nodes_view = nx_graph.call_method("nodes", (), Some(&kwargs))?;
        let nodes_iter = PyIterator::from_object(&nodes_view)?;
        for item in nodes_iter {
            let item = item?;
            let (node_obj, attrs): (Bound<PyAny>, Bound<PyAny>) = item.extract()?;
            let node_key = node_obj.str()?.to_string();
            let attr_val: i64 = match attrs.get_item("attr") {
                Ok(val) => val.extract().unwrap_or(0i64),
                Err(_) => 0,
            };
            // Try to preserve integer node IDs
            if let Ok(desired_py_id) = node_obj.extract::<usize>() {
                let new_id = dg.graph.add_node(attr_val);
                // Assign provided python id mapping
                dg.py_to_internal.insert(desired_py_id, new_id);
                dg.internal_to_py.insert(new_id, desired_py_id);
                if desired_py_id >= dg.next_id {
                    dg.next_id = desired_py_id + 1;
                }
                map.insert(node_key, desired_py_id);
            } else {
                let py_id = dg.add_node(attr_val);
                map.insert(node_key, py_id);
            }
        }
        // edges(data=True)
        let kwargs = PyDict::new(py);
        kwargs.set_item("data", true)?;
        let edges_view = nx_graph.call_method("edges", (), Some(&kwargs))?;
        let edges_iter = PyIterator::from_object(&edges_view)?;
        for item in edges_iter {
            let item = item?;
            let (u_obj, v_obj, data): (Bound<PyAny>, Bound<PyAny>, Bound<PyAny>) =
                item.extract()?;
            let u_key = u_obj.str()?.to_string();
            let v_key = v_obj.str()?.to_string();
            let pu = *map.get(&u_key).ok_or_else(|| {
                PyValueError::new_err("NetworkX edge source not found in node map")
            })?;
            let pv = *map.get(&v_key).ok_or_else(|| {
                PyValueError::new_err("NetworkX edge target not found in node map")
            })?;
            let weight: f64 = match data.get_item("weight") {
                Ok(val) => val.extract().unwrap_or(1.0),
                Err(_) => 1.0,
            };
            dg.add_edge(pu, pv, weight)?;
        }
        drop(dg);
        return Ok(cell.into_pyobject(py)?.unbind().into());
    } else {
        let cell = Py::new(py, PyGraph::new())?;
        let mut g = cell.borrow_mut(py);
        let mut map: HashMap<String, usize> = HashMap::new();

        // nodes(data=True)
        let kwargs = PyDict::new(py);
        kwargs.set_item("data", true)?;
        let nodes_view = nx_graph.call_method("nodes", (), Some(&kwargs))?;
        let nodes_iter = PyIterator::from_object(&nodes_view)?;
        for item in nodes_iter {
            let item = item?;
            let (node_obj, attrs): (Bound<PyAny>, Bound<PyAny>) = item.extract()?;
            let node_key = node_obj.str()?.to_string();
            let attr_val: i64 = match attrs.get_item("attr") {
                Ok(val) => val.extract().unwrap_or(0i64),
                Err(_) => 0,
            };
            if let Ok(desired_py_id) = node_obj.extract::<usize>() {
                let new_id = g.graph.add_node(attr_val);
                g.py_to_internal.insert(desired_py_id, new_id);
                g.internal_to_py.insert(new_id, desired_py_id);
                if desired_py_id >= g.next_id {
                    g.next_id = desired_py_id + 1;
                }
                map.insert(node_key, desired_py_id);
            } else {
                let py_id = g.add_node(attr_val);
                map.insert(node_key, py_id);
            }
        }
        // edges(data=True)
        let kwargs = PyDict::new(py);
        kwargs.set_item("data", true)?;
        let edges_view = nx_graph.call_method("edges", (), Some(&kwargs))?;
        let edges_iter = PyIterator::from_object(&edges_view)?;
        for item in edges_iter {
            let item = item?;
            let (u_obj, v_obj, data): (Bound<PyAny>, Bound<PyAny>, Bound<PyAny>) =
                item.extract()?;
            let u_key = u_obj.str()?.to_string();
            let v_key = v_obj.str()?.to_string();
            let pu = *map.get(&u_key).ok_or_else(|| {
                PyValueError::new_err("NetworkX edge source not found in node map")
            })?;
            let pv = *map.get(&v_key).ok_or_else(|| {
                PyValueError::new_err("NetworkX edge target not found in node map")
            })?;
            let weight: f64 = match data.get_item("weight") {
                Ok(val) => val.extract().unwrap_or(1.0),
                Err(_) => 1.0,
            };
            // For undirected graphs, ensure adding edge once is fine
            g.add_edge(pu, pv, weight)?;
        }
        drop(g);
        return Ok(cell.into_pyobject(py)?.unbind().into());
    }
}
