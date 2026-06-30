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

    def update_node(self, py_node: int, new_attr: int) -> bool:
        """
        Update the attribute of an existing node.

        Args:
            node: The node ID
            new_attr: The new attribute value

        Returns:
            True if the node exists and was updated, False otherwise
        """
        ...

    def try_update_node(self, py_node: int, new_attr: int) -> None:
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

    def remove_node(self, py_node: int) -> Optional[int]:
        """
        Remove a node from the graph.

        Removes all edges incident to this node as well.

        Args:
            node: The node ID to remove

        Returns:
            The node's attribute if it existed, None otherwise
        """
        ...

    def try_remove_node(self, py_node: int) -> int:
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

    def contains_node(self, py_node: int) -> bool:
        """Check if a node exists in the graph."""
        ...

    def contains_edge(self, source: int, target: int) -> bool:
        """Check if an edge exists between two nodes."""
        ...

    def neighbors(self, py_node: int) -> List[int]:
        """Get the neighbors of a node."""
        ...

    def get_node_attr(self, py_node: int) -> Optional[int]:
        """Get the attribute value of a node."""
        ...

    def clear(self) -> None:
        """Remove all nodes and edges from the graph."""
        ...

    def add_nodes_from(self, attrs: List[int]) -> List[int]:
        """Add several nodes at once and return their node IDs in order."""
        ...

    def add_edges_from(self, edges: List[Tuple[int, int, Optional[float]]]) -> List[int]:
        """Add several edges at once as (source, target, weight) triples (weight defaults to 1.0 if None); returns the edge IDs."""
        ...

    def nodes_with_attrs(self) -> List[Tuple[int, int]]:
        """Return a list of (node_id, attr) pairs for every node."""
        ...

    def is_empty(self) -> bool:
        """Check whether the graph has no nodes."""
        ...

    def is_connected(self) -> bool:
        """Check whether the graph is connected."""
        ...

    def is_bipartite(self) -> bool:
        """Check whether the graph is bipartite."""
        ...

    def has_negative_weights(self) -> bool:
        """Check whether any edge has a negative weight."""
        ...

    def has_self_loops(self) -> bool:
        """Check whether any node has an edge to itself."""
        ...

    def count_components(self) -> int:
        """Count the number of connected components."""
        ...

    def bellman_ford(self, start: int) -> Optional[Dict[int, Optional[float]]]:
        """Compute shortest-path distances from start using Bellman-Ford, mapping each node to its distance (None if unreachable). Returns None on a negative cycle."""
        ...

    def floyd_warshall(self) -> Optional[Dict[int, Dict[int, Optional[float]]]]:
        """Compute all-pairs shortest-path distances using Floyd-Warshall. Returns None on a negative cycle."""
        ...

    def save_json(self, path: str) -> None:
        """Serialize the graph to JSON at the given path."""
        ...

    def load_json(self, path: str) -> None:
        """Load the graph from a JSON file at the given path."""
        ...

    def save_binary(self, path: str) -> None:
        """Serialize the graph to a binary file at the given path."""
        ...

    def load_binary(self, path: str) -> None:
        """Load the graph from a binary file at the given path."""
        ...

    def save_graphml(self, path: str) -> None:
        """Serialize the graph to GraphML at the given path."""
        ...

    def save_edge_list(self, path: str, sep: str = " ") -> None:
        """Write the graph as an edge list to the given path, using sep as the field separator."""
        ...

    def load_edge_list(self, path: str, sep: str = " ") -> Tuple[int, int]:
        """Load the graph from an edge list at the given path; returns the (node_count, edge_count) read."""
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

    def ego_graph(self, center: int, radius: int) -> "PyGraph":
        """Extract the ego graph centered at a node within a given radius."""
        ...

    def k_hop_neighbors(self, start: int, k: int) -> List[int]:
        """Get all node IDs within k hops of the start node (k of 0 returns just the start)."""
        ...

    def connected_component(self, start: int) -> List[int]:
        """Get the node IDs in the connected component containing the start node."""
        ...

    def component_subgraph(self, start: int) -> "PyGraph":
        """Extract the subgraph for the connected component containing the start node."""
        ...

    def filter_nodes(self, predicate: Any) -> "PyGraph":
        """Return a new graph keeping only nodes for which predicate(node_id, attr) is true."""
        ...

    def filter_edges(self, predicate: Any) -> "PyGraph":
        """Return a new graph keeping only edges for which predicate(u, v, weight) is true."""
        ...

    def iddfs(self, start: int, target: int, max_depth: int) -> Optional[List[int]]:
        """Find a path from start to target using iterative deepening DFS, or None if none exists within max_depth."""
        ...

    def try_iddfs(self, start: int, target: int, max_depth: int) -> List[int]:
        """Like iddfs, but raise an error if no path exists within max_depth."""
        ...

    def bidirectional_search(self, start: int, target: int) -> Optional[List[int]]:
        """Find the unweighted shortest path from start to target using bidirectional BFS, or None if none exists."""
        ...

    def try_bidirectional_search(self, start: int, target: int) -> List[int]:
        """Like bidirectional_search, but raise an error if no path exists."""
        ...

    def diameter(self) -> Optional[int]:
        """Compute the diameter (longest shortest path). None if the graph is empty or disconnected."""
        ...

    def radius(self) -> Optional[int]:
        """Compute the radius (minimum eccentricity). None if the graph is empty or disconnected."""
        ...

    def transitivity(self) -> float:
        """Compute the transitivity (global clustering coefficient)."""
        ...

    def average_clustering(self) -> float:
        """Compute the average clustering coefficient over all nodes."""
        ...

    def clustering_of(self, py_node: int) -> float:
        """Compute the local clustering coefficient for a node."""
        ...

    def triangles_of(self, py_node: int) -> int:
        """Count the triangles containing a node."""
        ...

    def average_path_length(self) -> Optional[float]:
        """Compute the average shortest path length. None if the graph is empty or disconnected."""
        ...

    def assortativity(self) -> float:
        """Compute the degree assortativity coefficient."""
        ...

    @property
    def nodes(self) -> "NodeView":
        """Get a view of all nodes in the graph."""
        ...

    @property
    def degree(self) -> "DegreeView":
        """Get a view for accessing node degrees via bracket notation."""
        ...

    @property
    def edges(self) -> "EdgeView":
        """Get a view of all edges in the graph as (source, target, weight) tuples."""
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

    def remove_node(self, py_node: int) -> Optional[int]:
        """Remove a node and all its incident edges."""
        ...

    def remove_edge(self, source: int, target: int) -> bool:
        """Remove a directed edge from source to target."""
        ...

    def in_degree(self, py_node: int) -> Optional[int]:
        """Get the in-degree (number of incoming edges) of a node."""
        ...

    def out_degree(self, py_node: int) -> Optional[int]:
        """Get the out-degree (number of outgoing edges) of a node."""
        ...

    def in_neighbors(self, py_node: int) -> List[int]:
        """Get nodes that have edges pointing to this node."""
        ...

    def out_neighbors(self, py_node: int) -> List[int]:
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

    def update_node(self, py_node: int, new_attr: int) -> bool:
        """Update the attribute of an existing node. Returns True if the node exists."""
        ...

    def try_update_node(self, py_node: int, new_attr: int) -> None:
        """Update a node's attribute, raising an error if the node doesn't exist."""
        ...

    def try_remove_node(self, py_node: int) -> int:
        """Remove a node, raising an error if it doesn't exist; returns its attribute."""
        ...

    def try_remove_edge(self, source: int, target: int) -> None:
        """Remove an edge, raising an error if it doesn't exist."""
        ...

    def update_edge_weight(self, source: int, target: int, new_weight: float) -> bool:
        """Update the weight of an existing edge. Returns True if the edge exists."""
        ...

    def try_update_edge_weight(self, source: int, target: int, new_weight: float) -> None:
        """Update edge weight, raising an error if the edge doesn't exist."""
        ...

    def get_edge_weight(self, source: int, target: int) -> Optional[float]:
        """Get the weight of an edge, or None if it doesn't exist."""
        ...

    def get_node_attr(self, py_node: int) -> Optional[int]:
        """Get the attribute value of a node."""
        ...

    def contains_node(self, py_node: int) -> bool:
        """Check if a node exists in the graph."""
        ...

    def contains_edge(self, source: int, target: int) -> bool:
        """Check if a directed edge exists from source to target."""
        ...

    def neighbors(self, py_node: int) -> List[int]:
        """Get the neighbors of a node."""
        ...

    def density(self) -> float:
        """Calculate the density of the graph."""
        ...

    def clear(self) -> None:
        """Remove all nodes and edges from the graph."""
        ...

    def add_nodes_from(self, attrs: List[int]) -> List[int]:
        """Add several nodes at once and return their node IDs in order."""
        ...

    def add_edges_from(self, edges: List[Tuple[int, int, Optional[float]]]) -> List[int]:
        """Add several edges at once as (source, target, weight) triples (weight defaults to 1.0 if None); returns the edge IDs."""
        ...

    def is_empty(self) -> bool:
        """Check whether the graph has no nodes."""
        ...

    def is_connected(self) -> bool:
        """Check whether the graph is connected."""
        ...

    def is_bipartite(self) -> bool:
        """Check whether the graph is bipartite."""
        ...

    def has_negative_weights(self) -> bool:
        """Check whether any edge has a negative weight."""
        ...

    def has_self_loops(self) -> bool:
        """Check whether any node has an edge to itself."""
        ...

    def count_components(self) -> int:
        """Count the number of weakly connected components."""
        ...

    def dijkstra(self, start: int, cutoff: Optional[float] = None) -> Dict[int, Optional[float]]:
        """Compute shortest paths from start using Dijkstra's algorithm, mapping each node to its distance (None if unreachable)."""
        ...

    def shortest_path(self, start: int, target: int) -> Optional[Tuple[float, List[int]]]:
        """Find the shortest path between two nodes using Dijkstra's algorithm. Returns (distance, path) or None."""
        ...

    def bellman_ford(self, start: int) -> Optional[Dict[int, Optional[float]]]:
        """Compute shortest-path distances from start using Bellman-Ford, mapping each node to its distance (None if unreachable). Returns None on a negative cycle."""
        ...

    def floyd_warshall(self) -> Optional[Dict[int, Dict[int, Optional[float]]]]:
        """Compute all-pairs shortest-path distances using Floyd-Warshall. Returns None on a negative cycle."""
        ...

    def save_json(self, path: str) -> None:
        """Serialize the graph to JSON at the given path."""
        ...

    def load_json(self, path: str) -> None:
        """Load the graph from a JSON file at the given path."""
        ...

    def save_binary(self, path: str) -> None:
        """Serialize the graph to a binary file at the given path."""
        ...

    def load_binary(self, path: str) -> None:
        """Load the graph from a binary file at the given path."""
        ...

    def save_graphml(self, path: str) -> None:
        """Serialize the graph to GraphML at the given path."""
        ...

    def save_edge_list(self, path: str, sep: str = " ") -> None:
        """Write the graph as an edge list to the given path, using sep as the field separator."""
        ...

    def load_edge_list(self, path: str, sep: str = " ") -> Tuple[int, int]:
        """Load the graph from an edge list at the given path; returns the (node_count, edge_count) read."""
        ...

    def bfs(self, start: int) -> List[int]:
        """Perform breadth-first search from a starting node."""
        ...

    def dfs(self, start: int) -> List[int]:
        """Perform depth-first search from a starting node."""
        ...

    def iddfs(self, start: int, target: int, max_depth: int) -> Optional[List[int]]:
        """Find a path from start to target using iterative deepening DFS, or None if none exists within max_depth."""
        ...

    def try_iddfs(self, start: int, target: int, max_depth: int) -> List[int]:
        """Like iddfs, but raise an error if no path exists within max_depth."""
        ...

    def bidirectional_search(self, start: int, target: int) -> Optional[List[int]]:
        """Find the unweighted shortest path from start to target using bidirectional BFS, or None if none exists."""
        ...

    def try_bidirectional_search(self, start: int, target: int) -> List[int]:
        """Like bidirectional_search, but raise an error if no path exists."""
        ...

    def subgraph(self, nodes: List[int]) -> "PyDiGraph":
        """Create a subgraph containing the specified nodes."""
        ...

    def induced_subgraph(self, nodes: List[int]) -> "PyDiGraph":
        """Create an induced subgraph containing the specified nodes."""
        ...

    def ego_graph(self, center: int, radius: int) -> "PyDiGraph":
        """Extract the ego graph centered at a node within a given radius."""
        ...

    def k_hop_neighbors(self, start: int, k: int) -> List[int]:
        """Get all node IDs within k hops of the start node (k of 0 returns just the start)."""
        ...

    def connected_component(self, start: int) -> List[int]:
        """Get the node IDs in the weakly connected component containing the start node."""
        ...

    def component_subgraph(self, start: int) -> "PyDiGraph":
        """Extract the subgraph for the weakly connected component containing the start node."""
        ...

    def filter_nodes(self, predicate: Any) -> "PyDiGraph":
        """Return a new graph keeping only nodes for which predicate(node_id, attr) is true."""
        ...

    def filter_edges(self, predicate: Any) -> "PyDiGraph":
        """Return a new graph keeping only edges for which predicate(u, v, weight) is true."""
        ...

    def diameter(self) -> Optional[int]:
        """Compute the diameter (longest shortest path). None if the graph is empty or disconnected."""
        ...

    def radius(self) -> Optional[int]:
        """Compute the radius (minimum eccentricity). None if the graph is empty or disconnected."""
        ...

    def transitivity(self) -> float:
        """Compute the transitivity (global clustering coefficient)."""
        ...

    def average_clustering(self) -> float:
        """Compute the average clustering coefficient over all nodes."""
        ...

    def clustering_of(self, py_node: int) -> float:
        """Compute the local clustering coefficient for a node."""
        ...

    def triangles_of(self, py_node: int) -> int:
        """Count the triangles containing a node."""
        ...

    def average_path_length(self) -> Optional[float]:
        """Compute the average shortest path length. None if the graph is empty or disconnected."""
        ...

    def assortativity(self) -> float:
        """Compute the degree assortativity coefficient."""
        ...

    @property
    def nodes(self) -> "NodeView":
        """Get a view of all nodes in the graph."""
        ...

    @property
    def degree(self) -> "DegreeView":
        """Get a view for accessing total node degrees (in + out)."""
        ...

    @property
    def edges(self) -> "EdgeView":
        """Get a view of all edges in the graph as (source, target, weight) tuples."""
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


