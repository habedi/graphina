import pygraphina as pg


def build_graph():
    g = pg.PyGraph()
    a, b, c, d = g.add_nodes_from([1, 2, 3, 4])
    g.add_edges_from([(a, b, 1.0), (b, c, 2.0), (c, d, 3.0), (a, d, 4.0)])
    return (g, (a, b, c, d))


def test_filter_nodes_predicate():
    g, (a, b, c, d) = build_graph()
    g2 = g.filter_nodes(lambda node, attr: attr % 2 == 0)
    nodes = g2.nodes
    assert len(nodes) == 2, 'Should have 2 nodes with even attributes'
    attrs = {g2.get_node_attr(n) for n in nodes}
    assert attrs == {2, 4}, 'Should have nodes with attributes 2 and 4'
    for u, v in g2.edges:
        assert u in nodes and v in nodes


def test_filter_edges_predicate():
    g, (a, b, c, d) = build_graph()
    g2 = g.filter_edges(lambda u, v, w: w > 2.0)
    assert g2.edge_count() == 2
    edge_weights = {w for _, _, w in g2.edges.data('weight')}
    assert edge_weights == {3.0, 4.0}, 'Should have edges with weights 3.0 and 4.0'
