import pygraphina


def make_graph():
    g = pygraphina.PyGraph()
    n = [g.add_node(i) for i in range(5)]
    g.add_edge(n[0], n[1], 1.0)
    g.add_edge(n[1], n[2], 1.0)
    g.add_edge(n[2], n[3], 1.0)
    g.add_edge(n[3], n[4], 1.0)
    return g, n


def test_similarity_and_attachment():
    g, n = make_graph()
    jc = pygraphina.links.jaccard_coefficient(g)
    assert len(jc) > 0
    aa = pygraphina.links.adamic_adar_index(g)
    assert len(aa) > 0
    pa = pygraphina.links.preferential_attachment(g)
    assert len(pa) > 0


def test_common_neighbors_and_centrality():
    g, n = make_graph()
    cn = pygraphina.links.common_neighbors(g, n[1], n[3])
    assert isinstance(cn, int)
    ccc = pygraphina.links.common_neighbor_centrality(g, 0.5)
    assert len(ccc) > 0


def make_line(n=5):
    g = pygraphina.PyGraph()
    nodes = [g.add_node(i) for i in range(n)]
    for i in range(n - 1):
        g.add_edge(nodes[i], nodes[i + 1], 1.0)
    return g, nodes


def test_max_clique_and_size():
    g, _ = make_line(5)
    c = pygraphina.max_clique(g)
    assert isinstance(c, list)
    size = pygraphina.large_clique_size(g)
    assert size >= 1


def test_vertex_cover_and_diameter():
    g, nodes = make_line(6)
    cover = pygraphina.min_weighted_vertex_cover(g)
    assert isinstance(cover, list)
    d = pygraphina.diameter(g)
    assert d >= 0.0


def test_clique_removal():
    g, _ = make_line(4)
    parts = pygraphina.clique_removal(g)
    assert isinstance(parts, list)
