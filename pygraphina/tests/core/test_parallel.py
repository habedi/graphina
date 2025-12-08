import pygraphina
import pytest


class TestParallelAlgorithms:

    def create_test_graph(self):
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        n3 = g.add_node(3)
        n4 = g.add_node(4)
        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)
        g.add_edge(n2, n3, 1.0)
        g.add_edge(n3, n4, 1.0)
        return (g, [n0, n1, n2, n3, n4])

    def test_bfs_parallel_basic(self):
        g, nodes = self.create_test_graph()
        results = pygraphina.bfs_parallel(g, [nodes[0], nodes[1]])
        assert len(results) == 2
        assert len(results[0]) == 5
        assert len(results[1]) == 5

    def test_bfs_parallel_single_start(self):
        g, nodes = self.create_test_graph()
        results = pygraphina.bfs_parallel(g, [nodes[0]])
        assert len(results) == 1
        assert len(results[0]) > 0

    def test_bfs_parallel_multiple_starts(self):
        g = pygraphina.complete_graph(10)
        nodes = list(g.nodes)
        results = pygraphina.bfs_parallel(g, nodes[:3])
        assert len(results) == 3
        for result in results:
            assert len(result) == 10

    def test_bfs_parallel_empty_starts(self):
        g, _ = self.create_test_graph()
        results = pygraphina.bfs_parallel(g, [])
        assert len(results) == 0

    def test_bfs_parallel_all_nodes(self):
        g, nodes = self.create_test_graph()
        results = pygraphina.bfs_parallel(g, nodes)
        assert len(results) == len(nodes)
        for result in results:
            assert len(result) == len(nodes)

    def test_degrees_parallel_basic(self):
        g, nodes = self.create_test_graph()
        degrees = pygraphina.degrees_parallel(g)
        assert len(degrees) == 5
        assert degrees[nodes[0]] == 1
        assert degrees[nodes[4]] == 1
        assert degrees[nodes[2]] == 2

    def test_degrees_parallel_complete_graph(self):
        n = 10
        g = pygraphina.complete_graph(n)
        nodes = list(g.nodes)
        degrees = pygraphina.degrees_parallel(g)
        assert len(degrees) == n
        for node in nodes:
            assert degrees[node] == n - 1

    def test_degrees_parallel_star_graph(self):
        g = pygraphina.star_graph(10)
        nodes = list(g.nodes)
        degrees = pygraphina.degrees_parallel(g)
        degree_values = list(degrees.values())
        assert 9 in degree_values
        assert degree_values.count(1) == 9

    def test_degrees_parallel_empty_graph(self):
        g = pygraphina.PyGraph()
        degrees = pygraphina.degrees_parallel(g)
        assert len(degrees) == 0

    def test_degrees_parallel_single_node(self):
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        degrees = pygraphina.degrees_parallel(g)
        assert len(degrees) == 1
        assert degrees[n0] == 0

    def test_degrees_parallel_isolated_nodes(self):
        g = pygraphina.PyGraph()
        nodes = [g.add_node(i) for i in range(5)]
        degrees = pygraphina.degrees_parallel(g)
        for node in nodes:
            assert degrees[node] == 0

    def test_connected_components_parallel_basic(self):
        g, nodes = self.create_test_graph()
        component_map = pygraphina.connected_components_parallel(g)
        assert len(set(component_map.values())) == 1

    def test_connected_components_parallel_disconnected(self):
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)
        g.add_edge(n2, n0, 1.0)
        n3 = g.add_node(3)
        n4 = g.add_node(4)
        n5 = g.add_node(5)
        g.add_edge(n3, n4, 1.0)
        g.add_edge(n4, n5, 1.0)
        g.add_edge(n5, n3, 1.0)
        component_map = pygraphina.connected_components_parallel(g)
        unique_components = set(component_map.values())
        assert len(unique_components) == 2

    def test_connected_components_parallel_single_node(self):
        g = pygraphina.PyGraph()
        nodes = [g.add_node(i) for i in range(3)]
        component_map = pygraphina.connected_components_parallel(g)
        unique_components = set(component_map.values())
        assert len(unique_components) == 3

    def test_connected_components_empty_graph(self):
        g = pygraphina.PyGraph()
        component_map = pygraphina.connected_components_parallel(g)
        assert len(component_map) == 0

    def test_connected_components_many_components(self):
        g = pygraphina.PyGraph()
        for i in range(10):
            n1 = g.add_node(i * 2)
            n2 = g.add_node(i * 2 + 1)
            g.add_edge(n1, n2, 1.0)
        component_map = pygraphina.connected_components_parallel(g)
        unique_components = set(component_map.values())
        assert len(unique_components) == 10

    def test_bfs_parallel_invalid_node(self):
        g, nodes = self.create_test_graph()
        with pytest.raises(pygraphina.GraphinaError):
            pygraphina.bfs_parallel(g, [999])

    def test_bfs_parallel_mixed_valid_invalid(self):
        g, nodes = self.create_test_graph()
        with pytest.raises(pygraphina.GraphinaError):
            pygraphina.bfs_parallel(g, [nodes[0], 999])


