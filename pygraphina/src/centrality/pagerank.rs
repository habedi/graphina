use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::PyGraph;
use graphina::centrality::pagerank::pagerank as pagerank_core;

#[pyfunction]
pub fn pagerank(
    py_graph: &PyGraph,
    damping: f64,
    max_iter: usize,
    tolerance: f64,
) -> PyResult<HashMap<usize, f64>> {
    let res = pagerank_core(&py_graph.graph, damping, max_iter, tolerance)
        .map_err(|e| PyValueError::new_err(e.message))?;
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

pub fn register_pagerank(m: &pyo3::prelude::Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(pagerank, m)?)?;
    Ok(())
}
