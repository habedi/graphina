"""
Type stubs for PyGraphina - Python bindings for Graphina graph library.

This file provides type hints for IDEs and type checkers.
"""

from typing import Optional, Dict, List, Tuple, Union, Any, Iterator


class PyGraph:
    """
    A Python-accessible Graph class wrapping Graphina's core undirected graph.

    This class uses i64 as the node attribute type and f64 as the edge weight type.
    Internally, it maintains a mapping from Python-assigned node IDs (simple usize values)
    to the Graphina NodeIds.
    """

    def __init__(self) -> None:
        """Creates a new, empty graph."""
        ...

    def add_node(self, attr: int) -> int:
        """
        Add a node with an integer attribute to the graph.

        Args:
            attr: The attribute value for the node (must be in range -2^63 to 2^63-1)

        Returns:
            The node ID (non-negative integer, starts from 0)
        """
        ...

    def update_node(self, node: int, new_attr: int) -> bool:
        """
        Update the attribute of an existing node.

        Args:
            node: The node ID
            new_attr: The new attribute value

        Returns:
            True if the node exists and was updated, False otherwise
        """
        ...

    def try_update_node(self, node: int, new_attr: int) -> None:
        """
        Update a node's attribute, raising an error if the node doesn't exist.

        Args:
            node: The node ID
            new_attr: The new attribute value

        Raises:
            ValueError: If the node doesn't exist
        """
        ...

    def add_edge(self, source: int, target: int, weight: float) -> int:
        """
        Add an edge between two nodes with a weight.

        Args:
            source: The source node ID
            target: The target node ID
            weight: The edge weight (must be finite, not NaN or infinity)

        Returns:
            The edge ID

        Raises:
            ValueError: If either node doesn't exist or weight is not finite
        """
        ...

    def remove_node(self, node: int) -> Optional[int]:
        """
        Remove a node from the graph.

        Removes all edges incident to this node as well.

        Args:
            node: The node ID to remove

        Returns:
            The node's attribute if it existed, None otherwise
        """
        ...

    def try_remove_node(self, node: int) -> int:
        """
        Remove a node from the graph, raising an error if it doesn't exist.

        Args:
            node: The node ID to remove

        Returns:
            The node's attribute value

        Raises:
            ValueError: If the node doesn't exist
        """
        ...

    def remove_edge(self, source: int, target: int) -> bool:
        """
        Remove an edge between two nodes.

        Args:
            source: The source node ID
            target: The target node ID

        Returns:
            True if the edge was removed, False if it didn't exist

        Raises:
            ValueError: If either node doesn't exist
        """
        ...

    def try_remove_edge(self, source: int, target: int) -> None:
        """
        Remove an edge, raising an error if it doesn't exist.

        Args:
            source: The source node ID
            target: The target node ID

        Raises:
            ValueError: If either node doesn't exist or the edge doesn't exist
        """
        ...

    def get_edge_weight(self, source: int, target: int) -> Optional[float]:
        """
        Get the weight of an edge.

        Args:
            source: The source node ID
            target: The target node ID

        Returns:
            The edge weight, or None if the edge doesn't exist

        Raises:
            ValueError: If either node doesn't exist
        """
        ...

    def update_edge_weight(self, source: int, target: int, new_weight: float) -> bool:
        """Update the weight of an existing edge."""
        ...

    def try_update_edge_weight(self, source: int, target: int, new_weight: float) -> None:
        """Update edge weight, raising an error if the edge doesn't exist."""
        ...

    def node_count(self) -> int:
        """Get the number of nodes in the graph."""
        ...

    def edge_count(self) -> int:
        """Get the number of edges in the graph."""
        ...

    def is_directed(self) -> bool:
        """Check if the graph is directed (always False for PyGraph)."""
        ...

    def density(self) -> float:
        """Calculate the density of the graph (ratio of actual to possible edges)."""
        ...

    def contains_node(self, node: int) -> bool:
        """Check if a node exists in the graph."""
        ...

    def contains_edge(self, source: int, target: int) -> bool:
        """Check if an edge exists between two nodes."""
        ...

    def neighbors(self, node: int) -> List[int]:
        """Get the neighbors of a node."""
        ...

    def get_node_attr(self, node: int) -> Optional[int]:
        """Get the attribute value of a node."""
        ...

    def clear(self) -> None:
        """Remove all nodes and edges from the graph."""
        ...

    def bfs(self, start: int) -> List[int]:
        """Perform breadth-first search from a starting node."""
        ...

    def dfs(self, start: int) -> List[int]:
        """Perform depth-first search from a starting node."""
        ...

    def shortest_path(self, start: int, target: int) -> Optional[Tuple[float, List[int]]]:
        """
        Find the shortest path between two nodes using Dijkstra's algorithm.

        Returns:
            A tuple of (distance, path) if path exists, None otherwise
        """
        ...

    def dijkstra(self, start: int, cutoff: Optional[float] = None) -> Dict[int, Optional[float]]:
        """Compute shortest paths from start node using Dijkstra's algorithm."""
        ...

    def subgraph(self, nodes: List[int]) -> "PyGraph":
        """Create a subgraph containing the specified nodes."""
        ...

    def induced_subgraph(self, nodes: List[int]) -> "PyGraph":
        """Create an induced subgraph containing the specified nodes."""
        ...

    @property
    def nodes(self) -> "NodeView":
        """Get a view of all nodes in the graph."""
        ...

    @property
    def degree(self) -> "DegreeView":
        """Get a view for accessing node degrees via bracket notation."""
        ...

    def __len__(self) -> int:
        """Return the number of nodes in the graph."""
        ...

    def __contains__(self, node: int) -> bool:
        """Check if a node exists in the graph."""
        ...


