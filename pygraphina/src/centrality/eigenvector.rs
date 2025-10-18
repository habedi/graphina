use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::PyGraph;
use graphina::centrality::eigenvector::eigenvector_centrality;

#[pyfunction]
pub fn eigenvector(
    py_graph: &PyGraph,
    max_iter: usize,
    tolerance: f64,
) -> PyResult<HashMap<usize, f64>> {
    match eigenvector_centrality(&py_graph.graph, max_iter, tolerance) {
        Ok(map) => {
            let mut out = HashMap::new();
            for (nid, val) in map.into_iter() {
                let pyid = py_graph
                    .internal_to_py
                    .get(&nid)
                    .ok_or_else(|| PyValueError::new_err("Internal node id missing mapping"))?;
                out.insert(*pyid, val);
            }
            Ok(out)
        }
        Err(e) => Err(PyValueError::new_err(format!("eigenvector failed: {}", e))),
    }
}

pub fn register_eigenvector(m: &pyo3::prelude::Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(eigenvector, m)?)?;
    Ok(())
}
