"""
Unit tests for subgraph operations in pygraphina.
"""
import pygraphina
import pytest


def create_test_graph():
    """Create a simple graph for testing subgraphs."""
    g = pygraphina.PyGraph()
    n0 = g.add_node(0)
    n1 = g.add_node(1)
    n2 = g.add_node(2)
    n3 = g.add_node(3)
    n4 = g.add_node(4)

    g.add_edge(n0, n1, 1.0)
    g.add_edge(n1, n2, 2.0)
    g.add_edge(n2, n3, 3.0)
    g.add_edge(n3, n4, 4.0)
    g.add_edge(n1, n3, 5.0)

    return g, [n0, n1, n2, n3, n4]


class TestSubgraphs:
    """Tests for subgraph extraction."""

    def test_subgraph_basic(self):
        """Test basic subgraph extraction."""
        g, nodes = create_test_graph()

        # Extract subgraph with first 3 nodes
        sub = g.subgraph([nodes[0], nodes[1], nodes[2]])

        assert sub.node_count() == 3
        # Should have edges between these nodes
        assert sub.edge_count() >= 1

    def test_subgraph_preserves_edges(self):
        """Test that subgraph preserves edges between included nodes."""
        g, nodes = create_test_graph()

        # Extract subgraph with nodes that have edges between them
        sub = g.subgraph([nodes[1], nodes[2], nodes[3]])

        assert sub.node_count() == 3
        # Should have edges: 1-2, 2-3, and 1-3
        assert sub.edge_count() == 3

    def test_subgraph_single_node(self):
        """Test subgraph with single node."""
        g, nodes = create_test_graph()

        sub = g.subgraph([nodes[0]])

        assert sub.node_count() == 1
        assert sub.edge_count() == 0

    def test_subgraph_all_nodes(self):
        """Test subgraph with all nodes equals original."""
        g, nodes = create_test_graph()

        sub = g.subgraph(nodes)

        assert sub.node_count() == g.node_count()
        assert sub.edge_count() == g.edge_count()

    def test_subgraph_no_edges(self):
        """Test subgraph of disconnected nodes."""
        g, nodes = create_test_graph()

        # Extract nodes with no edges between them
        sub = g.subgraph([nodes[0], nodes[4]])

        assert sub.node_count() == 2
        assert sub.edge_count() == 0

    def test_induced_subgraph_basic(self):
        """Test induced subgraph extraction."""
        g, nodes = create_test_graph()

        induced = g.induced_subgraph([nodes[1], nodes[2], nodes[3]])

        assert induced.node_count() == 3
        # Should include all edges between these nodes
        assert induced.edge_count() >= 2

    def test_induced_subgraph_complete(self):
        """Test induced subgraph on complete graph."""
        g = pygraphina.complete_graph(10)
        nodes = g.nodes()

        # Extract induced subgraph of 5 nodes
        induced = g.induced_subgraph(nodes[:5])

        assert induced.node_count() == 5
        # Complete subgraph of 5 nodes should have 10 edges
        assert induced.edge_count() == 10

    def test_subgraph_invalid_node(self):
        """Test that subgraph raises error for invalid node."""
        g, nodes = create_test_graph()

        with pytest.raises(ValueError):
            g.subgraph([nodes[0], 999])

    def test_induced_subgraph_invalid_node(self):
        """Test that induced subgraph raises error for invalid node."""
        g, nodes = create_test_graph()

        with pytest.raises(ValueError):
            g.induced_subgraph([nodes[0], 999])

    def test_subgraph_empty_list(self):
        """Test subgraph with empty node list."""
        g, _ = create_test_graph()

        sub = g.subgraph([])

        assert sub.node_count() == 0
        assert sub.edge_count() == 0


