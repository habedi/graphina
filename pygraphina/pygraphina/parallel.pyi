"""Parallel (Rayon-backed) graph algorithms."""

from typing import Dict, List, Optional, Tuple, Union, TYPE_CHECKING

if TYPE_CHECKING:
    from pygraphina import PyGraph, PyDiGraph


def bfs_parallel(graph: Union[PyGraph, PyDiGraph], starts: List[int]) -> List[List[int]]:
    """Run BFS from multiple starting nodes in parallel, one visit-order list per start."""
    ...


def degrees_parallel(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, int]:
    """Compute the degree of every node in parallel."""
    ...


def connected_components_parallel(graph: PyGraph) -> Dict[int, int]:
    """Compute a node-to-component mapping in parallel."""
    ...


def pagerank_parallel(
    graph: Union[PyGraph, PyDiGraph],
    damping: float = 0.85,
    max_iterations: int = 100,
    tolerance: float = 1e-6,
    nstart: Optional[Dict[int, float]] = None
) -> Dict[int, float]:
    """Compute PageRank scores in parallel."""
    ...


def triangles_parallel(graph: PyGraph) -> Dict[int, int]:
    """Count the triangles incident to every node in parallel."""
    ...


def clustering_coefficients_parallel(graph: PyGraph) -> Dict[int, float]:
    """Compute the local clustering coefficient of every node in parallel."""
    ...


def shortest_paths_parallel(
    graph: Union[PyGraph, PyDiGraph],
    sources: List[int]
) -> List[Dict[int, int]]:
    """Compute unweighted shortest path distances (hop counts) from multiple sources in parallel."""
    ...
