"""
Tests for personalized PageRank in pygraphina.
"""
import pytest

import pygraphina


class TestPersonalizedPagerank:
    """Tests for personalized_pagerank function."""

    def test_personalized_pagerank_basic(self):
        """Test personalized PageRank without personalization vector."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)
        g.add_edge(n2, n0, 1.0)

        scores = pygraphina.centrality.personalized_pagerank(g, None, 0.85, 1e-6, 100)

        assert len(scores) == 3
        for node in [n0, n1, n2]:
            assert scores[node] > 0

    def test_personalized_pagerank_with_personalization(self):
        """Test personalized PageRank with personalization vector."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        g.add_edge(n0, n1, 1.0)

        # Bias towards first node
        scores = pygraphina.centrality.personalized_pagerank(g, [2.0, 1.0], 0.85, 1e-6, 100)

        assert len(scores) == 2
        # Node with higher personalization weight should have higher rank
        assert scores[n0] > scores[n1]

    def test_personalized_pagerank_vs_regular(self):
        """Test that personalized PageRank with None is similar to regular PageRank."""
        g = pygraphina.erdos_renyi(30, 0.2, 42)

        regular_scores = pygraphina.centrality.pagerank(g, 0.85, 100, 1e-6)
        personalized_scores = pygraphina.centrality.personalized_pagerank(g, None, 0.85, 1e-6, 100)

        # Results should be very similar
        for node in g.nodes():
            diff = abs(regular_scores[node] - personalized_scores[node])
            assert diff < 0.1  # Allow tolerance for implementation differences

    def test_personalized_pagerank_empty_graph(self):
        """Test personalized PageRank on empty graph."""
        g = pygraphina.PyGraph()

        # Empty graph should raise an error (no nodes)
        with pytest.raises(ValueError):
            pygraphina.centrality.personalized_pagerank(g, None, 0.85, 1e-6, 100)

    def test_personalized_pagerank_single_node(self):
        """Test personalized PageRank on single node."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)

        scores = pygraphina.centrality.personalized_pagerank(g, None, 0.85, 1e-6, 100)

        assert len(scores) == 1
        assert n0 in scores
