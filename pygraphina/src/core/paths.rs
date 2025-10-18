use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use graphina::core::paths::{bellman_ford, dijkstra_path_f64, floyd_warshall};
use graphina::core::types::NodeId;

use crate::PyGraph;

impl PyGraph {
    /// Dijkstra shortest paths (f64 weights) from `start` with optional cutoff.
    pub fn dijkstra_impl(
        &self,
        start: usize,
        cutoff: Option<f64>,
    ) -> PyResult<HashMap<usize, Option<f64>>> {
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
    pub fn shortest_path_impl(
        &self,
        start: usize,
        target: usize,
    ) -> PyResult<Option<(f64, Vec<usize>)>> {
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
                    if p == cur {
                        break;
                    }
                    cur = p;
                    path_ids.push(cur);
                    if cur == start_id {
                        break;
                    }
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

    /// Bellman-Ford single-source shortest paths. Returns None if negative cycle is detected.
    pub fn bellman_ford_impl(&self, start: usize) -> PyResult<Option<HashMap<usize, Option<f64>>>> {
        let start_id = *self
            .py_to_internal
            .get(&start)
            .ok_or_else(|| PyValueError::new_err("Invalid start node id"))?;
        let res = bellman_ford(&self.graph, start_id);
        Ok(res.map(|nm| {
            let mut out = HashMap::new();
            for (nid, dist_opt) in nm.into_iter() {
                if let Some(py) = self.internal_to_py.get(&nid) {
                    out.insert(*py, dist_opt.map(|x| x as f64));
                }
            }
            out
        }))
    }

    /// Floyd–Warshall all-pairs shortest paths. Returns None if negative cycle.
    pub fn floyd_warshall_impl(&self) -> Option<HashMap<usize, HashMap<usize, Option<f64>>>> {
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
}
