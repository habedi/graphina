import pygraphina


def make_chain_graph():
    g = pygraphina.PyGraph()
    n0 = g.add_node(1)
    n1 = g.add_node(2)
    n2 = g.add_node(3)
    g.add_edge(n0, n1, 1.0)
    g.add_edge(n1, n2, 1.0)
    return g, (n0, n1, n2)


def test_degree_centrality_basic():
    g, nodes = make_chain_graph()
    deg = pygraphina.centrality.degree(g)
    assert isinstance(deg, dict)
    assert set(deg.keys()) == set(nodes)
    # middle node should have degree 2
    assert deg[nodes[1]] == 2.0
    assert deg[nodes[0]] == 1.0


def test_in_out_degree_on_undirected():
    g, nodes = make_chain_graph()
    indeg = pygraphina.centrality.in_degree(g)
    outdeg = pygraphina.centrality.out_degree(g)
    assert indeg == outdeg
    assert indeg[nodes[1]] == 2.0


def test_betweenness_and_edge_betweenness():
    g, nodes = make_chain_graph()
    b = pygraphina.centrality.betweenness(g, False)
    eb = pygraphina.centrality.edge_betweenness(g, False)
    assert isinstance(b, dict)
    assert isinstance(eb, dict)
    # middle node should have positive betweenness
    assert b[nodes[1]] > 0.0
    # edges should be present
    assert (nodes[0], nodes[1]) in eb or (nodes[1], nodes[0]) in eb


def test_closeness_and_harmonic():
    g, nodes = make_chain_graph()
    cl = pygraphina.centrality.closeness(g)
    ha = pygraphina.centrality.harmonic(g)
    assert isinstance(cl, dict)
    assert isinstance(ha, dict)
    assert set(cl.keys()) == set(nodes)
    assert set(ha.keys()) == set(nodes)


def test_eigenvector_and_pagerank_katz():
    g, nodes = make_chain_graph()
    ev = pygraphina.centrality.eigenvector(g, 100, 1e-9)
    pr = pygraphina.centrality.pagerank(g, 0.85, 100, 1e-6)
    kz = pygraphina.centrality.katz(g, 0.1, 100, 1e-6)
    assert set(ev.keys()) == set(nodes)
    assert set(pr.keys()) == set(nodes)
    assert set(kz.keys()) == set(nodes)
