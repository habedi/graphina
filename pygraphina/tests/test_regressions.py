import math

import pygraphina as pg
import pytest

try:
    import networkx as nx
except Exception:
    nx = None


class TestTypeConsistencyFix:

    def test_generator_preserves_node_attributes(self):
        g = pg.complete_graph(5)
        nodes = g.nodes
        attrs = [g.get_node_attr(n) for n in nodes]
        assert len(attrs) == 5
        assert all((attr is not None for attr in attrs))
        assert all((attr >= 0 for attr in attrs))

    def test_generator_preserves_edge_weights(self):
        g = pg.erdos_renyi(10, 0.5, 42)
        edges = g.edges.data('weight')
        for u, v, w in edges:
            assert isinstance(w, float)
            assert w > 0
            assert math.isfinite(w)

    def test_subgraph_preserves_attributes(self):
        g = pg.PyGraph()
        n0 = g.add_node(100)
        n1 = g.add_node(200)
        n2 = g.add_node(300)
        g.add_edge(n0, n1, 1.5)
        g.add_edge(n1, n2, 2.5)
        sub = g.subgraph([n0, n1])
        nodes = sub.nodes
        assert len(nodes) == 2
        attrs = [sub.get_node_attr(n) for n in nodes]
        assert set(attrs) == {100, 200}

    def test_subgraph_preserves_edge_weights(self):
        g = pg.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        precise_weight = 1.123456789
        g.add_edge(n0, n1, precise_weight)
        g.add_edge(n1, n2, 2.0)
        sub = g.subgraph([n0, n1])
        edges = list(sub.edges.data('weight'))
        assert len(edges) == 1
        u, v, w = edges[0]
        assert abs(w - precise_weight) < 1e-09

    def test_induced_subgraph_preserves_data(self):
        g = pg.PyGraph()
        n0 = g.add_node(-100)
        n1 = g.add_node(-200)
        g.add_edge(n0, n1, 3.14159)
        induced = g.induced_subgraph([n0, n1])
        nodes = induced.nodes
        assert len(nodes) == 2
        attrs = [induced.get_node_attr(n) for n in nodes]
        assert set(attrs) == {-100, -200}
        edges = list(induced.edges.data('weight'))
        assert len(edges) == 1
        _, _, w = edges[0]
        assert abs(w - 3.14159) < 1e-09


class TestWeightValidation:

    def test_add_edge_rejects_nan(self):
        g = pg.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        with pytest.raises(ValueError, match='must be finite'):
            g.add_edge(n0, n1, float('nan'))

    def test_add_edge_rejects_positive_inf(self):
        g = pg.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        with pytest.raises(ValueError, match='must be finite'):
            g.add_edge(n0, n1, float('inf'))

    def test_add_edge_rejects_negative_inf(self):
        g = pg.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        with pytest.raises(ValueError, match='must be finite'):
            g.add_edge(n0, n1, float('-inf'))

    def test_add_edge_accepts_valid_weights(self):
        g = pg.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        g.add_edge(n0, n1, 1.0)
        g.add_edge(n0, n2, -5.0)
        g.add_edge(n1, n2, 0.0)
        assert g.edge_count() == 3

    def test_add_edges_from_validates_weights(self):
        g = pg.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        with pytest.raises(ValueError, match='must be finite'):
            g.add_edges_from([(n0, n1, float('nan'))])


