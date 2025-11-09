"""
Unit tests for Minimum Spanning Tree algorithms in pygraphina.
"""
import pytest

import pygraphina


class TestMST:
    """Tests for MST algorithms."""

    def create_simple_graph(self):
        """Create a simple weighted graph for testing."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        n3 = g.add_node(3)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n0, n2, 4.0)
        g.add_edge(n1, n2, 2.0)
        g.add_edge(n1, n3, 5.0)
        g.add_edge(n2, n3, 3.0)

        return g

    def test_prim_basic(self):
        """Test basic Prim's MST algorithm."""
        g = self.create_simple_graph()
        weight, edges = pygraphina.prim_mst(g)

        assert weight > 0
        assert len(edges) == 3  # MST of 4 nodes has 3 edges

    def test_kruskal_basic(self):
        """Test basic Kruskal's MST algorithm."""
        g = self.create_simple_graph()
        weight, edges = pygraphina.kruskal_mst(g)

        assert weight > 0
        assert len(edges) == 3

    def test_boruvka_basic(self):
        """Test basic Borůvka's MST algorithm."""
        g = self.create_simple_graph()
        weight, edges = pygraphina.boruvka_mst(g)

        assert weight > 0
        assert len(edges) == 3

    def test_mst_algorithms_agree(self):
        """Test that all MST algorithms produce same total weight."""
        g = self.create_simple_graph()

        prim_weight, _ = pygraphina.prim_mst(g)
        kruskal_weight, _ = pygraphina.kruskal_mst(g)
        boruvka_weight, _ = pygraphina.boruvka_mst(g)

        assert prim_weight == pytest.approx(kruskal_weight)
        assert prim_weight == pytest.approx(boruvka_weight)

    def test_mst_on_complete_graph(self):
        """Test MST on a complete graph."""
        g = pygraphina.complete_graph(5)

        weight, edges = pygraphina.prim_mst(g)
        assert len(edges) == 4  # MST of 5 nodes has 4 edges

    def test_mst_single_node(self):
        """Test MST on a single node graph."""
        g = pygraphina.PyGraph()
        g.add_node(0)

        weight, edges = pygraphina.prim_mst(g)
        assert weight == 0.0
        assert len(edges) == 0

    def test_mst_disconnected_graph(self):
        """Test MST on disconnected graph returns forest."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        n3 = g.add_node(3)

        # Two separate components
        g.add_edge(n0, n1, 1.0)
        g.add_edge(n2, n3, 2.0)

        weight, edges = pygraphina.prim_mst(g)
        assert len(edges) == 2  # One edge per component

    def test_mst_edge_weights(self):
        """Test that MST contains edges with correct weights."""
        g = self.create_simple_graph()
        _, edges = pygraphina.prim_mst(g)

        total = sum(w for _, _, w in edges)
        assert total > 0

    def test_mst_on_large_graph(self):
        """Test MST on a larger generated graph."""
        g = pygraphina.erdos_renyi(50, 0.3, 42)

        if g.edge_count() > 0:
            weight, edges = pygraphina.kruskal_mst(g)
            assert weight >= 0
            # MST should have at most n-1 edges
            assert len(edges) <= g.node_count() - 1


class TestMSTProperties:
    """Tests for MST algorithm properties."""

    def test_mst_tree_property(self):
        """Test that MST has tree property (n-1 edges for n nodes)."""
        g = pygraphina.complete_graph(10)
        _, edges = pygraphina.prim_mst(g)

        # Connected graph MST should have exactly n-1 edges
        assert len(edges) == 9

    def test_mst_no_cycles(self):
        """Test that MST contains no cycles (indirectly via edge count)."""
        g = pygraphina.erdos_renyi(30, 0.5, 42)

        if g.is_connected():
            _, edges = pygraphina.kruskal_mst(g)
            # Tree with n nodes has exactly n-1 edges
            assert len(edges) == g.node_count() - 1
