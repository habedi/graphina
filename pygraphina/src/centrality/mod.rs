pub mod betweenness;
pub mod closeness;
pub mod degree;
pub mod eigenvector;
pub mod harmonic;
pub mod katz;
pub mod pagerank;
pub mod utils;

use pyo3::prelude::*;

pub fn register_centrality(m: &pyo3::prelude::Bound<'_, PyModule>) -> PyResult<()> {
    degree::register_degree(m)?;
    betweenness::register_betweenness(m)?;
    closeness::register_closeness(m)?;
    harmonic::register_harmonic(m)?;
    eigenvector::register_eigenvector(m)?;
    katz::register_katz(m)?;
    pagerank::register_pagerank(m)?;
    Ok(())
}
