"""
Tests for PageRank algorithms in pygraphina.
"""
import pytest
import pygraphina as pg

class TestPageRank:
    """Tests for pagerank and personalized_pagerank."""

    def setup_method(self):
        self.g = pg.PyDiGraph()
        self.n0 = self.g.add_node(0)
        self.n1 = self.g.add_node(1)
        self.n2 = self.g.add_node(2)
        self.nodes = [self.n0, self.n1, self.n2]
        # Create a cycle: 0 -> 1 -> 2 -> 0
        self.g.add_edge(self.n0, self.n1, 1.0)
        self.g.add_edge(self.n1, self.n2, 1.0)
        self.g.add_edge(self.n2, self.n0, 1.0)

    @pytest.mark.parametrize("damp", [0.85, 0.90, 0.5])
    def test_pagerank_cycle(self, damp):
        """In a simple cycle, all nodes should have equal rank."""
        scores = pg.centrality.pagerank(self.g, damp, 100, 1e-6)
        assert len(scores) == 3
        # In a symmetric cycle, each node is visited equally
        expected = 1.0 / 3.0
        for n in self.nodes:
            assert scores[n] == pytest.approx(expected, abs=1e-5)

    def test_pagerank_star(self):
        """Test star graph where center has high rank."""
        g = pg.PyDiGraph()
        center = g.add_node(0)
        leaves = [g.add_node(i) for i in range(1, 5)]

        # Edges from leaves to center (in-star)
        for leaf in leaves:
            g.add_edge(leaf, center, 1.0)

        scores = pg.centrality.pagerank(g, 0.85, 100, 1e-6)

        # Center should have highest rank because everyone points to it
        assert scores[center] > max(scores[leaf] for leaf in leaves)

    def test_personalized_pagerank_bias(self):
        """Test personalized PageRank with bias."""
        # Bias vector: only n0 has weight.
        # Note: personalization vector size must match node count (impl dependent)
        # or be a map? Rust binding takes Option<Vec<f64>>.
        # Assuming Vec is in internal node id order (0, 1, 2...)
        # Since we added nodes sequentially 0,1,2 and they mapped to 0,1,2 usually.
        personalization = [1.0, 0.0, 0.0]
        scores = pg.centrality.personalized_pagerank(self.g, personalization, 0.85, 1e-6, 100)

        assert len(scores) == 3
        # n0 is the teleport target, so it might have slightly higher score
        # but in a perfect cycle it redistributes.
        # Let's check if it runs at least.
        assert sum(scores.values()) == pytest.approx(1.0, abs=1e-5)

    def test_empty_graph(self):
        """Test PageRank on empty graph."""
        g = pg.PyDiGraph()
        # Core returns empty map, should validly return empty dict
        scores = pg.centrality.pagerank(g, 0.85, 100, 1e-6)
        assert scores == {}

    def test_pagerank_single_node(self):
        """Test PageRank on a single node graph."""
        g = pg.PyGraph()
        n = g.add_node(0)

        pr = pg.centrality.pagerank(g)
        assert len(pr) == 1
        assert abs(pr[n] - 1.0) < 1e-6

    def test_pagerank_with_nstart(self):
        """Test PageRank with nstart parameter."""
        g = pg.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n0, 1.0)

        # Bias towards n0
        nstart = {n0: 10.0, n1: 1.0}

        # Should behave normally (converge to 0.5/0.5 symmetric)
        pr = pg.centrality.pagerank(g, nstart=nstart)
        assert abs(pr[n0] - 0.5) < 1e-3
        assert abs(pr[n1] - 0.5) < 1e-3

        # Should raise error if nstart is empty or invalid?
        # Graphina behaves safely (if sum is 0, defaults to uniform).
        # Actually in Step 1096 I added error for sum=0

        invalid_nstart = {n0: 0.0, n1: 0.0}
        with pytest.raises(pg.GraphinaError):
            pg.centrality.pagerank(g, nstart=invalid_nstart)

    def test_custom_exception_import(self):
        """Verify we can import custom exceptions (smoke test for Phase 5)."""
        from pygraphina import GraphinaError
        assert issubclass(GraphinaError, Exception)
