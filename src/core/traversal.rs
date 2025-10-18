/*!
# Graph Traversal Algorithms

This module implements a collection of graph traversal algorithms for the graph types defined in `src/core/types.rs`.
The algorithms include:

- **Breadth-First Search (BFS):**
  Traverses a graph level-by-level starting from a specified node.

- **Depth-First Search (DFS):**
  Recursively traverses a graph depth-first starting from a specified node.

- **Iterative Deepening Depth-First Search (IDDFS):**
  Combines the space efficiency of DFS with the optimality of BFS by iteratively deepening the search.

- **Bidirectional Search:**
  Simultaneously searches from the start and target nodes, potentially reducing search time by meeting in the middle.

For the search-based algorithms that find a path (IDDFS and Bidirectional Search), additional "try"
variants are provided that return a `Result` and use the custom exception
`graphina::core::exceptions::GraphinaNoPath` if no valid path exists.
*/

use crate::core::exceptions::GraphinaNoPath;
use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::{HashMap, HashSet, VecDeque};

/// Performs a breadth-first search (BFS) starting from `start`.
///
/// Returns a vector of nodes in the order they were visited.
///
/// # Arguments
///
/// * `graph` - A reference to a graph that implements `BaseGraph`.
/// * `start` - The starting node identifier.
///
/// # Complexity
///
/// - **Time:** O(V + E)
/// - **Space:** O(V)
///
/// # Example
///
/// ```rust
/// use graphina::core::types::{Graph, NodeId};
/// use graphina::core::traversal::bfs;
///
/// // Create a simple undirected graph with integer nodes.
/// let mut graph = Graph::<i32, ()>::new();
/// let n1 = graph.add_node(1);
/// let n2 = graph.add_node(2);
/// graph.add_edge(n1, n2, ());
///
/// let order = bfs(&graph, n1);
/// println!("BFS Order: {:?}", order);
/// ```
pub fn bfs<A, W, Ty>(graph: &BaseGraph<A, W, Ty>, start: NodeId) -> Vec<NodeId>
where
    Ty: GraphConstructor<A, W>,
{
    // Check if start node exists in the graph
    if graph.node_attr(start).is_none() {
        return Vec::new();
    }

    let mut visited = HashSet::new();
    let mut order = Vec::new();
    let mut queue = VecDeque::new();

    visited.insert(start);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        order.push(node);
        for neighbor in graph.neighbors(node) {
            if visited.insert(neighbor) {
                queue.push_back(neighbor);
            }
        }
    }
    order
}

/// Performs a depth-first search (DFS) starting from `start`.
///
/// Returns a vector of nodes in the order they were first visited.
///
/// # Arguments
///
/// * `graph` - A reference to a graph that implements `BaseGraph`.
/// * `start` - The starting node identifier.
///
/// # Complexity
///
/// - **Time:** O(V + E)
/// - **Space:** O(V)
///
/// # Example
///
/// ```rust
/// use graphina::core::types::{Graph, NodeId};
/// use graphina::core::traversal::dfs;
///
/// // Create a simple undirected graph.
/// let mut graph = Graph::<i32, ()>::new();
/// let n1 = graph.add_node(1);
/// let n2 = graph.add_node(2);
/// graph.add_edge(n1, n2, ());
///
/// let order = dfs(&graph, n1);
/// println!("DFS Order: {:?}", order);
/// ```
pub fn dfs<A, W, Ty>(graph: &BaseGraph<A, W, Ty>, start: NodeId) -> Vec<NodeId>
where
    Ty: GraphConstructor<A, W>,
{
    // Check if start node exists in the graph
    if graph.node_attr(start).is_none() {
        return Vec::new();
    }

    let mut visited = HashSet::new();
    let mut order = Vec::new();
    dfs_util(graph, start, &mut visited, &mut order);
    order
}

