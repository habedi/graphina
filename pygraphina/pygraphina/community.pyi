"""Community detection algorithms module."""

from typing import Dict, List, Optional, Tuple, Union, TYPE_CHECKING

if TYPE_CHECKING:
    from pygraphina import PyGraph, PyDiGraph


__all__ = [
    "connected_components",
    "label_propagation",
    "louvain",
    "girvan_newman",
    "spectral_clustering",
]


def label_propagation(py_graph: PyGraph, max_iter: int, seed: Optional[int] = None) -> Dict[int, int]:
    """Detect communities using the label propagation algorithm."""
    ...


def louvain(py_graph: PyGraph, seed: Optional[int] = None) -> List[List[int]]:
    """Detect communities using the Louvain method."""
    ...


def girvan_newman(py_graph: PyGraph, target_communities: int) -> List[List[int]]:
    """Detect communities using the Girvan-Newman algorithm."""
    ...


def spectral_clustering(py_graph: PyGraph, k: int, seed: Optional[int] = None) -> List[List[int]]:
    """Partition the graph into k clusters using spectral clustering."""
    ...


def connected_components(py_graph: PyGraph) -> List[List[int]]:
    """Find all connected components in the graph."""
    ...
