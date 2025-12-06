import pytest
import pygraphina

class TestViews:
    def setup_method(self):
        self.g = pygraphina.PyGraph()
        self.n0 = self.g.add_node(0)
        self.n1 = self.g.add_node(1)
        self.g.add_edge(self.n0, self.n1, 1.5)

    def test_node_view(self):
        nodes = self.g.nodes
        assert len(nodes) == 2
        assert self.n0 in nodes
        assert self.n1 in nodes
        assert 999 not in nodes

        # Test iterator
        assert set(nodes) == {self.n0, self.n1}

        # Test __getitem__
        assert nodes[0] == {'attr': 0} # internal index for n0 is 0, attr is 0 (from add_node(0))

        # Test data()
        data = nodes.data()
        assert len(list(data)) == 2

    def test_edge_view(self):
        edges = self.g.edges
        assert len(edges) == 1
        assert (self.n0, self.n1) in edges
        assert (self.n1, self.n0) in edges # Undirected

        # Test iterator
        edge_list = list(edges)
        print(f"Edges: {edge_list}")
        assert len(edge_list) == 1 # (n0, n1) normalized?
        # Typically undirected edges are returned as (u, v) once.

        # Test data
        data = edges.data("weight")
        dlist = list(data)
        assert len(dlist) == 1
        assert dlist[0][2] == 1.5

    def test_degree_view(self):
        degree = self.g.degree
        assert len(degree) == 2
        assert degree[self.n0] == 1
        assert degree[self.n1] == 1

        # Test iterator
        dlist = list(degree)
        assert len(dlist) == 2
        assert (self.n0, 1) in dlist

        # Test call
        # d = degree([n0])
        # assert len(list(d)) == 1
        # assert list(d)[0] == (self.n0, 1)

class TestDiGraphViews:
    def setup_method(self):
        self.g = pygraphina.PyDiGraph()
        self.n0 = self.g.add_node(0)
        self.n1 = self.g.add_node(1)
        self.g.add_edge(self.n0, self.n1, 1.5) # 0 -> 1

    def test_node_view(self):
        nodes = self.g.nodes
        assert len(nodes) == 2
        assert self.n0 in nodes

    def test_edge_view(self):
        edges = self.g.edges
        assert len(edges) == 1
        assert (self.n0, self.n1) in edges
        assert (self.n1, self.n0) not in edges # Directed

    def test_degree_view(self):
        degree = self.g.degree
        # Total degree (in + out)
        assert degree[self.n0] == 1 # out-degree 1
        assert degree[self.n1] == 1 # in-degree 1
