import pygraphina


def make_simple_graph():
    """Create a simple graph for testing"""
    g = pygraphina.PyGraph()
    nodes = [g.add_node(i) for i in range(5)]
    g.add_edge(nodes[0], nodes[1], 1.0)
    g.add_edge(nodes[1], nodes[2], 1.0)
    g.add_edge(nodes[2], nodes[3], 1.0)
    g.add_edge(nodes[3], nodes[4], 1.0)
    return g, nodes


def test_local_reaching_centrality():
    g, nodes = make_simple_graph()
    # Test with distance 2
    centrality = pygraphina.centrality.local_reaching_centrality(g, 2)
    assert isinstance(centrality, dict)
    assert len(centrality) == 5
    # Node in the middle should reach more nodes
    assert centrality[2] >= centrality[0]


def test_global_reaching_centrality():
    g, nodes = make_simple_graph()
    centrality = pygraphina.centrality.global_reaching_centrality(g)
    assert isinstance(centrality, dict)
    assert len(centrality) == 5
    # All nodes in a path should reach all other nodes
    assert all(val == 5.0 for val in centrality.values())

