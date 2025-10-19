pub mod connected_components;
pub mod girvan_newman;
pub mod label_propagation;
pub mod louvain;
pub mod spectral;

use pyo3::prelude::*;

pub fn register_community(m: &Bound<'_, PyModule>) -> PyResult<()> {
    connected_components::register_connected_components(m)?;
    label_propagation::register_label_propagation(m)?;
    louvain::register_louvain(m)?;
    girvan_newman::register_girvan_newman(m)?;
    spectral::register_spectral(m)?;
    Ok(())
}