def to_networkx(obj: Union[PyGraph, PyDiGraph]) -> Any:
    """
    Convert a PyGraphina graph to NetworkX.

    Args:
        graph: A PyGraph or PyDiGraph object

    Returns:
        A NetworkX Graph or DiGraph object
    """
    ...


def to_node_dataframe(obj: Union[PyGraph, PyDiGraph]) -> Any:
    """Convert a graph's nodes to a pandas DataFrame with columns 'node_id' and 'attr'."""
    ...


def to_edge_dataframe(obj: Union[PyGraph, PyDiGraph]) -> Any:
    """Convert a graph's edges to a pandas DataFrame with columns 'source', 'target', and 'weight'."""
    ...


# Graph generators (also available under the core submodule)
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


# Metrics convenience functions (also available as PyGraph/PyDiGraph methods)
def diameter(graph: PyGraph) -> Optional[int]:
    """Compute the diameter of the graph. None if the graph is empty or disconnected."""
    ...


def radius(graph: PyGraph) -> Optional[int]:
    """Compute the radius of the graph. None if the graph is empty or disconnected."""
    ...


def transitivity(graph: PyGraph) -> float:
    """Compute the transitivity (global clustering coefficient) of the graph."""
    ...


def average_clustering(graph: PyGraph) -> float:
    """Compute the average clustering coefficient of the graph."""
    ...


