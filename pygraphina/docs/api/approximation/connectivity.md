# Local Node Connectivity

Local node connectivity approximates the maximum number of vertex-disjoint paths between two nodes.

## Overview

Local node connectivity between two nodes is the minimum number of other nodes that need to be removed to disconnect
them. By Menger's theorem this equals the maximum number of paths between them that share no intermediate node.

## Function Signature

```python
pg.approximation.local_node_connectivity(
    graph: PyGraph,
    source: int,
    target: int
) -> int
```

## Parameters

- graph: The undirected graph to analyze
- source: The source node ID
- target: The target node ID

## Returns

Integer approximating the number of vertex-disjoint paths from source to target: a direct edge counts once, and each
further path shares no intermediate node with the paths already found.

## Algorithm

Greedy search for vertex-disjoint paths:
- A direct source-target edge is counted first
- Repeated BFS finds a shortest remaining path and blocks its intermediate nodes
- The count of found paths is a lower bound on the true connectivity

Time Complexity: O(k · (V + E)) for k found paths  
Space Complexity: O(V + E)

## Example

```python
import pygraphina as pg

# Create a network
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(8)]

# Create a graph with varying connectivity
edges = [
    (0, 1), (0, 2),           # Node 0 connects to 1, 2
    (1, 3), (2, 3),           # Nodes 1, 2 both connect to 3
    (3, 4), (3, 5),           # Node 3 connects to 4, 5
    (4, 6), (5, 6),           # Nodes 4, 5 both connect to 6
    (6, 7),                   # Node 6 connects to 7
    (1, 2),                   # Extra edge for robustness
]

for u, v in edges:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Check connectivity between different pairs
pairs = [(0, 7), (0, 1), (1, 3), (4, 6)]

print("Node connectivity between node pairs:")
for source, target in pairs:
    conn = pg.approximation.local_node_connectivity(g, nodes[source], nodes[target])
    print(f"  {source} → {target}: {conn} vertex-disjoint paths")

# Practical example: Network reliability
# Two cities connected by a direct highway and a route through a transfer hub
# have connectivity 2: both the direct edge and the indirect path count.
g_network = pg.PyGraph()
city_a = g_network.add_node(0)
city_b = g_network.add_node(1)
hub = g_network.add_node(2)

g_network.add_edge(city_a, city_b, 1.0)  # Direct highway
g_network.add_edge(city_a, hub, 1.0)  # Route through the hub
g_network.add_edge(hub, city_b, 1.0)

conn_ab = pg.approximation.local_node_connectivity(g_network, city_a, city_b)
print(f"\nNetwork reliability (City A to City B): {conn_ab} independent routes")

# Diamond graph - multiple paths
diamond = pg.PyGraph()
d_nodes = [diamond.add_node(i) for i in range(4)]
diamond.add_edge(d_nodes[0], d_nodes[1], 1.0)
diamond.add_edge(d_nodes[0], d_nodes[2], 1.0)
diamond.add_edge(d_nodes[1], d_nodes[3], 1.0)
diamond.add_edge(d_nodes[2], d_nodes[3], 1.0)

conn_diamond = pg.approximation.local_node_connectivity(diamond, d_nodes[0], d_nodes[3])
print(f"Connectivity in diamond graph (0 → 3): {conn_diamond} paths")
```

## Interpretation

Result Values:
- 0: Nodes are disconnected (no path exists)
- 1: Nodes are connected by a single edge or bridge
- k ≥ 2: Multiple disjoint paths exist between nodes

## Time Complexity

| Scenario | Complexity |
|----------|-----------|
| Single pair | O(V · E) |
| All pairs | O(V³) |

## Space Complexity

O(V + E) for storing the graph and flow network.

## Use Cases

- Network Reliability: How robust is the connection?
- Infrastructure Planning: How many independent routes exist?
- Bottleneck Detection: Which nodes are critical?
- Network Design: Where to add redundant links?
- Communication Networks: Failure tolerance assessment
- Supply Chain: Number of independent suppliers

## Applications

| Application | Meaning |
|-------------|---------|
| Transportation | Number of independent routes between cities |
| Communication | Number of independent channels between nodes |
| Power Grid | Number of independent power lines between stations |
| Social Networks | Strength of connection between influencers |
| Supply Chain | Redundancy in supplier networks |

## Related Concepts

- Global Node Connectivity: Minimum connectivity over all pairs
- Edge Connectivity: Same as local node connectivity
- Vertex Connectivity: Minimum vertices to remove to disconnect
- Max Flow: Theoretical basis (max-flow min-cut theorem)
- Robustness: Network resilience measure

## Algorithm Details

Based on Max-Flow Min-Cut Theorem:
1. Set all edge capacities to 1
2. Compute maximum flow from source to target
3. Result = maximum flow value = minimum cut size

## Handling Edge Cases

```python
# Same node
conn = pg.approximation.local_node_connectivity(g, nodes[0], nodes[0])
# Returns: 0 - there is no path from a node to itself

# Disconnected nodes
isolated = g.add_node(99)
conn = pg.approximation.local_node_connectivity(g, nodes[0], isolated)
# Returns: 0 - no path exists
```

## Performance Notes

- Very efficient for sparse graphs
- Scales well to medium-size networks (up to thousands of nodes)
- For dense graphs, consider caching results
- Computing all-pairs connectivity requires O(V³) time
