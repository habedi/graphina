use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::PyGraph;
use graphina::centrality::katz::katz_centrality;

#[pyfunction]
pub fn katz(
    py_graph: &PyGraph,
    alpha: f64,
    max_iter: usize,
    tolerance: f64,
) -> PyResult<HashMap<usize, f64>> {
    // We don't support a beta callback from Python; pass None
    let res = katz_centrality(&py_graph.graph, alpha, None, max_iter, tolerance)
        .map_err(|e| PyValueError::new_err(format!("Katz centrality failed: {}", e.to_string())))?;
    let mut out = HashMap::new();
    for (nid, val) in res.into_iter() {
        let pyid = py_graph
            .internal_to_py
            .get(&nid)
            .ok_or_else(|| PyValueError::new_err("Internal node id missing mapping"))?;
        out.insert(*pyid, val);
    }
    Ok(out)
}

pub fn register_katz(m: &pyo3::prelude::Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(katz, m)?)?;
    Ok(())
}