class PyDiGraph:
    """
    A Python-accessible DiGraph class for directed graphs.

    This class uses i64 as the node attribute type and f64 as the edge weight type.
    """

    def __init__(self) -> None:
        """Creates a new, empty directed graph."""
        ...

    def add_node(self, attr: int) -> int:
        """Add a node with an integer attribute to the directed graph."""
        ...

    def add_edge(self, source: int, target: int, weight: float) -> int:
        """Add a directed edge from source to target with a weight."""
        ...

    def remove_node(self, node: int) -> Optional[int]:
        """Remove a node and all its incident edges."""
        ...

    def remove_edge(self, source: int, target: int) -> bool:
        """Remove a directed edge from source to target."""
        ...

    def in_degree(self, node: int) -> Optional[int]:
        """Get the in-degree (number of incoming edges) of a node."""
        ...

    def out_degree(self, node: int) -> Optional[int]:
        """Get the out-degree (number of outgoing edges) of a node."""
        ...

    def in_neighbors(self, node: int) -> List[int]:
        """Get nodes that have edges pointing to this node."""
        ...

    def out_neighbors(self, node: int) -> List[int]:
        """Get nodes that this node has edges pointing to."""
        ...

    def node_count(self) -> int:
        """Get the number of nodes in the graph."""
        ...

    def edge_count(self) -> int:
        """Get the number of edges in the graph."""
        ...

    def is_directed(self) -> bool:
        """Check if the graph is directed (always True for PyDiGraph)."""
        ...

    @property
    def nodes(self) -> "NodeView":
        """Get a view of all nodes in the graph."""
        ...

    @property
    def degree(self) -> "DegreeView":
        """Get a view for accessing total node degrees (in + out)."""
        ...


class NodeView:
    """A view for iterating over graph nodes."""

    def __iter__(self) -> Iterator[int]:
        """Iterate over node IDs."""
        ...

    def __len__(self) -> int:
        """Get the number of nodes."""
        ...

    def __contains__(self, node: int) -> bool:
        """Check if a node exists."""
        ...

    def __getitem__(self, node: int) -> Dict[str, int]:
        """Get node data as a dictionary."""
        ...


class DegreeView:
    """A view for accessing node degrees."""

    def __getitem__(self, node: int) -> int:
        """Get the degree of a node."""
        ...

    def __iter__(self) -> Iterator[Tuple[int, int]]:
        """Iterate over (node, degree) pairs."""
        ...

    def __len__(self) -> int:
        """Get the number of nodes."""
        ...


class EdgeView:
    """A view for iterating over graph edges."""

    def __iter__(self) -> Iterator[Tuple[int, int, float]]:
        """Iterate over (source, target, weight) tuples."""
        ...

    def __len__(self) -> int:
        """Get the number of edges."""
        ...


