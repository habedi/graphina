/*!
Parallel triangle counting algorithms
*/

use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use petgraph::EdgeType;

/// Parallel triangle counting for all nodes.
///
/// Counts the number of triangles each node participates in, in parallel.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::Graph;
/// use graphina::parallel::triangles_parallel;
///
/// let mut g = Graph::<i32, f64>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// let n3 = g.add_node(3);
/// g.add_edge(n1, n2, 1.0);
/// g.add_edge(n2, n3, 1.0);
/// g.add_edge(n3, n1, 1.0);
///
/// let triangles = triangles_parallel(&g);
/// assert_eq!(triangles[&n1], 1);
/// ```
pub fn triangles_parallel<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> HashMap<NodeId, usize>
where
    A: Sync,
    W: Sync,
    Ty: GraphConstructor<A, W> + EdgeType + Sync,
{
    let nodes: Vec<NodeId> = graph.node_ids().collect();

    // Precompute every node's neighbor set once (O(V + E)) and share it read-only
    // across the rayon tasks, so the inner adjacency test is an O(1) hash lookup
    // instead of an O(degree) `contains_edge` scan.
    let adjacency: HashMap<NodeId, HashSet<NodeId>> = nodes
        .iter()
        .map(|&node| (node, graph.neighbors(node).collect()))
        .collect();

    nodes
        .par_iter()
        .map(|&node| {
            let neighbors: Vec<NodeId> = graph.neighbors(node).collect();
            let mut count = 0;

            for i in 0..neighbors.len() {
                let si = &adjacency[&neighbors[i]];
                for other in &neighbors[i + 1..] {
                    if si.contains(other) {
                        count += 1;
                    }
                }
            }

            (node, count)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_triangles_parallel() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);
        g.add_edge(n3, n1, 1.0);
        g.add_edge(n1, n4, 1.0);

        let triangles = triangles_parallel(&g);
        assert_eq!(triangles[&n1], 1);
        assert_eq!(triangles[&n2], 1);
        assert_eq!(triangles[&n3], 1);
        assert_eq!(triangles[&n4], 0);
    }
}
