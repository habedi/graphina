import pygraphina as pg
import pytest

try:
    import pandas as pd
except Exception:
    pd = None
pytestmark = pytest.mark.skipif(pd is None, reason='pandas not installed')


class TestNodeDataFrame:

    def test_pygraph_to_node_dataframe(self):
        g = pg.PyGraph()
        n0 = g.add_node(100)
        n1 = g.add_node(200)
        n2 = g.add_node(300)
        df = pg.to_node_dataframe(g)
        assert isinstance(df, pd.DataFrame)
        assert len(df) == 3
        assert 'node_id' in df.columns
        assert 'attr' in df.columns
        assert set(df['node_id'].tolist()) == {n0, n1, n2}
        assert set(df['attr'].tolist()) == {100, 200, 300}

    def test_pydigraph_to_node_dataframe(self):
        d = pg.PyDiGraph()
        n0 = d.add_node(10)
        n1 = d.add_node(20)
        df = pg.to_node_dataframe(d)
        assert isinstance(df, pd.DataFrame)
        assert len(df) == 2
        assert set(df['attr'].tolist()) == {10, 20}

    def test_empty_graph_node_dataframe(self):
        g = pg.PyGraph()
        df = pg.to_node_dataframe(g)
        assert isinstance(df, pd.DataFrame)
        assert len(df) == 0
        assert 'node_id' in df.columns
        assert 'attr' in df.columns


class TestEdgeDataFrame:

    def test_pygraph_to_edge_dataframe(self):
        g = pg.PyGraph()
        n0 = g.add_node(0)
        n1 = g.add_node(1)
        n2 = g.add_node(2)
        g.add_edge(n0, n1, 1.5)
        g.add_edge(n1, n2, 2.5)
        df = pg.to_edge_dataframe(g)
        assert isinstance(df, pd.DataFrame)
        assert len(df) == 2
        assert 'source' in df.columns
        assert 'target' in df.columns
        assert 'weight' in df.columns
        assert set(df['weight'].tolist()) == {1.5, 2.5}

    def test_pydigraph_to_edge_dataframe(self):
        d = pg.PyDiGraph()
        n0 = d.add_node(0)
        n1 = d.add_node(1)
        d.add_edge(n0, n1, 3.0)
        df = pg.to_edge_dataframe(d)
        assert isinstance(df, pd.DataFrame)
        assert len(df) == 1
        assert df['source'].iloc[0] == n0
        assert df['target'].iloc[0] == n1
        assert df['weight'].iloc[0] == pytest.approx(3.0)

    def test_empty_graph_edge_dataframe(self):
        g = pg.PyGraph()
        g.add_node(0)
        g.add_node(1)
        df = pg.to_edge_dataframe(g)
        assert isinstance(df, pd.DataFrame)
        assert len(df) == 0
        assert 'source' in df.columns
        assert 'target' in df.columns
        assert 'weight' in df.columns


class TestDataFrameRoundTrip:

    def test_dataframe_preserves_values(self):
        g = pg.PyGraph()
        n0 = g.add_node(42)
        n1 = g.add_node(-100)
        n2 = g.add_node(0)
        g.add_edge(n0, n1, 3.14159)
        g.add_edge(n1, n2, -1.0)
        nodes_df = pg.to_node_dataframe(g)
        edges_df = pg.to_edge_dataframe(g)
        node_dict = dict(zip(nodes_df['node_id'], nodes_df['attr']))
        assert node_dict[n0] == 42
        assert node_dict[n1] == -100
        assert node_dict[n2] == 0
        for _, row in edges_df.iterrows():
            w = g.get_edge_weight(int(row['source']), int(row['target']))
            assert w == pytest.approx(row['weight'])


if __name__ == '__main__':
    pytest.main([__file__, '-v'])