class TestNodeMappingConsistency:

    def test_remove_node_cleans_mapping(self):
        g = pg.PyGraph()
        n0 = g.add_node(10)
        n1 = g.add_node(20)
        n2 = g.add_node(30)
        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)
        attr = g.remove_node(n1)
        assert attr == 20
        assert not g.contains_node(n1)
        assert g.node_count() == 2
        assert g.contains_node(n0)
        assert g.contains_node(n2)

    def test_remove_single_node_cleans_mapping(self):
        g = pg.PyGraph()
        n0 = g.add_node(10)
        attr = g.remove_node(n0)
        assert attr == 10
        assert not g.contains_node(n0)
        assert g.node_count() == 0

    def test_clear_resets_all_mappings(self):
        g = pg.PyGraph()
        n0 = g.add_node(10)
        n1 = g.add_node(20)
        g.add_edge(n0, n1, 1.0)
        g.clear()
        assert g.node_count() == 0
        assert g.edge_count() == 0
        assert not g.contains_node(n0)
        assert not g.contains_node(n1)
        new_n0 = g.add_node(100)
        assert g.node_count() == 1
        assert g.get_node_attr(new_n0) == 100


class TestErrorMessages:

    def test_invalid_node_error_includes_id(self):
        g = pg.PyGraph()
        n0 = g.add_node(0)
        with pytest.raises(ValueError, match='999'):
            g.add_edge(n0, 999, 1.0)

    def test_invalid_source_node_error(self):
        g = pg.PyGraph()
        n0 = g.add_node(0)
        with pytest.raises(ValueError, match='source'):
            g.add_edge(999, n0, 1.0)

    def test_invalid_target_node_error(self):
        g = pg.PyGraph()
        n0 = g.add_node(0)
        with pytest.raises(ValueError, match='target'):
            g.add_edge(n0, 999, 1.0)


class TestFilterOperations:

    def test_filter_nodes_preserves_types(self):
        g = pg.PyGraph()
        n0 = g.add_node(10)
        n1 = g.add_node(20)
        n2 = g.add_node(30)
        g.add_edge(n0, n1, 1.5)
        g.add_edge(n1, n2, 2.5)
        filtered = g.filter_nodes(lambda nid, attr: attr >= 20)
        assert filtered.node_count() == 2
        nodes = filtered.nodes
        attrs = [filtered.get_node_attr(n) for n in nodes]
        assert set(attrs) == {20, 30}

    def test_filter_edges_preserves_types(self):
        g = pg.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        g.add_edge(n0, n1, 1.5)
        g.add_edge(n1, n2, 2.5)
        filtered = g.filter_edges(lambda u, v, w: w > 2.0)
        assert filtered.node_count() == 3
        assert filtered.edge_count() == 1
        edges = list(filtered.edges.data('weight'))
        assert len(edges) == 1
        _, _, w = edges[0]
        assert abs(w - 2.5) < 1e-09


class TestEgoGraphOperations:

    def test_ego_graph_preserves_attributes(self):
        g = pg.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        n3 = g.add_node(3)
        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)
        g.add_edge(n2, n3, 1.0)
        ego = g.ego_graph(n0, 2)
        assert ego.node_count() == 3
        nodes = ego.nodes
        attrs = [ego.get_node_attr(n) for n in nodes]
        assert set(attrs) == {0, 1, 2}


class TestComponentOperations:

    def test_component_subgraph_preserves_data(self):
        g = pg.PyGraph()
        n0 = g.add_node(10)
        n1 = g.add_node(20)
        n2 = g.add_node(30)
        n3 = g.add_node(40)
        g.add_edge(n0, n1, 1.5)
        g.add_edge(n2, n3, 2.5)
        comp = g.component_subgraph(n0)
        assert comp.node_count() == 2
        nodes = comp.nodes
        attrs = [comp.get_node_attr(n) for n in nodes]
        assert set(attrs) == {10, 20}
        edges = list(comp.edges.data('weight'))
        assert len(edges) == 1
        _, _, w = edges[0]
        assert abs(w - 1.5) < 1e-09


