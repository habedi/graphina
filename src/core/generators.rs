/*!
# Graph Generators

This module provides various graph generators that create classic graphs such as
Erdős–Rényi, complete, bipartite, star, cycle, Watts–Strogatz small-world, and
Barabási–Albert scale-free graphs. Each generator is generic over the graph type
(directed or undirected) using the `GraphConstructor` trait. Node attributes are fixed
to `u32` and edge weights to `f32`.

Most generators use a seeded random number generator for reproducibility. In case of
invalid parameters (e.g. probability out of [0, 1] or insufficient nodes), functions
return a `Result` with a relevant exception from `graphina::core::exceptions`.

# Examples

Generating an Erdős–Rényi graph:

```rust
use graphina::core::generators::erdos_renyi_graph;
use graphina::core::types::DigraphMarker;
use graphina::core::exceptions::GraphinaException;

let graph = erdos_renyi_graph::<DigraphMarker>(100, 0.1, 42)
    .expect("Failed to generate Erdős–Rényi graph");
```

Generating a Watts–Strogatz graph:

```rust
use graphina::core::generators::watts_strogatz_graph;
use graphina::core::types::GraphMarker;
use graphina::core::exceptions::GraphinaException;

let ws = watts_strogatz_graph::<GraphMarker>(100, 6, 0.3, 42)
    .expect("Failed to generate Watts–Strogatz graph");
```
*/

use crate::core::error::GraphinaError;
use crate::core::types::{BaseGraph, GraphConstructor};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Generates an Erdős–Rényi graph.
///
/// # Arguments
///
/// * `n` - The number of nodes (must be > 0).
/// * `p` - The probability of edge creation (must be in [0.0, 1.0]).
/// * `seed` - The seed for the random number generator.
///
/// # Type Parameters
///
/// * `Ty` - The graph type (directed or undirected) implementing `GraphConstructor<u32, f32>`.
///
/// # Returns
///
/// * `Result<BaseGraph<u32, f32, Ty>, GraphinaException>` - The generated graph, or an error if parameters are invalid.
pub fn erdos_renyi_graph<Ty: GraphConstructor<u32, f32>>(
    n: usize,
    p: f64,
    seed: u64,
) -> Result<BaseGraph<u32, f32, Ty>, GraphinaError> {
    if n == 0 {
        return Err(GraphinaError::InvalidArgument(
            "Number of nodes must be greater than zero.".into(),
        ));
    }
    if !(0.0..=1.0).contains(&p) {
        return Err(GraphinaError::InvalidArgument(
            "Probability p must be in the range [0.0, 1.0].".into(),
        ));
    }

    let mut graph = BaseGraph::<u32, f32, Ty>::new();
    let mut nodes = Vec::with_capacity(n);
    for i in 0..n {
        nodes.push(graph.add_node(i as u32));
    }
    let mut rng = StdRng::seed_from_u64(seed);
    if <Ty as GraphConstructor<u32, f32>>::is_directed() {
        for i in 0..n {
            for j in 0..n {
                if i != j && rng.random_bool(p) {
                    graph.add_edge(nodes[i], nodes[j], 1.0);
                }
            }
        }
    } else {
        for i in 0..n {
            for j in (i + 1)..n {
                if rng.random_bool(p) {
                    graph.add_edge(nodes[i], nodes[j], 1.0);
                }
            }
        }
    }
    Ok(graph)
}

