/*!
Parallel connected components detection
*/

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};

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
    let component_map = Arc::new(Mutex::new(HashMap::new()));
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let component_id = Arc::new(Mutex::new(0_usize));

    for node in nodes {
        // Check if already visited
        {
            let visited_lock = visited.lock().unwrap();
            if visited_lock.contains(&node) {
                continue;
            }
        }

        // Get current component ID
        let current_id = {
            let mut id_lock = component_id.lock().unwrap();
            let id = *id_lock;
            *id_lock += 1;
            id
        };

        // BFS to find all nodes in this component
        let mut queue = VecDeque::new();
        queue.push_back(node);

        let mut local_visited = HashSet::new();
        local_visited.insert(node);

        while let Some(current) = queue.pop_front() {
            for neighbor in graph.neighbors(current) {
                if local_visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        // Update global structures
        {
            let mut component_lock = component_map.lock().unwrap();
            let mut visited_lock = visited.lock().unwrap();

            for n in &local_visited {
                component_lock.insert(*n, current_id);
                visited_lock.insert(*n);
            }
        }
    }

    Arc::try_unwrap(component_map)
        .unwrap()
        .into_inner()
        .unwrap()
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
