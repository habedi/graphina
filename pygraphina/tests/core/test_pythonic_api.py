import pygraphina as pg


def test_pythonic_conveniences_basic():
    g = pg.PyGraph()
    # bulk add nodes
    ids = g.add_nodes_from([10, 20, 30])
    assert len(ids) == 3
    assert len(g) == 3
    # membership
    assert ids[0] in g
    assert 999 not in g

    # bulk add edges with optional weight
    eids = g.add_edges_from([
        (ids[0], ids[1], None),  # default weight 1.0
        (ids[1], ids[2], 2.5),  # weight ignored (we pass 3-tuple but fn expects Optional[f64])
    ])
    assert len(eids) == 2
    assert g.edge_count() == 2

    # nodes with attrs
    nodes_attrs = dict(g.nodes_with_attrs())
    assert nodes_attrs[ids[0]] == 10
    assert nodes_attrs[ids[1]] == 20
    assert nodes_attrs[ids[2]] == 30

    # edges helpers
    es = set(g.edges())
    assert (ids[0], ids[1]) in es or (ids[1], ids[0]) in es

    esw = {(u, v): w for (u, v, w) in g.edges_with_weights()}
    # both directions may appear depending on internal representation; check weight presence
    assert any(abs(w - 1.0) < 1e-9 for w in esw.values())

    # __repr__ sanity
    r = repr(g)
    assert "PyGraph(" in r and "nodes=" in r and "edges=" in r
