pub mod clique;
pub mod clustering;
pub mod connectivity;
pub mod diameter;
pub mod independent_set;
pub mod ramsey;
pub mod subgraph;
pub mod treewidth;
pub mod vertex_cover;

use pyo3::prelude::*;

pub fn register_approximation(m: &Bound<'_, PyModule>) -> PyResult<()> {
    clique::register_clique(m)?;
    diameter::register_diameter(m)?;
    vertex_cover::register_vertex_cover(m)?;
    independent_set::register_independent_set(m)?;
    subgraph::register_subgraph(m)?;
    clustering::register_clustering(m)?;
    connectivity::register_connectivity(m)?;
    treewidth::register_treewidth(m)?;
    ramsey::register_ramsey(m)?;
    Ok(())
}
