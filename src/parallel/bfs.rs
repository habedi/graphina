/*!
Parallel breadth-first search algorithms
*/

use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use petgraph::EdgeType;

/// Parallel breadth-first search from multiple starting nodes.
///
/// Processes multiple BFS searches in parallel, useful for computing shortest paths
/// from multiple sources simultaneously.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::Graph;
/// use graphina::parallel::bfs_parallel;
///
/// let mut g = Graph::<i32, f64>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// let n3 = g.add_node(3);
/// g.add_edge(n1, n2, 1.0);
/// g.add_edge(n2, n3, 1.0);
///
/// let starts = vec![n1, n2];
/// let results = bfs_parallel(&g, &starts);
/// assert_eq!(results.len(), 2);
/// ```
pub fn bfs_parallel<A, W, Ty>(graph: &BaseGraph<A, W, Ty>, starts: &[NodeId]) -> Vec<Vec<NodeId>>
where
    A: Sync,
    W: Sync,
    Ty: GraphConstructor<A, W> + EdgeType + Sync,
{
    starts
        .par_iter()
        .map(|&start| {
            let mut visited = Vec::new();
            let mut queue = VecDeque::new();
            let mut seen = HashSet::new();

            queue.push_back(start);
            seen.insert(start);

            while let Some(node) = queue.pop_front() {
                visited.push(node);

                for neighbor in graph.neighbors(node) {
                    if seen.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }

            visited
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_bfs_parallel() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);

        let results = bfs_parallel(&g, &[n1, n3]);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].len(), 3);
        assert_eq!(results[1].len(), 3);
    }
}
