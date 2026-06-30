"""Core graph generation and I/O functions."""

from typing import Dict, List, Optional, Tuple, Union, TYPE_CHECKING

if TYPE_CHECKING:
    from pygraphina import PyGraph, PyDiGraph


__all__ = [
    "erdos_renyi",
    "complete_graph",
    "bipartite",
    "star_graph",
    "cycle_graph",
    "watts_strogatz",
    "barabasi_albert",
]


def complete_graph(n: int) -> PyGraph:
    """Generate a complete graph with n nodes."""
    ...


def cycle_graph(n: int) -> PyGraph:
    """Generate a cycle graph where the n nodes form a ring."""
    ...


def star_graph(n: int) -> PyGraph:
    """Generate a star graph with one central node connected to all others."""
    ...


def erdos_renyi(n: int, p: float, seed: int) -> PyGraph:
    """Generate an Erdős-Rényi random graph."""
    ...


def barabasi_albert(n: int, m: int, seed: int) -> PyGraph:
    """Generate a Barabási-Albert scale-free network."""
    ...


def watts_strogatz(n: int, k: int, beta: float, seed: int) -> PyGraph:
    """Generate a Watts-Strogatz small-world network."""
    ...


def bipartite(n1: int, n2: int, p: float, seed: int) -> PyGraph:
    """Generate a random bipartite graph with parts of size n1 and n2."""
    ...
