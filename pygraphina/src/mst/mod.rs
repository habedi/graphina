use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::PyGraph;
use graphina::core::types::{Graph as CoreGraph, NodeId};
use graphina::mst::{
    MstEdge, boruvka_mst as boruvka_mst_core, kruskal_mst as kruskal_mst_core,
    prim_mst as prim_mst_core,
};
use ordered_float::OrderedFloat;

fn to_ordered_graph(
    py_graph: &PyGraph,
) -> (
    CoreGraph<i64, OrderedFloat<f64>>,
    std::collections::HashMap<NodeId, NodeId>,
) {
    let mut g: CoreGraph<i64, OrderedFloat<f64>> = CoreGraph::new();
    let mut old_to_new: std::collections::HashMap<NodeId, NodeId> =
        std::collections::HashMap::new();

    for (old_nid, &attr) in py_graph.graph.nodes() {
        let new_nid = g.add_node(attr);
        old_to_new.insert(old_nid, new_nid);
    }

    for (u, v, w) in py_graph.graph.edges() {
        let nu = old_to_new[&u];
        let nv = old_to_new[&v];
        g.add_edge(nu, nv, OrderedFloat((*w) as f64));
    }

    (g, old_to_new)
}

fn map_edges_to_py(
    py_graph: &PyGraph,
    new_to_old: &std::collections::HashMap<NodeId, NodeId>,
    edges: Vec<MstEdge<OrderedFloat<f64>>>,
) -> PyResult<Vec<(usize, usize, f64)>> {
    let mut out = Vec::with_capacity(edges.len());
    for e in edges.into_iter() {
        let ou = new_to_old
            .get(&e.u)
            .ok_or_else(|| PyValueError::new_err("missing mapping back to original node u"))?;
        let ov = new_to_old
            .get(&e.v)
            .ok_or_else(|| PyValueError::new_err("missing mapping back to original node v"))?;
        let pu = py_graph
            .internal_to_py
            .get(ou)
            .ok_or_else(|| PyValueError::new_err("missing node mapping for u"))?;
        let pv = py_graph
            .internal_to_py
            .get(ov)
            .ok_or_else(|| PyValueError::new_err("missing node mapping for v"))?;
        out.push((*pu, *pv, e.weight.0));
    }
    Ok(out)
}

/// Compute the Minimum Spanning Tree using Prim's algorithm.
#[pyfunction]
pub fn prim_mst(graph: &PyGraph) -> PyResult<(f64, Vec<(usize, usize, f64)>)> {
    let (og, old_to_new) = to_ordered_graph(graph);
    let mut new_to_old = std::collections::HashMap::new();
    for (old, new) in old_to_new.into_iter() {
        new_to_old.insert(new, old);
    }

    let (edges, total) =
        prim_mst_core(&og).map_err(|e| PyValueError::new_err(format!("Prim MST failed: {}", e)))?;
    let py_edges = map_edges_to_py(graph, &new_to_old, edges)?;
    Ok((total.0, py_edges))
}

/// Compute the Minimum Spanning Tree using Kruskal's algorithm.
#[pyfunction]
pub fn kruskal_mst(graph: &PyGraph) -> PyResult<(f64, Vec<(usize, usize, f64)>)> {
    let (og, old_to_new) = to_ordered_graph(graph);
    let mut new_to_old = std::collections::HashMap::new();
    for (old, new) in old_to_new.into_iter() {
        new_to_old.insert(new, old);
    }

    let (edges, total) = kruskal_mst_core(&og)
        .map_err(|e| PyValueError::new_err(format!("Kruskal MST failed: {}", e)))?;
    let py_edges = map_edges_to_py(graph, &new_to_old, edges)?;
    Ok((total.0, py_edges))
}

/// Compute the Minimum Spanning Tree using Borůvka's algorithm (parallel).
#[pyfunction]
pub fn boruvka_mst(graph: &PyGraph) -> PyResult<(f64, Vec<(usize, usize, f64)>)> {
    let (og, old_to_new) = to_ordered_graph(graph);
    let mut new_to_old = std::collections::HashMap::new();
    for (old, new) in old_to_new.into_iter() {
        new_to_old.insert(new, old);
    }

    let (edges, total) = boruvka_mst_core(&og)
        .map_err(|e| PyValueError::new_err(format!("Boruvka MST failed: {}", e)))?;
    let py_edges = map_edges_to_py(graph, &new_to_old, edges)?;
    Ok((total.0, py_edges))
}

pub fn register_mst(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(prim_mst, m)?)?;
    m.add_function(wrap_pyfunction!(kruskal_mst, m)?)?;
    m.add_function(wrap_pyfunction!(boruvka_mst, m)?)?;
    Ok(())
}
