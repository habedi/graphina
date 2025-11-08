/*!
Parallel degree computation algorithms
*/

use rayon::prelude::*;
use std::collections::HashMap;

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use petgraph::EdgeType;

/// Parallel computation of node degrees.
///
/// Computes the degree of all nodes in parallel.
///
/// # Example
///
/// ```rust
/// use graphina::core::types::Graph;
/// use graphina::parallel::degrees_parallel;
///
/// let mut g = Graph::<i32, f64>::new();
/// let n1 = g.add_node(1);
/// let n2 = g.add_node(2);
/// g.add_edge(n1, n2, 1.0);
///
/// let degrees = degrees_parallel(&g);
/// assert_eq!(degrees[&n1], 1);
/// assert_eq!(degrees[&n2], 1);
/// ```
pub fn degrees_parallel<A, W, Ty>(graph: &BaseGraph<A, W, Ty>) -> HashMap<NodeId, usize>
where
    A: Sync,
    W: Sync,
    Ty: GraphConstructor<A, W> + EdgeType + Sync,
{
    let nodes: Vec<NodeId> = graph.node_ids().collect();

    nodes
        .par_iter()
        .map(|&node| {
            let degree = graph.degree(node).unwrap_or(0);
            (node, degree)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_degrees_parallel() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 1.0);

        let degrees = degrees_parallel(&g);
        assert_eq!(degrees[&n1], 1);
        assert_eq!(degrees[&n2], 2);
        assert_eq!(degrees[&n3], 1);
    }
}