class TestParallelPerformance:

    def test_parallel_on_large_graph(self):
        g = pygraphina.erdos_renyi(100, 0.1, 42)
        nodes = list(g.nodes)
        results = pygraphina.bfs_parallel(g, nodes[:5])
        assert len(results) == 5
        degrees = pygraphina.degrees_parallel(g)
        assert len(degrees) == 100
        component_map = pygraphina.connected_components_parallel(g)
        assert len(component_map) > 0

    def test_parallel_consistency(self):
        g = pygraphina.erdos_renyi(30, 0.3, 42)
        nodes = list(g.nodes)
        if len(nodes) > 0:
            start = nodes[0]
            seq_result = g.bfs(start)
            par_results = pygraphina.bfs_parallel(g, [start])
            assert len(par_results) == 1
            assert set(seq_result) == set(par_results[0])


class TestPagerankParallel:

    def test_pagerank_parallel_basic(self):
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)
        g.add_edge(n2, n0, 1.0)
        scores = pygraphina.parallel.pagerank_parallel(g, 0.85, 100, 1e-06)
        assert len(scores) == 3
        for node in [n0, n1, n2]:
            assert scores[node] > 0

    def test_pagerank_parallel_vs_sequential(self):
        g = pygraphina.erdos_renyi(50, 0.1, 42)
        seq_scores = pygraphina.centrality.pagerank(g, 0.85, 100, 1e-06)
        par_scores = pygraphina.parallel.pagerank_parallel(g, 0.85, 100, 1e-06)
        seq_top = sorted(seq_scores, key=lambda n: seq_scores[n], reverse=True)[:10]
        par_top = sorted(par_scores, key=lambda n: par_scores[n], reverse=True)[:10]
        common = set(seq_top) & set(par_top)
        assert len(common) >= 6, f'Only {len(common)} nodes in common among top 10'

    def test_pagerank_parallel_empty_graph(self):
        g = pygraphina.PyGraph()
        scores = pygraphina.parallel.pagerank_parallel(g, 0.85, 100, 1e-06)
        assert len(scores) == 0

    def test_pagerank_parallel_single_node(self):
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        scores = pygraphina.parallel.pagerank_parallel(g, 0.85, 100, 1e-06)
        assert len(scores) == 1
        assert n0 in scores

    def test_pagerank_parallel_with_nstart(self):
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n0, 1.0)
        nstart = {n0: 5.0, n1: 1.0}
        scores = pygraphina.parallel.pagerank_parallel(g, 0.85, 100, 1e-06, nstart=nstart)
        assert abs(scores[n0] - 0.5) < 0.001
        assert abs(scores[n1] - 0.5) < 0.001


class TestTrianglesParallel:

    def test_triangles_parallel_basic(self):
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)
        g.add_edge(n2, n0, 1.0)
        triangles = pygraphina.parallel.triangles_parallel(g)
        assert len(triangles) == 3
        for node in [n0, n1, n2]:
            assert triangles[node] == 1

    def test_triangles_parallel_complete_graph(self):
        g = pygraphina.complete_graph(5)
        triangles = pygraphina.parallel.triangles_parallel(g)
        for node in list(g.nodes):
            assert triangles[node] == 6

    def test_triangles_parallel_no_triangles(self):
        g = pygraphina.PyGraph()
        nodes = [g.add_node(i) for i in range(5)]
        for i in range(4):
            g.add_edge(nodes[i], nodes[i + 1], 1.0)
        triangles = pygraphina.parallel.triangles_parallel(g)
        for node in list(g.nodes):
            assert triangles[node] == 0

    def test_triangles_parallel_empty_graph(self):
        g = pygraphina.PyGraph()
        triangles = pygraphina.parallel.triangles_parallel(g)
        assert len(triangles) == 0


class TestClusteringCoefficientsParallel:

    def test_clustering_parallel_complete_graph(self):
        g = pygraphina.complete_graph(5)
        coeffs = pygraphina.parallel.clustering_coefficients_parallel(g)
        for node in list(g.nodes):
            assert abs(coeffs[node] - 1.0) < 1e-06

    def test_clustering_parallel_star_graph(self):
        g = pygraphina.star_graph(5)
        coeffs = pygraphina.parallel.clustering_coefficients_parallel(g)
        for coeff in coeffs.values():
            assert coeff == 0.0

    def test_clustering_parallel_empty_graph(self):
        g = pygraphina.PyGraph()
        coeffs = pygraphina.parallel.clustering_coefficients_parallel(g)
        assert len(coeffs) == 0


class TestShortestPathsParallel:

    def test_shortest_paths_parallel_basic(self):
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        n3 = g.add_node(3)
        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)
        g.add_edge(n2, n3, 1.0)
        paths = pygraphina.parallel.shortest_paths_parallel(g, [n0, n3])
        assert len(paths) == 2
        assert paths[0][n0] == 0
        assert paths[0][n1] == 1
        assert paths[0][n2] == 2
        assert paths[0][n3] == 3
        assert paths[1][n3] == 0
        assert paths[1][n2] == 1

    def test_shortest_paths_parallel_complete_graph(self):
        g = pygraphina.complete_graph(5)
        nodes = list(g.nodes)
        paths = pygraphina.parallel.shortest_paths_parallel(g, [nodes[0]])
        for node in nodes[1:]:
            assert paths[0][node] == 1
        assert paths[0][nodes[0]] == 0

    def test_shortest_paths_parallel_invalid_source(self):
        g = pygraphina.PyGraph()
        g.add_node(0)
        with pytest.raises(pygraphina.GraphinaError):
            pygraphina.parallel.shortest_paths_parallel(g, [999])

    def test_shortest_paths_parallel_empty_sources(self):
        g = pygraphina.complete_graph(5)
        paths = pygraphina.parallel.shortest_paths_parallel(g, [])
        assert len(paths) == 0
