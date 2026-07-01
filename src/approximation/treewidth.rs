//! Approximation algorithms for treewidth problems.

use crate::core::types::{BaseGraph, GraphConstructor, NodeId};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Add fill-in edges so that every pair of `neighbors` is adjacent in `adj`, then
/// remove `node` from the working graph. This is the elimination-game step shared by
/// both heuristics: eliminating a node makes its neighborhood a clique.
fn eliminate(adj: &mut FxHashMap<NodeId, FxHashSet<NodeId>>, node: NodeId, neighbors: &[NodeId]) {
    for i in 0..neighbors.len() {
        for j in (i + 1)..neighbors.len() {
            let (a, b) = (neighbors[i], neighbors[j]);
            if adj.get_mut(&a).is_some_and(|s| s.insert(b)) {
                if let Some(s) = adj.get_mut(&b) {
                    s.insert(a);
                }
            }
        }
    }
    adj.remove(&node);
    for &v in neighbors {
        if let Some(s) = adj.get_mut(&v) {
            s.remove(&node);
        }
    }
}

/// Compute a treewidth upper bound and elimination order using the Minimum Degree
/// heuristic with a min-heap. Each eliminated node contributes its current degree in
/// the partially filled-in graph; the reported width is the maximum such degree.
pub fn treewidth_min_degree<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> (usize, Vec<NodeId>)
where
    Ty: GraphConstructor<A, f64>,
{
    let mut order = Vec::new();
    // Mutable working graph. The elimination game adds fill-in edges as nodes are
    // removed, so a node's degree can grow, not just shrink; the original graph's
    // adjacency alone underestimates the width (it yields the degeneracy instead).
    let mut adj: FxHashMap<NodeId, FxHashSet<NodeId>> = graph
        .nodes()
        .map(|(u, _)| (u, graph.neighbors(u).filter(|&v| v != u).collect()))
        .collect();

    // Early return for empty graph
    if adj.is_empty() {
        return (0, order);
    }

    let mut treewidth = 0;
    // Lazy-deletion min-heap keyed by current degree. A popped entry is stale if its
    // degree no longer matches the node's live degree (or the node is gone), so it is
    // skipped; every degree change re-pushes the affected node.
    let mut heap: BinaryHeap<Reverse<(usize, NodeId)>> = adj
        .iter()
        .map(|(&u, nbrs)| Reverse((nbrs.len(), u)))
        .collect();

    while !adj.is_empty() {
        // Find the next node to eliminate
        let u = loop {
            match heap.pop() {
                Some(Reverse((deg, node))) => match adj.get(&node) {
                    Some(nbrs) if nbrs.len() == deg => break node,
                    _ => {} // stale degree or already eliminated, skip
                },
                None => {
                    // Defensive: the heap drained but nodes remain. Re-seed from the
                    // live graph rather than looping forever.
                    match adj.keys().next() {
                        Some(&node) => break node,
                        None => return (treewidth, order),
                    }
                }
            }
        };

        let neighbors: Vec<NodeId> = adj
            .get(&u)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default();
        if neighbors.len() > treewidth {
            treewidth = neighbors.len();
        }

        eliminate(&mut adj, u, &neighbors);
        order.push(u);

        // Re-push the (former) neighbors, whose degrees changed via fill-in or removal.
        for &v in &neighbors {
            if let Some(s) = adj.get(&v) {
                heap.push(Reverse((s.len(), v)));
            }
        }
    }
    (treewidth, order)
}

/// Number of fill-in edges eliminating `node` would add: the count of non-adjacent
/// pairs among its current neighbors. Ordered pairs are counted and halved, which
/// avoids materializing the neighbor list on every call.
fn fill_in_of(adj: &FxHashMap<NodeId, FxHashSet<NodeId>>, node: NodeId) -> usize {
    let Some(nbrs) = adj.get(&node) else {
        return 0;
    };
    let mut missing = 0;
    for &a in nbrs {
        let na = &adj[&a];
        for &b in nbrs {
            if a != b && !na.contains(&b) {
                missing += 1;
            }
        }
    }
    missing / 2
}

