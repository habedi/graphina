import pygraphina


def make_two_components():
    g = pygraphina.PyGraph()
    a = [g.add_node(i) for i in range(3)]
    b = [g.add_node(i + 10) for i in range(3)]
    g.add_edge(a[0], a[1], 1.0)
    g.add_edge(a[1], a[2], 1.0)
    g.add_edge(b[0], b[1], 1.0)
    g.add_edge(b[1], b[2], 1.0)
    return g, a, b


def test_connected_components():
    g, a, b = make_two_components()
    comps = pygraphina.community.connected_components(g)
    assert len(comps) == 2
    assert sorted(len(c) for c in comps) == [3, 3]


def test_label_propagation_and_louvain():
    g, a, b = make_two_components()
    labels = pygraphina.community.label_propagation(g, 5, 42)
    assert isinstance(labels, dict)
    comms = pygraphina.community.louvain(g, 42)
    assert isinstance(comms, list)