/// Recursive helper function for DFS.
///
/// # Arguments
///
/// * `graph` - A reference to a graph that implements `BaseGraph`.
/// * `node` - The current node identifier.
/// * `visited` - A mutable set to track visited nodes.
/// * `order` - A mutable vector to record the visitation order.
fn dfs_util<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    node: NodeId,
    visited: &mut HashSet<NodeId>,
    order: &mut Vec<NodeId>,
) where
    Ty: GraphConstructor<A, W>,
{
    if !visited.insert(node) {
        return;
    }
    order.push(node);
    for neighbor in graph.neighbors(node) {
        if !visited.contains(&neighbor) {
            dfs_util(graph, neighbor, visited, order);
        }
    }
}

/// Performs iterative deepening depth-first search (IDDFS) to find a path from `start` to `target`.
///
/// The search is executed with increasing depth limits until `max_depth` is reached.
///
/// # Arguments
///
/// * `graph` - A reference to a graph that implements `BaseGraph`.
/// * `start` - The starting node identifier.
/// * `target` - The target node identifier.
/// * `max_depth` - The maximum depth to search.
///
/// # Returns
///
/// An `Option` containing the path as a vector of `NodeId` if found, or `None` if no path exists within the given depth.
///
/// # Complexity
///
/// - **Time:** In the worst-case, O(b^d) where `b` is the branching factor and `d` is the depth of the solution.
/// - **Space:** O(d), where `d` is the maximum search depth.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::{Graph, NodeId};
/// use graphina::core::traversal::iddfs;
///
/// // Create a simple undirected graph.
/// let mut graph = Graph::<i32, ()>::new();
/// let n1 = graph.add_node(1);
/// let n2 = graph.add_node(2);
/// let n3 = graph.add_node(3);
/// graph.add_edge(n1, n2, ());
/// graph.add_edge(n2, n3, ());
///
/// let path = iddfs(&graph, n1, n3, 5);
/// assert!(path.is_some());
/// ```
pub fn iddfs<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    start: NodeId,
    target: NodeId,
    max_depth: usize,
) -> Option<Vec<NodeId>>
where
    Ty: GraphConstructor<A, W>,
{
    for depth in 0..=max_depth {
        let mut path = Vec::new();
        let mut visited = HashSet::new();
        if dls(graph, start, target, depth, &mut visited, &mut path) {
            return Some(path);
        }
    }
    None
}

/// "Try" variant of `iddfs` that returns a `Result`, using a `GraphinaNoPath` exception
/// if no path is found within the given depth.
///
/// # Arguments
///
/// * `graph` - A reference to a graph that implements `BaseGraph`.
/// * `start` - The starting node identifier.
/// * `target` - The target node identifier.
/// * `max_depth` - The maximum depth to search.
///
/// # Returns
///
/// `Ok(Vec<NodeId>)` if a path is found; otherwise, `Err(GraphinaNoPath)`.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::{Graph, NodeId};
/// use graphina::core::traversal::try_iddfs;
///
/// let mut graph = Graph::<i32, ()>::new();
/// let n1 = graph.add_node(1);
/// let n2 = graph.add_node(2);
/// let n3 = graph.add_node(3);
/// graph.add_edge(n1, n2, ());
/// graph.add_edge(n2, n3, ());
///
/// let result = try_iddfs(&graph, n1, n3, 5);
/// match result {
///     Ok(path) => println!("IDDFS Path: {:?}", path),
///     Err(err) => println!("Error: {}", err),
/// }
/// ```
pub fn try_iddfs<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    start: NodeId,
    target: NodeId,
    max_depth: usize,
) -> Result<Vec<NodeId>, GraphinaNoPath>
where
    Ty: GraphConstructor<A, W>,
{
    iddfs(graph, start, target, max_depth).ok_or_else(|| {
        GraphinaNoPath::new("No path found using IDDFS within the given depth limit")
    })
}

/// Depth-limited search helper for IDDFS.
///
/// # Arguments
///
/// * `graph` - A reference to a graph that implements `BaseGraph`.
/// * `current` - The current node identifier.
/// * `target` - The target node identifier.
/// * `depth` - The current depth limit.
/// * `visited` - A mutable set of visited nodes.
/// * `path` - A mutable vector representing the current path.
///
/// # Returns
///
/// `true` if the target is found within the current depth, otherwise `false`.
fn dls<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    current: NodeId,
    target: NodeId,
    depth: usize,
    visited: &mut HashSet<NodeId>,
    path: &mut Vec<NodeId>,
) -> bool
where
    Ty: GraphConstructor<A, W>,
{
    path.push(current);
    visited.insert(current);

    if current == target {
        return true;
    }

    if depth > 0 {
        for neighbor in graph.neighbors(current) {
            if !visited.contains(&neighbor)
                && dls(graph, neighbor, target, depth - 1, visited, path)
            {
                return true;
            }
        }
    }

    path.pop();
    visited.remove(&current);
    false
}

