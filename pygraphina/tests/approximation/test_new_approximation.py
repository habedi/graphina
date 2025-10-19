import pygraphina


def make_simple_graph():
    """Create a simple graph for testing"""
    g = pygraphina.PyGraph()
    nodes = [g.add_node(i) for i in range(6)]
    # Create a triangle
    g.add_edge(nodes[0], nodes[1], 1.0)
    g.add_edge(nodes[1], nodes[2], 1.0)
    g.add_edge(nodes[2], nodes[0], 1.0)
    # Add more edges
    g.add_edge(nodes[3], nodes[4], 1.0)
    g.add_edge(nodes[4], nodes[5], 1.0)
    return g, nodes


def test_average_clustering_approx():
    g, nodes = make_simple_graph()
    clustering = pygraphina.average_clustering_approx(g)
    assert isinstance(clustering, float)
    assert clustering >= 0.0
    # Also test namespaced version
    clustering2 = pygraphina.approximation.average_clustering_approx(g)
    assert clustering == clustering2


def test_local_node_connectivity():
    g, nodes = make_simple_graph()
    # Test connectivity between nodes in the triangle
    conn = pygraphina.approximation.local_node_connectivity(g, nodes[0], nodes[2])
    assert isinstance(conn, int)
    assert conn >= 1  # At least one path exists


def test_treewidth_min_degree():
    g, nodes = make_simple_graph()
    width, order = pygraphina.approximation.treewidth_min_degree(g)
    assert isinstance(width, int)
    assert isinstance(order, list)
    assert len(order) == 6  # All nodes in elimination order


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
    # Also test namespaced version
    clique2, independent2 = pygraphina.approximation.ramsey_r2(g)
    assert len(clique) == len(clique2)
    assert len(independent) == len(independent2)
