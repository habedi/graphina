use crate::PyGraph;
use graphina::links::allocation::resource_allocation_index as resource_allocation_index_core;
use pyo3::prelude::*;
use std::collections::HashMap;

use super::similarity::{map_ebunch, map_pair_map_to_py};

#[pyfunction]
#[pyo3(signature = (py_graph, ebunch=None))]
pub fn resource_allocation_index(
    py_graph: &PyGraph,
    ebunch: Option<Vec<(usize, usize)>>,
) -> PyResult<HashMap<(usize, usize), f64>> {
    let res = match ebunch {
        Some(pairs) => {
            let mapped = map_ebunch(py_graph, &pairs)?;
            resource_allocation_index_core(&py_graph.graph, Some(&mapped))
        }
        None => resource_allocation_index_core(&py_graph.graph, None),
    };
    map_pair_map_to_py(py_graph, res)
}

pub fn register_allocation(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(resource_allocation_index, m)?)?;
    Ok(())
}
