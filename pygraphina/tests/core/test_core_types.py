import pygraphina
import pytest


def test_add_nodes():
    g = pygraphina.PyGraph()
    g.add_node(10)
    g.add_node(20)
    assert g.node_count() == 2, 'Expected 2 nodes in the graph'


def test_update_nodes():
    g = pygraphina.PyGraph()
    n0 = g.add_node(10)
    g.update_node(n0, 15)
    g.update_node(n0, 25)
    with pytest.raises(ValueError):
        g.update_node(999, 1)  # updating a missing node raises


def test_add_edge_and_neighbors():
    g = pygraphina.PyGraph()
    n0 = g.add_node(10)
    n1 = g.add_node(20)
    edge_id = g.add_edge(n0, n1, 3.14)
    assert g.edge_count() == 1, 'Expected 1 edge in the graph'
    neighbors_n0 = g.neighbors(n0)
    neighbors_n1 = g.neighbors(n1)
    assert n1 in neighbors_n0, 'n1 should be a neighbor of n0'
    assert n0 in neighbors_n1, 'n0 should be a neighbor of n1'


def test_add_edge_updates_existing_edge():
    # A duplicate add_edge updates the weight instead of creating a parallel edge.
    g = pygraphina.PyGraph()
    a = g.add_node(1)
    b = g.add_node(2)
    g.add_edge(a, b, 1.0)
    g.add_edge(a, b, 2.0)
    assert g.edge_count() == 1, 'Duplicate add_edge should not create a parallel edge'
    assert g.get_edge_weight(a, b) == 2.0, 'Duplicate add_edge should update the weight'
    # The reversed orientation is the same undirected edge.
    g.add_edge(b, a, 3.0)
    assert g.edge_count() == 1
    assert g.get_edge_weight(a, b) == 3.0


def test_add_edge_updates_existing_directed_edge():
    dg = pygraphina.PyDiGraph()
    x = dg.add_node(1)
    y = dg.add_node(2)
    dg.add_edge(x, y, 1.0)
    dg.add_edge(x, y, 2.0)
    assert dg.edge_count() == 1, 'Duplicate add_edge should not create a parallel edge'
    # The reverse direction is a distinct edge on a directed graph.
    dg.add_edge(y, x, 4.0)
    assert dg.edge_count() == 2


def test_update_edge_weight_matches_either_orientation():
    # On an undirected graph, update_edge_weight matches the edge in either
    # orientation and raises ValueError for a missing edge.
    g = pygraphina.PyGraph()
    a = g.add_node(1)
    b = g.add_node(2)
    c = g.add_node(3)
    g.add_edge(a, b, 1.0)
    g.update_edge_weight(b, a, 2.0)
    assert g.get_edge_weight(a, b) == 2.0
    with pytest.raises(ValueError):
        g.update_edge_weight(a, c, 1.0)


def test_update_edge_weight_directed_orientation():
    # On a directed graph, update_edge_weight matches the exact direction only.
    dg = pygraphina.PyDiGraph()
    x = dg.add_node(1)
    y = dg.add_node(2)
    dg.add_edge(x, y, 1.0)
    with pytest.raises(ValueError):
        dg.update_edge_weight(y, x, 2.0)
    dg.update_edge_weight(x, y, 2.0)
    assert dg.get_edge_weight(x, y) == 2.0


def test_remove_node():
    g = pygraphina.PyGraph()
    n0 = g.add_node(10)
    n1 = g.add_node(20)
    g.add_edge(n0, n1, 3.14)
    removed_attr = g.remove_node(n1)
    assert removed_attr == 20, 'Removed node should have attribute 20'
    assert g.node_count() == 1, 'Expected 1 node after removal'


def test_remove_node_error():
    g = pygraphina.PyGraph()
    g.add_node(10)
    with pytest.raises(ValueError):
        g.remove_node(999)
