/*!
Parallel shortest path algorithms
*/

use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use petgraph::EdgeType;

/// Parallel shortest path distances from multiple sources.
///
/// Computes shortest path distances from multiple source nodes in parallel.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::Graph;
/// use graphina::parallel::shortest_paths_parallel;
///
/// let mut g = Graph::<i32, f64>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// let n3 = g.add_node(3);
/// g.add_edge(n1, n2, 1.0);
/// g.add_edge(n2, n3, 1.0);
///
/// let sources = vec![n1, n3];
/// let distances = shortest_paths_parallel(&g, &sources);
/// assert_eq!(distances.len(), 2);
/// ```
pub fn shortest_paths_parallel<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    sources: &[NodeId],
) -> Vec<HashMap<NodeId, usize>>
where
    A: Sync,
    W: Sync,
    Ty: GraphConstructor<A, W> + EdgeType + Sync,
{
    sources
        .par_iter()
        .map(|&source| {
            let mut distances = HashMap::new();
            let mut queue = VecDeque::new();

            distances.insert(source, 0);
            queue.push_back(source);

            while let Some(node) = queue.pop_front() {
                let dist = distances[&node];

                for neighbor in graph.neighbors(node) {
                    if let std::collections::hash_map::Entry::Vacant(e) = distances.entry(neighbor)
                    {
                        e.insert(dist + 1);
                        queue.push_back(neighbor);
                    }
                }
            }

            distances
        })
        .collect()
}

/// Parallel unweighted all-pairs distance matrix.
///
/// Runs one breadth-first search per source across a Rayon thread pool, producing
/// the same dense hop-count matrix as the sequential
/// [`crate::core::paths::all_pairs_shortest_path_length`]. Each row is independent,
/// so the result is identical to the sequential version and independent of the
/// thread count. Reimplemented over `core` rather than calling that function, so
/// `parallel` stays dependent on `core` alone.
///
/// Returns the node ordering and a row-major matrix: `nodes[i]` is the source for
/// row `i`, and `matrix[i][j]` is the hop distance from `nodes[i]` to `nodes[j]`,
/// with `None` for an unreachable target and `Some(0)` on the diagonal. Cells are
/// `u32` to match the sequential core version and halve the O(V^2) write bandwidth.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::Graph;
/// use graphina::parallel::all_pairs_shortest_path_length_parallel;
///
/// let mut g = Graph::<i32, f64>::new();
/// let n0 = g.add_node(0);
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// g.add_edge(n0, n1, 1.0);
/// g.add_edge(n1, n2, 1.0);
///
/// let (nodes, matrix) = all_pairs_shortest_path_length_parallel(&g);
/// let pos = |node| nodes.iter().position(|&x| x == node).unwrap();
/// assert_eq!(matrix[pos(n0)][pos(n2)], Some(2));
/// ```
pub fn all_pairs_shortest_path_length_parallel<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
) -> (Vec<NodeId>, Vec<Vec<Option<u32>>>)
where
    A: Sync,
    W: Copy + Sync,
    Ty: GraphConstructor<A, W> + Sync,
{
    // Snapshot the adjacency into a compact CSR in position space [0, n) once, so
    // every per-source BFS walks contiguous `u32` slices instead of the
    // `StableGraph` adjacency (a linked list kept for NodeId stability). This mirrors
    // the sequential `core::paths::all_pairs_shortest_path_length`; the CSR build is
    // duplicated here (rather than shared) so `parallel` depends on `core` alone.
    let bound = graph
        .node_ids()
        .map(|n| n.index())
        .max()
        .map_or(0, |m| m + 1);
    let nodes: Vec<NodeId> = graph.node_ids().collect();
    let n = nodes.len();

    let mut pos = vec![0u32; bound];
    for (i, node) in nodes.iter().enumerate() {
        pos[node.index()] = i as u32;
    }
    let mut offsets = vec![0usize; n + 1];
    for (i, &node) in nodes.iter().enumerate() {
        offsets[i + 1] = graph.neighbors(node).count();
    }
    for i in 0..n {
        offsets[i + 1] += offsets[i];
    }
    let mut adj = vec![0u32; offsets[n]];
    let mut cursor = offsets.clone();
    for (i, &node) in nodes.iter().enumerate() {
        for v in graph.neighbors(node) {
            adj[cursor[i]] = pos[v.index()];
            cursor[i] += 1;
        }
    }

    // One BFS per source, spread over the thread pool. `map_init` gives each worker
    // its own reusable `u32`-sentinel distance buffer and queue, so a worker
    // amortizes the allocation across every source it handles. Iterating `0..n` with
    // `into_par_iter` is order-preserving, so row `i` corresponds to `nodes[i]`.
    let offsets = &offsets;
    let adj = &adj;
    let matrix: Vec<Vec<Option<u32>>> = (0..n)
        .into_par_iter()
        .map_init(
            || (vec![u32::MAX; n], VecDeque::<u32>::new()),
            |(dist, queue), source| {
                dist.iter_mut().for_each(|d| *d = u32::MAX);
                dist[source] = 0;
                queue.clear();
                queue.push_back(source as u32);
                while let Some(current) = queue.pop_front() {
                    let du = dist[current as usize];
                    for &next in &adj[offsets[current as usize]..offsets[current as usize + 1]] {
                        if dist[next as usize] == u32::MAX {
                            dist[next as usize] = du + 1;
                            queue.push_back(next);
                        }
                    }
                }
                dist.iter()
                    .map(|&d| if d == u32::MAX { None } else { Some(d) })
                    .collect()
            },
        )
        .collect();

    (nodes, matrix)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_shortest_paths_parallel() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);

        let distances = shortest_paths_parallel(&g, &[n1, n3]);
        assert_eq!(distances[0][&n3], 2); // n1 to n3 is 2 hops
        assert_eq!(distances[1][&n1], 2); // n3 to n1 is 2 hops
    }

    #[test]
    fn test_all_pairs_parallel_matches_sequential() {
        // The parallel all-pairs matrix must equal the sequential core version cell
        // for cell, on a graph with mixed degrees and an unreachable node.
        use crate::core::paths::all_pairs_shortest_path_length;

        let mut g = Graph::<i32, f64>::new();
        let a = g.add_node(0);
        let b = g.add_node(1);
        let c = g.add_node(2);
        let d = g.add_node(3);
        let iso = g.add_node(4);
        g.add_edge(a, b, 1.0);
        g.add_edge(b, c, 1.0);
        g.add_edge(c, a, 1.0);
        g.add_edge(c, d, 1.0);
        let _ = iso;

        let (seq_nodes, seq) = all_pairs_shortest_path_length(&g);
        let (par_nodes, par) = all_pairs_shortest_path_length_parallel(&g);
        assert_eq!(seq_nodes, par_nodes, "node ordering must match");
        assert_eq!(seq, par, "distance matrices must match cell for cell");
    }
}