/// Add the edge `a`-`b` to the working graph and update the affected fill-in counts by
/// a delta. Adding the edge introduces one non-adjacent pair `(b, x)` in `a`'s
/// neighborhood for every current neighbor `x` of `a` not adjacent to `b` (and
/// symmetrically for `b`), and removes the now-adjacent pair `(a, b)` from every common
/// neighbor's count. The precondition is that `a != b` and the edge is absent.
fn add_fill_edge(
    adj: &mut FxHashMap<NodeId, FxHashSet<NodeId>>,
    fill: &mut FxHashMap<NodeId, usize>,
    touched: &mut FxHashSet<NodeId>,
    a: NodeId,
    b: NodeId,
) {
    let (delta_a, delta_b, common) = {
        let na = &adj[&a];
        let nb = &adj[&b];
        let delta_a = na.iter().filter(|x| !nb.contains(x)).count();
        let delta_b = nb.iter().filter(|x| !na.contains(x)).count();
        let common: Vec<NodeId> = na.iter().copied().filter(|x| nb.contains(x)).collect();
        (delta_a, delta_b, common)
    };
    if let Some(f) = fill.get_mut(&a) {
        *f += delta_a;
    }
    if let Some(f) = fill.get_mut(&b) {
        *f += delta_b;
    }
    for &w in &common {
        if let Some(f) = fill.get_mut(&w) {
            *f -= 1;
        }
        touched.insert(w);
    }
    if let Some(s) = adj.get_mut(&a) {
        s.insert(b);
    }
    if let Some(s) = adj.get_mut(&b) {
        s.insert(a);
    }
    touched.insert(a);
    touched.insert(b);
}

/// Remove eliminated node `u` (whose `neighbors` are already a clique) and update the
/// fill-in counts of its neighbors by a delta. Each neighbor `v` loses the non-adjacent
/// pairs `(u, x)` from its count, where `x` is a neighbor of `v` outside `N(u)`.
fn remove_eliminated(
    adj: &mut FxHashMap<NodeId, FxHashSet<NodeId>>,
    fill: &mut FxHashMap<NodeId, usize>,
    touched: &mut FxHashSet<NodeId>,
    u: NodeId,
    neighbors: &[NodeId],
) {
    for &v in neighbors {
        let dec = {
            let nv = &adj[&v];
            let nu = &adj[&u];
            nv.iter().filter(|&&x| x != u && !nu.contains(&x)).count()
        };
        if let Some(f) = fill.get_mut(&v) {
            *f -= dec;
        }
        touched.insert(v);
    }
    for &v in neighbors {
        if let Some(s) = adj.get_mut(&v) {
            s.remove(&u);
        }
    }
    adj.remove(&u);
}