/// Performs a bidirectional search between `start` and `target`.
///
/// This algorithm expands both from the start and the target nodes, checking for an intersection to reconstruct the shortest path.
///
/// # Arguments
///
/// * `graph` - A reference to a graph that implements `BaseGraph`.
/// * `start` - The starting node identifier.
/// * `target` - The target node identifier.
///
/// # Complexity
///
/// - **Time:** On average O(b^(d/2)), but in the worst-case O(b^d) where `b` is the branching factor and `d` is the distance between nodes.
/// - **Space:** O(b^(d/2)), where `b` is the branching factor and `d` is the distance between nodes.
///
/// # Returns
///
/// An `Option` containing the shortest path (as a vector of `NodeId`) if found, or `None` if no path exists.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::{Graph, NodeId};
/// use graphina::core::traversal::bidis;
///
/// let mut graph = Graph::<i32, ()>::new();
/// let n1 = graph.add_node(1);
/// let n2 = graph.add_node(2);
/// let n3 = graph.add_node(3);
/// graph.add_edge(n1, n2, ());
/// graph.add_edge(n2, n3, ());
///
/// let path = bidis(&graph, n1, n3);
/// assert!(path.is_some());
/// ```
pub fn bidis<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    start: NodeId,
    target: NodeId,
) -> Option<Vec<NodeId>>
where
    Ty: GraphConstructor<A, W>,
{
    if start == target {
        return Some(vec![start]);
    }

    let mut forward_queue = VecDeque::new();
    let mut backward_queue = VecDeque::new();

    let mut forward_parents: HashMap<NodeId, Option<NodeId>> = HashMap::new();
    let mut backward_parents: HashMap<NodeId, Option<NodeId>> = HashMap::new();

    let mut forward_visited = HashSet::new();
    let mut backward_visited = HashSet::new();

    // Track the current frontier separately from visited
    let mut forward_frontier = HashSet::new();
    let mut backward_frontier = HashSet::new();

    forward_queue.push_back(start);
    forward_visited.insert(start);
    forward_frontier.insert(start);
    forward_parents.insert(start, None);

    backward_queue.push_back(target);
    backward_visited.insert(target);
    backward_frontier.insert(target);
    backward_parents.insert(target, None);

    let mut meeting_node = None;

    while !forward_queue.is_empty() && !backward_queue.is_empty() {
        // Check for intersection in current frontiers
        if let Some(&meet) = forward_frontier.intersection(&backward_frontier).next() {
            meeting_node = Some(meet);
            break;
        }

        // Clear previous frontier and expand forward one level
        forward_frontier.clear();
        let forward_level = forward_queue.len();
        for _ in 0..forward_level {
            let current = forward_queue.pop_front().unwrap();
            for neighbor in graph.neighbors(current) {
                if forward_visited.insert(neighbor) {
                    forward_queue.push_back(neighbor);
                    forward_frontier.insert(neighbor);
                    forward_parents.insert(neighbor, Some(current));
                }
            }
        }

        // Check if forward frontier intersects with backward visited
        if let Some(&meet) = forward_frontier.intersection(&backward_visited).next() {
            meeting_node = Some(meet);
            break;
        }

        // Clear previous frontier and expand backward one level
        backward_frontier.clear();
        let backward_level = backward_queue.len();
        for _ in 0..backward_level {
            let current = backward_queue.pop_front().unwrap();
            for neighbor in get_backward_neighbors(graph, current) {
                if backward_visited.insert(neighbor) {
                    backward_queue.push_back(neighbor);
                    backward_frontier.insert(neighbor);
                    backward_parents.insert(neighbor, Some(current));
                }
            }
        }

        // Check if backward frontier intersects with forward visited
        if let Some(&meet) = backward_frontier.intersection(&forward_visited).next() {
            meeting_node = Some(meet);
            break;
        }
    }

    if let Some(meet) = meeting_node {
        // Reconstruct the path from start to the meeting node.
        let mut forward_path = Vec::new();
        let mut cur = meet;
        while let Some(&Some(parent)) = forward_parents.get(&cur) {
            forward_path.push(cur);
            cur = parent;
        }
        forward_path.push(start);
        forward_path.reverse();

        // Reconstruct the path from the meeting node to target.
        let mut backward_path = Vec::new();
        cur = meet;
        while let Some(&Some(parent)) = backward_parents.get(&cur) {
            backward_path.push(parent);
            cur = parent;
        }

        let mut full_path = forward_path;
        full_path.extend(backward_path);
        return Some(full_path);
    }

    None
}

