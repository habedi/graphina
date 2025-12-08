import pygraphina as pg


def test_pythonic_conveniences_basic():
    g = pg.PyGraph()
    ids = g.add_nodes_from([10, 20, 30])
    assert len(ids) == 3
    assert len(g) == 3
    assert ids[0] in g
    assert 999 not in g
    eids = g.add_edges_from([(ids[0], ids[1], None), (ids[1], ids[2], 2.5)])
    assert len(eids) == 2
    assert g.edge_count() == 2
    nodes_attrs = dict(g.nodes_with_attrs())
    assert nodes_attrs[ids[0]] == 10
    assert nodes_attrs[ids[1]] == 20
    assert nodes_attrs[ids[2]] == 30
    es = set(g.edges)
    assert (ids[0], ids[1]) in es or (ids[1], ids[0]) in es
    esw = {(u, v): w for u, v, w in g.edges.data('weight')}
    assert any((abs(w - 1.0) < 1e-09 for w in esw.values()))
    r = repr(g)
    assert 'PyGraph(' in r and 'nodes=' in r and ('edges=' in r)
