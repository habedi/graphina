pub mod allocation;
pub mod attachment;
pub mod centrality;
pub mod similarity;

use pyo3::prelude::*;

pub fn register_links(m: &Bound<'_, PyModule>) -> PyResult<()> {
    similarity::register_similarity(m)?;
    attachment::register_attachment(m)?;
    centrality::register_links_centrality(m)?;
    allocation::register_allocation(m)?;
    Ok(())
}