# Minimum spanning tree functions
def prim_mst(graph: PyGraph) -> Tuple[float, List[Tuple[int, int, float]]]:
    """Compute the minimum spanning tree using Prim's algorithm. Returns (total_weight, edges)."""
    ...


def kruskal_mst(graph: PyGraph) -> Tuple[float, List[Tuple[int, int, float]]]:
    """Compute the minimum spanning tree using Kruskal's algorithm. Returns (total_weight, edges)."""
    ...


def boruvka_mst(graph: PyGraph) -> Tuple[float, List[Tuple[int, int, float]]]:
    """Compute the minimum spanning tree using Borůvka's algorithm (parallel). Returns (total_weight, edges)."""
    ...


# Parallel algorithm functions
def bfs_parallel(graph: Union[PyGraph, PyDiGraph], starts: List[int]) -> List[List[int]]:
    """Run BFS from multiple starting nodes in parallel, one visit-order list per start."""
    ...


def degrees_parallel(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, int]:
    """Compute the degree of every node in parallel."""
    ...


def connected_components_parallel(graph: PyGraph) -> Dict[int, int]:
    """Compute a node-to-component mapping in parallel."""
    ...


# Approximation functions
def max_clique(py_graph: PyGraph) -> List[int]:
    """Find a large clique using a greedy heuristic."""
    ...


