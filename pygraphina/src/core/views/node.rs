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
        // Delegate to graph.__len__ which is mapped to node_count
        obj.len()
    }

    fn __contains__(&self, py: Python<'_>, node: usize) -> PyResult<bool> {
        let obj = self.graph.bind(py);
        // Delegate to graph.__contains__
        obj.contains(node)
    }

    fn __iter__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        // The graph class already implements __iter__ returning node iterator
        let obj = self.graph.bind(py);
        Ok(obj.call_method0("__iter__")?.unbind())
    }

    fn __getitem__(&self, py: Python<'_>, node: usize) -> PyResult<Py<PyDict>> {
        let obj = self.graph.bind(py);
        // Call get_node_attr dynamically.
        // Takes usize, returns Option<i64>
        let attr_obj = obj.call_method1("get_node_attr", (node,))?;

        if attr_obj.is_none() {
            // We need to check if the node actually exists to distinguish "attr is None" from "node missing"
            // But get_node_attr impl returns None if node missing OR if attr is None?
            // Wait, Rust PyGraph::get_node_attr returns Option<i64>.
            // Actually, in `core/graph.rs`: `get_node_attr` returns `Option<i64>`.
            // `core/basic_ops.rs` logic: `self.mapper.py_to_internal.get(&py_node)?` returns None if missing.
            // If present, `node_attr` returns `Option<&T>`.

            // So if it returns None, either node is missing or valid node has no attr (unlikely in this design where attr is i64).
            // But wait, the previous code handled it as "None" -> Key Error.
            // Let's verify if node exists first.
            if obj.contains(node)? {
                // Node exists, but attribute is None (impl detail: i64 is always Copy, so Option<i64> is None only if graph says so)
                // In our simplified graph, nodes always have an attr (user provided or default).
                // Actually PyGraph::add_node takes `attr: i64`. So it should always return Some(i64) if node exists.
                // So if get_node_attr returns None, it means Node doesn't exist.
                Err(PyKeyError::new_err(format!("Node {} not found", node)))
            } else {
                Err(PyKeyError::new_err(format!("Node {} not found", node)))
            }
        } else {
            let val: i64 = attr_obj.extract()?;
            let dict = PyDict::new(py);
            dict.set_item("attr", val)?;
            Ok(dict.into())
        }
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let obj = self.graph.bind(py);
        // Use standard iterator to collect representation
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
        data: Option<Py<PyAny>>,
        default: Option<Py<PyAny>>,
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
    data_param: Option<Py<PyAny>>,
    default_val: Option<Py<PyAny>>,
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
        obj.len()
    }

    fn __repr__(&self, _py: Python<'_>) -> PyResult<String> {
        Ok("NodeDataView(...)".to_string())
    }
}

#[pyclass]
pub struct NodeDataIterator {
    graph: Py<PyAny>,
    data_param: Option<Py<PyAny>>,
    default_val: Option<Py<PyAny>>,
    nodes: Vec<usize>,
    idx: usize,
}

#[pymethods]
impl NodeDataIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> PyResult<Option<Py<PyAny>>> {
        if slf.idx >= slf.nodes.len() {
            return Ok(None);
        }
        let node_id = slf.nodes[slf.idx];
        slf.idx += 1;

        let py = slf.py();
        let graph_bound = slf.graph.bind(py);

        // Get attribute via duck typing
        let attr_obj = graph_bound.call_method1("get_node_attr", (node_id,))?;
        // Convert Option<i64> to Option<Bound<PyAny>> logic
        // If None, it implies node missing (unlikely here since we are iterating known nodes)
        // or just pass through.
        let attr_val: Option<Bound<PyAny>> = if attr_obj.is_none() {
            None
        } else {
            Some(attr_obj)
        };

        // Logic based on data_param
        let val_obj = if let Some(ref d) = slf.data_param {
            if let Ok(s) = d.extract::<String>(py) {
                if s == "attr" {
                    match attr_val {
                        Some(v) => v.unbind(),
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
                        // Extract i64 from wrapper
                        let val: i64 = v.extract()?;
                        dict.set_item("attr", val)?;
                    }
                    dict.into_any().unbind()
                } else {
                    // data=False -> return plain node id?
                    // Wait, generic behavior for G.nodes(data=False) is just the iterator over keys.
                    // But NodeDataView is usually created via G.nodes(data=True) or G.nodes.data().
                    // If user manually called G.nodes.data(data=False), it might be weird.
                    // Returning None to mimic "no data" but usually this case isn't hit in standard usage loops.
                    py.None()
                }
            } else {
                return Err(PyTypeError::new_err("Invalid data parameter"));
            }
        } else {
            // Default (None) -> dict
            let dict = PyDict::new(py);
            if let Some(v) = attr_val {
                let val: i64 = v.extract()?;
                dict.set_item("attr", val)?;
            }
            dict.into_any().unbind()
        };

        let tuple = (node_id, val_obj).into_pyobject(py)?;
        Ok(Some(tuple.into_any().unbind()))
    }
}
