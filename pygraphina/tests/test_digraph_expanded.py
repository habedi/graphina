"""
Expanded tests for PyDiGraph to ensure feature parity with PyGraph.

This test suite validates:
1. Subgraph operations (subgraph, induced_subgraph, ego_graph, component_subgraph)
2. Traversal operations (BFS, DFS, IDDFS, bidirectional search)
3. Path algorithms (Dijkstra, Bellman-Ford, Floyd-Warshall)
4. Metrics (diameter, radius, clustering, transitivity, etc.)
5. Validation methods (is_connected, has_negative_weights, etc.)
6. I/O operations (edge list, JSON, binary, GraphML)
7. In/out degree and neighbor queries
"""

import os
import pytest
import tempfile

import pygraphina as pg


class TestDigraphInOutDegrees:
    """Test directed graph in-degree and out-degree functionality."""

    def test_out_degree(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n0, n2, 1.0)

        assert g.out_degree(n0) == 2
        assert g.out_degree(n1) == 0
        assert g.out_degree(n2) == 0

    def test_in_degree(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n0, n2, 1.0)

        assert g.in_degree(n0) == 0
        assert g.in_degree(n1) == 1
        assert g.in_degree(n2) == 1

    def test_total_degree(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n0, 1.0)

        # Total degree = in + out
        assert g.degree(n0) == 2
        assert g.degree(n1) == 2

    def test_in_neighbors(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n2, 1.0)
        g.add_edge(n1, n2, 1.0)

        in_neighbors = g.in_neighbors(n2)
        assert set(in_neighbors) == {n0, n1}

    def test_out_neighbors(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n0, n2, 1.0)

        out_neighbors = g.out_neighbors(n0)
        assert set(out_neighbors) == {n1, n2}


