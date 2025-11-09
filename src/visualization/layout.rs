/*!
Graph layout algorithms and position computation
*/

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use petgraph::EdgeType;
use std::collections::HashMap;

/// Node position in 2D space
#[derive(Debug, Clone, Copy)]
pub struct NodePosition {
    pub x: f64,
    pub y: f64,
}

/// Layout algorithms for graph visualization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayoutAlgorithm {
    /// Force-directed layout (default)
    #[default]
    ForceDirected,
    /// Circular layout
    Circular,
    /// Hierarchical/tree layout
    Hierarchical,
    /// Grid layout
    Grid,
    /// Random layout
    Random,
}

/// Layout engine for computing node positions
pub struct LayoutEngine;

impl LayoutEngine {
    /// Compute node positions using the specified layout algorithm
    pub fn compute_layout<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
        graph: &BaseGraph<A, W, Ty>,
        algorithm: LayoutAlgorithm,
        width: f64,
        height: f64,
    ) -> HashMap<NodeId, NodePosition> {
        match algorithm {
            LayoutAlgorithm::ForceDirected => Self::force_directed_layout(graph, width, height),
            LayoutAlgorithm::Circular => Self::circular_layout(graph, width, height),
            LayoutAlgorithm::Hierarchical => Self::hierarchical_layout(graph, width, height),
            LayoutAlgorithm::Grid => Self::grid_layout(graph, width, height),
            LayoutAlgorithm::Random => Self::random_layout(graph, width, height),
        }
    }

    /// Force-directed layout using Fruchterman-Reingold algorithm
    fn force_directed_layout<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
        graph: &BaseGraph<A, W, Ty>,
        width: f64,
        height: f64,
    ) -> HashMap<NodeId, NodePosition> {
        let mut positions = HashMap::new();
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();

        if nodes.is_empty() {
            return positions;
        }

        // Initialize with random positions
        use rand::Rng;
        let mut rng = rand::rng();
        for node in &nodes {
            positions.insert(
                *node,
                NodePosition {
                    x: rng.random_range(0.0..width),
                    y: rng.random_range(0.0..height),
                },
            );
        }

        // Fruchterman-Reingold parameters
        let area = width * height;
        let k = (area / nodes.len() as f64).sqrt();
        let iterations = 50;
        let mut temperature = width.max(height) / 10.0;
        let cooling_factor = 0.95;

        for _ in 0..iterations {
            let mut displacements = HashMap::new();

            // Initialize displacements for all nodes to avoid panics
            for &node in &nodes {
                displacements.insert(node, (0.0, 0.0));
            }

            // Repulsive forces between all pairs
            for i in 0..nodes.len() {
                let mut dx = 0.0;
                let mut dy = 0.0;

                for j in 0..nodes.len() {
                    if i != j {
                        let pos_i = positions[&nodes[i]];
                        let pos_j = positions[&nodes[j]];

                        let delta_x = pos_i.x - pos_j.x;
                        let delta_y = pos_i.y - pos_j.y;
                        let distance = (delta_x * delta_x + delta_y * delta_y).sqrt().max(0.01);

                        let force = k * k / distance;
                        dx += (delta_x / distance) * force;
                        dy += (delta_y / distance) * force;
                    }
                }

                if let Some((dx_curr, dy_curr)) = displacements.get_mut(&nodes[i]) {
                    *dx_curr += dx;
                    *dy_curr += dy;
                }
            }

            // Attractive forces for edges
            for (src, tgt, _) in graph.edges() {
                if let (Some(&pos_src), Some(&pos_tgt)) = (positions.get(&src), positions.get(&tgt))
                {
                    let delta_x = pos_tgt.x - pos_src.x;
                    let delta_y = pos_tgt.y - pos_src.y;
                    let distance = (delta_x * delta_x + delta_y * delta_y).sqrt().max(0.01);
                    let force = distance * distance / k;
                    if let Some((dx_src, dy_src)) = displacements.get_mut(&src) {
                        *dx_src += (delta_x / distance) * force;
                        *dy_src += (delta_y / distance) * force;
                    }
                    if let Some((dx_tgt, dy_tgt)) = displacements.get_mut(&tgt) {
                        *dx_tgt -= (delta_x / distance) * force;
                        *dy_tgt -= (delta_y / distance) * force;
                    }
                }
            }

            // Apply displacements
            for node in &nodes {
                let (dx, dy) = displacements.get(node).copied().unwrap_or((0.0, 0.0));
                if let Some(pos) = positions.get_mut(node) {
                    let displacement = (dx * dx + dy * dy).sqrt();
                    if displacement > 0.0 {
                        let limited = displacement.min(temperature);
                        pos.x += (dx / displacement) * limited;
                        pos.y += (dy / displacement) * limited;

                        // Keep within bounds
                        pos.x = pos.x.max(0.0).min(width);
                        pos.y = pos.y.max(0.0).min(height);
                    }
                }
            }

            temperature *= cooling_factor;
        }

        positions
    }

    /// Circular layout
    fn circular_layout<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
        graph: &BaseGraph<A, W, Ty>,
        width: f64,
        height: f64,
    ) -> HashMap<NodeId, NodePosition> {
        let mut positions = HashMap::new();
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();

        if nodes.is_empty() {
            return positions;
        }

        let center_x = width / 2.0;
        let center_y = height / 2.0;
        let radius = width.min(height) / 2.5;

        for (i, node) in nodes.iter().enumerate() {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / nodes.len() as f64;
            positions.insert(
                *node,
                NodePosition {
                    x: center_x + radius * angle.cos(),
                    y: center_y + radius * angle.sin(),
                },
            );
        }

        positions
    }

    /// Hierarchical/tree layout
    fn hierarchical_layout<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
        graph: &BaseGraph<A, W, Ty>,
        width: f64,
        height: f64,
    ) -> HashMap<NodeId, NodePosition> {
        let mut positions = HashMap::new();
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();

        if nodes.is_empty() {
            return positions;
        }

        // Simple BFS-based layering
        let mut layers: Vec<Vec<NodeId>> = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();

        // Start from nodes with no incoming edges, or just the first node
        let start_nodes: Vec<_> = nodes
            .iter()
            .filter(|&&n| graph.in_degree(n).unwrap_or(0) == 0)
            .copied()
            .collect();

        if start_nodes.is_empty() {
            queue.push_back(nodes[0]);
        } else {
            for node in start_nodes {
                queue.push_back(node);
            }
        }

        while !queue.is_empty() {
            let layer_size = queue.len();
            let mut current_layer = Vec::new();

            for _ in 0..layer_size {
                if let Some(node) = queue.pop_front() {
                    if visited.insert(node) {
                        current_layer.push(node);

                        // Add neighbors to queue using direct neighbor iteration
                        for neighbor in graph.neighbors(node) {
                            if !visited.contains(&neighbor) {
                                queue.push_back(neighbor);
                            }
                        }
                    }
                }
            }

            if !current_layer.is_empty() {
                layers.push(current_layer);
            }
        }

        // Add any remaining unvisited nodes
        for node in nodes {
            if !visited.contains(&node) {
                layers.push(vec![node]);
            }
        }

        // Position nodes
        let layer_height = if layers.len() > 1 {
            height / (layers.len() - 1) as f64
        } else {
            height / 2.0
        };

        for (layer_idx, layer) in layers.iter().enumerate() {
            let y = layer_idx as f64 * layer_height;
            let layer_width = if layer.len() > 1 {
                width / (layer.len() - 1) as f64
            } else {
                width / 2.0
            };

            for (node_idx, &node) in layer.iter().enumerate() {
                let x = if layer.len() > 1 {
                    node_idx as f64 * layer_width
                } else {
                    width / 2.0
                };

                positions.insert(node, NodePosition { x, y });
            }
        }

        positions
    }

    /// Grid layout
    fn grid_layout<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
        graph: &BaseGraph<A, W, Ty>,
        width: f64,
        height: f64,
    ) -> HashMap<NodeId, NodePosition> {
        let mut positions = HashMap::new();
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();

        if nodes.is_empty() {
            return positions;
        }

        let cols = (nodes.len() as f64).sqrt().ceil() as usize;
        let rows = (nodes.len() as f64 / cols as f64).ceil() as usize;

        let cell_width = width / cols as f64;
        let cell_height = height / rows as f64;

        for (i, node) in nodes.iter().enumerate() {
            let row = i / cols;
            let col = i % cols;

            positions.insert(
                *node,
                NodePosition {
                    x: (col as f64 + 0.5) * cell_width,
                    y: (row as f64 + 0.5) * cell_height,
                },
            );
        }

        positions
    }

    /// Random layout
    fn random_layout<A, W, Ty: GraphConstructor<A, W> + EdgeType>(
        graph: &BaseGraph<A, W, Ty>,
        width: f64,
        height: f64,
    ) -> HashMap<NodeId, NodePosition> {
        use rand::Rng;
        let mut rng = rand::rng();
        let mut positions = HashMap::new();

        for (node, _) in graph.nodes() {
            positions.insert(
                node,
                NodePosition {
                    x: rng.random_range(0.0..width),
                    y: rng.random_range(0.0..height),
                },
            );
        }

        positions
    }
}