def clique_removal(py_graph: PyGraph) -> List[List[int]]:
    """Partition the graph into cliques by repeated greedy clique removal."""
    ...


def large_clique_size(py_graph: PyGraph) -> int:
    """Estimate the size of a large clique in the graph."""
    ...


def min_weighted_vertex_cover(py_graph: PyGraph) -> List[int]:
    """Find a vertex cover using a greedy 2-approximation."""
    ...


def average_clustering_approx(py_graph: PyGraph) -> float:
    """Estimate the average clustering coefficient by sampling."""
    ...


def ramsey_r2(py_graph: PyGraph) -> Tuple[List[int], List[int]]:
    """Return a clique and an independent set via the Ramsey R(2, t) heuristic."""
    ...


# Submodules
class centrality:
    """Centrality algorithms module."""

    @staticmethod
    def degree(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]:
        """Compute degree centrality for all nodes."""
        ...

    @staticmethod
    def in_degree(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]:
        """Compute in-degree centrality for all nodes."""
        ...

    @staticmethod
    def out_degree(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]:
        """Compute out-degree centrality for all nodes."""
        ...

    @staticmethod
    def betweenness(graph: Union[PyGraph, PyDiGraph], normalized: bool) -> Dict[int, float]:
        """Compute shortest-path betweenness centrality for all nodes."""
        ...

    @staticmethod
    def edge_betweenness(
        graph: Union[PyGraph, PyDiGraph],
        normalized: bool
    ) -> Dict[Tuple[int, int], float]:
        """Compute betweenness centrality for all edges."""
        ...

    @staticmethod
    def closeness(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]:
        """Compute closeness centrality for all nodes."""
        ...

    @staticmethod
    def harmonic(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]:
        """Compute harmonic centrality for all nodes."""
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

    @staticmethod
    def eigenvector(
        graph: Union[PyGraph, PyDiGraph],
        max_iter: int,
        tolerance: float
    ) -> Dict[int, float]:
        """Compute eigenvector centrality for all nodes."""
        ...

    @staticmethod
    def katz(
        graph: Union[PyGraph, PyDiGraph],
        alpha: float,
        max_iter: int,
        tolerance: float
    ) -> Dict[int, float]:
        """Compute Katz centrality for all nodes."""
        ...

    @staticmethod
    def local_reaching_centrality(
        graph: Union[PyGraph, PyDiGraph],
        distance: int
    ) -> Dict[int, float]:
        """Compute local reaching centrality for all nodes."""
        ...

    @staticmethod
    def global_reaching_centrality(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]:
        """Compute global reaching centrality for the graph."""
        ...