/// "Try" variant of `bidirectional_search` that returns a `Result`, using a `GraphinaNoPath` exception
/// if no path is found.
///
/// # Arguments
///
/// * `graph` - A reference to a graph that implements `BaseGraph`.
/// * `start` - The starting node identifier.
/// * `target` - The target node identifier.
///
/// # Returns
///
/// `Ok(Vec<NodeId>)` if a path is found; otherwise, `Err(GraphinaNoPath)`.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::{Graph, NodeId};
/// use graphina::core::traversal::try_bidirectional_search;
///
/// let mut graph = Graph::<i32, ()>::new();
/// let n1 = graph.add_node(1);
/// let n2 = graph.add_node(2);
/// let n3 = graph.add_node(3);
/// graph.add_edge(n1, n2, ());
/// graph.add_edge(n2, n3, ());
///
/// let result = try_bidirectional_search(&graph, n1, n3);
/// match result {
///     Ok(path) => println!("Bidirectional Search Path: {:?}", path),
///     Err(err) => println!("Error: {}", err),
/// }
/// ```
pub fn try_bidirectional_search<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    start: NodeId,
    target: NodeId,
) -> Result<Vec<NodeId>, GraphinaNoPath>
where
    Ty: GraphConstructor<A, W>,
{
    bidis(graph, start, target)
        .ok_or_else(|| GraphinaNoPath::new("No path exists between the specified nodes"))
}

/// Helper function to obtain backward neighbors for bidirectional search.
///
/// For directed graphs, returns all nodes `u` with an edge `u -> node`.
/// For undirected graphs, returns the symmetric neighbors.
///
/// # Arguments
///
/// * `graph` - A reference to a graph that implements `BaseGraph`.
/// * `node` - The node identifier for which to retrieve backward neighbors.
fn get_backward_neighbors<A, W, Ty>(graph: &BaseGraph<A, W, Ty>, node: NodeId) -> Vec<NodeId>
where
    Ty: GraphConstructor<A, W>,
{
    if <Ty as GraphConstructor<A, W>>::is_directed() {
        // For directed graphs, iterate over all edges and select those incoming to `node`.
        graph
            .edges()
            .filter(|(_, tgt, _)| *tgt == node)
            .map(|(src, _, _)| src)
            .collect()
    } else {
        // For undirected graphs, the neighbors are symmetric.
        graph.neighbors(node).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;
    #[test]
    fn test_bfs() {
        let mut graph = Graph::<i32, ()>::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());
        let visited = bfs(&graph, n1);
        assert_eq!(visited.len(), 3);
        assert!(visited.contains(&n1));
        assert!(visited.contains(&n2));
        assert!(visited.contains(&n3));
    }
    #[test]
    fn test_dfs() {
        let mut graph = Graph::<i32, ()>::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());
        let visited = dfs(&graph, n1);
        assert_eq!(visited.len(), 3);
        assert!(visited.contains(&n1));
        assert!(visited.contains(&n2));
        assert!(visited.contains(&n3));
    }
    #[test]
    fn test_bidis() {
        let mut graph = Graph::<i32, ()>::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());
        let path = bidis(&graph, n1, n3);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path[0], n1);
        assert_eq!(path[path.len() - 1], n3);
    }
}
