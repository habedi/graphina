use pyo3::exceptions::PyKeyError;
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

        // Get all nodes via iterator
        let nodes: Vec<usize> = obj
            .call_method0("__iter__")?
            .try_iter()?
            .map(|i| i.and_then(|x| x.extract::<usize>()))
            .collect::<PyResult<Vec<usize>>>()?;

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
        // Call internal _degree helper
        let deg: Option<usize> = obj.call_method1("_degree", (node,))?.extract()?;

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
        nbunch: Option<Py<PyAny>>,
        weight: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let _ = weight; // Silence unused warning
        let Some(nbunch) = nbunch else {
            // Return a new View (equivalent to self)
            return Ok(DegreeView {
                graph: self.graph.clone_ref(py),
            }
            .into_pyobject(py)?
            .into_any()
            .unbind());
        };

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
            // Duck typing _degree
            let d: usize = obj
                .call_method1("_degree", (node,))?
                .extract::<Option<usize>>()?
                .unwrap_or(0);

            Ok(Some((node, d)))
        } else {
            Ok(None)
        }
    }

    // For repr support or dict conversion
    fn __repr__(_slf: PyRef<'_, Self>) -> String {
        "DegreeIterator(...)".to_string()
    }
}