class community:
    """Community detection algorithms module."""

    @staticmethod
    def label_propagation(py_graph: PyGraph, max_iter: int, seed: Optional[int] = None) -> Dict[int, int]:
        """Detect communities using the label propagation algorithm."""
        ...

    @staticmethod
    def louvain(py_graph: PyGraph, seed: Optional[int] = None) -> List[List[int]]:
        """Detect communities using the Louvain method."""
        ...

    @staticmethod
    def girvan_newman(py_graph: PyGraph, target_communities: int) -> List[List[int]]:
        """Detect communities using the Girvan-Newman algorithm."""
        ...

    @staticmethod
    def spectral_clustering(py_graph: PyGraph, k: int, seed: Optional[int] = None) -> List[List[int]]:
        """Partition the graph into k clusters using spectral clustering."""
        ...

    @staticmethod
    def connected_components(py_graph: PyGraph) -> List[List[int]]:
        """Find all connected components in the graph."""
        ...


class links:
    """Link prediction algorithms module."""

    @staticmethod
    def jaccard_coefficient(
        py_graph: PyGraph,
        ebunch: Optional[List[Tuple[int, int]]] = None
    ) -> Dict[Tuple[int, int], float]:
        """Compute the Jaccard coefficient for node pairs."""
        ...

    @staticmethod
    def adamic_adar_index(
        py_graph: PyGraph,
        ebunch: Optional[List[Tuple[int, int]]] = None
    ) -> Dict[Tuple[int, int], float]:
        """Compute the Adamic-Adar index for node pairs."""
        ...

    @staticmethod
    def preferential_attachment(
        py_graph: PyGraph,
        ebunch: Optional[List[Tuple[int, int]]] = None
    ) -> Dict[Tuple[int, int], float]:
        """Compute preferential attachment scores for node pairs."""
        ...

    @staticmethod
    def resource_allocation_index(
        py_graph: PyGraph,
        ebunch: Optional[List[Tuple[int, int]]] = None
    ) -> Dict[Tuple[int, int], float]:
        """Compute the resource allocation index for node pairs."""
        ...

    @staticmethod
    def common_neighbor_centrality(
        py_graph: PyGraph,
        alpha: float,
        ebunch: Optional[List[Tuple[int, int]]] = None
    ) -> Dict[Tuple[int, int], float]:
        """Compute common neighbor centrality for node pairs."""
        ...

    @staticmethod
    def common_neighbors(py_graph: PyGraph, u: int, v: int) -> int:
        """Count the number of common neighbors between two nodes."""
        ...


