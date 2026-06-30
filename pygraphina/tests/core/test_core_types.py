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
