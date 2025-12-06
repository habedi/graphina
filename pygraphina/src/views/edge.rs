use crate::PyDiGraph;
use crate::PyGraph;
use pyo3::exceptions::{PyKeyError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

/// a View on the edges of the graph.
#[pyclass]
pub struct EdgeView {
    graph: Py<PyAny>,
}

impl EdgeView {
    pub fn new(graph: Py<PyAny>) -> Self {
        Self { graph }
    }
}

#[pymethods]
impl EdgeView {
    fn __len__(&self, py: Python<'_>) -> PyResult<usize> {
        let obj = self.graph.bind(py);
        if let Ok(g) = obj.extract::<PyRef<PyGraph>>() {
            Ok(g.edge_count())
        } else if let Ok(g) = obj.extract::<PyRef<PyDiGraph>>() {
            Ok(g.edge_count())
        } else {
            Err(PyTypeError::new_err("Unknown graph type"))
        }
    }

    fn __contains__(&self, py: Python<'_>, edge: (usize, usize)) -> PyResult<bool> {
        let obj = self.graph.bind(py);
        if let Ok(g) = obj.extract::<PyRef<PyGraph>>() {
            g.contains_edge(edge.0, edge.1)
        } else if let Ok(g) = obj.extract::<PyRef<PyDiGraph>>() {
            g.contains_edge(edge.0, edge.1)
        } else {
            Err(PyTypeError::new_err("Unknown graph type"))
        }
    }

    fn __iter__(&self, py: Python<'_>) -> PyResult<Py<EdgeIterator>> {
        let obj = self.graph.bind(py);
        let edges = if let Ok(g) = obj.extract::<PyRef<PyGraph>>() {
            g.graph
                .edges()
                .filter_map(|(u, v, _w)| {
                    let pu = g.internal_to_py.get(&u).copied();
                    let pv = g.internal_to_py.get(&v).copied();
                    match (pu, pv) {
                        (Some(a), Some(b)) => Some((a, b)),
                        _ => None,
                    }
                })
                .collect()
        } else if let Ok(g) = obj.extract::<PyRef<PyDiGraph>>() {
            g.graph
                .edges()
                .filter_map(|(u, v, _w)| {
                    let pu = g.internal_to_py.get(&u).copied();
                    let pv = g.internal_to_py.get(&v).copied();
                    match (pu, pv) {
                        (Some(a), Some(b)) => Some((a, b)),
                        _ => None,
                    }
                })
                .collect()
        } else {
            return Err(PyTypeError::new_err("Unknown graph type"));
        };

        let iter = EdgeIterator { edges, idx: 0 };
        Py::new(py, iter)
    }

    fn __getitem__(&self, py: Python<'_>, edge: (usize, usize)) -> PyResult<Py<PyDict>> {
        let obj = self.graph.bind(py);
        let weight = if let Ok(g) = obj.extract::<PyRef<PyGraph>>() {
            g.get_edge_weight(edge.0, edge.1)?
        } else if let Ok(g) = obj.extract::<PyRef<PyDiGraph>>() {
            g.get_edge_weight(edge.0, edge.1)?
        } else {
            return Err(PyTypeError::new_err("Unknown graph type"));
        };

        if let Some(w) = weight {
            let dict = PyDict::new(py);
            dict.set_item("weight", w)?;
            Ok(dict.into())
        } else {
            Err(PyKeyError::new_err(format!("Edge {:?} not found", edge)))
        }
    }

    #[pyo3(signature = (data=None, default=None))]
    fn data(
        &self,
        py: Python<'_>,
        data: Option<PyObject>,
        default: Option<PyObject>,
    ) -> PyResult<EdgeDataView> {
        Ok(EdgeDataView {
            graph: self.graph.clone_ref(py),
            data_param: data,
            default_val: default,
        })
    }
}

#[pyclass]
pub struct EdgeIterator {
    edges: Vec<(usize, usize)>,
    idx: usize,
}

#[pymethods]
impl EdgeIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(usize, usize)> {
        if slf.idx < slf.edges.len() {
            let item = slf.edges[slf.idx];
            slf.idx += 1;
            Some(item)
        } else {
            None
        }
    }
}

#[pyclass]
pub struct EdgeDataView {
    graph: Py<PyAny>,
    data_param: Option<PyObject>,
    default_val: Option<PyObject>,
}

#[pymethods]
impl EdgeDataView {
    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<EdgeDataIterator>> {
        let py = slf.py();
        let obj = slf.graph.bind(py);

        let edges_w = if let Ok(g) = obj.extract::<PyRef<PyGraph>>() {
            g.edges_with_weights()
        } else if let Ok(g) = obj.extract::<PyRef<PyDiGraph>>() {
            // Reimplement edges_with_weights logic for PyDiGraph if it doesn't exist
            g.graph
                .edges()
                .map(|(u, v, w)| {
                    let pu = g.internal_to_py[&u];
                    let pv = g.internal_to_py[&v];
                    (pu, pv, *w)
                })
                .collect()
        } else {
            return Err(PyTypeError::new_err("Unknown graph type"));
        };

        let iter = EdgeDataIterator {
            edges: edges_w,
            data_param: slf.data_param.as_ref().map(|o| o.clone_ref(py)),
            default_val: slf.default_val.as_ref().map(|o| o.clone_ref(py)),
            idx: 0,
        };
        Py::new(py, iter)
    }

    fn __len__(&self, py: Python<'_>) -> PyResult<usize> {
        let obj = self.graph.bind(py);
        obj.call_method0("edge_count")?.extract()
    }
}

#[pyclass]
pub struct EdgeDataIterator {
    edges: Vec<(usize, usize, f64)>,
    data_param: Option<PyObject>,
    default_val: Option<PyObject>,
    idx: usize,
}

#[pymethods]
impl EdgeDataIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> PyResult<Option<PyObject>> {
        if slf.idx >= slf.edges.len() {
            return Ok(None);
        }
        let (u, v, w) = slf.edges[slf.idx];
        slf.idx += 1;

        let py = slf.py();

        let val_obj = if let Some(ref d) = slf.data_param {
            if let Ok(s) = d.extract::<String>(py) {
                if s == "weight" {
                    w.into_pyobject(py)?.into_any().unbind()
                } else {
                    slf.default_val
                        .as_ref()
                        .map(|d| d.clone_ref(py))
                        .unwrap_or_else(|| py.None())
                }
            } else if let Ok(b) = d.extract::<bool>(py) {
                if b {
                    let dict = PyDict::new(py);
                    dict.set_item("weight", w)?;
                    dict.into_any().unbind()
                } else {
                    py.None()
                }
            } else {
                return Err(PyTypeError::new_err("Invalid data parameter"));
            }
        } else {
            let dict = PyDict::new(py);
            dict.set_item("weight", w)?;
            dict.into_any().unbind()
        };

        // Return (u, v, val)
        let tuple = (u, v, val_obj).into_pyobject(py)?;
        Ok(Some(tuple.into_any().unbind()))
    }
}
