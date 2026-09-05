"""Differential tests comparing PyGraphina against NetworkX as an oracle.

Each test builds the same graph in both libraries and compares per-node or
per-pair values up to floating-point tolerance. PyGraphina assigns sequential
integer node ids, so ids match the NetworkX node labels directly.

Conventions aligned here:
- Betweenness comparisons use normalized values on both sides.
- Eigenvector and Katz vectors are L2-normalized before comparison, since the
  libraries scale their results differently.
- Link predictors receive an explicit ebunch on both sides, because the
  default pair sets differ (PyGraphina scores all pairs, NetworkX non-edges).
- Graphs for Adamic-Adar carry a cycle backbone so every node has degree at
  least 2; NetworkX raises a division error on degree-1 common neighbors.
- The `self_loops` variants add a self-loop to some nodes. A self-loop is a
  single edge (one diagonal entry) and is not a neighbor for clustering, which
  is the NetworkX convention both libraries follow.
"""

import math
import random

import pygraphina as pg
import pytest

nx = pytest.importorskip('networkx')

SEEDS = [7, 21, 42]


def build_pair(n, p, seed, weighted=False, directed=False, backbone=False, self_loops=False):
    """Build the same random graph in PyGraphina and NetworkX."""
    rng = random.Random(seed)
    g = pg.PyDiGraph() if directed else pg.PyGraph()
    G = nx.DiGraph() if directed else nx.Graph()
    ids = [g.add_node(i) for i in range(n)]
    G.add_nodes_from(range(n))

    def add(i, j):
        w = round(rng.uniform(0.5, 3.0), 3) if weighted else 1.0
        g.add_edge(ids[i], ids[j], w)
        G.add_edge(i, j, weight=w)

    if backbone:
        for i in range(n):
            add(i, (i + 1) % n)
    for i in range(n):
        js = range(n) if directed else range(i + 1, n)
        for j in js:
            if i == j or G.has_edge(i, j):
                continue
            if rng.random() < p:
                add(i, j)
    if self_loops:
        looped = [i for i in range(n) if rng.random() < 0.4] or [0]
        for i in looped:
            add(i, i)
    return g, G


def assert_maps_close(ours, reference, tol=1e-9, context=''):
    assert set(ours) == set(reference), f'{context}: key sets differ'
    for k in reference:
        assert math.isclose(ours[k], reference[k], rel_tol=tol, abs_tol=tol), (
            f'{context}: value mismatch at {k}: {ours[k]} vs {reference[k]}'
        )


def l2_normalized(values):
    norm = math.sqrt(sum(v * v for v in values.values()))
    return {k: v / norm for k, v in values.items()} if norm > 0 else values


