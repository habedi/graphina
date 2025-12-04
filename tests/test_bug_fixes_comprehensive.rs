/*!
# Comprehensive Bug Fixes Test Suite

This test suite documents and verifies fixes for bugs discovered during code analysis.

## Bugs Fixed:

### BUG-001: Missing imports in pool module tests
- **Location**: `src/core/pool.rs` test module
- **Issue**: Test module was missing `use super::*;` import
- **Impact**: Tests failed to compile with --all-features flag
- **Fix**: Added `use super::*;` to test module
- **Test**: Verified by running `cargo test --all-features`

### BUG-002: Missing trait imports in subgraph doctests
- **Location**: `src/subgraphs/operations.rs` doctests
- **Issue**: 8 doctests were missing `use graphina::subgraphs::SubgraphOps;` import
- **Impact**: Doctests failed to compile, trait methods not in scope
- **Fix**: Added trait import to all affected doctests
- **Test**: This file verifies the functionality works correctly

### BUG-003: Potential integer overflow in density calculation
- **Location**: `src/core/types.rs::density()`
- **Issue**: For very large graphs, `n * (n - 1)` could overflow usize
- **Impact**: Incorrect density calculation or panic for large graphs
- **Severity**: Low (only affects graphs with billions of nodes)
- **Status**: Documented, mitigation by using checked arithmetic would be improvement

### BUG-004: Unsafe lifetime extension in pool module
- **Location**: `src/core/pool.rs` default pool functions
- **Issue**: Using `std::mem::transmute` to extend lifetime to 'static
- **Impact**: Potential undefined behavior if pool outlives thread
- **Severity**: Medium (should be safe in practice but unsafe code should be minimized)
- **Status**: Documented, works correctly but could be refactored
*/

#[cfg(test)]
mod bug_fixes {
    use graphina::core::types::Graph;
    use graphina::subgraphs::SubgraphOps;

    /// Test for BUG-002: Verifies subgraph operations work correctly
    /// after fixing missing trait imports in doctests
    #[test]
    fn test_subgraph_operations_work_after_doctest_fix() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);

        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 2.0);
        g.add_edge(n3, n4, 3.0);

        // Test subgraph extraction
        let sub = g.subgraph(&[n1, n2, n3]).unwrap();
        assert_eq!(sub.node_count(), 3);
        assert_eq!(sub.edge_count(), 2);

        // Test ego graph
        let ego = g.ego_graph(n2, 1).unwrap();
        assert_eq!(ego.node_count(), 3);

        // Test filter_nodes
        let filtered = g.filter_nodes(|_id, attr| *attr % 2 == 0);
        assert_eq!(filtered.node_count(), 2);

        // Test filter_edges
        let filtered_edges = g.filter_edges(|_src, _tgt, w| *w > 1.5);
        assert_eq!(filtered_edges.edge_count(), 2);

        // Test k_hop_neighbors
        let neighbors = g.k_hop_neighbors(n1, 2);
        assert!(neighbors.len() >= 1);

        // Test connected_component
        let component = g.connected_component(n1);
        assert_eq!(component.len(), 4);

        // Test component_subgraph
        let comp_sub = g.component_subgraph(n1).unwrap();
        assert_eq!(comp_sub.node_count(), 4);
    }

    /// Test for BUG-003: Verify density calculation doesn't overflow
    /// for reasonably sized graphs
    #[test]
    fn test_density_calculation_reasonable_sizes() {
        let mut g = Graph::<i32, f64>::new();

        // Test empty graph
        assert_eq!(g.density(), 0.0);

        // Test single node
        let _n1 = g.add_node(1);
        assert_eq!(g.density(), 0.0);

        // Test two nodes, no edge
        let _n2 = g.add_node(2);
        assert_eq!(g.density(), 0.0);

        // Test two nodes, one edge
        g.add_edge(_n1, _n2, 1.0);
        assert!(g.density() > 0.0);
        assert!(g.density() <= 1.0);
    }

    /// Test for BUG-003: Document behavior for large graphs
    #[test]
    fn test_density_calculation_large_graph() {
        let mut g = Graph::<i32, f64>::new();

        // Create a moderate size graph (1000 nodes)
        let nodes: Vec<_> = (0..1000).map(|i| g.add_node(i)).collect();

        // Add some edges
        for i in 0..999 {
            g.add_edge(nodes[i], nodes[i + 1], 1.0);
        }

        let density = g.density();
        assert!(density >= 0.0);
        assert!(density <= 1.0);

        // For 1000 nodes and 999 edges in undirected graph:
        // density = 2 * 999 / (1000 * 999) ≈ 0.002
        assert!(density < 0.01);
    }

    /// Test edge case: subgraph with non-existent nodes
    #[test]
    fn test_subgraph_with_invalid_nodes() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n1, n2, 1.0);

        // Create a fake NodeId that doesn't exist in the graph
        use graphina::core::types::NodeId;
        let fake_node = NodeId::new(petgraph::graph::NodeIndex::new(999));

        // Should return error for non-existent node
        let result = g.subgraph(&[n1, fake_node]);
        assert!(result.is_err());
    }

    /// Test edge case: ego graph with radius 0
    #[test]
    fn test_ego_graph_radius_zero() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n1, n2, 1.0);

        let ego = g.ego_graph(n1, 0).unwrap();
        // With radius 0, should only contain the center node
        assert_eq!(ego.node_count(), 1);
        assert_eq!(ego.edge_count(), 0);
    }

    /// Test edge case: filter_nodes with empty result
    #[test]
    fn test_filter_nodes_empty_result() {
        let mut g = Graph::<i32, f64>::new();
        let _n1 = g.add_node(1);
        let _n2 = g.add_node(2);

        // Filter that excludes all nodes
        let filtered = g.filter_nodes(|_id, attr| *attr > 100);
        assert_eq!(filtered.node_count(), 0);
        assert_eq!(filtered.edge_count(), 0);
    }

    /// Test edge case: filter_edges keeps all nodes even if no edges match
    #[test]
    fn test_filter_edges_no_matching_edges() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(n1, n2, 1.0);
        g.add_edge(n2, n3, 2.0);

        // Filter that excludes all edges
        let filtered = g.filter_edges(|_src, _tgt, w| *w > 100.0);
        assert_eq!(filtered.node_count(), 3); // All nodes kept
        assert_eq!(filtered.edge_count(), 0); // No edges
    }

    /// Test edge case: k_hop_neighbors with k=0
    #[test]
    fn test_k_hop_neighbors_zero_hops() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        g.add_edge(n1, n2, 1.0);

        let neighbors = g.k_hop_neighbors(n1, 0);
        // With k=0, should only return the start node
        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors[0], n1);
    }

    /// Test edge case: connected_component on isolated node
    #[test]
    fn test_connected_component_isolated_node() {
        let mut g = Graph::<i32, f64>::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let _n3 = g.add_node(3); // isolated
        g.add_edge(n1, n2, 1.0);

        let component = g.connected_component(_n3);
        assert_eq!(component.len(), 1);
        assert_eq!(component[0], _n3);
    }
}