# Module-level functions
def from_networkx(nx_graph: Any) -> Union[PyGraph, PyDiGraph]:
    """
    Convert a NetworkX graph to PyGraphina.

    Args:
        nx_graph: A NetworkX Graph or DiGraph object

    Returns:
        A PyGraph for undirected graphs, PyDiGraph for directed graphs
    """
    ...


def to_networkx(graph: Union[PyGraph, PyDiGraph]) -> Any:
    """
    Convert a PyGraphina graph to NetworkX.

    Args:
        graph: A PyGraph or PyDiGraph object

    Returns:
        A NetworkX Graph or DiGraph object
    """
    ...


# Submodules
class centrality:
    """Centrality algorithms module."""

    @staticmethod
    def degree(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]:
        """Compute degree centrality for all nodes."""
        ...

    @staticmethod
    def betweenness(graph: Union[PyGraph, PyDiGraph], normalized: bool = True) -> Dict[int, float]:
        """Compute betweenness centrality for all nodes."""
        ...

    @staticmethod
    def closeness(graph: Union[PyGraph, PyDiGraph], normalized: bool = True) -> Dict[int, float]:
        """Compute closeness centrality for all nodes."""
        ...

    @staticmethod
    def pagerank(
        graph: Union[PyGraph, PyDiGraph],
        damping: float = 0.85,
        max_iter: int = 100,
        tolerance: float = 1e-6,
        nstart: Optional[Dict[int, float]] = None
    ) -> Dict[int, float]:
        """Compute PageRank centrality for all nodes."""
        ...

    @staticmethod
    def eigenvector(
        graph: Union[PyGraph, PyDiGraph],
        max_iter: int = 100,
        tolerance: float = 1e-6
    ) -> Dict[int, float]:
        """Compute eigenvector centrality for all nodes."""
        ...


class community:
    """Community detection algorithms module."""

    @staticmethod
    def label_propagation(graph: PyGraph, max_iter: int, seed: Optional[int] = None) -> Dict[int, int]:
        """Detect communities using label propagation algorithm."""
        ...

    @staticmethod
    def louvain(graph: PyGraph, seed: Optional[int] = None) -> List[List[int]]:
        """Detect communities using Louvain method."""
        ...

    @staticmethod
    def connected_components(graph: PyGraph) -> List[List[int]]:
        """Find all connected components in the graph."""
        ...


class links:
    """Link prediction algorithms module."""

    @staticmethod
    def jaccard_coefficient(
        graph: PyGraph,
        ebunch: Optional[List[Tuple[int, int]]] = None
    ) -> Dict[Tuple[int, int], float]:
        """Compute Jaccard coefficient for node pairs."""
        ...

    @staticmethod
    def adamic_adar_index(
        graph: PyGraph,
        ebunch: Optional[List[Tuple[int, int]]] = None
    ) -> Dict[Tuple[int, int], float]:
        """Compute Adamic-Adar index for node pairs."""
        ...

    @staticmethod
    def preferential_attachment(graph: PyGraph) -> Dict[Tuple[int, int], float]:
        """Compute preferential attachment scores for node pairs."""
        ...

    @staticmethod
    def resource_allocation_index(
        graph: PyGraph,
        ebunch: Optional[List[Tuple[int, int]]] = None
    ) -> Dict[Tuple[int, int], float]:
        """Compute resource allocation index for node pairs."""
        ...

    @staticmethod
    def common_neighbors(graph: PyGraph, u: int, v: int) -> int:
        """Count the number of common neighbors between two nodes."""
        ...


class core:
    """Core graph generation and I/O functions."""

    @staticmethod
    def complete_graph(n: int) -> PyGraph:
        """Generate a complete graph with n nodes."""
        ...

    @staticmethod
    def erdos_renyi(n: int, p: float, seed: Optional[int] = None) -> PyGraph:
        """Generate an Erdős-Rényi random graph."""
        ...

    @staticmethod
    def barabasi_albert(n: int, m: int, seed: Optional[int] = None) -> PyGraph:
        """Generate a Barabási-Albert scale-free network."""
        ...

    @staticmethod
    def watts_strogatz(n: int, k: int, beta: float, seed: Optional[int] = None) -> PyGraph:
        """Generate a Watts-Strogatz small-world network."""
        ...


# Exception classes
class GraphinaError(Exception):
    """Base exception for Graphina errors."""
    ...