/// Generates a complete graph.
///
/// # Arguments
///
/// * `n` - The number of nodes (must be > 0).
///
/// # Type Parameters
///
/// * `Ty` - The graph type implementing `GraphConstructor<u32, f32>`.
///
/// # Returns
///
/// * `Result<BaseGraph<u32, f32, Ty>, GraphinaException>` - The complete graph.
pub fn complete_graph<Ty: GraphConstructor<u32, f32>>(
    n: usize,
) -> Result<BaseGraph<u32, f32, Ty>, GraphinaError> {
    if n == 0 {
        return Err(GraphinaError::InvalidArgument(
            "Number of nodes must be greater than zero.".into(),
        ));
    }
    let mut graph = BaseGraph::<u32, f32, Ty>::new();
    let mut nodes = Vec::with_capacity(n);
    for i in 0..n {
        nodes.push(graph.add_node(i as u32));
    }
    if <Ty as GraphConstructor<u32, f32>>::is_directed() {
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    graph.add_edge(nodes[i], nodes[j], 1.0);
                }
            }
        }
    } else {
        for i in 0..n {
            for j in (i + 1)..n {
                graph.add_edge(nodes[i], nodes[j], 1.0);
            }
        }
    }
    Ok(graph)
}

/// Generates a bipartite graph.
///
/// # Arguments
///
/// * `n1` - The number of nodes in the first set (must be > 0).
/// * `n2` - The number of nodes in the second set (must be > 0).
/// * `p` - The probability of edge creation (must be in [0.0, 1.0]).
/// * `seed` - The seed for the random number generator.
///
/// # Type Parameters
///
/// * `Ty` - The graph type implementing `GraphConstructor<u32, f32>`.
///
/// # Returns
///
/// * `Result<BaseGraph<u32, f32, Ty>, GraphinaException>` - The generated bipartite graph.
pub fn bipartite_graph<Ty: GraphConstructor<u32, f32>>(
    n1: usize,
    n2: usize,
    p: f64,
    seed: u64,
) -> Result<BaseGraph<u32, f32, Ty>, GraphinaError> {
    if n1 == 0 || n2 == 0 {
        return Err(GraphinaError::InvalidArgument(
            "Both partitions must have at least one node.".into(),
        ));
    }
    if !(0.0..=1.0).contains(&p) {
        return Err(GraphinaError::InvalidArgument(
            "Probability p must be in the range [0.0, 1.0].".into(),
        ));
    }
    let mut graph = BaseGraph::<u32, f32, Ty>::new();
    let mut group1 = Vec::with_capacity(n1);
    let mut group2 = Vec::with_capacity(n2);
    for i in 0..n1 {
        group1.push(graph.add_node(i as u32));
    }
    for j in 0..n2 {
        group2.push(graph.add_node((n1 + j) as u32));
    }
    let mut rng = StdRng::seed_from_u64(seed);
    for &u in &group1 {
        for &v in &group2 {
            if rng.random_bool(p) {
                graph.add_edge(u, v, 1.0);
            }
        }
    }
    Ok(graph)
}

/// Generates a star graph.
///
/// # Arguments
///
/// * `n` - The total number of nodes (must be > 0).
///
/// # Type Parameters
///
/// * `Ty` - The graph type implementing `GraphConstructor<u32, f32>`.
///
/// # Returns
///
/// * `Result<BaseGraph<u32, f32, Ty>, GraphinaException>` - The generated star graph.
pub fn star_graph<Ty: GraphConstructor<u32, f32>>(
    n: usize,
) -> Result<BaseGraph<u32, f32, Ty>, GraphinaError> {
    let mut graph = BaseGraph::<u32, f32, Ty>::new();
    if n == 0 {
        return Err(GraphinaError::InvalidArgument(
            "Star graph must have at least one node.".into(),
        ));
    }
    let center = graph.add_node(0);
    for i in 1..n {
        let node = graph.add_node(i as u32);
        graph.add_edge(center, node, 1.0);
    }
    Ok(graph)
}

/// Generates a cycle graph.
///
/// # Arguments
///
/// * `n` - The number of nodes (must be > 0).
///
/// # Type Parameters
///
/// * `Ty` - The graph type implementing `GraphConstructor<u32, f32>`.
///
/// # Returns
///
/// * `Result<BaseGraph<u32, f32, Ty>, GraphinaException>` - The generated cycle graph.
pub fn cycle_graph<Ty: GraphConstructor<u32, f32>>(
    n: usize,
) -> Result<BaseGraph<u32, f32, Ty>, GraphinaError> {
    if n < 3 {
        return Err(GraphinaError::InvalidArgument(
            "Cycle graph must have at least three nodes.".into(),
        ));
    }
    let mut graph = BaseGraph::<u32, f32, Ty>::new();
    let mut nodes = Vec::with_capacity(n);
    for i in 0..n {
        nodes.push(graph.add_node(i as u32));
    }
    for i in 0..n {
        let j = (i + 1) % n;
        graph.add_edge(nodes[i], nodes[j], 1.0);
    }
    Ok(graph)
}