class core:
    """Core graph generation and I/O functions."""

    @staticmethod
    def complete_graph(n: int) -> PyGraph:
        """Generate a complete graph with n nodes."""
        ...

    @staticmethod
    def cycle_graph(n: int) -> PyGraph:
        """Generate a cycle graph where the n nodes form a ring."""
        ...

    @staticmethod
    def star_graph(n: int) -> PyGraph:
        """Generate a star graph with one central node connected to all others."""
        ...

    @staticmethod
    def erdos_renyi(n: int, p: float, seed: int) -> PyGraph:
        """Generate an Erdős-Rényi random graph."""
        ...

    @staticmethod
    def barabasi_albert(n: int, m: int, seed: int) -> PyGraph:
        """Generate a Barabási-Albert scale-free network."""
        ...

    @staticmethod
    def watts_strogatz(n: int, k: int, beta: float, seed: int) -> PyGraph:
        """Generate a Watts-Strogatz small-world network."""
        ...

    @staticmethod
    def bipartite(n1: int, n2: int, p: float, seed: int) -> PyGraph:
        """Generate a random bipartite graph with parts of size n1 and n2."""
        ...


class approximation:
    """Approximation algorithms for NP-hard problems.

    All functions operate on undirected graphs (PyGraph).
    """

    @staticmethod
    def max_clique(py_graph: PyGraph) -> List[int]:
        """Find a large clique using a greedy heuristic."""
        ...

    @staticmethod
    def clique_removal(py_graph: PyGraph) -> List[List[int]]:
        """Partition the graph into cliques by repeated greedy clique removal."""
        ...

    @staticmethod
    def large_clique_size(py_graph: PyGraph) -> int:
        """Estimate the size of a large clique in the graph."""
        ...

    @staticmethod
    def maximum_independent_set(py_graph: PyGraph) -> List[int]:
        """Find a maximal independent set using a greedy heuristic."""
        ...

    @staticmethod
    def min_weighted_vertex_cover(py_graph: PyGraph) -> List[int]:
        """Find a vertex cover using a greedy 2-approximation."""
        ...

    @staticmethod
    def densest_subgraph(py_graph: PyGraph) -> List[int]:
        """Find an approximately densest subgraph by greedy peeling."""
        ...

    @staticmethod
    def average_clustering_approx(py_graph: PyGraph) -> float:
        """Estimate the average clustering coefficient by sampling."""
        ...

    @staticmethod
    def ramsey_r2(py_graph: PyGraph) -> Tuple[List[int], List[int]]:
        """Return a clique and an independent set via the Ramsey R(2, t) heuristic."""
        ...

    @staticmethod
    def local_node_connectivity(py_graph: PyGraph, source: int, target: int) -> int:
        """Approximate the local node connectivity between source and target."""
        ...

    @staticmethod
    def treewidth_min_degree(py_graph: PyGraph) -> Tuple[int, List[int]]:
        """Compute a treewidth upper bound and elimination ordering using the min-degree heuristic."""
        ...

    @staticmethod
    def treewidth_min_fill_in(py_graph: PyGraph) -> Tuple[int, List[int]]:
        """Compute a treewidth upper bound and elimination ordering using the min-fill-in heuristic."""
        ...