class TestSubgraphProperties:
    """Tests for properties of extracted subgraphs."""

    def test_subgraph_independence(self):
        """Test that modifying subgraph doesn't affect original."""
        g = pygraphina.complete_graph(5)
        nodes = g.nodes()

        sub = g.subgraph(nodes[:3])
        original_count = g.node_count()

        # Add node to subgraph
        sub.add_node(999)

        # Original should be unchanged
        assert g.node_count() == original_count

    def test_subgraph_node_mapping(self):
        """Test that subgraph creates new node IDs."""
        g, nodes = create_test_graph()

        sub = g.subgraph([nodes[0], nodes[1]])
        sub_nodes = sub.nodes()

        # Subgraph should have its own node IDs
        assert len(sub_nodes) == 2

    def test_multiple_subgraphs(self):
        """Test extracting multiple subgraphs from same graph."""
        g = pygraphina.complete_graph(10)
        nodes = g.nodes()

        sub1 = g.subgraph(nodes[:5])
        sub2 = g.subgraph(nodes[5:])

        assert sub1.node_count() == 5
        assert sub2.node_count() == 5
        # Original should be unchanged
        assert g.node_count() == 10

    def test_subgraph_of_generated_graph(self):
        """Test subgraph extraction from generated graph."""
        g = pygraphina.erdos_renyi(50, 0.3, 42)
        nodes = g.nodes()

        if len(nodes) >= 10:
            sub = g.subgraph(nodes[:10])
            assert sub.node_count() == 10
            assert sub.edge_count() >= 0


"""
Unit tests for graph generator functions in pygraphina.
"""


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

    def test_complete_graph(self):
        """Test complete graph generation."""
        n = 10
        g = pygraphina.complete_graph(n)

        assert g.node_count() == n
        # Complete graph should have n*(n-1)/2 edges
        expected_edges = n * (n - 1) // 2
        assert g.edge_count() == expected_edges

    def test_complete_graph_small(self):
        """Test complete graph with small size."""
        g = pygraphina.complete_graph(3)
        assert g.node_count() == 3
        assert g.edge_count() == 3

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
        # Should have n1 * n2 edges
        assert g.edge_count() == n1 * n2

    def test_star_graph(self):
        """Test star graph generation."""
        n = 10
        g = pygraphina.star_graph(n)

        assert g.node_count() == n
        # Star graph has n-1 edges
        assert g.edge_count() == n - 1

    def test_star_graph_degrees(self):
        """Test that star graph has correct degree distribution."""
        g = pygraphina.star_graph(10)
        nodes = g.nodes()
        degrees = [g.degree(node) for node in nodes]

        # One node (center) should have degree 9, others degree 1
        assert 9 in degrees
        assert degrees.count(1) == 9

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
        # Regular ring lattice has n*k/2 edges
        assert g.edge_count() == n * k // 2

    def test_barabasi_albert(self):
        """Test Barabási-Albert scale-free graph."""
        g = pygraphina.barabasi_albert(100, 3, 42)

        assert g.node_count() == 100
        assert g.edge_count() > 0

    def test_barabasi_albert_min_edges(self):
        """Test that B-A graph has at least expected edges."""
        n, m = 50, 2
        g = pygraphina.barabasi_albert(n, m, 42)

        # Should have at least (n-m)*m edges
        assert g.edge_count() >= (n - m) * m

    def test_invalid_erdos_renyi_probability(self):
        """Test that invalid probability raises error."""
        with pytest.raises(ValueError):
            pygraphina.erdos_renyi(10, 1.5, 42)

    def test_invalid_cycle_too_small(self):
        """Test that cycle with < 3 nodes raises error."""
        with pytest.raises(ValueError):
            pygraphina.cycle_graph(2)

    def test_complete_graph_single_node(self):
        """Test complete graph with single node."""
        g = pygraphina.complete_graph(1)
        assert g.node_count() == 1
        assert g.edge_count() == 0


class TestGeneratorProperties:
    """Tests for properties of generated graphs."""

    def test_erdos_renyi_connectivity(self):
        """Test that dense E-R graph is likely connected."""
        # High probability should create connected graph
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
