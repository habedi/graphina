"""
Unit tests for parallel algorithms in pygraphina.
"""
import pytest

import pygraphina


class TestParallelAlgorithms:
    """Tests for parallel graph algorithms."""

    def create_test_graph(self):
        """Create a simple graph for testing parallel algorithms."""
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

        return g, [n0, n1, n2, n3, n4]

    def test_bfs_parallel_basic(self):
        """Test parallel BFS from multiple starting nodes."""
        g, nodes = self.create_test_graph()

        # Run BFS from first two nodes
        results = pygraphina.bfs_parallel(g, [nodes[0], nodes[1]])

        assert len(results) == 2
        assert len(results[0]) == 5  # Should visit all 5 nodes
        assert len(results[1]) == 5

    def test_bfs_parallel_single_start(self):
        """Test parallel BFS with single starting node."""
        g, nodes = self.create_test_graph()

        results = pygraphina.bfs_parallel(g, [nodes[0]])

        assert len(results) == 1
        assert len(results[0]) > 0

    def test_bfs_parallel_multiple_starts(self):
        """Test parallel BFS with multiple starting nodes."""
        g = pygraphina.complete_graph(10)
        nodes = g.nodes()

        # Run from 3 different nodes
        results = pygraphina.bfs_parallel(g, nodes[:3])

        assert len(results) == 3
        # In complete graph, all nodes reachable from any start
        for result in results:
            assert len(result) == 10

    # Edge cases
    def test_bfs_parallel_empty_starts(self):
        """Test parallel BFS with empty start list."""
        g, _ = self.create_test_graph()
        results = pygraphina.bfs_parallel(g, [])
        assert len(results) == 0

    def test_bfs_parallel_all_nodes(self):
        """Test parallel BFS from all nodes."""
        g, nodes = self.create_test_graph()
        results = pygraphina.bfs_parallel(g, nodes)
        assert len(results) == len(nodes)
        for result in results:
            assert len(result) == len(nodes)

    def test_degrees_parallel_basic(self):
        """Test parallel degree computation."""
        g, nodes = self.create_test_graph()

        degrees = pygraphina.degrees_parallel(g)

        assert len(degrees) == 5
        # End nodes should have degree 1
        assert degrees[nodes[0]] == 1
        assert degrees[nodes[4]] == 1
        # Middle nodes should have degree 2
        assert degrees[nodes[2]] == 2

    def test_degrees_parallel_complete_graph(self):
        """Test parallel degrees on complete graph."""
        n = 10
        g = pygraphina.complete_graph(n)
        nodes = g.nodes()

        degrees = pygraphina.degrees_parallel(g)

        assert len(degrees) == n
        # All nodes in complete graph should have degree n-1
        for node in nodes:
            assert degrees[node] == n - 1

    def test_degrees_parallel_star_graph(self):
        """Test parallel degrees on star graph."""
        g = pygraphina.star_graph(10)
        nodes = g.nodes()

        degrees = pygraphina.degrees_parallel(g)

        # One center node with degree 9, others with degree 1
        degree_values = list(degrees.values())
        assert 9 in degree_values
        assert degree_values.count(1) == 9

    # Edge cases
    def test_degrees_parallel_empty_graph(self):
        """Test parallel degrees on empty graph."""
        g = pygraphina.PyGraph()
        degrees = pygraphina.degrees_parallel(g)
        assert len(degrees) == 0

    def test_degrees_parallel_single_node(self):
        """Test parallel degrees on single node."""
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        degrees = pygraphina.degrees_parallel(g)
        assert len(degrees) == 1
        assert degrees[n0] == 0

    def test_degrees_parallel_isolated_nodes(self):
        """Test parallel degrees with isolated nodes."""
        g = pygraphina.PyGraph()
        nodes = [g.add_node(i) for i in range(5)]
        degrees = pygraphina.degrees_parallel(g)
        for node in nodes:
            assert degrees[node] == 0

    def test_connected_components_parallel_basic(self):
        """Test parallel connected components detection."""
        g, nodes = self.create_test_graph()

        component_map = pygraphina.connected_components_parallel(g)

        # All nodes should be in the same component
        assert len(set(component_map.values())) == 1

    def test_connected_components_parallel_disconnected(self):
        """Test parallel components on disconnected graph."""
        g = pygraphina.PyGraph()

        # First triangle
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)
        g.add_edge(n2, n0, 1.0)

        # Second triangle
        n3 = g.add_node(3)
        n4 = g.add_node(4)
        n5 = g.add_node(5)
        g.add_edge(n3, n4, 1.0)
        g.add_edge(n4, n5, 1.0)
        g.add_edge(n5, n3, 1.0)

        component_map = pygraphina.connected_components_parallel(g)

        # Should have 2 unique component IDs
        unique_components = set(component_map.values())
        assert len(unique_components) == 2

    def test_connected_components_parallel_single_node(self):
        """Test parallel components on graph with isolated nodes."""
        g = pygraphina.PyGraph()
        nodes = [g.add_node(i) for i in range(3)]

        component_map = pygraphina.connected_components_parallel(g)

        # Each isolated node is its own component
        unique_components = set(component_map.values())
        assert len(unique_components) == 3

    # Edge cases
    def test_connected_components_empty_graph(self):
        """Test components on empty graph."""
        g = pygraphina.PyGraph()
        component_map = pygraphina.connected_components_parallel(g)
        assert len(component_map) == 0

    def test_connected_components_many_components(self):
        """Test with many small components."""
        g = pygraphina.PyGraph()
        # Create 10 pairs of connected nodes
        for i in range(10):
            n1 = g.add_node(i * 2)
            n2 = g.add_node(i * 2 + 1)
            g.add_edge(n1, n2, 1.0)

        component_map = pygraphina.connected_components_parallel(g)
        unique_components = set(component_map.values())
        assert len(unique_components) == 10

    def test_bfs_parallel_invalid_node(self):
        """Test that parallel BFS raises error for invalid node."""
        g, nodes = self.create_test_graph()

        with pytest.raises(ValueError):
            pygraphina.bfs_parallel(g, [999])

    def test_bfs_parallel_mixed_valid_invalid(self):
        """Test parallel BFS with mix of valid and invalid nodes."""
        g, nodes = self.create_test_graph()

        with pytest.raises(ValueError):
            pygraphina.bfs_parallel(g, [nodes[0], 999])


class TestParallelPerformance:
    """Tests for parallel algorithm behavior on larger graphs."""

    def test_parallel_on_large_graph(self):
        """Test parallel algorithms on larger generated graph."""
        g = pygraphina.erdos_renyi(100, 0.1, 42)
        nodes = g.nodes()

        # Test parallel BFS
        results = pygraphina.bfs_parallel(g, nodes[:5])
        assert len(results) == 5

        # Test parallel degrees
        degrees = pygraphina.degrees_parallel(g)
        assert len(degrees) == 100

        # Test parallel components
        component_map = pygraphina.connected_components_parallel(g)
        assert len(component_map) > 0

    def test_parallel_consistency(self):
        """Test that parallel and sequential BFS give same results."""
        g = pygraphina.erdos_renyi(30, 0.3, 42)
        nodes = g.nodes()

        if len(nodes) > 0:
            start = nodes[0]
            # Sequential BFS
            seq_result = g.bfs(start)
            # Parallel BFS with single start
            par_results = pygraphina.bfs_parallel(g, [start])

            assert len(par_results) == 1
            assert set(seq_result) == set(par_results[0])
