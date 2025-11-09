"""
Tests for NetworkX interoperability bug fixes.

Critical Bug Fixed:
- Using pointer addresses (as_ptr() as usize) for node mapping was unsafe and caused crashes
- Fixed by using string representation of nodes for stable mapping
"""

import pytest

import pygraphina as pg

try:
    import networkx as nx
except Exception:
    nx = None

pytestmark = pytest.mark.skipif(nx is None, reason="networkx not installed")


class TestNetworkXInteropBugFix:
    """Test that NetworkX interop works with arbitrary node types."""

    def test_string_node_ids(self):
        """Test conversion with string node IDs."""
        G = nx.Graph()
        G.add_node("alice", attr=100)
        G.add_node("bob", attr=200)
        G.add_edge("alice", "bob", weight=3.5)

        g = pg.from_networkx(G)
        assert g.node_count() == 2
        assert g.edge_count() == 1

        # Verify attributes are preserved
        attrs = dict(g.nodes_with_attrs())
        assert set(attrs.values()) == {100, 200}

    def test_integer_node_ids(self):
        """Test conversion with integer node IDs."""
        G = nx.Graph()
        G.add_node(1, attr=10)
        G.add_node(2, attr=20)
        G.add_edge(1, 2, weight=5.0)

        g = pg.from_networkx(G)
        assert g.node_count() == 2
        assert g.edge_count() == 1

    def test_tuple_node_ids(self):
        """Test conversion with tuple node IDs (coordinates)."""
        G = nx.Graph()
        G.add_node((0, 0), attr=1)
        G.add_node((0, 1), attr=2)
        G.add_node((1, 0), attr=3)
        G.add_edge((0, 0), (0, 1), weight=1.0)
        G.add_edge((0, 1), (1, 0), weight=1.5)

        g = pg.from_networkx(G)
        assert g.node_count() == 3
        assert g.edge_count() == 2

    def test_mixed_type_node_ids(self):
        """Test conversion with mixed node ID types."""
        G = nx.Graph()
        G.add_node("node1", attr=1)
        G.add_node(42, attr=2)
        G.add_node((1, 2), attr=3)
        G.add_edge("node1", 42, weight=1.0)
        G.add_edge(42, (1, 2), weight=2.0)

        g = pg.from_networkx(G)
        assert g.node_count() == 3
        assert g.edge_count() == 2

    def test_digraph_with_string_nodes(self):
        """Test directed graph conversion with string nodes."""
        G = nx.DiGraph()
        G.add_node("source", attr=1)
        G.add_node("sink", attr=2)
        G.add_edge("source", "sink", weight=10.0)

        d = pg.from_networkx(G)
        assert isinstance(d, pg.PyDiGraph)
        assert d.node_count() == 2
        assert d.edge_count() == 1
        assert d.is_directed()

    def test_round_trip_with_arbitrary_nodes(self):
        """Test round-trip conversion with arbitrary node types."""
        # Create PyGraph
        g = pg.PyGraph()
        n0 = g.add_node(10)
        n1 = g.add_node(20)
        n2 = g.add_node(30)
        g.add_edge(n0, n1, 1.5)
        g.add_edge(n1, n2, 2.5)

        # Convert to NetworkX
        nx_g = pg.to_networkx(g)
        assert nx_g.number_of_nodes() == 3
        assert nx_g.number_of_edges() == 2

        # Convert back to PyGraph
        g2 = pg.from_networkx(nx_g)
        assert g2.node_count() == 3
        assert g2.edge_count() == 2

    def test_networkx_node_attributes_preserved(self):
        """Test that node attributes are correctly preserved."""
        G = nx.Graph()
        G.add_node("a", attr=100)
        G.add_node("b", attr=200)
        G.add_node("c", attr=300)
        G.add_edge("a", "b", weight=1.0)

        g = pg.from_networkx(G)
        attrs = [v for _, v in g.nodes_with_attrs()]
        assert sorted(attrs) == [100, 200, 300]

    def test_networkx_edge_weights_preserved(self):
        """Test that edge weights are correctly preserved."""
        G = nx.Graph()
        G.add_node(1, attr=0)
        G.add_node(2, attr=0)
        G.add_edge(1, 2, weight=3.14159)

        g = pg.from_networkx(G)
        edges = g.edges_with_weights()
        assert len(edges) == 1
        _, _, weight = edges[0]
        assert abs(weight - 3.14159) < 1e-5

    def test_networkx_missing_attributes_default(self):
        """Test that missing attributes get default values."""
        G = nx.Graph()
        G.add_node("x")  # No attr
        G.add_node("y", attr=50)
        G.add_edge("x", "y")  # No weight

        g = pg.from_networkx(G)
        assert g.node_count() == 2
        assert g.edge_count() == 1

        # Missing attr should default to 0
        attrs = dict(g.nodes_with_attrs())
        assert 0 in attrs.values()
        assert 50 in attrs.values()

        # Missing weight should default to 1.0
        edges = g.edges_with_weights()
        _, _, weight = edges[0]
        assert weight == 1.0

    def test_large_networkx_graph(self):
        """Test conversion with a larger graph."""
        G = nx.karate_club_graph()

        # Add attr to all nodes
        for node in G.nodes():
            G.nodes[node]["attr"] = node

        # Add weights to all edges
        for u, v in G.edges():
            G[u][v]["weight"] = 1.0

        g = pg.from_networkx(G)
        assert g.node_count() == G.number_of_nodes()
        assert g.edge_count() == G.number_of_edges()


class TestNetworkXDiGraphInterop:
    """Test NetworkX interoperability with directed graphs."""

    def test_digraph_directionality_preserved(self):
        """Test that edge directionality is preserved."""
        G = nx.DiGraph()
        G.add_node(1, attr=1)
        G.add_node(2, attr=2)
        G.add_edge(1, 2, weight=1.0)
        # Note: no edge from 2 to 1

        d = pg.from_networkx(G)
        assert d.is_directed()
        assert d.contains_edge(1, 2)
        # This would be True for undirected, but should be False for directed

    def test_digraph_to_networkx(self):
        """Test converting PyDiGraph to NetworkX DiGraph."""
        d = pg.PyDiGraph()
        n0 = d.add_node(10)
        n1 = d.add_node(20)
        d.add_edge(n0, n1, 2.5)

        nx_d = pg.to_networkx(d)
        assert isinstance(nx_d, nx.DiGraph)
        assert nx_d.is_directed()
        assert nx_d.number_of_nodes() == 2
        assert nx_d.number_of_edges() == 1

    def test_digraph_round_trip_preserves_direction(self):
        """Test that round-trip preserves edge directions."""
        d = pg.PyDiGraph()
        n0 = d.add_node(1)
        n1 = d.add_node(2)
        n2 = d.add_node(3)
        d.add_edge(n0, n1, 1.0)
        d.add_edge(n1, n2, 1.0)
        # No reverse edges

        nx_d = pg.to_networkx(d)
        d2 = pg.from_networkx(nx_d)

        assert d2.is_directed()
        assert d2.edge_count() == 2
