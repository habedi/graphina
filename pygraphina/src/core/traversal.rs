use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use graphina::core::traversal::{bfs, bidis, dfs, iddfs, try_bidirectional_search, try_iddfs};

use crate::PyGraph;

impl PyGraph {
    /// Breadth‑first search order starting from `start` (Python node id).
    pub fn bfs_impl(&self, start: usize) -> PyResult<Vec<usize>> {
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
    pub fn dfs_impl(&self, start: usize) -> PyResult<Vec<usize>> {
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

    /// Iterative Deepening DFS path from start to target up to max_depth.
    pub fn iddfs_impl(
        &self,
        start: usize,
        target: usize,
        max_depth: usize,
    ) -> PyResult<Option<Vec<usize>>> {
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
    pub fn try_iddfs_impl(
        &self,
        start: usize,
        target: usize,
        max_depth: usize,
    ) -> PyResult<Vec<usize>> {
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
    pub fn bidirectional_search_impl(
        &self,
        start: usize,
        target: usize,
    ) -> PyResult<Option<Vec<usize>>> {
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
    pub fn try_bidirectional_search_impl(
        &self,
        start: usize,
        target: usize,
    ) -> PyResult<Vec<usize>> {
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
}
