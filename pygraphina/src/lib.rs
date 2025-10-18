use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use graphina::core::types::{Graph, NodeId};
use graphina::core::traversal::{bfs, dfs};
use graphina::core::paths::dijkstra_path_f64;
use graphina::core::paths::{bellman_ford, floyd_warshall};
use graphina::core::io::{read_edge_list, write_edge_list};
use graphina::core::traversal::{iddfs, try_iddfs, bidis, try_bidirectional_search};
use graphina::core::metrics::{
    diameter, radius, average_clustering_coefficient, clustering_coefficient, transitivity,
    triangles, average_path_length, assortativity,
};
use graphina::core::validation::{
    is_empty as v_is_empty, is_connected as v_is_connected, has_negative_weights as v_has_negative,
    has_self_loops as v_has_self_loops, is_bipartite as v_is_bipartite, count_components as v_count_components,
};

/// A Python-accessible Graph class wrapping Graphina's core undirected graph.
///
/// This class uses `i64` as the node attribute type and `f64` as the edge weight type.
/// Internally, it maintains a mapping from Python-assigned node IDs (simple `usize` values)
/// to the Graphina `NodeId`s.
#[pyclass]
struct PyGraph {
    graph: Graph<i64, f64>,
    py_to_internal: HashMap<usize, NodeId>,
    internal_to_py: HashMap<NodeId, usize>,
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
            py_to_internal: HashMap::new(),
            internal_to_py: HashMap::new(),
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
        self.py_to_internal.insert(py_id, node_id);
        self.internal_to_py.insert(node_id, py_id);
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
        if let Some(node_id) = self.py_to_internal.get(&py_node) {
            Ok(self.graph.update_node(*node_id, new_attr))
        } else {
            Ok(false)
        }
    }

    /// Attempts to update the attribute of an existing node.
    ///
    /// Raises a ValueError on error.
    ///
    /// Example:
    ///     >>> g.try_update_node(0, 200)
    fn try_update_node(&mut self, py_node: usize, new_attr: i64) -> PyResult<()> {
        let node_id = self
            .py_to_internal
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err("Invalid node id"))?;
        self.graph
            .try_update_node(*node_id, new_attr)
            .map_err(|e| PyValueError::new_err(format!("Error: {}", e)))
    }

    /// Adds an edge between two nodes with the given weight.
    ///
    /// Returns the internal edge identifier (as an integer).
    ///
    /// Example:
    ///     >>> edge_id = g.add_edge(0, 1, 3.14)
    fn add_edge(&mut self, source: usize, target: usize, weight: f64) -> PyResult<usize> {
        let s_id = self
            .py_to_internal
            .get(&source)
            .ok_or_else(|| PyValueError::new_err("Invalid source node id"))?;
        let t_id = self
            .py_to_internal
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
        if let Some(node_id) = self.py_to_internal.get(&py_node).copied() {
            let result = self.graph.remove_node(node_id);
            if result.is_some() {
                self.py_to_internal.remove(&py_node);
                self.internal_to_py.remove(&node_id);
            }
            Ok(result)
        } else {
            Ok(None)
        }
    }

    /// Attempts to remove a node from the graph.
    ///
    /// Raises a ValueError if the node does not exist.
    ///
    /// Example:
    ///     >>> attr = g.try_remove_node(0)
    fn try_remove_node(&mut self, py_node: usize) -> PyResult<i64> {
        let node_id = self
            .py_to_internal
            .get(&py_node)
            .copied()
            .ok_or_else(|| PyValueError::new_err("Invalid node id"))?;
        let result = self
            .graph
            .try_remove_node(node_id)
            .map_err(|e| PyValueError::new_err(format!("Error: {}", e)))?;

        self.py_to_internal.remove(&py_node);
        self.internal_to_py.remove(&node_id);

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

    /// Returns True if the graph is directed (always False for PyGraph which wraps an undirected graph).
    fn is_directed(&self) -> bool {
        self.graph.is_directed()
    }

    /// Returns the density of the graph (0.0 if fewer than 2 nodes).
    fn density(&self) -> f64 {
        self.graph.density()
    }

    /// Returns True if the given Python node id exists in the graph.
    fn contains_node(&self, py_node: usize) -> bool {
        self.py_to_internal
            .get(&py_node)
            .map(|nid| self.graph.contains_node(*nid))
            .unwrap_or(false)
    }

    /// Returns True if there is an edge between the two Python node ids.
    fn contains_edge(&self, source: usize, target: usize) -> PyResult<bool> {
        let s_id = self
            .py_to_internal
            .get(&source)
            .ok_or_else(|| PyValueError::new_err("Invalid source node id"))?;
        let t_id = self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err("Invalid target node id"))?;
        Ok(self.graph.contains_edge(*s_id, *t_id))
    }

    /// Returns a list of all Python-level node IDs currently in the graph.
    fn nodes(&self) -> Vec<usize> {
        let mut ids: Vec<usize> = self.internal_to_py.values().copied().collect();
        ids.sort_unstable();
        ids
    }

    /// Returns a list of neighbors for the given node as Python-level node IDs.
    ///
    /// Example:
    ///     >>> neighbors = g.neighbors(0)
    fn neighbors(&self, py_node: usize) -> PyResult<Vec<usize>> {
        let node_id = self
            .py_to_internal
            .get(&py_node)
            .ok_or_else(|| PyValueError::new_err("Invalid node id"))?;

        let result = self
            .graph
            .neighbors(*node_id)
            .filter_map(|internal_neighbor| self.internal_to_py.get(&internal_neighbor).copied())
            .collect();

        Ok(result)
    }

    /// Breadth‑first search order starting from `start` (Python node id).
    fn bfs(&self, start: usize) -> PyResult<Vec<usize>> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err("Invalid start node id"))?;
        let visited = bfs(&self.graph, start_id);
        Ok(visited
            .into_iter()
            .filter_map(|nid| self.internal_to_py.get(&nid).copied())
            .collect())
    }

    /// Depth‑first search order starting from `start` (Python node id).
    fn dfs(&self, start: usize) -> PyResult<Vec<usize>> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err("Invalid start node id"))?;
        let visited = dfs(&self.graph, start_id);
        Ok(visited
            .into_iter()
            .filter_map(|nid| self.internal_to_py.get(&nid).copied())
            .collect())
    }

    /// Dijkstra shortest paths (f64 weights) from `start` with optional cutoff.
    /// Returns a dict mapping Python node id -> distance (float) or None if unreachable.
    #[pyo3(signature = (start, cutoff=None))]
    fn dijkstra(&self, start: usize, cutoff: Option<f64>) -> PyResult<HashMap<usize, Option<f64>>> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err("Invalid start node id"))?;
        let (costs, _trace) = dijkstra_path_f64(&self.graph, start_id, cutoff)
            .map_err(|e| PyValueError::new_err(format!("Dijkstra error: {}", e)))?;
        let mut out: HashMap<usize, Option<f64>> = HashMap::new();
        for (node_id, dist_opt) in costs.into_iter() {
            if let Some(py_id) = self.internal_to_py.get(&node_id) {
                out.insert(*py_id, dist_opt);
            }
        }
        Ok(out)
    }

    /// Shortest path from start to target using Dijkstra (f64 weights).
    /// Returns None if target is unreachable, otherwise (total_cost, path_py_node_ids).
    fn shortest_path(&self, start: usize, target: usize) -> PyResult<Option<(f64, Vec<usize>)>> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err("Invalid start node id"))?;
        let target_id = *self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err("Invalid target node id"))?;
        let (costs, prev) = dijkstra_path_f64(&self.graph, start_id, None)
            .map_err(|e| PyValueError::new_err(format!("Dijkstra error: {}", e)))?;
        match costs[&target_id] {
            Some(total) => {
                // Reconstruct path via prev map
                let mut path_ids: Vec<NodeId> = Vec::new();
                let mut cur = target_id;
                path_ids.push(cur);
                while let Some(p) = prev[&cur] {
                    if p == cur { break; }
                    cur = p;
                    path_ids.push(cur);
                    if cur == start_id { break; }
                }
                if *path_ids.last().unwrap() != start_id {
                    return Ok(None);
                }
                path_ids.reverse();
                let py_path: Vec<usize> = path_ids
                    .into_iter()
                    .filter_map(|nid| self.internal_to_py.get(&nid).copied())
                    .collect();
                Ok(Some((total, py_path)))
            }
            None => Ok(None),
        }
    }

    /// Iterative Deepening DFS path from start to target up to max_depth. Returns None if not found.
    fn iddfs(&self, start: usize, target: usize, max_depth: usize) -> PyResult<Option<Vec<usize>>> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err("Invalid start node id"))?;
        let target_id = *self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err("Invalid target node id"))?;
        let path_opt = iddfs(&self.graph, start_id, target_id, max_depth);
        Ok(path_opt.map(|path| {
            path.into_iter()
                .filter_map(|nid| self.internal_to_py.get(&nid).copied())
                .collect()
        }))
    }

    /// Like iddfs(), but raises ValueError if no path exists within depth.
    fn try_iddfs(&self, start: usize, target: usize, max_depth: usize) -> PyResult<Vec<usize>> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err("Invalid start node id"))?;
        let target_id = *self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err("Invalid target node id"))?;
        let path = try_iddfs(&self.graph, start_id, target_id, max_depth)
            .map_err(|e| PyValueError::new_err(format!("{}", e)))?;
        Ok(path
            .into_iter()
            .filter_map(|nid| self.internal_to_py.get(&nid).copied())
            .collect())
    }

    /// Bidirectional BFS shortest path from start to target, or None if not found.
    fn bidirectional_search(&self, start: usize, target: usize) -> PyResult<Option<Vec<usize>>> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err("Invalid start node id"))?;
        let target_id = *self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err("Invalid target node id"))?;
        let path_opt = bidis(&self.graph, start_id, target_id);
        Ok(path_opt.map(|path| {
            path.into_iter()
                .filter_map(|nid| self.internal_to_py.get(&nid).copied())
                .collect()
        }))
    }

    /// Like bidirectional_search(), but raises ValueError if no path exists.
    fn try_bidirectional_search(&self, start: usize, target: usize) -> PyResult<Vec<usize>> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err("Invalid start node id"))?;
        let target_id = *self
            .py_to_internal
            .get(&target)
            .ok_or_else(|| PyValueError::new_err("Invalid target node id"))?;
        let path = try_bidirectional_search(&self.graph, start_id, target_id)
            .map_err(|e| PyValueError::new_err(format!("{}", e)))?;
        Ok(path
            .into_iter()
            .filter_map(|nid| self.internal_to_py.get(&nid).copied())
            .collect())
    }

    /// Clear the graph and reset node id mappings.
    fn clear(&mut self) {
        self.graph = Graph::new();
        self.py_to_internal.clear();
        self.internal_to_py.clear();
        self.next_id = 0;
    }

    /// Get the degree of a node (Python node id). Returns None if not found.
    fn degree(&self, py_node: usize) -> Option<usize> {
        self.py_to_internal
            .get(&py_node)
            .and_then(|nid| self.graph.degree(*nid))
    }

    /// Return the attribute of a node (Python node id). Returns None if not found.
    fn get_node_attr(&self, py_node: usize) -> Option<i64> {
        self.py_to_internal
            .get(&py_node)
            .and_then(|nid| self.graph.node_attr(*nid).copied())
    }

    /// Load an edge list file into this graph. This resets the current graph contents.
    /// Lines starting with '#' or with trailing '#' comments are ignored.
    /// Returns (node_count, edge_count).
    #[pyo3(signature = (path, sep = " "))]
    fn load_edge_list(&mut self, path: &str, sep: &str) -> PyResult<(usize, usize)> {
        let sep_char = sep.chars().next().ok_or_else(|| {
            PyValueError::new_err("Separator must be a non-empty string (first char used)")
        })?;

        // Read into a temporary Graph<i32, f64> using core I/O utilities
        let mut tmp: graphina::core::types::Graph<i32, f64> = graphina::core::types::Graph::new();
        read_edge_list(path, &mut tmp, sep_char)
            .map_err(|e| PyValueError::new_err(format!("Failed to read edge list: {}", e)))?;

        // Reset current graph and mappings
        self.clear();

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
    /// Note: node attributes are cast to i32 and weights to f32 to match the I/O routine.
    #[pyo3(signature = (path, sep = " "))]
    fn save_edge_list(&self, path: &str, sep: &str) -> PyResult<()> {
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

    /// Diameter (longest shortest path). None if graph is empty or disconnected.
    fn diameter(&self) -> Option<usize> { diameter(&self.graph) }

    /// Radius (minimum eccentricity). None if graph is empty or disconnected.
    fn radius(&self) -> Option<usize> { radius(&self.graph) }

    /// Average clustering coefficient over all nodes.
    fn average_clustering(&self) -> f64 { average_clustering_coefficient(&self.graph) }

    /// Local clustering coefficient for a given node id.
    fn clustering_of(&self, py_node: usize) -> PyResult<f64> {
        let nid = *self.py_to_internal.get(&py_node).ok_or_else(|| PyValueError::new_err("Invalid node id"))?;
        Ok(clustering_coefficient(&self.graph, nid))
    }

    /// Global transitivity (global clustering coefficient).
    fn transitivity(&self) -> f64 { transitivity(&self.graph) }

    /// Number of triangles containing the given node.
    fn triangles_of(&self, py_node: usize) -> PyResult<usize> {
        let nid = *self.py_to_internal.get(&py_node).ok_or_else(|| PyValueError::new_err("Invalid node id"))?;
        Ok(triangles(&self.graph, nid))
    }

    /// Average shortest path length. None if disconnected or empty.
    fn average_path_length(&self) -> Option<f64> { average_path_length(&self.graph) }

    /// Degree assortativity coefficient.
    fn assortativity(&self) -> f64 { assortativity(&self.graph) }

    /// Validation helpers.
    fn is_empty(&self) -> bool { v_is_empty(&self.graph) }
    fn is_connected(&self) -> bool { v_is_connected(&self.graph) }
    fn has_negative_weights(&self) -> bool { v_has_negative(&self.graph) }
    fn has_self_loops(&self) -> bool { v_has_self_loops(&self.graph) }
    fn is_bipartite(&self) -> bool { v_is_bipartite(&self.graph) }
    fn count_components(&self) -> usize { v_count_components(&self.graph) }

    /// Bellman-Ford single-source shortest paths. Returns None if negative cycle is detected.
    fn bellman_ford(&self, start: usize) -> PyResult<Option<HashMap<usize, Option<f64>>>> {
        let start_id = *self.py_to_internal.get(&start).ok_or_else(|| PyValueError::new_err("Invalid start node id"))?;
        let res = bellman_ford(&self.graph, start_id);
        Ok(res.map(|nm| {
            let mut out = HashMap::new();
            for (nid, dist_opt) in nm.into_iter() {
                if let Some(py) = self.internal_to_py.get(&nid) { out.insert(*py, dist_opt.map(|x| x as f64)); }
            }
            out
        }))
    }

    /// Floyd–Warshall all-pairs shortest paths. Returns None if negative cycle.
    fn floyd_warshall(&self) -> Option<HashMap<usize, HashMap<usize, Option<f64>>>> {
        floyd_warshall(&self.graph).map(|outer| {
            let mut out_outer: HashMap<usize, HashMap<usize, Option<f64>>> = HashMap::new();
            for (u, inner) in outer.into_iter() {
                if let Some(py_u) = self.internal_to_py.get(&u) {
                    let mut out_inner = HashMap::new();
                    for (v, dopt) in inner.into_iter() {
                        if let Some(py_v) = self.internal_to_py.get(&v) {
                            out_inner.insert(*py_v, dopt.map(|x| x as f64));
                        }
                    }
                    out_outer.insert(*py_u, out_inner);
                }
            }
            out_outer
        })
    }

    /// Save graph as JSON (nodes: i64, edges: f64).
    fn save_json(&self, path: &str) -> PyResult<()> {
        self.graph.save_json(path).map_err(|e| PyValueError::new_err(format!("{}", e)))
    }

    /// Load graph from JSON (resets current graph; expects nodes i64 and edges f64).
    fn load_json(&mut self, path: &str) -> PyResult<()> {
        let loaded = graphina::core::types::Graph::<i64, f64>::load_json(path)
            .map_err(|e| PyValueError::new_err(format!("{}", e)))?;
        self.clear();
        // rebuild mappings
        let mut map: HashMap<NodeId, NodeId> = HashMap::new();
        for (nid, &attr) in loaded.nodes() {
            let new_id = self.graph.add_node(attr);
            let py_id = self.next_id; self.next_id += 1;
            self.py_to_internal.insert(py_id, new_id);
            self.internal_to_py.insert(new_id, py_id);
            map.insert(nid, new_id);
        }
        for (u, v, &w) in loaded.edges() { let iu = map[&u]; let iv = map[&v]; self.graph.add_edge(iu, iv, w); }
        Ok(())
    }

    /// Save graph as binary file.
    fn save_binary(&self, path: &str) -> PyResult<()> {
        self.graph.save_binary(path).map_err(|e| PyValueError::new_err(format!("{}", e)))
    }

    /// Load graph from binary file (resets current graph; expects nodes i64 and edges f64).
    fn load_binary(&mut self, path: &str) -> PyResult<()> {
        let loaded = graphina::core::types::Graph::<i64, f64>::load_binary(path)
            .map_err(|e| PyValueError::new_err(format!("{}", e)))?;
        self.clear();
        let mut map: HashMap<NodeId, NodeId> = HashMap::new();
        for (nid, &attr) in loaded.nodes() {
            let new_id = self.graph.add_node(attr);
            let py_id = self.next_id; self.next_id += 1;
            self.py_to_internal.insert(py_id, new_id);
            self.internal_to_py.insert(new_id, py_id);
            map.insert(nid, new_id);
        }
        for (u, v, &w) in loaded.edges() { let iu = map[&u]; let iv = map[&v]; self.graph.add_edge(iu, iv, w); }
        Ok(())
    }

    /// Save graph in GraphML format.
    fn save_graphml(&self, path: &str) -> PyResult<()> {
        self.graph.save_graphml(path).map_err(|e| PyValueError::new_err(format!("{}", e)))
    }
}

/// The Python module declaration.
#[pymodule]
fn pygraphina(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyGraph>()?;
    Ok(())
}