/// Generates a Watts–Strogatz small-world graph.
///
/// # Arguments
///
/// * `n` - The number of nodes (must be > 0).
/// * `k` - Each node is joined with its `k` nearest neighbors in a ring topology (must be even and less than n).
/// * `beta` - The probability of rewiring each edge (must be in [0.0, 1.0]).
/// * `seed` - The seed for the random number generator.
///
/// # Type Parameters
///
/// * `Ty` - The graph type implementing `GraphConstructor<u32, f32>`. This generator is typically used with undirected graphs.
///
/// # Returns
///
/// * `Result<BaseGraph<u32, f32, Ty>, GraphinaException>` - The generated Watts–Strogatz graph.
///
/// # Notes
///
/// In the rewiring phase, each eligible edge is removed with probability `beta` and replaced by a new edge
/// from the source node to a randomly chosen target (avoiding self-loops). This implementation uses the public
/// API method `find_edge` to locate and remove an existing edge.
pub fn watts_strogatz_graph<Ty: GraphConstructor<u32, f32>>(
    n: usize,
    k: usize,
    beta: f64,
    seed: u64,
) -> Result<BaseGraph<u32, f32, Ty>, GraphinaError> {
    if n == 0 {
        return Err(GraphinaError::InvalidArgument(
            "Number of nodes must be greater than zero.".into(),
        ));
    }
    if k % 2 != 0 || k >= n {
        return Err(GraphinaError::InvalidArgument(
            "k must be even and less than n.".into(),
        ));
    }
    if !(0.0..=1.0).contains(&beta) {
        return Err(GraphinaError::InvalidArgument(
            "Beta must be in the range [0.0, 1.0].".into(),
        ));
    }

    let mut graph = BaseGraph::<u32, f32, Ty>::new();
    let mut nodes = Vec::with_capacity(n);
    for i in 0..n {
        nodes.push(graph.add_node(i as u32));
    }
    let mut rng = StdRng::seed_from_u64(seed);
    let half_k = k / 2;
    // Create ring lattice.
    for i in 0..n {
        for j in 1..=half_k {
            let neighbor = (i + j) % n;
            graph.add_edge(nodes[i], nodes[neighbor], 1.0);
        }
    }
    // Rewire edges: for each edge in the original lattice, with probability beta, remove it and add a new edge.
    for i in 0..n {
        for j in 1..=half_k {
            if rng.random_bool(beta) {
                let neighbor = (i + j) % n;
                // Use the public API method `find_edge` to locate the edge.
                if let Some(eid) = graph.find_edge(nodes[i], nodes[neighbor]) {
                    let _ = graph.remove_edge(eid);
                    // Choose a new target at random (avoiding self-loop and existing edges).
                    let max_attempts = n * 2; // Prevent infinite loop
                    let mut attempts = 0;
                    let mut found_valid_target = false;

                    let new_target = loop {
                        let target = rng.random_range(0..n);
                        attempts += 1;
                        // Check: not self-loop, not the original neighbor, and edge doesn't already exist (in either direction for undirected graphs)
                        let edge_exists = graph.find_edge(nodes[i], nodes[target]).is_some()
                            || graph.find_edge(nodes[target], nodes[i]).is_some();

                        if target != i && target != neighbor && !edge_exists {
                            found_valid_target = true;
                            break target;
                        }
                        // Fallback: if we've tried many times, skip this rewiring
                        if attempts >= max_attempts {
                            break neighbor; // Use original neighbor as fallback
                        }
                    };

                    if found_valid_target {
                        graph.add_edge(nodes[i], nodes[new_target], 1.0);
                    } else {
                        // Re-add the original edge if rewiring failed
                        graph.add_edge(nodes[i], nodes[neighbor], 1.0);
                    }
                }
            }
        }
    }
    Ok(graph)
}

