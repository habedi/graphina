use crate::PyDiGraph;
use crate::PyGraph;
use pyo3::exceptions::{PyKeyError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::PyAnyMethods;

#[pyclass]
pub struct DegreeView {
    graph: Py<PyAny>,
}

impl DegreeView {
    pub fn new(graph: Py<PyAny>) -> Self {
        Self { graph }
    }
}

#[pymethods]
impl DegreeView {
    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<DegreeIterator>> {
        let py = slf.py();
        let obj = slf.graph.bind(py);

        let nodes = if let Ok(g) = obj.extract::<PyRef<PyGraph>>() {
            g.nodes_impl()
        } else if let Ok(g) = obj.extract::<PyRef<PyDiGraph>>() {
            g.graph
                .nodes()
                .filter_map(|(nid, _)| g.internal_to_py.get(&nid).copied())
                .collect()
        } else {
            return Err(PyTypeError::new_err("Unknown graph type"));
        };

        let iter = DegreeIterator {
            graph: slf.graph.clone_ref(py),
            nodes,
            idx: 0,
        };
        Py::new(py, iter)
    }

    fn __len__(&self, py: Python<'_>) -> PyResult<usize> {
        let obj = self.graph.bind(py);
        obj.call_method0("__len__")?.extract()
    }

    fn __getitem__(&self, py: Python<'_>, node: usize) -> PyResult<usize> {
        let obj = self.graph.bind(py);
        let deg = if let Ok(g) = obj.extract::<PyRef<PyGraph>>() {
            g.degree_impl(node)
        } else if let Ok(g) = obj.extract::<PyRef<PyDiGraph>>() {
            g.degree_impl(node)
        } else {
            return Err(PyTypeError::new_err("Unknown graph type"));
        };

        if let Some(d) = deg {
            Ok(d)
        } else {
            Err(PyKeyError::new_err(format!("Node {} not found", node)))
        }
    }

    #[pyo3(signature = (nbunch=None, weight=None))]
    fn __call__(
        &self,
        py: Python<'_>,
        nbunch: Option<PyObject>,
        weight: Option<String>,
    ) -> PyResult<PyObject> {
        // If nbunch is None, return self (the View).
        // If nbunch is not None, return a DegreeView of those nodes? Or list of (n, d)?
        // NetworkX returns a DiDegreeView if nbunch is provided? Or just iterator?
        // Actually G.degree([1,2]) returns a DegreeView over 1,2.
        // For simplicity, let's just return a list of (n, d) if nbunch is provided,
        // to mimic iterator behavior, or return self if None.
        // Wait, G.degree() (called with no args) returns self (the view).

        if nbunch.is_none() {
            // Return a new View (equivalent to self)
            return Ok(DegreeView {
                graph: self.graph.clone_ref(py),
            }
            .into_pyobject(py)?
            .into_any()
            .unbind());
        }

        // If nbunch is provided, we should filter.
        // Implementing restricted view is complex.
        // Let's just return a list of (node, degree) for now?
        // NX 2 returns a DegreeView.
        // If I return list, it might break code expecting view methods.
        // I will return a restricted iterator (DegreeIterator with subsets).

        let nbunch = nbunch.unwrap();
        // Extract nodes from nbunch (iterator or single node?)
        // If single node, return degree? No, G.degree(n) returns degree int in NX 1.x but NX 2.x?
        // NX 2: G.degree[n] returns int. G.degree(n) ?
        // "G.degree(nbunch) returns a DegreeView... if nbunch is a single node, returns degree?"
        // checking docs... "G.degree[n]" is the preferred way.
        // G.degree(n) returns the degree of node n (int).

        if let Ok(node) = nbunch.extract::<usize>(py) {
            return self
                .__getitem__(py, node)
                .map(|i| i.into_pyobject(py).unwrap().into_any().unbind());
        }

        // Assume iterable
        let iterator = nbunch.bind(py).try_iter()?;
        let mut nodes = Vec::new();
        for item in iterator {
            nodes.push(item?.extract::<usize>()?);
        }

        // Return iterator over these nodes
        let iter = DegreeIterator {
            graph: self.graph.clone_ref(py),
            nodes,
            idx: 0,
        };
        Ok(iter.into_pyobject(py)?.into_any().unbind())
    }
}

#[pyclass]
pub struct DegreeIterator {
    graph: Py<PyAny>,
    nodes: Vec<usize>,
    idx: usize,
}

#[pymethods]
impl DegreeIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> PyResult<Option<(usize, usize)>> {
        if slf.idx < slf.nodes.len() {
            let node = slf.nodes[slf.idx];
            slf.idx += 1;

            let py = slf.py();
            let obj = slf.graph.bind(py);
            let d = if let Ok(g) = obj.extract::<PyRef<PyGraph>>() {
                g.degree_impl(node).unwrap_or(0) // Handle missing node? shouldn't happen if filtered correctly
            } else if let Ok(g) = obj.extract::<PyRef<PyDiGraph>>() {
                g.degree_impl(node).unwrap_or(0)
            } else {
                0
            };
            Ok(Some((node, d)))
        } else {
            Ok(None)
        }
    }

    // For repr support or dict conversion
    fn __repr__(slf: PyRef<'_, Self>) -> String {
        "DegreeIterator(...)".to_string()
    }
}
