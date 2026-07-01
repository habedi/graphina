use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::PyGraph;
use graphina::core::types::NodeId;
use graphina::links::similarity::{
    adamic_adar_index as adamic_adar_index_core, common_neighbors as common_neighbors_core,
    jaccard_coefficient as jaccard_coefficient_core,
};

pub(super) fn map_ebunch(
    py_graph: &PyGraph,
    ebunch: &[(usize, usize)],
) -> PyResult<Vec<(NodeId, NodeId)>> {
    let mut pairs = Vec::with_capacity(ebunch.len());
    for &(pu, pv) in ebunch {
        let iu = py_graph
            .mapper
            .py_to_internal
            .get(&pu)
            .ok_or_else(|| PyValueError::new_err("Invalid source node id in ebunch"))?;
        let iv = py_graph
            .mapper
            .py_to_internal
            .get(&pv)
            .ok_or_else(|| PyValueError::new_err("Invalid target node id in ebunch"))?;
        pairs.push((*iu, *iv));
    }
    Ok(pairs)
}

/// Builds the `{(u, v): score}` Python dict directly, remapping internal node ids
/// to public ids. Building the `PyDict` in one pass avoids the intermediate
/// `std::HashMap` (default SipHash) that PyO3 would then re-convert; the
/// Python-visible return type is unchanged.
pub(super) fn map_pair_map_to_py(
    py: Python<'_>,
    py_graph: &PyGraph,
    pairs: Vec<((NodeId, NodeId), f64)>,
) -> PyResult<Py<PyDict>> {
    let dict = PyDict::new(py);
    for ((u, v), score) in pairs.into_iter() {
        let pu = *py_graph
            .mapper
            .internal_to_py
            .get(&u)
            .ok_or_else(|| PyValueError::new_err("Missing node mapping for u"))?;
        let pv = *py_graph
            .mapper
            .internal_to_py
            .get(&v)
            .ok_or_else(|| PyValueError::new_err("Missing node mapping for v"))?;
        dict.set_item((pu, pv), score)?;
    }
    Ok(dict.unbind())
}

#[pyfunction]
#[pyo3(signature = (py_graph, ebunch=None))]
pub fn jaccard_coefficient(
    py: Python<'_>,
    py_graph: &PyGraph,
    ebunch: Option<Vec<(usize, usize)>>,
) -> PyResult<Py<PyDict>> {
    let res = match ebunch {
        Some(pairs) => {
            let mapped = map_ebunch(py_graph, &pairs)?;
            jaccard_coefficient_core(&py_graph.graph, Some(&mapped))
        }
        None => jaccard_coefficient_core(&py_graph.graph, None),
    };
    map_pair_map_to_py(py, py_graph, res)
}

#[pyfunction]
#[pyo3(signature = (py_graph, ebunch=None))]
pub fn adamic_adar_index(
    py: Python<'_>,
    py_graph: &PyGraph,
    ebunch: Option<Vec<(usize, usize)>>,
) -> PyResult<Py<PyDict>> {
    let res = match ebunch {
        Some(pairs) => {
            let mapped = map_ebunch(py_graph, &pairs)?;
            adamic_adar_index_core(&py_graph.graph, Some(&mapped))
        }
        None => adamic_adar_index_core(&py_graph.graph, None),
    };
    map_pair_map_to_py(py, py_graph, res)
}

#[pyfunction]
pub fn common_neighbors(py_graph: &PyGraph, u: usize, v: usize) -> PyResult<usize> {
    let iu = *py_graph
        .mapper
        .py_to_internal
        .get(&u)
        .ok_or_else(|| PyValueError::new_err("Invalid source node id"))?;
    let iv = *py_graph
        .mapper
        .py_to_internal
        .get(&v)
        .ok_or_else(|| PyValueError::new_err("Invalid target node id"))?;
    Ok(common_neighbors_core(&py_graph.graph, iu, iv))
}

pub fn register_similarity(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(jaccard_coefficient, m)?)?;
    m.add_function(wrap_pyfunction!(adamic_adar_index, m)?)?;
    m.add_function(wrap_pyfunction!(common_neighbors, m)?)?;
    Ok(())
}
