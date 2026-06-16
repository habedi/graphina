pub mod connected_components;
pub mod girvan_newman;
pub mod infomap;
pub mod label_propagation;
pub mod louvain;
pub mod node_maps;
pub mod spectral;

pub use node_maps::{infomap_map, label_propagation_map};
