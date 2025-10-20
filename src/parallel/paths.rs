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
}
