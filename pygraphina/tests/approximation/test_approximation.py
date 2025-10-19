import pygraphina


def make_line(n=5):
    g = pygraphina.PyGraph()
    nodes = [g.add_node(i) for i in range(n)]
    for i in range(n - 1):
        g.add_edge(nodes[i], nodes[i + 1], 1.0)
    return g, nodes


def test_max_clique_and_size():
    g, _ = make_line(5)
    c = pygraphina.approximation.max_clique(g)
    assert isinstance(c, list)
    size = pygraphina.approximation.large_clique_size(g)
    assert size >= 1


def test_vertex_cover_and_diameter():
    g, nodes = make_line(6)
    cover = pygraphina.approximation.min_weighted_vertex_cover(g)
    assert isinstance(cover, list)
    d = pygraphina.approximation.diameter(g)
    assert d >= 0.0


def test_clique_removal():
    g, _ = make_line(4)
    parts = pygraphina.approximation.clique_removal(g)
    assert isinstance(parts, list)
