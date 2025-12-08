import math

import networkx as nx
import pygraphina
import pytest


def build_triangle_pg():
    g = pygraphina.PyGraph()
    a = g.add_node(1)
    b = g.add_node(2)
    c = g.add_node(3)
    g.add_edge(a, b, 1.0)
    g.add_edge(b, c, 1.0)
    g.add_edge(c, a, 1.0)
    return (g, (a, b, c))


def build_triangle_nx():
    H = nx.Graph()
    H.add_edge(0, 1, weight=1.0)
    H.add_edge(1, 2, weight=1.0)
    H.add_edge(2, 0, weight=1.0)
    return H


def test_metrics_triangle_parity():
    g, (a, b, c) = build_triangle_pg()
    H = build_triangle_nx()
    assert g.is_connected() is True
    assert g.is_empty() is False
    assert g.has_self_loops() is False
    assert g.has_negative_weights() is False
    assert g.diameter() == nx.diameter(H)
    assert g.radius() == nx.radius(H)
    assert math.isclose(g.average_clustering(), nx.average_clustering(H), rel_tol=1e-06,
                        abs_tol=1e-09)
    c_local = g.clustering_of(a)
    assert math.isclose(c_local, nx.clustering(H, 0), rel_tol=1e-06, abs_tol=1e-09)
    assert math.isclose(g.transitivity(), nx.transitivity(H), rel_tol=1e-06, abs_tol=1e-09)
    assert g.triangles_of(a) == 1
    assert g.triangles_of(b) == 1
    assert g.triangles_of(c) == 1
    assert math.isclose(g.average_path_length() or 0.0, nx.average_shortest_path_length(H),
                        rel_tol=1e-06, abs_tol=1e-09)
    nx_assort = nx.degree_assortativity_coefficient(H)
    if not math.isnan(nx_assort):
        assert math.isclose(g.assortativity(), nx_assort, rel_tol=1e-06, abs_tol=1e-09)


def test_validation_components_and_bipartite():
    g = pygraphina.PyGraph()
    n0 = g.add_node(0)
    n1 = g.add_node(1)
    n2 = g.add_node(2)
    n3 = g.add_node(3)
    n4 = g.add_node(4)
    g.add_edge(n0, n1, 1.0)
    g.add_edge(n1, n2, 1.0)
    g.add_edge(n3, n4, 1.0)
    assert g.is_connected() is False
    assert g.count_components() == 2
    assert g.is_bipartite() is True
    g.add_edge(n0, n0, 1.0)
    assert g.has_self_loops() is True
    assert g.is_bipartite() is False


def test_serialization_roundtrip_json_and_binary(tmp_path: 'pytest.TempPathFactory'):
    g, (a, b, c) = build_triangle_pg()
    json_path = tmp_path.joinpath('g.json')
    bin_path = tmp_path.joinpath('g.bin')
    g.save_json(str(json_path))
    g.save_binary(str(bin_path))
    g2 = pygraphina.PyGraph()
    g2.load_json(str(json_path))
    assert g2.node_count() == g.node_count()
    assert g2.edge_count() == g.edge_count()
    assert math.isclose(g2.density(), g.density(), rel_tol=1e-09)
    g3 = pygraphina.PyGraph()
    g3.load_binary(str(bin_path))
    assert g3.node_count() == g.node_count()
    assert g3.edge_count() == g.edge_count()


def test_edge_list_roundtrip(tmp_path: 'pytest.TempPathFactory'):
    g = pygraphina.PyGraph()
    ids = [g.add_node(i) for i in range(5)]
    for i in range(4):
        g.add_edge(ids[i], ids[i + 1], float(i + 1))
    path = tmp_path.joinpath('edges.txt')
    g.save_edge_list(str(path), sep=',')
    g2 = pygraphina.PyGraph()
    g2.load_edge_list(str(path), sep=',')
    assert g2.node_count() == g.node_count()
    assert g2.edge_count() == g.edge_count()

    def degseq(pg: pygraphina.PyGraph):
        ds = []
        for nid in pg.nodes:
            ds.append(pg.degree(nid) or 0)
        return sorted(ds)

    assert degseq(g2) == degseq(g)
