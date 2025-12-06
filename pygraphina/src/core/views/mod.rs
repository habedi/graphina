pub mod degree;
pub mod edge;
pub mod node;

use pyo3::prelude::*;

pub fn register_views(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<node::NodeView>()?;
    m.add_class::<node::NodeDataView>()?;
    m.add_class::<node::NodeDataIterator>()?;

    m.add_class::<edge::EdgeView>()?;
    m.add_class::<edge::EdgeDataView>()?;
    m.add_class::<edge::EdgeIterator>()?;
    m.add_class::<edge::EdgeDataIterator>()?;

    m.add_class::<degree::DegreeView>()?;
    m.add_class::<degree::DegreeIterator>()?;
    Ok(())
}
