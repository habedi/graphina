"""Centrality algorithms module."""

from typing import Dict, List, Optional, Tuple, Union, TYPE_CHECKING

if TYPE_CHECKING:
    from pygraphina import PyGraph, PyDiGraph


def degree(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]:
    """Compute degree centrality for all nodes."""
    ...


def in_degree(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]:
    """Compute in-degree centrality for all nodes."""
    ...


def out_degree(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]:
    """Compute out-degree centrality for all nodes."""
    ...


def betweenness(graph: Union[PyGraph, PyDiGraph], normalized: bool) -> Dict[int, float]:
    """Compute shortest-path betweenness centrality for all nodes."""
    ...


def edge_betweenness(
    graph: Union[PyGraph, PyDiGraph],
    normalized: bool
) -> Dict[Tuple[int, int], float]:
    """Compute betweenness centrality for all edges."""
    ...


def closeness(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]:
    """Compute closeness centrality for all nodes."""
    ...


def harmonic(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]:
    """Compute harmonic centrality for all nodes."""
    ...


def pagerank(
    graph: Union[PyGraph, PyDiGraph],
    damping: float = 0.85,
    max_iter: int = 100,
    tolerance: float = 1e-6,
    nstart: Optional[Dict[int, float]] = None
) -> Dict[int, float]:
    """Compute PageRank centrality for all nodes."""
    ...


def personalized_pagerank(
    graph: Union[PyGraph, PyDiGraph],
    personalization: Optional[List[float]] = None,
    damping: float = 0.85,
    tolerance: float = 1e-6,
    max_iter: int = 100,
    nstart: Optional[Dict[int, float]] = None
) -> Dict[int, float]:
    """Compute personalized PageRank with an optional personalization vector."""
    ...


def eigenvector(
    graph: Union[PyGraph, PyDiGraph],
    max_iter: int,
    tolerance: float
) -> Dict[int, float]:
    """Compute eigenvector centrality for all nodes."""
    ...


def katz(
    graph: Union[PyGraph, PyDiGraph],
    alpha: float,
    max_iter: int,
    tolerance: float
) -> Dict[int, float]:
    """Compute Katz centrality for all nodes."""
    ...


def local_reaching_centrality(
    graph: Union[PyGraph, PyDiGraph],
    distance: int
) -> Dict[int, float]:
    """Compute local reaching centrality for all nodes."""
    ...


def global_reaching_centrality(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]:
    """Compute global reaching centrality for the graph."""
    ...
