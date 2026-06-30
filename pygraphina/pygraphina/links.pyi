"""Link prediction algorithms module."""

from typing import Dict, List, Optional, Tuple, Union, TYPE_CHECKING

if TYPE_CHECKING:
    from pygraphina import PyGraph, PyDiGraph


def jaccard_coefficient(
    py_graph: PyGraph,
    ebunch: Optional[List[Tuple[int, int]]] = None
) -> Dict[Tuple[int, int], float]:
    """Compute the Jaccard coefficient for node pairs."""
    ...


def adamic_adar_index(
    py_graph: PyGraph,
    ebunch: Optional[List[Tuple[int, int]]] = None
) -> Dict[Tuple[int, int], float]:
    """Compute the Adamic-Adar index for node pairs."""
    ...


def preferential_attachment(
    py_graph: PyGraph,
    ebunch: Optional[List[Tuple[int, int]]] = None
) -> Dict[Tuple[int, int], float]:
    """Compute preferential attachment scores for node pairs."""
    ...


def resource_allocation_index(
    py_graph: PyGraph,
    ebunch: Optional[List[Tuple[int, int]]] = None
) -> Dict[Tuple[int, int], float]:
    """Compute the resource allocation index for node pairs."""
    ...


def common_neighbor_centrality(
    py_graph: PyGraph,
    alpha: float,
    ebunch: Optional[List[Tuple[int, int]]] = None
) -> Dict[Tuple[int, int], float]:
    """Compute common neighbor centrality for node pairs."""
    ...


def common_neighbors(py_graph: PyGraph, u: int, v: int) -> int:
    """Count the number of common neighbors between two nodes."""
    ...
