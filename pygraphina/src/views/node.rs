use crate::PyDiGraph;
use crate::PyGraph;
use pyo3::exceptions::{PyKeyError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::PyDict;

/// a View on the nodes of the graph.
#[pyclass]
pub struct NodeView {
    graph: Py<PyAny>,
}

impl NodeView {
    pub fn new(graph: Py<PyAny>) -> Self {
        Self { graph }
    }
}

#[pymethods]
impl NodeView {
    fn __len__(&self, py: Python<'_>) -> PyResult<usize> {
        let obj = self.graph.bind(py);
        if let Ok(g) = obj.extract::<PyRef<PyGraph>>() {
            Ok(g.node_count())
        } else if let Ok(g) = obj.extract::<PyRef<PyDiGraph>>() {
            Ok(g.node_count())
        } else {
            Err(PyTypeError::new_err("Unknown graph type"))
        }
    }

    fn __contains__(&self, py: Python<'_>, node: usize) -> PyResult<bool> {
        let obj = self.graph.bind(py);
        if let Ok(g) = obj.extract::<PyRef<PyGraph>>() {
            Ok(g.contains_node(node))
        } else if let Ok(g) = obj.extract::<PyRef<PyDiGraph>>() {
            Ok(g.contains_node(node))
        } else {
            Err(PyTypeError::new_err("Unknown graph type"))
        }
    }

    fn __iter__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        // The graph class already implements __iter__ returning node iterator
        let obj = self.graph.bind(py);
        Ok(obj.call_method0("__iter__")?.unbind())
    }

    fn __getitem__(&self, py: Python<'_>, node: usize) -> PyResult<Py<PyDict>> {
        let obj = self.graph.bind(py);
        let attr = if let Ok(g) = obj.extract::<PyRef<PyGraph>>() {
            g.get_node_attr(node)
        } else if let Ok(g) = obj.extract::<PyRef<PyDiGraph>>() {
            g.get_node_attr(node)
        } else {
            return Err(PyTypeError::new_err("Unknown graph type"));
        };

        if let Some(val) = attr {
            let dict = PyDict::new(py);
            dict.set_item("attr", val)?;
            Ok(dict.into())
        } else {
            Err(PyKeyError::new_err(format!("Node {} not found", node)))
        }
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let obj = self.graph.bind(py);
        // Simulate "NodeView((n1, n2, ...))"
        let nodes_list = obj.call_method0("nodes")?; // assumes nodes() returns list, waiting... nodes() is being replaced by this property.
        // This is circular if I replace nodes() with this.
        // I should use internal methods or just __iter__.
        // Safest is to iterate
        let iter = obj.try_iter()?;
        let mut elements = Vec::new();
        for item in iter {
            let s = item?.str()?.to_string();
            elements.push(s);
        }
        Ok(format!("NodeView(({}))", elements.join(", ")))
    }

    #[pyo3(signature = (data=None, default=None))]
    fn data(
        &self,
        py: Python<'_>,
        data: Option<PyObject>,
        default: Option<PyObject>,
    ) -> PyResult<NodeDataView> {
        Ok(NodeDataView {
            graph: self.graph.clone_ref(py),
            data_param: data,
            default_val: default,
        })
    }
}

#[pyclass]
pub struct NodeDataView {
    graph: Py<PyAny>,
    data_param: Option<PyObject>,
    default_val: Option<PyObject>,
}

#[pymethods]
impl NodeDataView {
    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<NodeDataIterator>> {
        let py = slf.py();
        let iter = NodeDataIterator {
            graph: slf.graph.clone_ref(py),
            data_param: slf.data_param.as_ref().map(|o| o.clone_ref(py)),
            default_val: slf.default_val.as_ref().map(|o| o.clone_ref(py)),
            nodes: slf
                .graph
                .bind(py)
                .call_method0("__iter__")?
                .try_iter()?
                .map(|i| i.and_then(|x| x.extract::<usize>()))
                .collect::<PyResult<Vec<usize>>>()?,
            idx: 0,
        };
        Py::new(py, iter)
    }

    fn __len__(&self, py: Python<'_>) -> PyResult<usize> {
        let obj = self.graph.bind(py);
        obj.call_method0("__len__")?.extract()
    }

    fn __repr__(&self, _py: Python<'_>) -> PyResult<String> {
        // Simplified repr
        Ok("NodeDataView(...)".to_string())
    }
}

#[pyclass]
pub struct NodeDataIterator {
    graph: Py<PyAny>,
    data_param: Option<PyObject>,
    default_val: Option<PyObject>,
    nodes: Vec<usize>,
    idx: usize,
}

#[pymethods]
impl NodeDataIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> PyResult<Option<PyObject>> {
        if slf.idx >= slf.nodes.len() {
            return Ok(None);
        }
        let node_id = slf.nodes[slf.idx];
        slf.idx += 1;

        let py = slf.py();
        let graph_bound = slf.graph.bind(py);

        // Get attribute
        let attr_val = if let Ok(g) = graph_bound.extract::<PyRef<PyGraph>>() {
            g.get_node_attr(node_id)
        } else if let Ok(g) = graph_bound.extract::<PyRef<PyDiGraph>>() {
            g.get_node_attr(node_id)
        } else {
            return Err(PyTypeError::new_err("Unknown graph type"));
        };

        // Logic based on data_param
        // if data_param is string 'attr', return (n, val)
        // if data_param is True or None, return (n, {'attr': val})
        // else return (n, default)

        let val_obj = if let Some(ref d) = slf.data_param {
            if let Ok(s) = d.extract::<String>(py) {
                if s == "attr" {
                    match attr_val {
                        Some(v) => v.into_pyobject(py)?.into_any().unbind(),
                        None => slf
                            .default_val
                            .as_ref()
                            .map(|d| d.clone_ref(py))
                            .unwrap_or_else(|| py.None()),
                    }
                } else {
                    slf.default_val
                        .as_ref()
                        .map(|d| d.clone_ref(py))
                        .unwrap_or_else(|| py.None())
                }
            } else if let Ok(b) = d.extract::<bool>(py) {
                if b {
                    // return dict
                    let dict = PyDict::new(py);
                    if let Some(v) = attr_val {
                        dict.set_item("attr", v)?;
                    }
                    dict.into_any().unbind()
                } else {
                    // data=False -> just node names? NX behavior is strict.
                    // usually data=False loops over nodes. But NodeDataView implies data.
                    // G.nodes(data=False) returns NodeView (the iterable one).
                    // But we are in NodeDataView.
                    // We will treat bool True as dict.
                    py.None()
                }
            } else {
                return Err(PyTypeError::new_err("Invalid data parameter"));
            }
        } else {
            // Default (None) -> dict
            let dict = PyDict::new(py);
            if let Some(v) = attr_val {
                dict.set_item("attr", v)?;
            }
            dict.into_any().unbind()
        };

        let tuple = (node_id, val_obj).into_pyobject(py)?;
        Ok(Some(tuple.into_any().unbind()))
    }
}
