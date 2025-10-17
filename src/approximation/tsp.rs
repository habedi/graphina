//! Approximation algorithms for traveling salesman problems.

use crate::core::exceptions::GraphinaException;
use crate::core::paths::dijkstra;
use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
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
    let mut unvisited: HashSet<NodeId> = graph.nodes().map(|(u, _)| u).collect();
    let mut tour = Vec::new();
    let mut cost = 0.0;
    let mut current = source;
    tour.push(current);
    unvisited.remove(&current);
    while !unvisited.is_empty() {
        let distances = dijkstra(graph, current)?;
        let (next_node, next_cost) = unvisited
            .iter()
            .filter_map(|v| distances[v.index()].map(|d| (*v, d.0)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .ok_or_else(|| {
                GraphinaException::new("Could not find a path to any unvisited node.")
            })?;
        tour.push(next_node);
        cost += next_cost;
        current = next_node;
        unvisited.remove(&current);
    }
    let distances = dijkstra(graph, current)?;
    if let Some(d) = distances[source.index()] {
        cost += d.0;
        tour.push(source);
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
    let mut cost = 0.0;
    for i in 0..tour.len() - 1 {
        let distances = dijkstra(graph, tour[i])?;
        if let Some(d) = distances[tour[i + 1].index()] {
            cost += d.0;
        }
    }
    Ok(cost)
}
