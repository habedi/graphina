use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::PyGraph;
use graphina::mst::{
    MstEdge, boruvka_mst as boruvka_mst_core, kruskal_mst as kruskal_mst_core,
    prim_mst as prim_mst_core,
};

/// Total weight of the tree together with its edges as `(u, v, weight)` triples.
type MstResult = PyResult<(f64, Vec<(usize, usize, f64)>)>;

/// Maps the tree edges back to Python node ids. The core algorithms run directly
/// on the stored `f64` graph, so no per-call copy of the graph is needed.
fn map_edges_to_py(
    py_graph: &PyGraph,
    edges: Vec<MstEdge<f64>>,
) -> PyResult<Vec<(usize, usize, f64)>> {
    let mut out = Vec::with_capacity(edges.len());
    for e in edges {
        let pu = py_graph
            .mapper
            .internal_to_py
            .get(&e.u)
            .ok_or_else(|| PyValueError::new_err("missing node mapping for u"))?;
        let pv = py_graph
            .mapper
            .internal_to_py
            .get(&e.v)
            .ok_or_else(|| PyValueError::new_err("missing node mapping for v"))?;
        out.push((*pu, *pv, e.weight));
    }
    Ok(out)
}

/// Compute the Minimum Spanning Tree using Prim's algorithm.
#[pyfunction]
pub fn prim_mst(graph: &PyGraph) -> MstResult {
    let (edges, total) = prim_mst_core(&graph.graph)
        .map_err(|e| PyValueError::new_err(format!("Prim MST failed: {}", e)))?;
    Ok((total, map_edges_to_py(graph, edges)?))
}

/// Compute the Minimum Spanning Tree using Kruskal's algorithm.
#[pyfunction]
pub fn kruskal_mst(graph: &PyGraph) -> MstResult {
    let (edges, total) = kruskal_mst_core(&graph.graph)
        .map_err(|e| PyValueError::new_err(format!("Kruskal MST failed: {}", e)))?;
    Ok((total, map_edges_to_py(graph, edges)?))
}

/// Compute the Minimum Spanning Tree using Borůvka's algorithm (parallel).
#[pyfunction]
pub fn boruvka_mst(graph: &PyGraph) -> MstResult {
    let (edges, total) = boruvka_mst_core(&graph.graph)
        .map_err(|e| PyValueError::new_err(format!("Boruvka MST failed: {}", e)))?;
    Ok((total, map_edges_to_py(graph, edges)?))
}

pub fn register_mst(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(prim_mst, m)?)?;
    m.add_function(wrap_pyfunction!(kruskal_mst, m)?)?;
    m.add_function(wrap_pyfunction!(boruvka_mst, m)?)?;
    Ok(())
}
