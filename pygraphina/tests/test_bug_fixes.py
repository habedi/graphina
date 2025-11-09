"""
Tests for bug fixes in PyGraphina Python bindings.

This module tests the critical bugs that were identified and fixed:
1. Type inconsistency in populate_from_internal
2. Subgraph type conversion issues
3. Weight validation (NaN, Inf)
4. Node ID mapping consistency
"""

import math
import pytest

import pygraphina


class TestTypeConsistencyFix:
    """Test that type conversions don't lose data integrity."""

    def test_generator_preserves_node_attributes(self):
        """Test that generated graphs maintain correct node attributes."""
        g = pygraphina.complete_graph(5)

        # Generators create nodes with sequential attributes starting at 0
        nodes = g.nodes()
        attrs = [g.get_node_attr(n) for n in nodes]

        # All attributes should be present and non-negative
        assert len(attrs) == 5
        assert all(attr is not None for attr in attrs)
        assert all(attr >= 0 for attr in attrs)

    def test_generator_preserves_edge_weights(self):
        """Test that generated graphs maintain correct edge weights."""
        g = pygraphina.erdos_renyi(10, 0.5, 42)

        edges = g.edges_with_weights()

        # All edge weights should be valid floats
        for u, v, w in edges:
            assert isinstance(w, float)
            assert w > 0  # Generators use weight 1.0
            assert math.isfinite(w)

    def test_subgraph_preserves_attributes(self):
        """Test that subgraph operations don't lose node attributes."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(100)
        n1 = g.add_node(200)
        n2 = g.add_node(300)
        g.add_edge(n0, n1, 1.5)
        g.add_edge(n1, n2, 2.5)

        sub = g.subgraph([n0, n1])

        # Check that attributes are preserved
        nodes = sub.nodes()
        assert len(nodes) == 2
        attrs = [sub.get_node_attr(n) for n in nodes]

        # Original attributes should be preserved (100, 200)
        assert set(attrs) == {100, 200}

    def test_subgraph_preserves_edge_weights(self):
        """Test that subgraph operations don't lose edge weight precision."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        # Use a weight that would lose precision in f32
        precise_weight = 1.123456789
        g.add_edge(n0, n1, precise_weight)
        g.add_edge(n1, n2, 2.0)

        sub = g.subgraph([n0, n1])
        edges = sub.edges_with_weights()

        assert len(edges) == 1
        u, v, w = edges[0]

        # Weight should be preserved with f64 precision
        assert abs(w - precise_weight) < 1e-9

    def test_induced_subgraph_preserves_data(self):
        """Test that induced subgraph doesn't lose data."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(-100)  # Negative to test i64 range
        n1 = g.add_node(-200)
        g.add_edge(n0, n1, 3.14159)

        induced = g.induced_subgraph([n0, n1])
        nodes = induced.nodes()

        assert len(nodes) == 2
        attrs = [induced.get_node_attr(n) for n in nodes]
        assert set(attrs) == {-100, -200}

        edges = induced.edges_with_weights()
        assert len(edges) == 1
        _, _, w = edges[0]
        assert abs(w - 3.14159) < 1e-9


class TestWeightValidation:
    """Test that invalid edge weights are properly rejected."""

    def test_add_edge_rejects_nan(self):
        """Test that NaN weights are rejected."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)

        with pytest.raises(ValueError, match="must be finite"):
            g.add_edge(n0, n1, float('nan'))

    def test_add_edge_rejects_positive_inf(self):
        """Test that positive infinity weights are rejected."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)

        with pytest.raises(ValueError, match="must be finite"):
            g.add_edge(n0, n1, float('inf'))

    def test_add_edge_rejects_negative_inf(self):
        """Test that negative infinity weights are rejected."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)

        with pytest.raises(ValueError, match="must be finite"):
            g.add_edge(n0, n1, float('-inf'))

    def test_add_edge_accepts_valid_weights(self):
        """Test that valid weights including negative and zero are accepted."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        # These should all succeed
        g.add_edge(n0, n1, 1.0)
        g.add_edge(n0, n2, -5.0)  # Negative weight
        g.add_edge(n1, n2, 0.0)  # Zero weight

        assert g.edge_count() == 3

    def test_add_edges_from_validates_weights(self):
        """Test that bulk edge addition also validates weights."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)

        with pytest.raises(ValueError, match="must be finite"):
            g.add_edges_from([(n0, n1, float('nan'))])