/// Compute a treewidth upper bound and elimination order using the Minimum Fill-in
/// heuristic. At each step the node whose elimination adds the fewest fill-in edges is
/// removed; the reported width is the maximum degree at elimination time in the
/// partially filled-in graph.
///
/// The fill-in count of each node is held in a lazy-deletion min-heap and maintained by
/// deltas as edges are added and a node is removed, so an elimination costs work
/// proportional to the changed neighborhoods rather than a full rescan of every
/// remaining node (the earlier O(V^2 * d^2) per-step cost).
pub fn treewidth_min_fill_in<A, Ty>(graph: &BaseGraph<A, f64, Ty>) -> (usize, Vec<NodeId>)
where
    Ty: GraphConstructor<A, f64>,
{
    let mut order = Vec::new();
    // Mutable working graph carrying the fill-in accumulated so far, so both the
    // fill-in count and the width are measured against the current chordal
    // completion rather than the original edges.
    let mut adj: FxHashMap<NodeId, FxHashSet<NodeId>> = graph
        .nodes()
        .map(|(u, _)| (u, graph.neighbors(u).filter(|&v| v != u).collect()))
        .collect();

    // Early return for empty graph
    if adj.is_empty() {
        return (0, order);
    }

    let mut fill: FxHashMap<NodeId, usize> =
        adj.keys().map(|&u| (u, fill_in_of(&adj, u))).collect();
    // Min-heap by (fill-in, node id); ties break toward the lower node id, matching the
    // previous linear scan. A popped entry is stale if its fill-in no longer matches
    // the node's live value (or the node is gone), so it is skipped.
    let mut heap: BinaryHeap<Reverse<(usize, NodeId)>> =
        fill.iter().map(|(&u, &f)| Reverse((f, u))).collect();

    let mut treewidth = 0;
    while !adj.is_empty() {
        let u = loop {
            match heap.pop() {
                Some(Reverse((f, node))) => match fill.get(&node) {
                    Some(&cur) if cur == f => break node,
                    _ => {} // stale entry or already eliminated, skip
                },
                None => match adj.keys().next() {
                    Some(&node) => break node,
                    None => return (treewidth, order),
                },
            }
        };

        let neighbors: Vec<NodeId> = adj
            .get(&u)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default();
        if neighbors.len() > treewidth {
            treewidth = neighbors.len();
        }

        // Add the fill-in edges among u's neighbors, then remove u, maintaining each
        // affected fill-in count by a delta rather than a full recompute.
        let mut touched: FxHashSet<NodeId> = FxHashSet::default();
        for i in 0..neighbors.len() {
            for j in (i + 1)..neighbors.len() {
                let (a, b) = (neighbors[i], neighbors[j]);
                if adj.get(&a).is_some_and(|s| !s.contains(&b)) {
                    add_fill_edge(&mut adj, &mut fill, &mut touched, a, b);
                }
            }
        }
        remove_eliminated(&mut adj, &mut fill, &mut touched, u, &neighbors);
        fill.remove(&u);
        order.push(u);

        for v in touched {
            if let Some(&f) = fill.get(&v) {
                heap.push(Reverse((f, v)));
            }
        }
    }
    (treewidth, order)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Graph;

    #[test]
    fn test_treewidth_empty_graph() {
        let graph: Graph<i32, f64> = Graph::new();
        let (tw, order) = treewidth_min_degree(&graph);
        assert_eq!(tw, 0);
        assert!(order.is_empty());
    }

    #[test]
    fn test_treewidth_single_node() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let (tw, order) = treewidth_min_degree(&graph);
        assert_eq!(tw, 0);
        assert_eq!(order.len(), 1);
        assert_eq!(order[0], n1);
    }

    #[test]
    fn test_treewidth_path() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        graph.add_edge(n1, n2, 1.0);
        graph.add_edge(n2, n3, 1.0);

        let (tw, order) = treewidth_min_degree(&graph);
        assert!(tw <= 1); // Path has treewidth 1
        assert_eq!(order.len(), 3);
    }

    #[test]
    fn test_treewidth_min_fill_in_empty() {
        let graph: Graph<i32, f64> = Graph::new();
        let (tw, order) = treewidth_min_fill_in(&graph);
        assert_eq!(tw, 0);
        assert!(order.is_empty());
    }

    fn build_grid(rows: usize, cols: usize) -> Graph<i32, f64> {
        let mut graph = Graph::new();
        let ids: Vec<_> = (0..(rows * cols))
            .map(|i| graph.add_node(i as i32))
            .collect();
        let idx = |r: usize, c: usize| r * cols + c;
        for r in 0..rows {
            for c in 0..cols {
                if c + 1 < cols {
                    graph.add_edge(ids[idx(r, c)], ids[idx(r, c + 1)], 1.0);
                }
                if r + 1 < rows {
                    graph.add_edge(ids[idx(r, c)], ids[idx(r + 1, c)], 1.0);
                }
            }
        }
        graph
    }

    #[test]
    fn test_treewidth_min_degree_complete_graph() {
        // K4 is already chordal with treewidth 3; no fill-in is needed, so this pins
        // the baseline result for both the old and the corrected code.
        let mut graph = Graph::new();
        let nodes: Vec<_> = (0..4).map(|i| graph.add_node(i)).collect();
        for i in 0..4 {
            for j in (i + 1)..4 {
                graph.add_edge(nodes[i], nodes[j], 1.0);
            }
        }
        let (tw, order) = treewidth_min_degree(&graph);
        assert_eq!(tw, 3);
        assert_eq!(order.len(), 4);
    }

    #[test]
    fn test_treewidth_min_degree_grid_requires_fill_in() {
        // A 6x6 grid has maximum degree 4 but treewidth 6. The elimination game must
        // add fill-in edges among an eliminated node's neighbors, so any correct
        // heuristic reports at least the true treewidth (6). Without the fill-in step
        // the result collapses to the degeneracy (2), so this pins the fix.
        let graph = build_grid(6, 6);
        let (tw, _order) = treewidth_min_degree(&graph);
        assert!(tw >= 6, "expected treewidth >= 6 for a 6x6 grid, got {tw}");
    }

    #[test]
    fn test_treewidth_min_fill_in_grid_requires_fill_in() {
        // Same 6x6 grid: without the fill-in step the reported width never exceeds
        // the maximum original degree (4), well below the treewidth (6).
        let graph = build_grid(6, 6);
        let (tw, _order) = treewidth_min_fill_in(&graph);
        assert!(tw >= 6, "expected treewidth >= 6 for a 6x6 grid, got {tw}");
    }

    #[test]
    fn test_treewidth_min_fill_in_cycle() {
        // A cycle has treewidth 2, and the min fill-in heuristic reports it exactly:
        // eliminating any degree-2 node adds one fill-in edge and yields a triangle.
        // This pins the deterministic output so an optimized rewrite stays faithful.
        let mut graph = Graph::new();
        let nodes: Vec<_> = (0..6).map(|i| graph.add_node(i)).collect();
        for i in 0..6 {
            graph.add_edge(nodes[i], nodes[(i + 1) % 6], 1.0);
        }
        let (tw, order) = treewidth_min_fill_in(&graph);
        assert_eq!(tw, 2);
        assert_eq!(order.len(), 6);
    }
}