class mst:
    """Minimum spanning tree algorithms.

    Each function operates on an undirected graph (PyGraph) and returns a tuple
    of (total_weight, edges), where edges is a list of (u, v, weight) triples.
    """

    @staticmethod
    def prim_mst(graph: PyGraph) -> Tuple[float, List[Tuple[int, int, float]]]:
        """Compute the minimum spanning tree using Prim's algorithm."""
        ...

    @staticmethod
    def kruskal_mst(graph: PyGraph) -> Tuple[float, List[Tuple[int, int, float]]]:
        """Compute the minimum spanning tree using Kruskal's algorithm."""
        ...

    @staticmethod
    def boruvka_mst(graph: PyGraph) -> Tuple[float, List[Tuple[int, int, float]]]:
        """Compute the minimum spanning tree using Borůvka's algorithm (parallel)."""
        ...


class parallel:
    """Parallel (Rayon-backed) graph algorithms."""

    @staticmethod
    def bfs_parallel(graph: Union[PyGraph, PyDiGraph], starts: List[int]) -> List[List[int]]:
        """Run BFS from multiple starting nodes in parallel, one visit-order list per start."""
        ...

    @staticmethod
    def degrees_parallel(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, int]:
        """Compute the degree of every node in parallel."""
        ...

    @staticmethod
    def connected_components_parallel(graph: PyGraph) -> Dict[int, int]:
        """Compute a node-to-component mapping in parallel."""
        ...

    @staticmethod
    def pagerank_parallel(
        graph: Union[PyGraph, PyDiGraph],
        damping: float = 0.85,
        max_iterations: int = 100,
        tolerance: float = 1e-6,
        nstart: Optional[Dict[int, float]] = None
    ) -> Dict[int, float]:
        """Compute PageRank scores in parallel."""
        ...

    @staticmethod
    def triangles_parallel(graph: PyGraph) -> Dict[int, int]:
        """Count the triangles incident to every node in parallel."""
        ...

    @staticmethod
    def clustering_coefficients_parallel(graph: PyGraph) -> Dict[int, float]:
        """Compute the local clustering coefficient of every node in parallel."""
        ...

    @staticmethod
    def shortest_paths_parallel(
        graph: Union[PyGraph, PyDiGraph],
        sources: List[int]
    ) -> List[Dict[int, int]]:
        """Compute unweighted shortest path distances (hop counts) from multiple sources in parallel."""
        ...


class metrics:
    """Graph metrics namespace.

    The metric functions are exposed as methods on PyGraph and PyDiGraph (for
    example, ``graph.diameter()`` and ``graph.transitivity()``). The graph-level
    metrics ``diameter``, ``radius``, ``transitivity``, and ``average_clustering``
    are also available as top-level functions. This module is a namespace
    placeholder reserved for future standalone functions.
    """


class traversal:
    """Graph traversal namespace.

    The traversal algorithms are exposed as methods on PyGraph and PyDiGraph
    (for example, ``graph.bfs(start)``, ``graph.dfs(start)``, ``graph.iddfs(...)``,
    and ``graph.bidirectional_search(...)``). This module is a namespace
    placeholder reserved for future standalone functions.
    """


class subgraphs:
    """Subgraph extraction namespace.

    The subgraph operations are exposed as methods on PyGraph and PyDiGraph (for
    example, ``graph.subgraph(...)``, ``graph.ego_graph(...)``,
    ``graph.k_hop_neighbors(...)``, and ``graph.connected_component(...)``). This
    module is a namespace placeholder reserved for future standalone functions.
    """


# Type aliases (Pythonic names without the "Py" prefix)
Graph = PyGraph
DiGraph = PyDiGraph


# Exception classes
class GraphinaError(Exception):
    """Base exception for Graphina errors."""
    ...


class ConvergenceError(GraphinaError):
    """Raised when an iterative algorithm fails to converge."""
    ...


class NodeNotFoundError(GraphinaError):
    """Raised when a referenced node does not exist in the graph."""
    ...
