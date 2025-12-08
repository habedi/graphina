import os
import tempfile

import pygraphina
import pytest


class TestComputeLayout:

    def test_compute_layout_force_directed(self):
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        g.add_edge(n0, n1, 1.0)
        g.add_edge(n1, n2, 1.0)
        positions = pygraphina.visualization.compute_layout(g, 'force_directed', 800, 600)
        assert len(positions) == 3
        for node in [n0, n1, n2]:
            assert node in positions
            x, y = positions[node]
            assert 0 <= x <= 800
            assert 0 <= y <= 600

    def test_compute_layout_circular(self):
        g = pygraphina.complete_graph(5)
        positions = pygraphina.visualization.compute_layout(g, 'circular', 800, 600)
        assert len(positions) == 5
        for node in g.nodes:
            assert node in positions
            x, y = positions[node]
            assert isinstance(x, float)
            assert isinstance(y, float)

    def test_compute_layout_hierarchical(self):
        g = pygraphina.PyGraph()
        nodes = [g.add_node(i) for i in range(5)]
        for i in range(4):
            g.add_edge(nodes[i], nodes[i + 1], 1.0)
        positions = pygraphina.visualization.compute_layout(g, 'hierarchical', 800, 600)
        assert len(positions) == 5

    def test_compute_layout_grid(self):
        g = pygraphina.complete_graph(9)
        positions = pygraphina.visualization.compute_layout(g, 'grid', 800, 600)
        assert len(positions) == 9

    def test_compute_layout_random(self):
        g = pygraphina.complete_graph(5)
        positions = pygraphina.visualization.compute_layout(g, 'random', 800, 600)
        assert len(positions) == 5

    def test_compute_layout_invalid_algorithm(self):
        g = pygraphina.PyGraph()
        g.add_node(0)
        with pytest.raises(pygraphina.GraphinaError):
            pygraphina.visualization.compute_layout(g, 'invalid_algorithm', 800, 600)

    def test_compute_layout_empty_graph(self):
        g = pygraphina.PyGraph()
        positions = pygraphina.visualization.compute_layout(g, 'circular', 800, 600)
        assert len(positions) == 0


class TestToD3Json:

    def test_to_d3_json_basic(self):
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        g.add_edge(n0, n1, 1.0)
        json_str = pygraphina.visualization.to_d3_json(g)
        assert isinstance(json_str, str)
        assert 'nodes' in json_str
        assert 'links' in json_str

    def test_to_d3_json_empty_graph(self):
        g = pygraphina.PyGraph()
        json_str = pygraphina.visualization.to_d3_json(g)
        assert isinstance(json_str, str)
        assert 'nodes' in json_str


class TestToAsciiArt:

    def test_to_ascii_art_basic(self):
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        g.add_edge(n0, n1, 1.0)
        ascii_art = pygraphina.visualization.to_ascii_art(g)
        assert isinstance(ascii_art, str)
        assert 'Nodes: 2' in ascii_art
        assert 'Edges: 1' in ascii_art
        assert 'Undirected' in ascii_art

    def test_to_ascii_art_empty_graph(self):
        g = pygraphina.PyGraph()
        ascii_art = pygraphina.visualization.to_ascii_art(g)
        assert isinstance(ascii_art, str)
        assert 'Nodes: 0' in ascii_art


class TestSaveAsHtml:

    def test_save_as_html_basic(self):
        g = pygraphina.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        g.add_edge(n0, n1, 1.0)
        with tempfile.NamedTemporaryFile(suffix='.html', delete=False) as f:
            path = f.name
        try:
            pygraphina.visualization.save_as_html(g, path)
            assert os.path.exists(path)
            with open(path, 'r') as f:
                content = f.read()
            assert '<!DOCTYPE html>' in content
            assert 'd3' in content
        finally:
            if os.path.exists(path):
                os.unlink(path)

    def test_save_as_html_with_layout(self):
        g = pygraphina.complete_graph(5)
        with tempfile.NamedTemporaryFile(suffix='.html', delete=False) as f:
            path = f.name
        try:
            pygraphina.visualization.save_as_html(g, path, layout='circular')
            assert os.path.exists(path)
        finally:
            if os.path.exists(path):
                os.unlink(path)
