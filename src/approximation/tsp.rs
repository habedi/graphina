//! Approximation algorithms for traveling salesman problems.

use crate::core::error::{GraphinaError, Result};
use crate::core::paths::dijkstra_path_f64;
use crate::core::types::{BaseGraph, GraphConstructor, GraphinaGraph, NodeId};
use std::collections::HashSet;
use std::fmt::Debug;

/// Greedy TSP approximation.
pub fn greedy_tsp<A, Ty>(graph: &BaseGraph<A, f64, Ty>, start: NodeId) -> Result<(Vec<NodeId>, f64)>
where
    A: Debug,
    Ty: GraphConstructor<A, f64>,
    BaseGraph<A, f64, Ty>: GraphinaGraph<A, f64>,
{
    if graph.is_empty() {
        return Err(GraphinaError::invalid_graph(
            "Cannot run TSP on an empty graph.",
        ));
    }

    if graph.node_count() == 1 {
        return Err(GraphinaError::invalid_graph(
            "Cannot run TSP on a single-node graph.",
        ));
    }

    let mut tour = Vec::new();
    let mut visited: HashSet<NodeId> = HashSet::new();
    let mut current = start;

    tour.push(current);
    visited.insert(current);

    let mut unvisited: Vec<NodeId> = graph
        .nodes()
        .map(|(id, _)| id)
        .filter(|id| !visited.contains(id))
        .collect();

    // Visit all remaining nodes
    while !unvisited.is_empty() {
        if let Ok((distance_map, _)) = dijkstra_path_f64(graph, current, None) {
            let mut nearest = None;
            let mut min_dist = f64::INFINITY;

            for &candidate in &unvisited {
                if candidate == current {
                    continue;
                }
                if let Some(Some(d)) = distance_map.get(&candidate) {
                    if *d < min_dist {
                        min_dist = *d;
                        nearest = Some(candidate);
                    }
                }
            }

            if let Some(next) = nearest {
                tour.push(next);
                visited.insert(next);
                current = next;
            } else {
                return Err(GraphinaError::algorithm_error(
                    "Could not find a path to any unvisited node.",
                ));
            }
        } else {
            return Err(GraphinaError::algorithm_error(
                "Could not compute distances from the current node.",
            ));
        }

        unvisited = graph
            .nodes()
            .map(|(id, _)| id)
            .filter(|id| !visited.contains(id))
            .collect();
    }

    // Add return to start to complete the tour
    tour.push(start);

    let total_cost = tour_cost(graph, &tour)?;
    if total_cost.is_infinite() {
        return Err(GraphinaError::algorithm_error(
            "Could not compute finite tour cost (possibly disconnected).",
        ));
    }

    Ok((tour, total_cost))
}

fn tour_cost<A, Ty>(graph: &BaseGraph<A, f64, Ty>, tour: &[NodeId]) -> Result<f64>
where
    A: Debug,
    Ty: GraphConstructor<A, f64>,
    BaseGraph<A, f64, Ty>: GraphinaGraph<A, f64>,
{
    if tour.len() < 2 {
        return Ok(0.0);
    }

    let mut total_cost = 0.0;

    // Sum costs between consecutive nodes in the tour
    for i in 0..tour.len() - 1 {
        let u = tour[i];
        let v = tour[i + 1];

        // Find shortest path distance from u to v
        if let Ok((dist_map, _)) = dijkstra_path_f64(graph, u, None) {
            if let Some(Some(d)) = dist_map.get(&v) {
                total_cost += *d;
            } else {
                // No path from u to v - graph is disconnected
                return Ok(f64::INFINITY);
            }
        } else {
            return Ok(f64::INFINITY);
        }
    }

    Ok(total_cost)
}

#[cfg(test)]
mod tests {
    // Bring required items into scope for tests
    use super::{greedy_tsp, tour_cost};
    use crate::core::types::Graph;

    #[test]
    fn greedy_tsp_on_square_graph() {
        // Build a 4-node cycle (square) with unit weights; minimal tour cost should be 4
        let mut g: Graph<i32, f64> = Graph::new();
        let n0 = g.add_node(0);
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(n0, n1, 1.0);
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n0, 1.0);

        let (tour, cost) = greedy_tsp(&g, n0).expect("greedy tsp should succeed");
        assert!(tour.len() >= 5); // include return to source
        assert!((cost - 4.0).abs() < 1e-9);
        assert_eq!(*tour.first().unwrap(), n0);
        assert_eq!(*tour.last().unwrap(), n0);
    }

    #[test]
    fn tour_cost_handles_short_tour() {
        let mut g: Graph<i32, f64> = Graph::new();
        let n0 = g.add_node(0);
        // empty tour
        assert_eq!(tour_cost(&g, &[]).unwrap(), 0.0);
        // single node tour
        assert_eq!(tour_cost(&g, &[n0]).unwrap(), 0.0);
    }

    #[test]
    fn greedy_tsp_errors_when_disconnected() {
        let mut g: Graph<i32, f64> = Graph::new();
        let n0 = g.add_node(0);
        let _n1 = g.add_node(1);
        // no edges, disconnected
        let err = greedy_tsp(&g, n0).unwrap_err();
        assert!(format!("{}", err).contains("Could not find a path"));
    }
}