/// Generates a Barabási–Albert scale-free graph.
///
/// # Arguments
///
/// * `n` - The total number of nodes (must be >= m).
/// * `m` - The number of edges to attach from a new node to existing nodes (must be > 0).
/// * `seed` - The seed for the random number generator.
///
/// # Type Parameters
///
/// * `Ty` - The graph type implementing `GraphConstructor<u32, f32>`. Typically used with undirected graphs.
///
/// # Returns
///
/// * `Result<BaseGraph<u32, f32, Ty>, GraphinaException>` - The generated Barabási–Albert graph.
///
/// # Notes
///
/// The algorithm starts with a complete graph of m nodes, then each new node attaches to m existing nodes
/// with probability proportional to their degree (preferential attachment). This implementation uses a simple
/// linear scan for degree selection and may be less efficient for very large graphs.
pub fn barabasi_albert_graph<Ty: GraphConstructor<u32, f32>>(
    n: usize,
    m: usize,
    seed: u64,
) -> Result<BaseGraph<u32, f32, Ty>, GraphinaError> {
    if m == 0 || n < m {
        return Err(GraphinaError::InvalidArgument(
            "n must be at least m and m must be > 0.".into(),
        ));
    }
    let mut graph = BaseGraph::<u32, f32, Ty>::new();
    // Start with a complete graph of m nodes.
    let mut nodes = Vec::with_capacity(n);
    for i in 0..m {
        nodes.push(graph.add_node(i as u32));
    }
    for i in 0..m {
        for j in (i + 1)..m {
            graph.add_edge(nodes[i], nodes[j], 1.0);
        }
    }
    let mut rng = StdRng::seed_from_u64(seed);
    let mut degrees: Vec<usize> = vec![m - 1; m];
    let mut total_degree = m * (m - 1);
    for i in m..n {
        let new_node = graph.add_node(i as u32);
        nodes.push(new_node);
        let mut targets = Vec::new();
        let max_attempts = n * 10; // Prevent infinite loop
        let mut attempts = 0;

        // Use preferential attachment only if total_degree > 0
        if total_degree > 0 {
            while targets.len() < m && attempts < max_attempts {
                let r = rng.random_range(0..total_degree);
                let mut cumulative = 0;
                for (idx, &deg) in degrees.iter().enumerate() {
                    cumulative += deg;
                    if r < cumulative {
                        let candidate = nodes[idx];
                        // Only add if not already in targets and not the new node itself
                        if !targets.contains(&candidate) && candidate != new_node {
                            targets.push(candidate);
                        }
                        break;
                    }
                }
                attempts += 1;
            }
        }

        // If we couldn't find m targets through preferential attachment,
        // fill remaining slots with random selection from available nodes
        if targets.len() < m {
            for node in nodes.iter().take(i) {
                if targets.len() >= m {
                    break;
                }
                if !targets.contains(node) {
                    targets.push(*node);
                }
            }
        }

        for target in &targets {
            graph.add_edge(new_node, *target, 1.0);
            let idx = nodes.iter().position(|&x| x == *target).unwrap();
            degrees[idx] += 1;
        }
        degrees.push(targets.len());
        total_degree += 2 * targets.len();
    }
    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{Directed, Undirected};

    #[test]
    fn test_erdos_renyi_directed() {
        let graph = erdos_renyi_graph::<Directed>(3, 1.0, 42)
            .expect("Failed to generate directed Erdős–Rényi graph");
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 6);
    }

    #[test]
    fn test_erdos_renyi_undirected() {
        let graph = erdos_renyi_graph::<Undirected>(3, 1.0, 42)
            .expect("Failed to generate undirected Erdős–Rényi graph");
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 3);
    }

    #[test]
    fn test_complete_graph_directed() {
        let graph =
            complete_graph::<Directed>(4).expect("Failed to generate directed complete graph");
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 12);
    }

    #[test]
    fn test_complete_graph_undirected() {
        let graph =
            complete_graph::<Undirected>(4).expect("Failed to generate undirected complete graph");
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 6);
    }

    #[test]
    fn test_bipartite_graph() {
        let graph = bipartite_graph::<Undirected>(3, 2, 1.0, 42)
            .expect("Failed to generate bipartite graph");
        assert_eq!(graph.node_count(), 5);
        assert_eq!(graph.edge_count(), 6);
    }

    #[test]
    fn test_star_graph() {
        let graph = star_graph::<Undirected>(5).expect("Failed to generate star graph");
        assert_eq!(graph.node_count(), 5);
        assert_eq!(graph.edge_count(), 4);
    }

    #[test]
    fn test_cycle_graph() {
        let graph = cycle_graph::<Undirected>(5).expect("Failed to generate cycle graph");
        assert_eq!(graph.node_count(), 5);
        assert_eq!(graph.edge_count(), 5);
    }

    #[test]
    fn test_cycle_graph_invalid_n() {
        assert!(cycle_graph::<Undirected>(0).is_err());
        assert!(cycle_graph::<Undirected>(1).is_err());
        assert!(cycle_graph::<Undirected>(2).is_err());
    }

    #[test]
    fn test_watts_strogatz_graph() {
        let n = 10;
        let k = 4;
        let beta = 0.5;
        let seed = 42;
        let graph = watts_strogatz_graph::<Undirected>(n, k, beta, seed)
            .expect("Failed to generate Watts–Strogatz graph");
        assert_eq!(graph.node_count(), n);
        assert!(graph.edge_count() >= n * k / 2);
    }

    #[test]
    fn test_barabasi_albert_graph() {
        let n = 20;
        let m = 3;
        let seed = 42;
        let graph = barabasi_albert_graph::<Undirected>(n, m, seed)
            .expect("Failed to generate Barabási–Albert graph");
        assert_eq!(graph.node_count(), n);
        let expected_edges = (m * (m - 1) / 2) + (n - m) * m;
        assert_eq!(graph.edge_count(), expected_edges);
    }

    #[test]
    fn invalid_erdos_params_rejected() {
        assert!(matches!(
            erdos_renyi_graph::<Undirected>(0, 0.5, 1),
            Err(GraphinaError::InvalidArgument(_))
        ));
        assert!(matches!(
            erdos_renyi_graph::<Undirected>(10, 1.5, 1),
            Err(GraphinaError::InvalidArgument(_))
        ));
    }

    #[test]
    fn invalid_ws_params_rejected() {
        assert!(matches!(
            watts_strogatz_graph::<Undirected>(0, 2, 0.1, 1),
            Err(GraphinaError::InvalidArgument(_))
        ));
        assert!(matches!(
            watts_strogatz_graph::<Undirected>(10, 3, 0.1, 1),
            Err(GraphinaError::InvalidArgument(_))
        ));
        assert!(matches!(
            watts_strogatz_graph::<Undirected>(10, 2, 1.5, 1),
            Err(GraphinaError::InvalidArgument(_))
        ));
    }

    #[test]
    fn invalid_cycle_rejected() {
        assert!(matches!(
            cycle_graph::<Undirected>(2),
            Err(GraphinaError::InvalidArgument(_))
        ));
    }

    #[test]
    fn invalid_star_rejected() {
        assert!(matches!(
            star_graph::<Directed>(0),
            Err(GraphinaError::InvalidArgument(_))
        ));
    }

    #[test]
    fn invalid_ba_params_rejected() {
        assert!(matches!(
            barabasi_albert_graph::<Undirected>(5, 0, 1),
            Err(GraphinaError::InvalidArgument(_))
        ));
        assert!(matches!(
            barabasi_albert_graph::<Undirected>(3, 4, 1),
            Err(GraphinaError::InvalidArgument(_))
        ));
    }
}
