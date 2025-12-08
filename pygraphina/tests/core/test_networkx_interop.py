import pygraphina as pg
import pytest

try:
    import networkx as nx
except Exception:  # pragma: no cover
    nx = None

pytestmark = pytest.mark.skipif(nx is None, reason="networkx not installed")


def test_to_networkx_graph_and_back():
    g = pg.PyGraph()
    a = g.add_node(10)
    b = g.add_node(20)
    c = g.add_node(30)
    g.add_edge(a, b, 1.5)
    g.add_edge(b, c, 2.5)

    nx_g = pg.to_networkx(g)
    assert isinstance(nx_g, nx.Graph)
    # Node attributes
    assert nx_g.nodes[a]["attr"] == 10
    assert nx_g.nodes[b]["attr"] == 20
    assert nx_g.nodes[c]["attr"] == 30
    # Edge weights
    assert nx_g[a][b]["weight"] == pytest.approx(1.5)
    assert nx_g[b][c]["weight"] == pytest.approx(2.5)

    # Round-trip
    g2 = pg.from_networkx(nx_g)
    assert isinstance(g2, pg.PyGraph)
    assert g2.node_count() == 3
    assert g2.edge_count() == 2
    # Attributes preserved by position mapping
    attrs = dict(g2.nodes_with_attrs())
    assert set(attrs.values()) == {10, 20, 30}


def test_to_networkx_digraph_and_back():
    d = pg.PyDiGraph()
    a = d.add_node(1)
    b = d.add_node(2)
    d.add_edge(a, b, 3.0)

    nx_d = pg.to_networkx(d)
    assert isinstance(nx_d, nx.DiGraph)
    assert nx_d.nodes[a]["attr"] == 1
    assert nx_d[a][b]["weight"] == pytest.approx(3.0)

    d2 = pg.from_networkx(nx_d)
    assert isinstance(d2, pg.PyDiGraph)
    assert d2.is_directed()
    assert d2.edge_count() == 1
    assert d2.contains_edge(a, b)


def test_from_networkx_arbitrary_node_ids():
    G = nx.Graph()
    G.add_node("A", attr=5)
    G.add_node("B", attr=6)
    G.add_edge("A", "B", weight=4.2)

    g = pg.from_networkx(G)
    assert isinstance(g, pg.PyGraph)
    assert g.node_count() == 2
    assert g.edge_count() == 1
    # Attributes preserved (values may map to any ids; verify multiset)
    attrs = [v for _, v in g.nodes_with_attrs()]
    assert sorted(attrs) == [5, 6]


def test_from_networkx_id_collision_handling():
    """Test that ID collisions are handled gracefully when converting from NetworkX."""
    # Create a NetworkX graph with integer node IDs that could collide
    G = nx.Graph()
    G.add_node(0, attr=100)
    G.add_node(1, attr=200)
    G.add_node(2, attr=300)
    G.add_edge(0, 1, weight=1.0)
    G.add_edge(1, 2, weight=2.0)

    # Convert to PyGraphina - should handle sequential IDs without collision
    g = pg.from_networkx(G)
    assert g.node_count() == 3
    assert g.edge_count() == 2

    # Verify all nodes are accessible and have correct attributes
    attrs = dict(g.nodes_with_attrs())
    assert len(attrs) == 3
    assert set(attrs.values()) == {100, 200, 300}

    # Test with directed graph as well
    DG = nx.DiGraph()
    DG.add_node(0, attr=10)
    DG.add_node(1, attr=20)
    DG.add_edge(0, 1, weight=5.0)

    dg = pg.from_networkx(DG)
    assert isinstance(dg, pg.PyDiGraph)
    assert dg.node_count() == 2
    assert dg.edge_count() == 1

    attrs = dict(dg.nodes_with_attrs())
    assert len(attrs) == 2
    assert set(attrs.values()) == {10, 20}
