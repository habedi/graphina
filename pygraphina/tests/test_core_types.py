import pygraphina
import pytest


# Using the Arrange-Act-Assert pattern for testing

def test_add_nodes():
    # Arrange
    g = pygraphina.PyGraph()

    # Act
    g.add_node(10)
    g.add_node(20)

    # Assert
    assert g.node_count() == 2, "Expected 2 nodes in the graph"


def test_update_nodes():
    # Arrange
    g = pygraphina.PyGraph()
    n0 = g.add_node(10)

    # Act
    success = g.update_node(n0, 15)

    # Assert
    assert success is True, "update_node should return True on success"

    # Arrange for error-returning API
    # Act & Assert: Should not raise an exception.
    g.try_update_node(n0, 25)


def test_add_edge_and_neighbors():
    # Arrange
    g = pygraphina.PyGraph()
    n0 = g.add_node(10)
    n1 = g.add_node(20)

    # Act
    edge_id = g.add_edge(n0, n1, 3.14)

    # Assert: Verify edge addition.
    assert g.edge_count() == 1, "Expected 1 edge in the graph"

    # Arrange: Retrieve neighbors for each node.
    neighbors_n0 = g.neighbors(n0)
    neighbors_n1 = g.neighbors(n1)

    # Assert: Verify neighbor relationships.
    assert n1 in neighbors_n0, "n1 should be a neighbor of n0"
    assert n0 in neighbors_n1, "n0 should be a neighbor of n1"


def test_remove_node():
    # Arrange
    g = pygraphina.PyGraph()
    n0 = g.add_node(10)
    n1 = g.add_node(20)
    g.add_edge(n0, n1, 3.14)

    # Act
    removed_attr = g.remove_node(n1)

    # Assert: Check the removed node's attribute.
    assert removed_attr == 20, "Removed node should have attribute 20"
    # Assert: Verify node count after removal.
    assert g.node_count() == 1, "Expected 1 node after removal"


def test_try_remove_node_error():
    # Arrange
    g = pygraphina.PyGraph()
    g.add_node(10)

    # Act & Assert: Attempting to remove a non-existent node should raise ValueError.
    with pytest.raises(ValueError):
        g.try_remove_node(999)  # 999 does not exist
