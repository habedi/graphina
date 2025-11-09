import pygraphina as pg


def test_digraph_basic_ops():
    g = pg.PyDiGraph()
    a = g.add_node(1)
    b = g.add_node(2)
    assert g.is_directed()
    assert len(g) == 2

    eid = g.add_edge(a, b, 1.0)
    assert isinstance(eid, int)
    assert g.edge_count() == 1
    assert g.contains_edge(a, b)
    # Ensure directionality
    assert not g.contains_edge(b, a)

    # dijkstra distances
    d = g.dijkstra(a)
    assert d[a] == 0.0
    assert d[b] == 1.0
