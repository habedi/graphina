"""
Test Pythonic class aliases (Graph, DiGraph) to ensure more intuitive API.
"""
import pygraphina as pg


def test_graph_alias():
    """Test that pg.Graph() works as an alias for pg.PyGraph()."""
    # Test with new Pythonic alias
    g1 = pg.Graph()
    assert g1 is not None
    assert len(g1) == 0

    n1 = g1.add_node(10)
    n2 = g1.add_node(20)
    g1.add_edge(n1, n2, 1.5)

    assert len(g1) == 2
    assert g1.edge_count() == 1
    assert n1 in g1

    # Test that old API still works (backward compatibility)
    g2 = pg.PyGraph()
    assert g2 is not None
    assert len(g2) == 0


def test_digraph_alias():
    """Test that pg.DiGraph() works as an alias for pg.PyDiGraph()."""
    # Test with new Pythonic alias
    dg1 = pg.DiGraph()
    assert dg1 is not None
    assert len(dg1) == 0

    n1 = dg1.add_node(100)
    n2 = dg1.add_node(200)
    dg1.add_edge(n1, n2, 2.5)

    assert len(dg1) == 2
    assert dg1.edge_count() == 1
    assert n1 in dg1
    assert dg1.is_directed()

    # Test that old API still works (backward compatibility)
    dg2 = pg.PyDiGraph()
    assert dg2 is not None
    assert len(dg2) == 0


def test_graph_vs_digraph_distinction():
    """Test that Graph is undirected and DiGraph is directed."""
    g = pg.Graph()
    dg = pg.DiGraph()

    assert not g.is_directed(), "Graph should be undirected"
    assert dg.is_directed(), "DiGraph should be directed"


def test_repr_with_new_aliases():
    """Test that __repr__ works with new aliases."""
    g = pg.Graph()
    g.add_node(1)
    g.add_node(2)

    repr_str = repr(g)
    assert "nodes=2" in repr_str
    assert "edges=0" in repr_str


def test_bulk_operations_with_aliases():
    """Test that bulk operations work with new aliases."""
    g = pg.Graph()
    nodes = g.add_nodes_from([10, 20, 30, 40])

    assert len(nodes) == 4
    assert len(g) == 4

    edges = g.add_edges_from([
        (nodes[0], nodes[1], 1.0),
        (nodes[1], nodes[2], 2.0),
        (nodes[2], nodes[3], 3.0)
    ])

    assert len(edges) == 3
    assert g.edge_count() == 3

    # Test iteration
    for node in nodes:
        assert node in g


def test_iterator_protocol_graph():
    """Test that Graph supports iterator protocol (for node in graph:)."""
    g = pg.Graph()
    nodes = g.add_nodes_from([100, 200, 300])

    # Test direct iteration
    iterated_nodes = []
    for node in g:
        iterated_nodes.append(node)

    assert len(iterated_nodes) == 3
    assert set(iterated_nodes) == set(nodes)

    # Test with list()
    all_nodes = list(g)
    assert len(all_nodes) == 3
    assert set(all_nodes) == set(nodes)


def test_iterator_protocol_digraph():
    """Test that DiGraph supports iterator protocol (for node in digraph:)."""
    dg = pg.DiGraph()
    nodes = dg.add_nodes_from([10, 20, 30, 40])

    # Test direct iteration
    iterated_nodes = []
    for node in dg:
        iterated_nodes.append(node)

    assert len(iterated_nodes) == 4
    assert set(iterated_nodes) == set(nodes)

    # Test with list()
    all_nodes = list(dg)
    assert len(all_nodes) == 4
    assert set(all_nodes) == set(nodes)


def test_iterator_with_operations():
    """Test iterator protocol with graph operations."""
    g = pg.Graph()
    nodes = g.add_nodes_from([1, 2, 3, 4, 5])
    g.add_edges_from([
        (nodes[0], nodes[1], 1.0),
        (nodes[1], nodes[2], 1.0),
        (nodes[2], nodes[3], 1.0),
    ])

    # Count nodes using iteration
    node_count = sum(1 for _ in g)
    assert node_count == 5

    # Check all nodes exist
    for node in g:
        assert node in g
        assert g.get_node_attr(node) is not None

    # Use list comprehension with iterator
    node_list = [n for n in g]
    assert len(node_list) == 5
