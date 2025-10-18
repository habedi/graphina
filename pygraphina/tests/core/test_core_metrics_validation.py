# filepath: /home/hassan/Workspace/RustRoverProjects/graphina/pygraphina/tests/test_core_metrics_validation.py
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
    return g, (a, b, c)


def build_triangle_nx():
    H = nx.Graph()
    H.add_edge(0, 1, weight=1.0)
    H.add_edge(1, 2, weight=1.0)
    H.add_edge(2, 0, weight=1.0)
    return H


def test_metrics_triangle_parity():
    g, (a, b, c) = build_triangle_pg()
    H = build_triangle_nx()

    # Graph connectivity-dependent metrics
    assert g.is_connected() is True
    assert g.is_empty() is False
    assert g.has_self_loops() is False
    assert g.has_negative_weights() is False

    # Diameter/Radius
    assert g.diameter() == nx.diameter(H)
    assert g.radius() == nx.radius(H)

    # Clustering
    assert math.isclose(g.average_clustering(), nx.average_clustering(H), rel_tol=1e-6,
                        abs_tol=1e-9)
    c_local = g.clustering_of(a)
    assert math.isclose(c_local, nx.clustering(H, 0), rel_tol=1e-6, abs_tol=1e-9)

    # Transitivity
    assert math.isclose(g.transitivity(), nx.transitivity(H), rel_tol=1e-6, abs_tol=1e-9)

    # Triangles per node
    assert g.triangles_of(a) == 1
    assert g.triangles_of(b) == 1
    assert g.triangles_of(c) == 1

    # Average path length (connected)
    assert math.isclose(g.average_path_length() or 0.0, nx.average_shortest_path_length(H),
                        rel_tol=1e-6, abs_tol=1e-9)

    # Assortativity (on small graphs this can be nan in nx; guard for finite values)
    nx_assort = nx.degree_assortativity_coefficient(H)
    if not math.isnan(nx_assort):
        assert math.isclose(g.assortativity(), nx_assort, rel_tol=1e-6, abs_tol=1e-9)


def test_validation_components_and_bipartite():
    g = pygraphina.PyGraph()
    # Two components: chain of 3 and chain of 2
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

    # This graph is bipartite
    assert g.is_bipartite() is True

    # Self-loop makes not bipartite
    g.add_edge(n0, n0, 1.0)
    assert g.has_self_loops() is True
    # Self-loop breaks bipartite property in our validator
    assert g.is_bipartite() is False


def test_serialization_roundtrip_json_and_binary(tmp_path: "pytest.TempPathFactory"):
    g, (a, b, c) = build_triangle_pg()

    json_path = tmp_path.joinpath("g.json")
    bin_path = tmp_path.joinpath("g.bin")

    g.save_json(str(json_path))
    g.save_binary(str(bin_path))

    # Load JSON into same instance
    g2 = pygraphina.PyGraph()
    g2.load_json(str(json_path))
    assert g2.node_count() == g.node_count()
    assert g2.edge_count() == g.edge_count()
    assert math.isclose(g2.density(), g.density(), rel_tol=1e-9)

    # Load binary
    g3 = pygraphina.PyGraph()
    g3.load_binary(str(bin_path))
    assert g3.node_count() == g.node_count()
    assert g3.edge_count() == g.edge_count()


def test_edge_list_roundtrip(tmp_path: "pytest.TempPathFactory"):
    g = pygraphina.PyGraph()
    ids = [g.add_node(i) for i in range(5)]
    for i in range(4):
        g.add_edge(ids[i], ids[i + 1], float(i + 1))

    path = tmp_path.joinpath("edges.txt")
    g.save_edge_list(str(path), sep=",")

    g2 = pygraphina.PyGraph()
    g2.load_edge_list(str(path), sep=",")

    assert g2.node_count() == g.node_count()
    assert g2.edge_count() == g.edge_count()

    # Compare degree sequences
    def degseq(pg: pygraphina.PyGraph):
        ds = []
        for nid in pg.nodes():
            ds.append(pg.degree(nid) or 0)
        return sorted(ds)

    assert degseq(g2) == degseq(g)
