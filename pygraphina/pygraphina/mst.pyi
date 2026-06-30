"""Minimum spanning tree algorithms.

Each function operates on an undirected graph (PyGraph) and returns a tuple
of (total_weight, edges), where edges is a list of (u, v, weight) triples.
"""

from typing import Dict, List, Optional, Tuple, Union, TYPE_CHECKING

if TYPE_CHECKING:
    from pygraphina import PyGraph, PyDiGraph


def prim_mst(graph: PyGraph) -> Tuple[float, List[Tuple[int, int, float]]]:
    """Compute the minimum spanning tree using Prim's algorithm."""
    ...


def kruskal_mst(graph: PyGraph) -> Tuple[float, List[Tuple[int, int, float]]]:
    """Compute the minimum spanning tree using Kruskal's algorithm."""
    ...


def boruvka_mst(graph: PyGraph) -> Tuple[float, List[Tuple[int, int, float]]]:
    """Compute the minimum spanning tree using Borůvka's algorithm (parallel)."""
    ...
