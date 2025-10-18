use pyo3::prelude::*;
use std::collections::HashMap;

use graphina::core::types::{Graph, NodeId};

mod approximation;
mod centrality;
mod community;
mod core;
mod links;

/// A Python-accessible Graph class wrapping Graphina's core undirected graph.
///
/// This class uses `i64` as the node attribute type and `f64` as the edge weight type.
/// Internally, it maintains a mapping from Python-assigned node IDs (simple `usize` values)
/// to the Graphina `NodeId`s.
#[pyclass]
pub struct PyGraph {
    pub(crate) graph: Graph<i64, f64>,
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
            graph: Graph::new(),
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

    // Paths
    #[pyo3(signature = (start, cutoff=None))]
    fn dijkstra(&self, start: usize, cutoff: Option<f64>) -> PyResult<HashMap<usize, Option<f64>>> {
        self.dijkstra_impl(start, cutoff)
    }
    fn shortest_path(&self, start: usize, target: usize) -> PyResult<Option<(f64, Vec<usize>)>> {
        self.shortest_path_impl(start, target)
    }
    fn bellman_ford(&self, start: usize) -> PyResult<Option<HashMap<usize, Option<f64>>>> {
        self.bellman_ford_impl(start)
    }
    fn floyd_warshall(&self) -> Option<HashMap<usize, HashMap<usize, Option<f64>>>> {
        self.floyd_warshall_impl()
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

    // I/O
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

    // Subgraphs
    fn subgraph(&self, nodes: Vec<usize>) -> PyResult<PyGraph> {
        self.subgraph_impl(nodes)
    }
    fn induced_subgraph(&self, nodes: Vec<usize>) -> PyResult<PyGraph> {
        self.induced_subgraph_impl(nodes)
    }
}

impl PyGraph {
    /// Helper method to populate PyGraph from an internal Graph<u32, f32>
    pub(crate) fn populate_from_internal(&mut self, graph: graphina::core::types::Graph<u32, f32>) {
        self.clear_impl();
        let mut node_map: HashMap<NodeId, NodeId> = HashMap::new();

        // Add all nodes
        for (nid, &attr) in graph.nodes() {
            let new_id = self.graph.add_node(attr as i64);
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
            self.graph.add_edge(iu, iv, w as f64);
        }
    }
}

/// The Python module declaration.
#[pymodule]
fn pygraphina(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyGraph>()?;

    // Register core module functions
    core::generators::register_generators(m)?;
    core::mst::register_mst(m)?;
    core::parallel::register_parallel(m)?;
    core::subgraphs::register_subgraphs(m)?;

    // Register centrality functions
    centrality::register_centrality(m)?;

    // Register additional modules
    approximation::register_approximation(m)?;
    community::register_community(m)?;
    links::register_links(m)?;

    Ok(())
}
