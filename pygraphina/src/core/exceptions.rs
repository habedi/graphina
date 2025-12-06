use pyo3::create_exception;

create_exception!(pygraphina, GraphinaError, pyo3::exceptions::PyException);
create_exception!(pygraphina, ConvergenceError, GraphinaError);
create_exception!(pygraphina, NodeNotFoundError, GraphinaError);