class TestDigraphTraversal:
    """Test traversal algorithms on directed graphs."""

    def test_bfs(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)

        visited = g.bfs(n0)
        assert set(visited) == {n0, n1, n2}

    def test_dfs(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)

        visited = g.dfs(n0)
        assert set(visited) == {n0, n1, n2}

    def test_iddfs_finds_path(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)

        path = g.iddfs(n0, n2, 10)
        assert path is not None
        assert path[0] == n0
        assert path[-1] == n2

    def test_iddfs_no_path(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        # No edge to n2

        path = g.iddfs(n0, n2, 10)
        assert path is None

    def test_bidirectional_search(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)

        path = g.bidirectional_search(n0, n2)
        assert path is not None
        assert path[0] == n0
        assert path[-1] == n2


class TestDigraphSubgraphs:
    """Test subgraph extraction operations on directed graphs."""

    def test_subgraph(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(10)
        n1 = g.add_node(20)
        n2 = g.add_node(30)

        g.add_edge(n0, n1, 1.5)
        g.add_edge(n1, n2, 2.5)

        sub = g.subgraph([n0, n1])
        assert sub.node_count() == 2
        assert sub.edge_count() == 1
        assert sub.is_directed()

    def test_induced_subgraph(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(100)
        n1 = g.add_node(200)
        n2 = g.add_node(300)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 2.0)

        induced = g.induced_subgraph([n0, n1])
        assert induced.node_count() == 2
        assert induced.edge_count() == 1

    def test_ego_graph(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        n3 = g.add_node(3)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)
        g.add_edge(n2, n3, 1.0)

        ego = g.ego_graph(n0, 1)
        assert ego.node_count() == 2  # n0 and n1

    def test_k_hop_neighbors(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)

        neighbors = g.k_hop_neighbors(n0, 2)
        assert n1 in neighbors
        assert n2 in neighbors

    def test_connected_component(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        n3 = g.add_node(3)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)
        # n3 is isolated

        component = g.connected_component(n0)
        assert set(component) == {n0, n1, n2}

    def test_component_subgraph(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)

        comp = g.component_subgraph(n0)
        assert comp.node_count() == 3
        assert comp.edge_count() == 2


class TestDigraphPaths:
    """Test path algorithms on directed graphs."""

    def test_dijkstra(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 2.0)

        distances = g.dijkstra(n0)
        assert distances[n0] == 0.0
        assert distances[n1] == 1.0
        assert distances[n2] == 3.0

    def test_shortest_path(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 2.0)

        result = g.shortest_path(n0, n2)
        assert result is not None
        cost, path = result
        assert cost == 3.0
        assert path == [n0, n1, n2]

    def test_bellman_ford(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 2.0)

        distances = g.bellman_ford(n0)
        assert distances is not None
        assert distances[n0] == 0.0
        assert distances[n1] == 1.0
        assert distances[n2] == 3.0

    def test_floyd_warshall(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 2.0)

        all_pairs = g.floyd_warshall()
        assert all_pairs is not None
        assert all_pairs[n0][n1] == 1.0
        assert all_pairs[n0][n2] == 3.0


class TestDigraphMetrics:
    """Test metric computations on directed graphs."""

    def test_diameter(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)
        g.add_edge(n2, n0, 1.0)

        diam = g.diameter()
        assert diam is not None
        assert diam >= 2

    def test_radius(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)
        g.add_edge(n2, n0, 1.0)

        rad = g.radius()
        assert rad is not None

    def test_average_clustering(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)
        g.add_edge(n2, n0, 1.0)

        avg_clust = g.average_clustering()
        assert avg_clust >= 0.0
        assert avg_clust <= 1.0

    def test_clustering_of(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)

        clust = g.clustering_of(n0)
        assert clust >= 0.0
        assert clust <= 1.0

    def test_transitivity(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)
        g.add_edge(n2, n0, 1.0)

        trans = g.transitivity()
        assert trans >= 0.0
        assert trans <= 1.0

    def test_triangles_of(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)

        triangles = g.triangles_of(n0)
        assert triangles >= 0

    def test_assortativity(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)

        assortativity = g.assortativity()
        assert isinstance(assortativity, float)


class TestDigraphValidation:
    """Test validation methods on directed graphs."""

    def test_is_empty(self):
        g = pg.PyDiGraph()
        assert g.is_empty()

        g.add_node(0)
        assert not g.is_empty()

    def test_is_connected(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n0, 1.0)

        # Weakly connected
        assert g.is_connected()

    def test_has_negative_weights(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)

        g.add_edge(n0, n1, -1.0)

        assert g.has_negative_weights()

    def test_has_self_loops(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)

        g.add_edge(n0, n0, 1.0)

        assert g.has_self_loops()

    def test_count_components(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        n3 = g.add_node(3)

        g.add_edge(n0, n1, 1.0)
        # n2 and n3 are isolated

        components = g.count_components()
        assert components >= 1


class TestDigraphIO:
    """Test I/O operations on directed graphs."""

    def test_edge_list_round_trip(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        g.add_edge(n0, n1, 1.5)

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            path = f.name

        try:
            g.save_edge_list(path)

            g2 = pg.PyDiGraph()
            nodes, edges = g2.load_edge_list(path)
            assert nodes >= 2
            assert edges >= 1
        finally:
            if os.path.exists(path):
                os.remove(path)

    def test_json_round_trip(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(10)
        n1 = g.add_node(20)
        g.add_edge(n0, n1, 2.5)

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.json') as f:
            path = f.name

        try:
            g.save_json(path)

            g2 = pg.PyDiGraph()
            g2.load_json(path)
            assert g2.node_count() == 2
            assert g2.edge_count() == 1
        finally:
            if os.path.exists(path):
                os.remove(path)

    def test_binary_round_trip(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(100)
        n1 = g.add_node(200)
        g.add_edge(n0, n1, 3.14)

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.bin') as f:
            path = f.name

        try:
            g.save_binary(path)

            g2 = pg.PyDiGraph()
            g2.load_binary(path)
            assert g2.node_count() == 2
            assert g2.edge_count() == 1
        finally:
            if os.path.exists(path):
                os.remove(path)


class TestDigraphFilters:
    """Test filter operations on directed graphs."""

    def test_filter_nodes(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(10)
        n1 = g.add_node(20)
        n2 = g.add_node(30)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)

        # Filter nodes with attr > 15
        filtered = g.filter_nodes(lambda nid, attr: attr > 15)
        assert filtered.node_count() == 2

    def test_filter_edges(self):
        g = pg.PyDiGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)

        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 5.0)

        # Filter edges with weight > 2.0
        filtered = g.filter_edges(lambda u, v, w: w > 2.0)
        assert filtered.edge_count() == 1
