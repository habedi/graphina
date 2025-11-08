//! Approximation algorithms for Ramsey theory problems.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use std::collections::HashSet;

/// Approximate Ramsey R2 by computing a maximum clique and a maximum independent set.
pub fn ramsey_r2<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> (HashSet<NodeId>, HashSet<NodeId>)
where
    Ty: GraphConstructor<A, f64>,
{
    let clique = super::clique::max_clique(graph);
    let independent_set = super::independent_set::maximum_independent_set(graph);
    (clique, independent_set)
}
