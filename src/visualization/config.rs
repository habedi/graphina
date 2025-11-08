/*!
Visualization configuration and settings
*/

use super::layout::LayoutAlgorithm;

/// Configuration for graph visualization
#[derive(Debug, Clone)]
pub struct VisualizationConfig {
    /// Width of the visualization in pixels
    pub width: u32,
    /// Height of the visualization in pixels
    pub height: u32,
    /// Layout algorithm to use
    pub layout: LayoutAlgorithm,
    /// Node color (hex format, e.g., "#69b3a2")
    pub node_color: String,
    /// Edge color (hex format)
    pub edge_color: String,
    /// Node size
    pub node_size: f64,
    /// Edge width
    pub edge_width: f64,
    /// Whether to show node labels
    pub show_labels: bool,
    /// Whether to show edge labels
    pub show_edge_labels: bool,
    /// Background color
    pub background_color: String,
    /// Font size for labels
    pub font_size: u32,
}

impl Default for VisualizationConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            layout: LayoutAlgorithm::ForceDirected,
            node_color: "#69b3a2".to_string(),
            edge_color: "#999999".to_string(),
            node_size: 10.0,
            edge_width: 2.0,
            show_labels: true,
            show_edge_labels: false,
            background_color: "#ffffff".to_string(),
            font_size: 12,
        }
    }
}