class TestNodeMappingConsistency:
    """Test that node ID mappings remain consistent."""

    def test_remove_node_cleans_mapping(self):
        """Test that removing a node cleans up all mappings."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(10)
        n1 = g.add_node(20)
        n2 = g.add_node(30)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)

        # Remove middle node
        attr = g.remove_node(n1)
        assert attr == 20

        # Node should no longer exist
        assert not g.contains_node(n1)
        assert g.node_count() == 2

        # Other nodes should still work
        assert g.contains_node(n0)
        assert g.contains_node(n2)

    def test_try_remove_node_cleans_mapping(self):
        """Test that try_remove_node also cleans up mappings."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(10)

        attr = g.try_remove_node(n0)
        assert attr == 10
        assert not g.contains_node(n0)
        assert g.node_count() == 0

    def test_clear_resets_all_mappings(self):
        """Test that clear() properly resets the graph state."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(10)
        n1 = g.add_node(20)
        g.add_edge(n0, n1, 1.0)

        g.clear()

        assert g.node_count() == 0
        assert g.edge_count() == 0
        assert not g.contains_node(n0)
        assert not g.contains_node(n1)

        # Should be able to add new nodes with fresh IDs
        new_n0 = g.add_node(100)
        assert g.node_count() == 1
        assert g.get_node_attr(new_n0) == 100


class TestErrorMessages:
    """Test that error messages are clear and helpful."""

    def test_invalid_node_error_includes_id(self):
        """Test that error messages include the invalid node ID."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)

        with pytest.raises(ValueError, match="999"):
            g.add_edge(n0, 999, 1.0)

    def test_invalid_source_node_error(self):
        """Test error message for invalid source node."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)

        with pytest.raises(ValueError, match="source"):
            g.add_edge(999, n0, 1.0)

    def test_invalid_target_node_error(self):
        """Test error message for invalid target node."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)

        with pytest.raises(ValueError, match="target"):
            g.add_edge(n0, 999, 1.0)


class TestFilterOperations:
    """Test that filter operations work correctly with fixed type system."""

    def test_filter_nodes_preserves_types(self):
        """Test that node filtering preserves data types."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(10)
        n1 = g.add_node(20)
        n2 = g.add_node(30)
        g.add_edge(n0, n1, 1.5)
        g.add_edge(n1, n2, 2.5)

        # Filter to keep only nodes with attr >= 20
        filtered = g.filter_nodes(lambda nid, attr: attr >= 20)

        assert filtered.node_count() == 2
        nodes = filtered.nodes()
        attrs = [filtered.get_node_attr(n) for n in nodes]
        assert set(attrs) == {20, 30}

    def test_filter_edges_preserves_types(self):
        """Test that edge filtering preserves data types."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        g.add_edge(n0, n1, 1.5)
        g.add_edge(n1, n2, 2.5)

        # Filter to keep only edges with weight > 2.0
        filtered = g.filter_edges(lambda u, v, w: w > 2.0)

        assert filtered.node_count() == 3  # All nodes preserved
        assert filtered.edge_count() == 1  # Only one edge kept

        edges = filtered.edges_with_weights()
        assert len(edges) == 1
        _, _, w = edges[0]
        assert abs(w - 2.5) < 1e-9


class TestEgoGraphOperations:
    """Test ego graph operations with fixed types."""

    def test_ego_graph_preserves_attributes(self):
        """Test that ego graph extraction preserves node attributes."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        n3 = g.add_node(3)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)
        g.add_edge(n2, n3, 1.0)

        ego = g.ego_graph(n0, 2)

        # Should include n0, n1, n2 but not n3
        assert ego.node_count() == 3
        nodes = ego.nodes()
        attrs = [ego.get_node_attr(n) for n in nodes]
        assert set(attrs) == {0, 1, 2}


class TestComponentOperations:
    """Test connected component operations with fixed types."""

    def test_component_subgraph_preserves_data(self):
        """Test that component subgraph extraction preserves data."""
        g = pygraphina.PyGraph()

        # Create two components
        n0 = g.add_node(10)
        n1 = g.add_node(20)
        n2 = g.add_node(30)
        n3 = g.add_node(40)

        g.add_edge(n0, n1, 1.5)
        g.add_edge(n2, n3, 2.5)

        # Get component containing n0
        comp = g.component_subgraph(n0)

        assert comp.node_count() == 2
        nodes = comp.nodes()
        attrs = [comp.get_node_attr(n) for n in nodes]
        assert set(attrs) == {10, 20}

        edges = comp.edges_with_weights()
        assert len(edges) == 1
        _, _, w = edges[0]
        assert abs(w - 1.5) < 1e-9


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
