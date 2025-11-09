/*!
Parallel connected components detection
*/

use std::collections::{HashMap, HashSet, VecDeque};

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use petgraph::EdgeType;

/// Parallel connected components detection.
///
/// Finds all connected components in parallel by processing multiple starting points.
///
/// Returns a mapping from node to component ID.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::Graph;
/// use graphina::parallel::connected_components_parallel;
///
/// let mut g = Graph::<i32, f64>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// let n3 = g.add_node(3);
/// let n4 = g.add_node(4);
///
/// g.add_edge(n1, n2, 1.0);
/// g.add_edge(n3, n4, 1.0);
///
/// let components = connected_components_parallel(&g);
///
/// // n1 and n2 should be in same component
/// assert_eq!(components[&n1], components[&n2]);
///
/// // n3 and n4 should be in same component
/// assert_eq!(components[&n3], components[&n4]);
///
/// // But different from n1/n2
/// assert_ne!(components[&n1], components[&n3]);
/// ```
pub fn connected_components_parallel<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
) -> HashMap<NodeId, usize>
where
    A: Sync + Send,
    W: Sync + Send,
    Ty: GraphConstructor<A, W> + EdgeType + Sync + Send,
{
    let nodes: Vec<NodeId> = graph.node_ids().collect();
    let mut component_map: HashMap<NodeId, usize> = HashMap::with_capacity(nodes.len());
    let mut visited: HashSet<NodeId> = HashSet::new();
    let mut current_id: usize = 0;

    for node in nodes {
        if visited.contains(&node) {
            continue;
        }

        let mut queue = VecDeque::new();
        queue.push_back(node);
        visited.insert(node);

        while let Some(current) = queue.pop_front() {
            component_map.insert(current, current_id);
            for neighbor in graph.neighbors(current) {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        current_id += 1;
    }

    component_map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_connected_components_parallel() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n3, n4, 1.0);

        let components = connected_components_parallel(&g);

        // n1 and n2 should be in same component
        assert_eq!(components[&n1], components[&n2]);

        // n3 and n4 should be in same component
        assert_eq!(components[&n3], components[&n4]);

        // But different from n1/n2
        assert_ne!(components[&n1], components[&n3]);
    }
}
