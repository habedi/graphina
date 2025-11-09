"""
Unit tests for graph generator functions in pygraphina.
"""
import pytest

import pygraphina


class TestGenerators:
    """Tests for graph generation functions."""

    def test_erdos_renyi_basic(self):
        """Test basic Erdős-Rényi graph generation."""
        g = pygraphina.erdos_renyi(100, 0.1, 42)
        assert g.node_count() == 100
        assert g.edge_count() > 0
        assert not g.is_directed()

    def test_erdos_renyi_edge_probability(self):
        """Test that edge probability affects edge count."""
        g_sparse = pygraphina.erdos_renyi(50, 0.05, 42)
        g_dense = pygraphina.erdos_renyi(50, 0.5, 42)

        assert g_sparse.edge_count() < g_dense.edge_count()

    def test_erdos_renyi_deterministic(self):
        """Test that same seed produces same graph."""
        g1 = pygraphina.erdos_renyi(30, 0.2, 123)
        g2 = pygraphina.erdos_renyi(30, 0.2, 123)

        assert g1.node_count() == g2.node_count()
        assert g1.edge_count() == g2.edge_count()

    # Edge cases
    def test_erdos_renyi_zero_probability(self):
        """Test E-R graph with p=0 (no edges)."""
        g = pygraphina.erdos_renyi(10, 0.0, 42)
        assert g.node_count() == 10
        assert g.edge_count() == 0

    def test_erdos_renyi_one_probability(self):
        """Test E-R graph with p=1 (complete graph)."""
        n = 10
        g = pygraphina.erdos_renyi(n, 1.0, 42)
        assert g.node_count() == n
        # Should have n*(n-1)/2 edges for undirected graph
        assert g.edge_count() == n * (n - 1) // 2

    def test_erdos_renyi_single_node(self):
        """Test E-R graph with single node."""
        g = pygraphina.erdos_renyi(1, 0.5, 42)
        assert g.node_count() == 1
        assert g.edge_count() == 0

    def test_complete_graph(self):
        """Test complete graph generation."""
        n = 10
        g = pygraphina.complete_graph(n)

        assert g.node_count() == n
        expected_edges = n * (n - 1) // 2
        assert g.edge_count() == expected_edges

    def test_complete_graph_small(self):
        """Test complete graph with small size."""
        g = pygraphina.complete_graph(3)
        assert g.node_count() == 3
        assert g.edge_count() == 3

    # Edge cases
    def test_complete_graph_two_nodes(self):
        """Test complete graph with two nodes."""
        g = pygraphina.complete_graph(2)
        assert g.node_count() == 2
        assert g.edge_count() == 1

    def test_bipartite_graph(self):
        """Test bipartite graph generation."""
        g = pygraphina.bipartite(10, 15, 0.3, 42)
        assert g.node_count() == 25
        assert g.edge_count() > 0

    def test_bipartite_full(self):
        """Test fully connected bipartite graph."""
        n1, n2 = 5, 7
        g = pygraphina.bipartite(n1, n2, 1.0, 42)

        assert g.node_count() == n1 + n2
        assert g.edge_count() == n1 * n2

    # Edge cases
    def test_bipartite_zero_probability(self):
        """Test bipartite with p=0."""
        g = pygraphina.bipartite(5, 5, 0.0, 42)
        assert g.node_count() == 10
        assert g.edge_count() == 0

    def test_bipartite_asymmetric(self):
        """Test bipartite with very different partition sizes."""
        g = pygraphina.bipartite(1, 20, 1.0, 42)
        assert g.node_count() == 21
        assert g.edge_count() == 20

    def test_star_graph(self):
        """Test star graph generation."""
        n = 10
        g = pygraphina.star_graph(n)

        assert g.node_count() == n
        assert g.edge_count() == n - 1

    def test_star_graph_degrees(self):
        """Test that star graph has correct degree distribution."""
        g = pygraphina.star_graph(10)
        nodes = g.nodes()
        degrees = [g.degree(node) for node in nodes]

        assert 9 in degrees
        assert degrees.count(1) == 9

    # Edge cases
    def test_star_graph_two_nodes(self):
        """Test smallest possible star graph."""
        g = pygraphina.star_graph(2)
        assert g.node_count() == 2
        assert g.edge_count() == 1

    def test_cycle_graph(self):
        """Test cycle graph generation."""
        n = 10
        g = pygraphina.cycle_graph(n)

        assert g.node_count() == n
        assert g.edge_count() == n

    def test_cycle_graph_all_degree_2(self):
        """Test that all nodes in cycle have degree 2."""
        g = pygraphina.cycle_graph(8)
        nodes = g.nodes()

        for node in nodes:
            assert g.degree(node) == 2

    # Edge cases
    def test_cycle_graph_minimum(self):
        """Test cycle with minimum nodes (3)."""
        g = pygraphina.cycle_graph(3)
        assert g.node_count() == 3
        assert g.edge_count() == 3
        # All nodes should have degree 2
        nodes = g.nodes()
        for node in nodes:
            assert g.degree(node) == 2

    def test_watts_strogatz(self):
        """Test Watts-Strogatz small-world graph."""
        g = pygraphina.watts_strogatz(100, 6, 0.3, 42)

        assert g.node_count() == 100
        assert g.edge_count() > 0

    def test_watts_strogatz_ring_lattice(self):
        """Test W-S with beta=0 creates regular ring lattice."""
        n, k = 20, 4
        g = pygraphina.watts_strogatz(n, k, 0.0, 42)

        assert g.node_count() == n
        assert g.edge_count() == n * k // 2

    # Edge cases
    def test_watts_strogatz_full_rewiring(self):
        """Test W-S with beta=1 (complete rewiring)."""
        n, k = 20, 4
        g = pygraphina.watts_strogatz(n, k, 1.0, 42)
        assert g.node_count() == n
        # Edges may vary due to rewiring

    def test_barabasi_albert(self):
        """Test Barabási-Albert scale-free graph."""
        g = pygraphina.barabasi_albert(100, 3, 42)

        assert g.node_count() == 100
        assert g.edge_count() > 0

    def test_barabasi_albert_min_edges(self):
        """Test that B-A graph has at least expected edges."""
        n, m = 50, 2
        g = pygraphina.barabasi_albert(n, m, 42)

        assert g.edge_count() >= (n - m) * m

    # Edge cases
    def test_barabasi_albert_m_equals_1(self):
        """Test B-A with m=1 (minimum attachment)."""
        g = pygraphina.barabasi_albert(20, 1, 42)
        assert g.node_count() == 20
        # Should have n-1 edges minimum for m=1
        assert g.edge_count() >= 19

    def test_invalid_erdos_renyi_probability(self):
        """Test that invalid probability raises error."""
        with pytest.raises(ValueError):
            pygraphina.erdos_renyi(10, 1.5, 42)

    def test_invalid_erdos_renyi_negative_probability(self):
        """Test negative probability raises error."""
        with pytest.raises(ValueError):
            pygraphina.erdos_renyi(10, -0.1, 42)

    def test_invalid_cycle_too_small(self):
        """Test that cycle with < 3 nodes raises error."""
        with pytest.raises(ValueError):
            pygraphina.cycle_graph(2)

    def test_invalid_bipartite_zero_partition(self):
        """Test bipartite with zero-sized partition."""
        with pytest.raises(ValueError):
            pygraphina.bipartite(0, 5, 0.5, 42)

    def test_invalid_watts_strogatz_odd_k(self):
        """Test W-S with odd k (should fail)."""
        with pytest.raises(ValueError):
            pygraphina.watts_strogatz(20, 5, 0.3, 42)

    def test_invalid_watts_strogatz_k_too_large(self):
        """Test W-S with k >= n."""
        with pytest.raises(ValueError):
            pygraphina.watts_strogatz(10, 10, 0.3, 42)

    def test_complete_graph_single_node(self):
        """Test complete graph with single node."""
        g = pygraphina.complete_graph(1)
        assert g.node_count() == 1
        assert g.edge_count() == 0


class TestGeneratorProperties:
    """Tests for properties of generated graphs."""

    def test_erdos_renyi_connectivity(self):
        """Test that dense E-R graph is likely connected."""
        g = pygraphina.erdos_renyi(50, 0.2, 42)
        assert g.node_count() == 50

    def test_complete_graph_density(self):
        """Test that complete graph has density 1.0."""
        g = pygraphina.complete_graph(10)
        assert g.density() == pytest.approx(1.0)

    def test_star_graph_not_empty(self):
        """Test that star graph is not empty."""
        g = pygraphina.star_graph(5)
        assert not g.is_empty()
        assert g.is_connected()

    def test_complete_graph_all_connected(self):
        """Test that complete graph is fully connected."""
        g = pygraphina.complete_graph(8)
        nodes = g.nodes()
        for node in nodes:
            # Every node should be connected to all others
            assert g.degree(node) == 7

    def test_bipartite_is_bipartite(self):
        """Test that generated bipartite graph is actually bipartite."""
        g = pygraphina.bipartite(5, 5, 1.0, 42)
        assert g.is_bipartite()

    def test_cycle_graph_is_connected(self):
        """Test that cycle graph is connected."""
        g = pygraphina.cycle_graph(10)
        assert g.is_connected()
