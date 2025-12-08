import pygraphina
import pytest


class TestMST:

    def create_simple_graph(self):
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
        g = self.create_simple_graph()
        weight, edges = pygraphina.prim_mst(g)
        assert weight > 0
        assert len(edges) == 3

    def test_kruskal_basic(self):
        g = self.create_simple_graph()
        weight, edges = pygraphina.kruskal_mst(g)
        assert weight > 0
        assert len(edges) == 3

    def test_boruvka_basic(self):
        g = self.create_simple_graph()
        weight, edges = pygraphina.boruvka_mst(g)
        assert weight > 0
        assert len(edges) == 3

    def test_mst_algorithms_agree(self):
        g = self.create_simple_graph()
        prim_weight, _ = pygraphina.prim_mst(g)
        kruskal_weight, _ = pygraphina.kruskal_mst(g)
        boruvka_weight, _ = pygraphina.boruvka_mst(g)
        assert prim_weight == pytest.approx(kruskal_weight)
        assert prim_weight == pytest.approx(boruvka_weight)

    def test_mst_on_complete_graph(self):
        g = pygraphina.complete_graph(5)
        weight, edges = pygraphina.prim_mst(g)
        assert len(edges) == 4

    def test_mst_single_node(self):
        g = pygraphina.PyGraph()
        g.add_node(0)
        weight, edges = pygraphina.prim_mst(g)
        assert weight == 0.0
        assert len(edges) == 0

    def test_mst_disconnected_graph(self):
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        n3 = g.add_node(3)
        g.add_edge(n0, n1, 1.0)
        g.add_edge(n2, n3, 2.0)
        weight, edges = pygraphina.prim_mst(g)
        assert len(edges) == 2

    def test_mst_edge_weights(self):
        g = self.create_simple_graph()
        _, edges = pygraphina.prim_mst(g)
        total = sum((w for _, _, w in edges))
        assert total > 0

    def test_mst_on_large_graph(self):
        g = pygraphina.erdos_renyi(50, 0.3, 42)
        if g.edge_count() > 0:
            weight, edges = pygraphina.kruskal_mst(g)
            assert weight >= 0
            assert len(edges) <= g.node_count() - 1


class TestMSTProperties:

    def test_mst_tree_property(self):
        g = pygraphina.complete_graph(10)
        _, edges = pygraphina.prim_mst(g)
        assert len(edges) == 9

    def test_mst_no_cycles(self):
        g = pygraphina.erdos_renyi(30, 0.5, 42)
        if g.is_connected():
            _, edges = pygraphina.kruskal_mst(g)
            assert len(edges) == g.node_count() - 1
