import pygraphina as pg


def build_graph():
    g = pg.PyGraph()
    a, b, c, d = g.add_nodes_from([1, 2, 3, 4])
    g.add_edges_from([
        (a, b, 1.0),
        (b, c, 2.0),
        (c, d, 3.0),
        (a, d, 4.0),
    ])
    return g, (a, b, c, d)


def test_filter_nodes_predicate():
    g, (a, b, c, d) = build_graph()

    # keep only even attribute nodes (2 and 4)
    g2 = g.filter_nodes(lambda node, attr: attr % 2 == 0)

    # Filtered graph creates new node IDs, so check by attributes instead
    nodes = g2.nodes()
    assert len(nodes) == 2, "Should have 2 nodes with even attributes"

    # Get attributes of filtered nodes
    attrs = {g2.get_node_attr(n) for n in nodes}
    assert attrs == {2, 4}, "Should have nodes with attributes 2 and 4"

    # edges should include only edges among kept nodes (if any)
    for u, v in g2.edges():
        assert u in nodes and v in nodes


def test_filter_edges_predicate():
    g, (a, b, c, d) = build_graph()

    # keep only edges with w > 2.0
    g2 = g.filter_edges(lambda u, v, w: w > 2.0)

    assert g2.edge_count() == 2
    # Check by edge weights instead of node IDs since filter creates new graph
    edge_weights = {w for _, _, w in g2.edges_with_weights()}
    assert edge_weights == {3.0, 4.0}, "Should have edges with weights 3.0 and 4.0"