class TestCentralityDifferential:

    @pytest.mark.parametrize('seed', SEEDS)
    def test_betweenness_normalized_matches_networkx(self, seed):
        g, G = build_pair(12, 0.3, seed)
        ours = pg.centrality.betweenness(g, True)
        reference = nx.betweenness_centrality(G, normalized=True)
        assert_maps_close(ours, reference, 1e-9, f'betweenness seed={seed}')

    @pytest.mark.parametrize('seed', SEEDS)
    def test_edge_betweenness_normalized_matches_networkx(self, seed):
        g, G = build_pair(10, 0.3, seed)
        ours = pg.centrality.edge_betweenness(g, True)
        reference = nx.edge_betweenness_centrality(G, normalized=True)
        for (u, v), val in reference.items():
            key = (u, v) if (u, v) in ours else (v, u)
            assert key in ours, f'edge ({u}, {v}) missing, seed={seed}'
            assert math.isclose(ours[key], val, rel_tol=1e-9, abs_tol=1e-9), (
                f'edge betweenness mismatch at ({u}, {v}): '
                f'{ours[key]} vs {val}, seed={seed}'
            )

    @pytest.mark.parametrize('seed', SEEDS)
    def test_harmonic_matches_networkx(self, seed):
        g, G = build_pair(12, 0.25, seed, weighted=True, backbone=True)
        ours = pg.centrality.harmonic(g)
        reference = nx.harmonic_centrality(G, distance='weight')
        assert_maps_close(ours, reference, 1e-6, f'harmonic seed={seed}')

    @pytest.mark.parametrize('seed', SEEDS)
    def test_closeness_matches_networkx_wasserman_faust(self, seed):
        g, G = build_pair(12, 0.25, seed, weighted=True, backbone=True)
        ours = pg.centrality.closeness(g)
        reference = nx.closeness_centrality(G, distance='weight')
        assert_maps_close(ours, reference, 1e-6, f'closeness seed={seed}')

    @pytest.mark.parametrize('self_loops', [False, True])
    @pytest.mark.parametrize('seed', SEEDS)
    def test_eigenvector_matches_networkx(self, seed, self_loops):
        g, G = build_pair(12, 0.3, seed, backbone=True, self_loops=self_loops)
        ours = l2_normalized(pg.centrality.eigenvector(g, 1000, 1e-10))
        reference = l2_normalized(nx.eigenvector_centrality(G, max_iter=1000, tol=1e-10))
        assert_maps_close(ours, reference, 1e-4, f'eigenvector seed={seed}')

    @pytest.mark.parametrize('seed', SEEDS)
    def test_katz_matches_networkx(self, seed):
        g, G = build_pair(12, 0.3, seed, backbone=True)
        ours = l2_normalized(pg.centrality.katz(g, 0.05, 1000, 1e-10))
        reference = l2_normalized(nx.katz_centrality(G, alpha=0.05, max_iter=1000, tol=1e-10))
        assert_maps_close(ours, reference, 1e-4, f'katz seed={seed}')

    @pytest.mark.parametrize('self_loops', [False, True])
    @pytest.mark.parametrize('directed', [False, True])
    @pytest.mark.parametrize('seed', SEEDS)
    def test_pagerank_weighted_matches_networkx(self, seed, directed, self_loops):
        g, G = build_pair(12, 0.3, seed, weighted=True, directed=directed, self_loops=self_loops)
        ours = pg.centrality.pagerank(g, 0.85, 1000, 1e-12)
        reference = nx.pagerank(G, alpha=0.85, max_iter=1000, tol=1e-12, weight='weight')
        assert_maps_close(ours, reference, 1e-6, f'pagerank seed={seed}')

    @pytest.mark.parametrize('seed', SEEDS)
    def test_personalized_pagerank_matches_networkx(self, seed):
        g, G = build_pair(10, 0.3, seed, directed=True)
        n = 10
        weights = [1.0 if i < 3 else 0.0 for i in range(n)]
        total = sum(weights)
        ours = pg.centrality.personalized_pagerank(
            g, personalization=weights, damping=0.85, tolerance=1e-12, max_iter=1000
        )
        reference = nx.pagerank(
            G,
            alpha=0.85,
            max_iter=1000,
            tol=1e-12,
            personalization={i: w / total for i, w in enumerate(weights)},
            weight='weight',
        )
        assert_maps_close(ours, reference, 1e-6, f'personalized pagerank seed={seed}')


class TestLinkPredictionDifferential:

    def all_pairs(self, n):
        return [(i, j) for i in range(n) for j in range(i + 1, n)]

    @pytest.mark.parametrize('seed', SEEDS)
    def test_jaccard_matches_networkx(self, seed):
        g, G = build_pair(12, 0.3, seed, backbone=True)
        pairs = self.all_pairs(12)
        ours = pg.links.jaccard_coefficient(g, pairs)
        for u, v, score in nx.jaccard_coefficient(G, pairs):
            assert math.isclose(ours[(u, v)], score, rel_tol=1e-9, abs_tol=1e-9), (
                f'jaccard mismatch at ({u}, {v}), seed={seed}'
            )

    @pytest.mark.parametrize('seed', SEEDS)
    def test_adamic_adar_matches_networkx(self, seed):
        g, G = build_pair(12, 0.3, seed, backbone=True)
        pairs = self.all_pairs(12)
        ours = pg.links.adamic_adar_index(g, pairs)
        for u, v, score in nx.adamic_adar_index(G, pairs):
            assert math.isclose(ours[(u, v)], score, rel_tol=1e-9, abs_tol=1e-9), (
                f'adamic-adar mismatch at ({u}, {v}), seed={seed}'
            )

    @pytest.mark.parametrize('seed', SEEDS)
    def test_resource_allocation_matches_networkx(self, seed):
        g, G = build_pair(12, 0.3, seed, backbone=True)
        pairs = self.all_pairs(12)
        ours = pg.links.resource_allocation_index(g, pairs)
        for u, v, score in nx.resource_allocation_index(G, pairs):
            assert math.isclose(ours[(u, v)], score, rel_tol=1e-9, abs_tol=1e-9), (
                f'resource allocation mismatch at ({u}, {v}), seed={seed}'
            )

    @pytest.mark.parametrize('seed', SEEDS)
    def test_preferential_attachment_matches_networkx(self, seed):
        g, G = build_pair(12, 0.3, seed, backbone=True)
        pairs = self.all_pairs(12)
        ours = pg.links.preferential_attachment(g, pairs)
        for u, v, score in nx.preferential_attachment(G, pairs):
            assert math.isclose(ours[(u, v)], score, rel_tol=1e-9, abs_tol=1e-9), (
                f'preferential attachment mismatch at ({u}, {v}), seed={seed}'
            )

    @pytest.mark.parametrize('seed', SEEDS)
    def test_common_neighbors_matches_networkx(self, seed):
        g, G = build_pair(12, 0.3, seed, backbone=True)
        for u, v in self.all_pairs(12):
            ours = pg.links.common_neighbors(g, u, v)
            reference = len(list(nx.common_neighbors(G, u, v)))
            assert ours == reference, f'common neighbors mismatch at ({u}, {v}), seed={seed}'


