pub mod betweenness;
pub mod closeness;
pub mod degree;
pub mod eigenvector;
pub mod harmonic;
pub mod katz;
pub mod pagerank;
pub mod reaching;
pub mod utils;

use pyo3::prelude::*;

pub fn register_centrality(m: &pyo3::prelude::Bound<'_, PyModule>) -> PyResult<()> {
    degree::register_degree(m)?;
    closeness::register_closeness(m)?;
    betweenness::register_betweenness(m)?;
    eigenvector::register_eigenvector(m)?;
    pagerank::register_pagerank(m)?;
    katz::register_katz(m)?;
    harmonic::register_harmonic(m)?;
    reaching::register_reaching_centrality(m)?;
    Ok(())
}
