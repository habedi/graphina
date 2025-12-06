//! PyGraphina - Python bindings for the Graphina graph library.
//!
//! This crate provides Python-accessible graph classes and algorithms.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

// Graph type modules
mod digraph;
mod digraph_ops;
mod graph;

// Algorithm and utility modules
mod approximation;
mod centrality;
mod community;
mod core;
mod links;
mod metrics;
mod mst;
mod parallel;
mod subgraphs;
mod traversal;

// Re-export graph types for use in other modules
pub use digraph::PyDiGraph;
pub use graph::PyGraph;

/// Top-level convenience wrappers for common functions
#[pyfunction]
fn diameter(graph: &PyGraph) -> Option<usize> {
    graph.diameter()
}
#[pyfunction]
fn radius(graph: &PyGraph) -> Option<usize> {
    graph.radius()
}
#[pyfunction]
fn transitivity(graph: &PyGraph) -> f64 {
    graph.transitivity()
}
#[pyfunction]
fn average_clustering(graph: &PyGraph) -> f64 {
    graph.average_clustering()
}

/// The Python module declaration.
#[pymodule]
fn pygraphina(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyGraph>()?;
    m.add_class::<PyDiGraph>()?;

    // Add Pythonic aliases (without "Py" prefix) for more intuitive API
    // This allows users to write: pg.Graph() instead of pg.PyGraph()
    m.add("Graph", m.getattr("PyGraph")?)?;
    m.add("DiGraph", m.getattr("PyDiGraph")?)?;

    // Register core generators at top-level for backward compatibility
    core::generators::register_generators(m)?;

    // Also expose a few commonly used functions at top-level for backward compatibility
    // Parallel algorithms
    m.add_function(wrap_pyfunction!(parallel::bfs_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(parallel::degrees_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(
        parallel::connected_components_parallel,
        m
    )?)?;

    // Approximation / links helpers
    m.add_function(wrap_pyfunction!(approximation::clique::max_clique, m)?)?;
    m.add_function(wrap_pyfunction!(approximation::clique::clique_removal, m)?)?;
    m.add_function(wrap_pyfunction!(
        approximation::clique::large_clique_size,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        approximation::vertex_cover::min_weighted_vertex_cover,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        approximation::clustering::average_clustering_approx,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(approximation::ramsey::ramsey_r2, m)?)?;

    // Metrics convenience
    m.add_function(wrap_pyfunction!(diameter, m)?)?;
    m.add_function(wrap_pyfunction!(radius, m)?)?;
    m.add_function(wrap_pyfunction!(transitivity, m)?)?;
    m.add_function(wrap_pyfunction!(average_clustering, m)?)?;

    // MST algorithms
    m.add_function(wrap_pyfunction!(mst::prim_mst, m)?)?;
    m.add_function(wrap_pyfunction!(mst::kruskal_mst, m)?)?;
    m.add_function(wrap_pyfunction!(mst::boruvka_mst, m)?)?;

    // Create namespaced submodules matching Graphina structure

    // Core submodules (kept as pygraphina.core.*)
    let core_mod = PyModule::new(m.py(), "core")?;
    core::generators::register_generators(&core_mod)?;
    m.add_submodule(&core_mod)?;

    // Extension modules (pygraphina.metrics.*, pygraphina.mst.*, etc.)
    let metrics_mod = PyModule::new(m.py(), "metrics")?;
    metrics::register_metrics(&metrics_mod)?;
    m.add_submodule(&metrics_mod)?;

    let mst_mod = PyModule::new(m.py(), "mst")?;
    mst::register_mst(&mst_mod)?;
    m.add_submodule(&mst_mod)?;

    let traversal_mod = PyModule::new(m.py(), "traversal")?;
    traversal::register_traversal(&traversal_mod)?;
    m.add_submodule(&traversal_mod)?;

    let subgraphs_mod = PyModule::new(m.py(), "subgraphs")?;
    subgraphs::register_subgraphs(&subgraphs_mod)?;
    m.add_submodule(&subgraphs_mod)?;

    let parallel_mod = PyModule::new(m.py(), "parallel")?;
    parallel::register_parallel(&parallel_mod)?;
    m.add_submodule(&parallel_mod)?;

    let centrality_mod = PyModule::new(m.py(), "centrality")?;
    centrality::register_centrality(&centrality_mod)?;
    m.add_submodule(&centrality_mod)?;

    let approximation_mod = PyModule::new(m.py(), "approximation")?;
    approximation::register_approximation(&approximation_mod)?;
    m.add_submodule(&approximation_mod)?;

    let community_mod = PyModule::new(m.py(), "community")?;
    community::register_community(&community_mod)?;
    m.add_submodule(&community_mod)?;

    let links_mod = PyModule::new(m.py(), "links")?;
    links::register_links(&links_mod)?;
    m.add_submodule(&links_mod)?;

    #[cfg(feature = "networkx")]
    {
        m.add_function(wrap_pyfunction!(to_networkx, m)?)?;
        m.add_function(wrap_pyfunction!(from_networkx, m)?)?;
        m.add_function(wrap_pyfunction!(to_node_dataframe, m)?)?;
        m.add_function(wrap_pyfunction!(to_edge_dataframe, m)?)?;
    }

    Ok(())
}

#[cfg(feature = "networkx")]
#[pyfunction]
fn to_networkx(py: pyo3::Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    let nx = PyModule::import(py, "networkx")?;

    // Try PyGraph first
    if let Ok(graph) = obj.extract::<PyRef<PyGraph>>() {
        let g = nx.getattr("Graph")?.call0()?;
        // Add nodes with 'attr'
        for (nid, &attr) in graph.graph.nodes() {
            if let Some(&py_id) = graph.internal_to_py.get(&nid) {
                g.call_method("add_node", (py_id,), None)?;
                let nodes_view = g.getattr("nodes")?;
                let node_entry = nodes_view.call_method1("__getitem__", (py_id,))?;
                node_entry.call_method1("__setitem__", ("attr", attr))?;
            }
        }
        // Add edges with 'weight' (set after creation to avoid kwargs complexity)
        for (u, v, &w) in graph.graph.edges() {
            let pu = graph.internal_to_py.get(&u).copied().unwrap();
            let pv = graph.internal_to_py.get(&v).copied().unwrap();
            g.call_method("add_edge", (pu, pv), None)?;
            let adj = g.getattr("adj")?;
            let u_adj = adj.call_method1("__getitem__", (pu,))?;
            let uv = u_adj.call_method1("__getitem__", (pv,))?;
            uv.call_method1("__setitem__", ("weight", w))?;
        }
        return Ok(g.into_pyobject(py)?.unbind());
    }

    // Try PyDiGraph
    if let Ok(digraph) = obj.extract::<PyRef<PyDiGraph>>() {
        let g = nx.getattr("DiGraph")?.call0()?;
        for (nid, &attr) in digraph.graph.nodes() {
            if let Some(&py_id) = digraph.internal_to_py.get(&nid) {
                g.call_method("add_node", (py_id,), None)?;
                let nodes_view = g.getattr("nodes")?;
                let node_entry = nodes_view.call_method1("__getitem__", (py_id,))?;
                node_entry.call_method1("__setitem__", ("attr", attr))?;
            }
        }
        for (u, v, &w) in digraph.graph.edges() {
            let pu = digraph.internal_to_py.get(&u).copied().unwrap();
            let pv = digraph.internal_to_py.get(&v).copied().unwrap();
            g.call_method("add_edge", (pu, pv), None)?;
            let adj = g.getattr("adj")?;
            let u_adj = adj.call_method1("__getitem__", (pu,))?;
            let uv = u_adj.call_method1("__getitem__", (pv,))?;
            uv.call_method1("__setitem__", ("weight", w))?;
        }
        return Ok(g.into_pyobject(py)?.unbind());
    }

    Err(PyValueError::new_err(
        "to_networkx expects a PyGraph or PyDiGraph instance",
    ))
}

#[cfg(feature = "networkx")]
#[pyfunction]
fn from_networkx(py: pyo3::Python<'_>, nx_graph: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    // Determine directedness via method is_directed()
    let directed: bool = nx_graph.call_method0("is_directed")?.extract::<bool>()?;

    use pyo3::types::PyIterator;

    if directed {
        let cell = Py::new(py, PyDiGraph::new())?;
        let mut dg = cell.borrow_mut(py);
        let mut map: HashMap<String, usize> = HashMap::new();

        // nodes(data=True)
        let kwargs = PyDict::new(py);
        kwargs.set_item("data", true)?;
        let nodes_view = nx_graph.call_method("nodes", (), Some(&kwargs))?;
        let nodes_iter = PyIterator::from_object(&nodes_view)?;
        for item in nodes_iter {
            let item = item?;
            let (node_obj, attrs): (Bound<PyAny>, Bound<PyAny>) = item.extract()?;
            let node_key = node_obj.str()?.to_string();
            let attr: i64 = attrs
                .get_item("attr")
                .ok()
                .and_then(|v| v.extract().ok())
                .unwrap_or(0);
            // Try to use the original id if it's a small integer
            let py_id = if let Ok(int_id) = node_obj.extract::<usize>() {
                // If node_key is an integer that fits in usize, use it directly
                // but we still need to assign internal id ourselves
                let nid = dg.graph.add_node(attr);
                let assigned = dg.next_id;
                dg.py_to_internal.insert(assigned, nid);
                dg.internal_to_py.insert(nid, assigned);
                dg.next_id = dg.next_id.max(int_id + 1);
                // remap: use int_id as py id if possible
                dg.py_to_internal.remove(&assigned);
                dg.py_to_internal.insert(int_id, nid);
                dg.internal_to_py.insert(nid, int_id);
                int_id
            } else {
                let nid = dg.graph.add_node(attr);
                let py_id = dg.next_id;
                dg.py_to_internal.insert(py_id, nid);
                dg.internal_to_py.insert(nid, py_id);
                dg.next_id += 1;
                py_id
            };
            map.insert(node_key, py_id);
        }

        // edges(data=True)
        let edges_view = nx_graph.call_method("edges", (), Some(&kwargs))?;
        let edges_iter = PyIterator::from_object(&edges_view)?;
        for item in edges_iter {
            let item = item?;
            let (u_obj, v_obj, eattrs): (Bound<PyAny>, Bound<PyAny>, Bound<PyAny>) =
                item.extract()?;
            let uk = u_obj.str()?.to_string();
            let vk = v_obj.str()?.to_string();
            let weight: f64 = eattrs
                .get_item("weight")
                .ok()
                .and_then(|v| v.extract().ok())
                .unwrap_or(1.0);
            let pu = *map.get(&uk).unwrap();
            let pv = *map.get(&vk).unwrap();
            let g = &mut dg;
            g.add_edge(pu, pv, weight)?;
        }
        drop(dg);
        return Ok(cell.into_pyobject(py)?.unbind().into());
    } else {
        let cell = Py::new(py, PyGraph::new())?;
        let mut g = cell.borrow_mut(py);
        let mut map: HashMap<String, usize> = HashMap::new();

        let kwargs = PyDict::new(py);
        kwargs.set_item("data", true)?;
        let nodes_view = nx_graph.call_method("nodes", (), Some(&kwargs))?;
        let nodes_iter = PyIterator::from_object(&nodes_view)?;
        for item in nodes_iter {
            let item = item?;
            let (node_obj, attrs): (Bound<PyAny>, Bound<PyAny>) = item.extract()?;
            let node_key = node_obj.str()?.to_string();
            let attr: i64 = attrs
                .get_item("attr")
                .ok()
                .and_then(|v| v.extract().ok())
                .unwrap_or(0);
            let py_id = if let Ok(int_id) = node_obj.extract::<usize>() {
                let nid = g.graph.add_node(attr);
                let assigned = g.next_id;
                g.py_to_internal.insert(assigned, nid);
                g.internal_to_py.insert(nid, assigned);
                g.next_id = g.next_id.max(int_id + 1);
                g.py_to_internal.remove(&assigned);
                g.py_to_internal.insert(int_id, nid);
                g.internal_to_py.insert(nid, int_id);
                int_id
            } else {
                let nid = g.graph.add_node(attr);
                let py_id = g.next_id;
                g.py_to_internal.insert(py_id, nid);
                g.internal_to_py.insert(nid, py_id);
                g.next_id += 1;
                py_id
            };
            map.insert(node_key, py_id);
        }

        let edges_view = nx_graph.call_method("edges", (), Some(&kwargs))?;
        let edges_iter = PyIterator::from_object(&edges_view)?;
        for item in edges_iter {
            let item = item?;
            let (u_obj, v_obj, eattrs): (Bound<PyAny>, Bound<PyAny>, Bound<PyAny>) =
                item.extract()?;
            let uk = u_obj.str()?.to_string();
            let vk = v_obj.str()?.to_string();
            let weight: f64 = eattrs
                .get_item("weight")
                .ok()
                .and_then(|v| v.extract().ok())
                .unwrap_or(1.0);
            let pu = *map.get(&uk).unwrap();
            let pv = *map.get(&vk).unwrap();
            g.add_edge(pu, pv, weight)?;
        }
        drop(g);
        return Ok(cell.into_pyobject(py)?.unbind().into());
    }
}

/// Convert a PyGraph or PyDiGraph nodes to a pandas DataFrame.
///
/// Returns a DataFrame with columns: 'node_id' (int) and 'attr' (int).
#[cfg(feature = "networkx")]
#[pyfunction]
fn to_node_dataframe(py: pyo3::Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    let pd = PyModule::import(py, "pandas")?;

    // Try PyGraph first
    if let Ok(graph) = obj.extract::<PyRef<PyGraph>>() {
        let nodes_with_attrs = graph.nodes_with_attrs();
        let node_ids: Vec<usize> = nodes_with_attrs.iter().map(|(id, _)| *id).collect();
        let attrs: Vec<i64> = nodes_with_attrs.iter().map(|(_, attr)| *attr).collect();

        let kwargs = PyDict::new(py);
        kwargs.set_item("node_id", node_ids)?;
        kwargs.set_item("attr", attrs)?;

        let df = pd.call_method("DataFrame", (kwargs,), None)?;
        return Ok(df.into_pyobject(py)?.unbind());
    }

    // Try PyDiGraph
    if let Ok(digraph) = obj.extract::<PyRef<PyDiGraph>>() {
        let nodes_with_attrs: Vec<(usize, i64)> = digraph
            .graph
            .nodes()
            .filter_map(|(nid, &attr)| {
                digraph
                    .internal_to_py
                    .get(&nid)
                    .copied()
                    .map(|py| (py, attr))
            })
            .collect();
        let node_ids: Vec<usize> = nodes_with_attrs.iter().map(|(id, _)| *id).collect();
        let attrs: Vec<i64> = nodes_with_attrs.iter().map(|(_, attr)| *attr).collect();

        let kwargs = PyDict::new(py);
        kwargs.set_item("node_id", node_ids)?;
        kwargs.set_item("attr", attrs)?;

        let df = pd.call_method("DataFrame", (kwargs,), None)?;
        return Ok(df.into_pyobject(py)?.unbind());
    }

    Err(PyValueError::new_err(
        "to_node_dataframe expects a PyGraph or PyDiGraph instance",
    ))
}

/// Convert a PyGraph or PyDiGraph edges to a pandas DataFrame.
///
/// Returns a DataFrame with columns: 'source' (int), 'target' (int), and 'weight' (float).
#[cfg(feature = "networkx")]
#[pyfunction]
fn to_edge_dataframe(py: pyo3::Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    let pd = PyModule::import(py, "pandas")?;

    // Try PyGraph first
    if let Ok(graph) = obj.extract::<PyRef<PyGraph>>() {
        let edges = graph.edges_with_weights();
        let sources: Vec<usize> = edges.iter().map(|(u, _, _)| *u).collect();
        let targets: Vec<usize> = edges.iter().map(|(_, v, _)| *v).collect();
        let weights: Vec<f64> = edges.iter().map(|(_, _, w)| *w).collect();

        let kwargs = PyDict::new(py);
        kwargs.set_item("source", sources)?;
        kwargs.set_item("target", targets)?;
        kwargs.set_item("weight", weights)?;

        let df = pd.call_method("DataFrame", (kwargs,), None)?;
        return Ok(df.into_pyobject(py)?.unbind());
    }

    // Try PyDiGraph
    if let Ok(digraph) = obj.extract::<PyRef<PyDiGraph>>() {
        let edges: Vec<(usize, usize, f64)> = digraph
            .graph
            .edges()
            .filter_map(|(u, v, &w)| {
                let pu = digraph.internal_to_py.get(&u).copied();
                let pv = digraph.internal_to_py.get(&v).copied();
                match (pu, pv) {
                    (Some(a), Some(b)) => Some((a, b, w)),
                    _ => None,
                }
            })
            .collect();
        let sources: Vec<usize> = edges.iter().map(|(u, _, _)| *u).collect();
        let targets: Vec<usize> = edges.iter().map(|(_, v, _)| *v).collect();
        let weights: Vec<f64> = edges.iter().map(|(_, _, w)| *w).collect();

        let kwargs = PyDict::new(py);
        kwargs.set_item("source", sources)?;
        kwargs.set_item("target", targets)?;
        kwargs.set_item("weight", weights)?;

        let df = pd.call_method("DataFrame", (kwargs,), None)?;
        return Ok(df.into_pyobject(py)?.unbind());
    }

    Err(PyValueError::new_err(
        "to_edge_dataframe expects a PyGraph or PyDiGraph instance",
    ))
}