class TestPathsDifferential:

    @pytest.mark.parametrize('seed', SEEDS)
    def test_dijkstra_matches_networkx(self, seed):
        g, G = build_pair(12, 0.3, seed, weighted=True, directed=True)
        for source in range(0, 12, 4):
            ours = g.dijkstra(source)
            reference = nx.single_source_dijkstra_path_length(G, source, weight='weight')
            for node, dist in ours.items():
                if dist is None:
                    assert node not in reference, (
                        f'node {node} reachable in nx but not graphina, seed={seed}'
                    )
                else:
                    assert math.isclose(dist, reference[node], rel_tol=1e-9), (
                        f'dijkstra mismatch at {node}: {dist} vs {reference[node]}, seed={seed}'
                    )

    @pytest.mark.parametrize('seed', SEEDS)
    def test_floyd_warshall_matches_networkx(self, seed):
        g, G = build_pair(10, 0.3, seed, weighted=True, directed=True)
        ours = g.floyd_warshall()
        assert ours is not None
        reference = nx.floyd_warshall(G, weight='weight')
        for u in range(10):
            for v in range(10):
                ref = reference[u][v]
                val = ours[u][v]
                if math.isinf(ref):
                    assert val is None, f'({u}, {v}) should be unreachable, seed={seed}'
                else:
                    assert val is not None and math.isclose(val, ref, rel_tol=1e-9), (
                        f'floyd_warshall mismatch at ({u}, {v}): {val} vs {ref}, seed={seed}'
                    )


class TestCommunityDifferential:

    @pytest.mark.parametrize('seed', SEEDS)
    def test_connected_components_match_networkx(self, seed):
        # Sparse graph without a backbone, so several components are likely.
        g, G = build_pair(14, 0.08, seed)
        ours = {frozenset(c) for c in pg.community.connected_components(g)}
        reference = {frozenset(c) for c in nx.connected_components(G)}
        assert ours == reference, f'components differ, seed={seed}'


class TestParallelDifferential:
    """The parallel twins never had oracle coverage; compare against NetworkX."""

    @pytest.mark.parametrize('self_loops', [False, True])
    @pytest.mark.parametrize('seed', SEEDS)
    def test_triangles_parallel_matches_networkx(self, seed, self_loops):
        g, G = build_pair(12, 0.3, seed, self_loops=self_loops)
        ours = pg.parallel.triangles_parallel(g)
        assert_maps_close(
            {k: float(v) for k, v in ours.items()},
            {k: float(v) for k, v in nx.triangles(G).items()},
            1e-12,
            f'triangles seed={seed}',
        )

    @pytest.mark.parametrize('self_loops', [False, True])
    @pytest.mark.parametrize('seed', SEEDS)
    def test_clustering_parallel_matches_networkx(self, seed, self_loops):
        g, G = build_pair(12, 0.3, seed, self_loops=self_loops)
        ours = pg.parallel.clustering_coefficients_parallel(g)
        reference = nx.clustering(G)
        assert_maps_close(ours, reference, 1e-9, f'clustering seed={seed}')

    @pytest.mark.parametrize('seed', SEEDS)
    def test_degrees_parallel_matches_networkx(self, seed):
        g, G = build_pair(12, 0.3, seed)
        ours = pg.parallel.degrees_parallel(g)
        for node, deg in G.degree():
            assert ours[node] == deg, f'degree mismatch at {node}, seed={seed}'

    @pytest.mark.parametrize('seed', SEEDS)
    def test_components_parallel_matches_networkx(self, seed):
        g, G = build_pair(14, 0.08, seed)
        component_map = pg.parallel.connected_components_parallel(g)
        by_id = {}
        for node, cid in component_map.items():
            by_id.setdefault(cid, set()).add(node)
        ours = {frozenset(c) for c in by_id.values()}
        reference = {frozenset(c) for c in nx.connected_components(G)}
        assert ours == reference, f'parallel components differ, seed={seed}'

    @pytest.mark.parametrize('seed', SEEDS)
    def test_shortest_paths_parallel_matches_networkx_hops(self, seed):
        g, G = build_pair(12, 0.3, seed, weighted=True)
        sources = [0, 5, 11]
        results = pg.parallel.shortest_paths_parallel(g, sources)
        for source, dist_map in zip(sources, results):
            reference = nx.single_source_shortest_path_length(G, source)
            for node, dist in dist_map.items():
                if dist is None:
                    assert node not in reference, (
                        f'node {node} reachable in nx from {source}, seed={seed}'
                    )
                else:
                    assert dist == reference[node], (
                        f'hop mismatch from {source} at {node}, seed={seed}'
                    )


if __name__ == '__main__':
    pytest.main([__file__, '-v'])
