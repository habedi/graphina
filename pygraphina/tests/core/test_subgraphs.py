import pygraphina
import pytest


def create_test_graph():
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
    return (g, [n0, n1, n2, n3, n4])


class TestSubgraphs:

    def test_subgraph_basic(self):
        g, nodes = create_test_graph()
        sub = g.subgraph([nodes[0], nodes[1], nodes[2]])
        assert sub.node_count() == 3
        assert sub.edge_count() >= 1

    def test_subgraph_preserves_edges(self):
        g, nodes = create_test_graph()
        sub = g.subgraph([nodes[1], nodes[2], nodes[3]])
        assert sub.node_count() == 3
        assert sub.edge_count() == 3

    def test_subgraph_single_node(self):
        g, nodes = create_test_graph()
        sub = g.subgraph([nodes[0]])
        assert sub.node_count() == 1
        assert sub.edge_count() == 0

    def test_subgraph_all_nodes(self):
        g, nodes = create_test_graph()
        sub = g.subgraph(nodes)
        assert sub.node_count() == g.node_count()
        assert sub.edge_count() == g.edge_count()

    def test_subgraph_no_edges(self):
        g, nodes = create_test_graph()
        sub = g.subgraph([nodes[0], nodes[4]])
        assert sub.node_count() == 2
        assert sub.edge_count() == 0

    def test_induced_subgraph_basic(self):
        g, nodes = create_test_graph()
        induced = g.induced_subgraph([nodes[1], nodes[2], nodes[3]])
        assert induced.node_count() == 3
        assert induced.edge_count() >= 2

    def test_induced_subgraph_complete(self):
        g = pygraphina.complete_graph(10)
        nodes = list(g.nodes)
        induced = g.induced_subgraph(nodes[:5])
        assert induced.node_count() == 5
        assert induced.edge_count() == 10

    def test_subgraph_invalid_node(self):
        g, nodes = create_test_graph()
        with pytest.raises(ValueError):
            g.subgraph([nodes[0], 999])

    def test_induced_subgraph_invalid_node(self):
        g, nodes = create_test_graph()
        with pytest.raises(ValueError):
            g.induced_subgraph([nodes[0], 999])

    def test_subgraph_empty_list(self):
        g, _ = create_test_graph()
        sub = g.subgraph([])
        assert sub.node_count() == 0
        assert sub.edge_count() == 0


class TestSubgraphProperties:

    def test_subgraph_independence(self):
        g = pygraphina.complete_graph(5)
        nodes = list(g.nodes)
        sub = g.subgraph(nodes[:3])
        original_count = g.node_count()
        sub.add_node(999)
        assert g.node_count() == original_count

    def test_subgraph_node_mapping(self):
        g, nodes = create_test_graph()
        sub = g.subgraph([nodes[0], nodes[1]])
        sub_nodes = sub.nodes
        assert len(sub_nodes) == 2

    def test_multiple_subgraphs(self):
        g = pygraphina.complete_graph(10)
        nodes = list(g.nodes)
        sub1 = g.subgraph(nodes[:5])
        sub2 = g.subgraph(nodes[5:])
        assert sub1.node_count() == 5
        assert sub2.node_count() == 5
        assert g.node_count() == 10

    def test_subgraph_of_generated_graph(self):
        g = pygraphina.erdos_renyi(50, 0.3, 42)
        nodes = list(g.nodes)
        if len(nodes) >= 10:
            sub = g.subgraph(nodes[:10])
            assert sub.node_count() == 10
            assert sub.edge_count() >= 0
