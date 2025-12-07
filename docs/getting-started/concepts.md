# Core Concepts

## Graphs and Digraphs

Graphina provides two main graph types:

*   **`Graph`**: Undirected graph. Edges are symmetric (A -> B implies B -> A).
*   **`Digraph`**: Directed graph. Edges have direction.

## NodeIds

Nodes are identified by `NodeId` handles returned when you add a node. This allows for efficient internal storage.
