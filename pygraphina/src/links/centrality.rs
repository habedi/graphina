use crate::PyGraph;
use graphina::links::centrality::common_neighbor_centrality as common_neighbor_centrality_core;
use pyo3::prelude::*;
use std::collections::HashMap;

use super::similarity::{map_ebunch, map_pair_map_to_py};

#[pyfunction]
#[pyo3(signature = (py_graph, alpha, ebunch=None))]
pub fn common_neighbor_centrality(
    py_graph: &PyGraph,
    alpha: f64,
    ebunch: Option<Vec<(usize, usize)>>,
) -> PyResult<HashMap<(usize, usize), f64>> {
    let res = match ebunch {
        Some(pairs) => {
            let mapped = map_ebunch(py_graph, &pairs)?;
            common_neighbor_centrality_core(&py_graph.graph, Some(&mapped), alpha)
        }
        None => common_neighbor_centrality_core(&py_graph.graph, None, alpha),
    };
    map_pair_map_to_py(py_graph, res)
}

pub fn register_links_centrality(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(common_neighbor_centrality, m)?)?;
    Ok(())
}
