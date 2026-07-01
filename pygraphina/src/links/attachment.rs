use crate::PyGraph;
use graphina::links::attachment::preferential_attachment as preferential_attachment_core;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use super::similarity::{map_ebunch, map_pair_map_to_py};

#[pyfunction]
#[pyo3(signature = (py_graph, ebunch=None))]
pub fn preferential_attachment(
    py: Python<'_>,
    py_graph: &PyGraph,
    ebunch: Option<Vec<(usize, usize)>>,
) -> PyResult<Py<PyDict>> {
    let res = match ebunch {
        Some(pairs) => {
            let mapped = map_ebunch(py_graph, &pairs)?;
            preferential_attachment_core(&py_graph.graph, Some(&mapped))
        }
        None => preferential_attachment_core(&py_graph.graph, None),
    };
    map_pair_map_to_py(py, py_graph, res)
}

pub fn register_attachment(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(preferential_attachment, m)?)?;
    Ok(())
}
