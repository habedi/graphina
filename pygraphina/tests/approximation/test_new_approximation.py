import pygraphina


def make_simple_graph():
    g = pygraphina.PyGraph()
    nodes = [g.add_node(i) for i in range(6)]
    g.add_edge(nodes[0], nodes[1], 1.0)
    g.add_edge(nodes[1], nodes[2], 1.0)
    g.add_edge(nodes[2], nodes[0], 1.0)
    g.add_edge(nodes[3], nodes[4], 1.0)
    g.add_edge(nodes[4], nodes[5], 1.0)
    return (g, nodes)


def test_average_clustering_approx():
    g, nodes = make_simple_graph()
    clustering = pygraphina.average_clustering_approx(g)
    assert isinstance(clustering, float)
    assert clustering >= 0.0
    clustering2 = pygraphina.approximation.average_clustering_approx(g)
    assert clustering == clustering2


def test_local_node_connectivity():
    g, nodes = make_simple_graph()
    conn = pygraphina.approximation.local_node_connectivity(g, nodes[0], nodes[2])
    assert isinstance(conn, int)
    assert conn >= 1


def test_treewidth_min_degree():
    g, nodes = make_simple_graph()
    width, order = pygraphina.approximation.treewidth_min_degree(g)
    assert isinstance(width, int)
    assert isinstance(order, list)
    assert len(order) == 6


def test_treewidth_min_fill_in():
    g, nodes = make_simple_graph()
    width, order = pygraphina.approximation.treewidth_min_fill_in(g)
    assert isinstance(width, int)
    assert isinstance(order, list)
    assert len(order) == 6


def test_ramsey_r2():
    g, nodes = make_simple_graph()
    clique, independent = pygraphina.ramsey_r2(g)
    assert isinstance(clique, list)
    assert isinstance(independent, list)
    clique2, independent2 = pygraphina.approximation.ramsey_r2(g)
    assert len(clique) == len(clique2)
    assert len(independent) == len(independent2)
