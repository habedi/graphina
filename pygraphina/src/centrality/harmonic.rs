use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::PyGraph;
use crate::centrality::utils::to_ordered_graph;
use graphina::centrality::harmonic::harmonic_centrality;
use graphina::core::types::NodeId;

#[pyfunction]
pub fn harmonic(py_graph: &PyGraph) -> PyResult<HashMap<usize, f64>> {
    let (og, old_to_new) = to_ordered_graph(py_graph);
    let mut new_to_old: std::collections::HashMap<NodeId, NodeId> =
        std::collections::HashMap::new();
    for (old, new) in old_to_new.iter() {
        new_to_old.insert(*new, *old);
    }

    match harmonic_centrality(&og) {
        Ok(map) => {
            let mut out = HashMap::new();
            for (new_nid, val) in map.into_iter() {
                let old_nid = new_to_old.get(&new_nid).ok_or_else(|| {
                    PyValueError::new_err("missing mapping back to original node")
                })?;
                let pyid = py_graph
                    .internal_to_py
                    .get(old_nid)
                    .ok_or_else(|| PyValueError::new_err("Internal node id missing mapping"))?;
                out.insert(*pyid, val);
            }
            Ok(out)
        }
        Err(e) => Err(PyValueError::new_err(format!("harmonic failed: {}", e))),
    }
}

pub fn register_harmonic(m: &pyo3::prelude::Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(harmonic, m)?)?;
    Ok(())
}
