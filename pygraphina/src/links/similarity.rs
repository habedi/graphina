use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

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
            .py_to_internal
            .get(&pu)
            .ok_or_else(|| PyValueError::new_err("Invalid source node id in ebunch"))?;
        let iv = py_graph
            .py_to_internal
            .get(&pv)
            .ok_or_else(|| PyValueError::new_err("Invalid target node id in ebunch"))?;
        pairs.push((*iu, *iv));
    }
    Ok(pairs)
}

pub(super) fn map_pair_map_to_py(
    py_graph: &PyGraph,
    pairs: Vec<((NodeId, NodeId), f64)>,
) -> PyResult<HashMap<(usize, usize), f64>> {
    let mut out = HashMap::with_capacity(pairs.len());
    for ((u, v), score) in pairs.into_iter() {
        let pu = *py_graph
            .internal_to_py
            .get(&u)
            .ok_or_else(|| PyValueError::new_err("Missing node mapping for u"))?;
        let pv = *py_graph
            .internal_to_py
            .get(&v)
            .ok_or_else(|| PyValueError::new_err("Missing node mapping for v"))?;
        out.insert((pu, pv), score);
    }
    Ok(out)
}

#[pyfunction]
#[pyo3(signature = (py_graph, ebunch=None))]
pub fn jaccard_coefficient(
    py_graph: &PyGraph,
    ebunch: Option<Vec<(usize, usize)>>,
) -> PyResult<HashMap<(usize, usize), f64>> {
    let res = match ebunch {
        Some(pairs) => {
            let mapped = map_ebunch(py_graph, &pairs)?;
            jaccard_coefficient_core(&py_graph.graph, Some(&mapped))
        }
        None => jaccard_coefficient_core(&py_graph.graph, None),
    };
    map_pair_map_to_py(py_graph, res)
}

#[pyfunction]
#[pyo3(signature = (py_graph, ebunch=None))]
pub fn adamic_adar_index(
    py_graph: &PyGraph,
    ebunch: Option<Vec<(usize, usize)>>,
) -> PyResult<HashMap<(usize, usize), f64>> {
    let res = match ebunch {
        Some(pairs) => {
            let mapped = map_ebunch(py_graph, &pairs)?;
            adamic_adar_index_core(&py_graph.graph, Some(&mapped))
        }
        None => adamic_adar_index_core(&py_graph.graph, None),
    };
    map_pair_map_to_py(py_graph, res)
}

#[pyfunction]
pub fn common_neighbors(py_graph: &PyGraph, u: usize, v: usize) -> PyResult<usize> {
    let iu = *py_graph
        .py_to_internal
        .get(&u)
        .ok_or_else(|| PyValueError::new_err("Invalid source node id"))?;
    let iv = *py_graph
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
