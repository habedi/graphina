import pygraphina as pg
import pytest


class TestPageRank:

    def setup_method(self):
        self.g = pg.PyDiGraph()
        self.n0 = self.g.add_node(0)
        self.n1 = self.g.add_node(1)
        self.n2 = self.g.add_node(2)
        self.nodes = [self.n0, self.n1, self.n2]
        self.g.add_edge(self.n0, self.n1, 1.0)
        self.g.add_edge(self.n1, self.n2, 1.0)
        self.g.add_edge(self.n2, self.n0, 1.0)

    @pytest.mark.parametrize('damp', [0.85, 0.9, 0.5])
    def test_pagerank_cycle(self, damp):
        scores = pg.centrality.pagerank(self.g, damp, 100, 1e-06)
        assert len(scores) == 3
        expected = 1.0 / 3.0
        for n in self.nodes:
            assert scores[n] == pytest.approx(expected, abs=1e-05)

    def test_pagerank_star(self):
        g = pg.PyDiGraph()
        center = g.add_node(0)
        leaves = [g.add_node(i) for i in range(1, 5)]
        for leaf in leaves:
            g.add_edge(leaf, center, 1.0)
        scores = pg.centrality.pagerank(g, 0.85, 100, 1e-06)
        assert scores[center] > max((scores[leaf] for leaf in leaves))

    def test_personalized_pagerank_bias(self):
        personalization = [1.0, 0.0, 0.0]
        scores = pg.centrality.personalized_pagerank(self.g, personalization, 0.85, 1e-06, 100)
        assert len(scores) == 3
        assert sum(scores.values()) == pytest.approx(1.0, abs=1e-05)

    def test_empty_graph(self):
        g = pg.PyDiGraph()
        scores = pg.centrality.pagerank(g, 0.85, 100, 1e-06)
        assert scores == {}

    def test_pagerank_single_node(self):
        g = pg.PyGraph()
        n = g.add_node(0)
        pr = pg.centrality.pagerank(g)
        assert len(pr) == 1
        assert abs(pr[n] - 1.0) < 1e-06

    def test_pagerank_with_nstart(self):
        g = pg.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n0, 1.0)
        nstart = {n0: 10.0, n1: 1.0}
        pr = pg.centrality.pagerank(g, nstart=nstart)
        assert abs(pr[n0] - 0.5) < 0.001
        assert abs(pr[n1] - 0.5) < 0.001
        invalid_nstart = {n0: 0.0, n1: 0.0}
        with pytest.raises(pg.GraphinaError):
            pg.centrality.pagerank(g, nstart=invalid_nstart)

    def test_custom_exception_import(self):
        from pygraphina import GraphinaError
        assert issubclass(GraphinaError, Exception)