@pytest.mark.skipif(nx is None, reason='networkx not installed')
class TestNetworkXInteropBugFix:

    def test_string_node_ids(self):
        G = nx.Graph()
        G.add_node('alice', attr=100)
        G.add_node('bob', attr=200)
        G.add_edge('alice', 'bob', weight=3.5)
        g = pg.from_networkx(G)
        assert g.node_count() == 2
        assert g.edge_count() == 1
        attrs = dict(g.nodes_with_attrs())
        assert set(attrs.values()) == {100, 200}

    def test_integer_node_ids(self):
        G = nx.Graph()
        G.add_node(1, attr=10)
        G.add_node(2, attr=20)
        G.add_edge(1, 2, weight=5.0)
        g = pg.from_networkx(G)
        assert g.node_count() == 2
        assert g.edge_count() == 1

    def test_tuple_node_ids(self):
        G = nx.Graph()
        G.add_node((0, 0), attr=1)
        G.add_node((0, 1), attr=2)
        G.add_node((1, 0), attr=3)
        G.add_edge((0, 0), (0, 1), weight=1.0)
        G.add_edge((0, 1), (1, 0), weight=1.5)
        g = pg.from_networkx(G)
        assert g.node_count() == 3
        assert g.edge_count() == 2

    def test_mixed_type_node_ids(self):
        G = nx.Graph()
        G.add_node('node1', attr=1)
        G.add_node(42, attr=2)
        G.add_node((1, 2), attr=3)
        G.add_edge('node1', 42, weight=1.0)
        G.add_edge(42, (1, 2), weight=2.0)
        g = pg.from_networkx(G)
        assert g.node_count() == 3
        assert g.edge_count() == 2

    def test_digraph_with_string_nodes(self):
        G = nx.DiGraph()
        G.add_node('source', attr=1)
        G.add_node('sink', attr=2)
        G.add_edge('source', 'sink', weight=10.0)
        d = pg.from_networkx(G)
        assert isinstance(d, pg.PyDiGraph)
        assert d.node_count() == 2
        assert d.edge_count() == 1
        assert d.is_directed()


class TestCorrectnessAuditFixes:
    """Regressions for correctness bugs fixed in the core library audit."""

    def test_floyd_warshall_negative_cycle_returns_none(self):
        g = pg.PyDiGraph()
        a = g.add_node(0)
        b = g.add_node(1)
        g.add_edge(a, b, -1.0)
        g.add_edge(b, a, -1.0)
        assert g.floyd_warshall() is None

    def test_girvan_newman_repeated_runs_are_identical(self):
        # A 6-cycle ties every edge's betweenness, so each removal step hits a
        # tie; the tie-break must be deterministic across runs.
        g = pg.PyGraph()
        nodes = [g.add_node(i) for i in range(6)]
        for i in range(6):
            g.add_edge(nodes[i], nodes[(i + 1) % 6], 1.0)
        first = pg.community.girvan_newman(g, 2)
        for _ in range(25):
            assert pg.community.girvan_newman(g, 2) == first

    def test_spectral_clustering_separates_two_triangles(self):
        # Two triangles joined by one bridge: k=2 must recover the triangles,
        # which requires the smallest-eigenvalue eigenvectors, not arbitrary ones.
        g = pg.PyGraph()
        nodes = [g.add_node(i) for i in range(6)]
        for u, v in [(0, 1), (1, 2), (0, 2), (3, 4), (4, 5), (3, 5)]:
            g.add_edge(nodes[u], nodes[v], 1.0)
        g.add_edge(nodes[2], nodes[3], 1.0)
        clusters = pg.community.spectral_clustering(g, 2, seed=42)
        sorted_clusters = sorted(sorted(c) for c in clusters)
        assert sorted_clusters == [[0, 1, 2], [3, 4, 5]]

    def test_local_node_connectivity_counts_direct_and_indirect_paths(self):
        # A triangle has two vertex-disjoint paths between adjacent nodes: the
        # direct edge and the path through the third node.
        g = pg.PyGraph()
        s = g.add_node(0)
        t = g.add_node(1)
        a = g.add_node(2)
        g.add_edge(s, t, 1.0)
        g.add_edge(s, a, 1.0)
        g.add_edge(a, t, 1.0)
        assert pg.approximation.local_node_connectivity(g, s, t) == 2


if __name__ == '__main__':
    pytest.main([__file__, '-v'])
