def make_simple_graph():
    """Create a simple graph for link prediction testing"""
    g = pygraphina.PyGraph()
    nodes = [g.add_node(i) for i in range(5)]
    # Create a path with some common neighbors
    g.add_edge(nodes[0], nodes[1], 1.0)
    g.add_edge(nodes[0], nodes[2], 1.0)
    g.add_edge(nodes[1], nodes[2], 1.0)
    g.add_edge(nodes[2], nodes[3], 1.0)
    g.add_edge(nodes[3], nodes[4], 1.0)
    return g, nodes


def test_resource_allocation_index():
    g, nodes = make_simple_graph()
    # Test with all pairs
    ra_scores = pygraphina.links.resource_allocation_index(g)
    assert isinstance(ra_scores, dict)
    assert len(ra_scores) > 0
    # All scores should be non-negative
    assert all(score >= 0.0 for score in ra_scores.values())


def test_resource_allocation_index_with_ebunch():
    g, nodes = make_simple_graph()
    # Test with specific node pairs
    ebunch = [(nodes[0], nodes[3]), (nodes[1], nodes[4])]
    ra_scores = pygraphina.links.resource_allocation_index(g, ebunch=ebunch)
    assert isinstance(ra_scores, dict)
    assert len(ra_scores) == 2


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


def test_girvan_newman():
    g, a, b = make_two_components()
    communities = pygraphina.community.girvan_newman(g, target_communities=2)
    assert isinstance(communities, list)
    assert len(communities) >= 2
    # Should detect the two separate components
    total_nodes = sum(len(c) for c in communities)
    assert total_nodes == 6


def test_spectral_clustering():
    g, a, b = make_two_components()
    clusters = pygraphina.community.spectral_clustering(g, k=2, seed=42)
    assert isinstance(clusters, list)
    assert len(clusters) == 2
    total_nodes = sum(len(c) for c in clusters)
    assert total_nodes == 6
