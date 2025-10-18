//! Approximation algorithms for traveling salesman problems.

use crate::core::exceptions::GraphinaException;
use crate::core::paths::dijkstra;
use crate::core::types::{BaseGraph, Graph, GraphConstructor, NodeId};
use ordered_float::OrderedFloat;
use std::collections::HashSet;

/// Approximate a solution to the TSP using Christofides' algorithm (placeholder).
pub fn christofides<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
) -> Result<(Vec<NodeId>, f64), GraphinaException>
where
    A: Clone,
    Ty: GraphConstructor<A, f64> + GraphConstructor<A, OrderedFloat<f64>>,
{
    let start_node = graph
        .nodes()
        .next()
        .map(|(u, _)| u)
        .ok_or_else(|| GraphinaException::new("Cannot run TSP on an empty graph."))?;
    greedy_tsp(&graph.convert::<OrderedFloat<f64>>(), start_node)
}

/// Approximate the TSP solution using a greedy algorithm.
pub fn traveling_salesman_problem<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
) -> Result<(Vec<NodeId>, f64), GraphinaException>
where
    A: Clone,
    Ty: GraphConstructor<A, f64> + GraphConstructor<A, OrderedFloat<f64>>,
{
    let start_node = graph
        .nodes()
        .next()
        .map(|(u, _)| u)
        .ok_or_else(|| GraphinaException::new("Cannot run TSP on an empty graph."))?;
    greedy_tsp(&graph.convert::<OrderedFloat<f64>>(), start_node)
}

/// Greedy TSP: starting at `source`, repeatedly go to the nearest unvisited node.
/// This function requires the graph to use OrderedFloat<f64> weights.
pub fn greedy_tsp<A, Ty>(
    graph: &BaseGraph<A, OrderedFloat<f64>, Ty>,
    source: NodeId,
) -> Result<(Vec<NodeId>, f64), GraphinaException>
where
    Ty: GraphConstructor<A, OrderedFloat<f64>>,
{
    if !graph.contains_node(source) {
        return Err(GraphinaException::new(
            "Source node does not exist in the graph.",
        ));
    }

    let mut unvisited: HashSet<NodeId> = graph.nodes().map(|(u, _)| u).collect();
    if unvisited.is_empty() {
        return Err(GraphinaException::new("Cannot run TSP on an empty graph."));
    }

    let mut tour = Vec::new();
    let mut cost = 0.0;
    let mut current = source;
    tour.push(current);
    unvisited.remove(&current);
    while !unvisited.is_empty() {
        let distances = dijkstra(graph, current)?;
        let (next_node, next_cost) = unvisited
            .iter()
            .filter_map(|v| distances[v].map(|d| (*v, d.0)))
            .min_by(|&(_, a_cost), &(_, b_cost)| {
                a_cost
                    .partial_cmp(&b_cost)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .ok_or_else(|| {
                GraphinaException::new("Could not find a path to any unvisited node.")
            })?;
        tour.push(next_node);
        cost += next_cost;
        current = next_node;
        unvisited.remove(&current);
    }
    let distances = dijkstra(graph, current)?;
    if let Some(d) = distances[&source] {
        cost += d.0;
        tour.push(source);
    } else {
        return Err(GraphinaException::new(
            "Could not return to the starting node to complete the tour.",
        ));
    }
    Ok((tour, cost))
}

/// Simulated Annealing TSP (placeholder): returns the initial cycle.
pub fn simulated_annealing_tsp<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    init_cycle: Vec<NodeId>,
) -> Result<(Vec<NodeId>, f64), GraphinaException>
where
    A: Clone,
    Ty: GraphConstructor<A, f64> + GraphConstructor<A, OrderedFloat<f64>>,
{
    let cost = tour_cost(&graph.convert::<OrderedFloat<f64>>(), &init_cycle)?;
    Ok((init_cycle, cost))
}

/// Threshold Accepting TSP (placeholder): returns the initial cycle.
pub fn threshold_accepting_tsp<A, Ty>(
    graph: &BaseGraph<A, f64, Ty>,
    init_cycle: Vec<NodeId>,
) -> Result<(Vec<NodeId>, f64), GraphinaException>
where
    A: Clone,
    Ty: GraphConstructor<A, f64> + GraphConstructor<A, OrderedFloat<f64>>,
{
    let cost = tour_cost(&graph.convert::<OrderedFloat<f64>>(), &init_cycle)?;
    Ok((init_cycle, cost))
}

/// Asadpour ATSP (not implemented).
pub fn asadpour_atsp<A, Ty>(_graph: &BaseGraph<A, f64, Ty>) -> (Vec<NodeId>, f64)
where
    Ty: GraphConstructor<A, f64>,
{
    unimplemented!("Asadpour ATSP algorithm is not implemented yet")
}

/// Helper: Compute the total cost of a given tour.
fn tour_cost<A, Ty>(
    graph: &BaseGraph<A, OrderedFloat<f64>, Ty>,
    tour: &[NodeId],
) -> Result<f64, GraphinaException>
where
    Ty: GraphConstructor<A, OrderedFloat<f64>>,
{
    if tour.len() < 2 {
        return Ok(0.0);
    }
    let mut cost = 0.0;
    for i in 0..(tour.len() - 1) {
        let distances = dijkstra(graph, tour[i])?;
        match distances[&tour[i + 1]] {
            Some(d) => cost += d.0,
            None => return Err(GraphinaException::new("Tour contains an unreachable step.")),
        }
    }
    Ok(cost)
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let g_ord = g.convert::<OrderedFloat<f64>>();
        let (tour, cost) = greedy_tsp(&g_ord, n0).expect("greedy tsp should succeed");
        assert!(tour.len() >= 5); // include return to source
        assert!((cost - 4.0).abs() < 1e-9);
        assert_eq!(*tour.first().unwrap(), n0);
        assert_eq!(*tour.last().unwrap(), n0);
    }

    #[test]
    fn tour_cost_handles_short_tour() {
        let mut g: Graph<i32, f64> = Graph::new();
        let n0 = g.add_node(0);
        let g_ord = g.convert::<OrderedFloat<f64>>();
        // empty tour
        assert_eq!(tour_cost(&g_ord, &[]).unwrap(), 0.0);
        // single node tour
        assert_eq!(tour_cost(&g_ord, &[n0]).unwrap(), 0.0);
    }

    #[test]
    fn greedy_tsp_errors_when_disconnected() {
        let mut g: Graph<i32, f64> = Graph::new();
        let n0 = g.add_node(0);
        let _n1 = g.add_node(1);
        // no edges, disconnected
        let g_ord = g.convert::<OrderedFloat<f64>>();
        let err = greedy_tsp(&g_ord, n0).unwrap_err();
        assert!(format!("{}", err).contains("Could not find a path"));
    }
}
