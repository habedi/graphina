"""
Unit tests for subgraph operations in pygraphina.
"""
import pygraphina
import pytest


def create_test_graph():
    """Create a simple graph for testing subgraphs."""
    g = pygraphina.PyGraph()
    n0 = g.add_node(0)
    n1 = g.add_node(1)
    n2 = g.add_node(2)
    n3 = g.add_node(3)
    n4 = g.add_node(4)

    g.add_edge(n0, n1, 1.0)
    g.add_edge(n1, n2, 2.0)
    g.add_edge(n2, n3, 3.0)
    g.add_edge(n3, n4, 4.0)
    g.add_edge(n1, n3, 5.0)

    return g, [n0, n1, n2, n3, n4]


class TestSubgraphs:
    """Tests for subgraph extraction."""

    def test_subgraph_basic(self):
        """Test basic subgraph extraction."""
        g, nodes = create_test_graph()

        # Extract subgraph with first 3 nodes
        sub = g.subgraph([nodes[0], nodes[1], nodes[2]])

        assert sub.node_count() == 3
        # Should have edges between these nodes
        assert sub.edge_count() >= 1

    def test_subgraph_preserves_edges(self):
        """Test that subgraph preserves edges between included nodes."""
        g, nodes = create_test_graph()

        # Extract subgraph with nodes that have edges between them
        sub = g.subgraph([nodes[1], nodes[2], nodes[3]])

        assert sub.node_count() == 3
        # Should have edges: 1-2, 2-3, and 1-3
        assert sub.edge_count() == 3

    def test_subgraph_single_node(self):
        """Test subgraph with single node."""
        g, nodes = create_test_graph()

        sub = g.subgraph([nodes[0]])

        assert sub.node_count() == 1
        assert sub.edge_count() == 0

    def test_subgraph_all_nodes(self):
        """Test subgraph with all nodes equals original."""
        g, nodes = create_test_graph()

        sub = g.subgraph(nodes)

        assert sub.node_count() == g.node_count()
        assert sub.edge_count() == g.edge_count()

    def test_subgraph_no_edges(self):
        """Test subgraph of disconnected nodes."""
        g, nodes = create_test_graph()

        # Extract nodes with no edges between them
        sub = g.subgraph([nodes[0], nodes[4]])

        assert sub.node_count() == 2
        assert sub.edge_count() == 0

    def test_induced_subgraph_basic(self):
        """Test induced subgraph extraction."""
        g, nodes = create_test_graph()

        induced = g.induced_subgraph([nodes[1], nodes[2], nodes[3]])

        assert induced.node_count() == 3
        # Should include all edges between these nodes
        assert induced.edge_count() >= 2

    def test_induced_subgraph_complete(self):
        """Test induced subgraph on complete graph."""
        g = pygraphina.complete_graph(10)
        nodes = list(g.nodes)

        # Extract induced subgraph of 5 nodes
        induced = g.induced_subgraph(nodes[:5])

        assert induced.node_count() == 5
        # Complete subgraph of 5 nodes should have 10 edges
        assert induced.edge_count() == 10

    def test_subgraph_invalid_node(self):
        """Test that subgraph raises error for invalid node."""
        g, nodes = create_test_graph()

        with pytest.raises(ValueError):
            g.subgraph([nodes[0], 999])

    def test_induced_subgraph_invalid_node(self):
        """Test that induced subgraph raises error for invalid node."""
        g, nodes = create_test_graph()

        with pytest.raises(ValueError):
            g.induced_subgraph([nodes[0], 999])

    def test_subgraph_empty_list(self):
        """Test subgraph with empty node list."""
        g, _ = create_test_graph()

        sub = g.subgraph([])

        assert sub.node_count() == 0
        assert sub.edge_count() == 0


class TestSubgraphProperties:
    """Tests for properties of extracted subgraphs."""

    def test_subgraph_independence(self):
        """Test that modifying subgraph doesn't affect original."""
        g = pygraphina.complete_graph(5)
        nodes = list(g.nodes)

        sub = g.subgraph(nodes[:3])
        original_count = g.node_count()

        # Add node to subgraph
        sub.add_node(999)

        # Original should be unchanged
        assert g.node_count() == original_count

    def test_subgraph_node_mapping(self):
        """Test that subgraph creates new node IDs."""
        g, nodes = create_test_graph()

        sub = g.subgraph([nodes[0], nodes[1]])
        sub_nodes = sub.nodes

        # Subgraph should have its own node IDs
        assert len(sub_nodes) == 2

    def test_multiple_subgraphs(self):
        """Test extracting multiple subgraphs from same graph."""
        g = pygraphina.complete_graph(10)
        nodes = list(g.nodes)

        sub1 = g.subgraph(nodes[:5])
        sub2 = g.subgraph(nodes[5:])

        assert sub1.node_count() == 5
        assert sub2.node_count() == 5
        # Original should be unchanged
        assert g.node_count() == 10

    def test_subgraph_of_generated_graph(self):
        """Test subgraph extraction from generated graph."""
        g = pygraphina.erdos_renyi(50, 0.3, 42)
        nodes = list(g.nodes)

        if len(nodes) >= 10:
            sub = g.subgraph(nodes[:10])
            assert sub.node_count() == 10
            assert sub.edge_count() >= 0
