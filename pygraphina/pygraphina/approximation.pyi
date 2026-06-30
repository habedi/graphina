"""Approximation algorithms for NP-hard problems.

All functions operate on undirected graphs (PyGraph).
"""

from typing import Dict, List, Optional, Tuple, Union, TYPE_CHECKING

if TYPE_CHECKING:
    from pygraphina import PyGraph, PyDiGraph


def max_clique(py_graph: PyGraph) -> List[int]:
    """Find a large clique using a greedy heuristic."""
    ...


def clique_removal(py_graph: PyGraph) -> List[List[int]]:
    """Partition the graph into cliques by repeated greedy clique removal."""
    ...


def large_clique_size(py_graph: PyGraph) -> int:
    """Estimate the size of a large clique in the graph."""
    ...


def maximum_independent_set(py_graph: PyGraph) -> List[int]:
    """Find a maximal independent set using a greedy heuristic."""
    ...


def min_weighted_vertex_cover(py_graph: PyGraph) -> List[int]:
    """Find a vertex cover using a greedy 2-approximation."""
    ...


def densest_subgraph(py_graph: PyGraph) -> List[int]:
    """Find an approximately densest subgraph by greedy peeling."""
    ...


def average_clustering_approx(py_graph: PyGraph) -> float:
    """Estimate the average clustering coefficient by sampling."""
    ...


def ramsey_r2(py_graph: PyGraph) -> Tuple[List[int], List[int]]:
    """Return a clique and an independent set via the Ramsey R(2, t) heuristic."""
    ...


def local_node_connectivity(py_graph: PyGraph, source: int, target: int) -> int:
    """Approximate the local node connectivity between source and target."""
    ...


def treewidth_min_degree(py_graph: PyGraph) -> Tuple[int, List[int]]:
    """Compute a treewidth upper bound and elimination ordering using the min-degree heuristic."""
    ...


def treewidth_min_fill_in(py_graph: PyGraph) -> Tuple[int, List[int]]:
    """Compute a treewidth upper bound and elimination ordering using the min-fill-in heuristic."""
    ...
