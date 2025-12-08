# Quick Start

This tutorial will walk you through creating your first graph with PyGraphina and performing basic operations.

## Creating a Graph

Import PyGraphina and create an empty graph:

```python
import pygraphina as pg

# Create an undirected graph
g = pg.PyGraph()
```

## Adding Nodes

Nodes can be added with attributes (in PyGraphina, node attributes are integers):

```python
# Add nodes with attributes
node_a = g.add_node(100)  # attribute = 100
node_b = g.add_node(200)  # attribute = 200
node_c = g.add_node(300)  # attribute = 300
node_d = g.add_node(400)  # attribute = 400

print(f"Graph has {g.node_count()} nodes")
# Output: Graph has 4 nodes
```

The `add_node()` method returns a node ID that you can use to reference the node later.

## Adding Edges

Connect nodes with weighted edges:

```python
# Add weighted edges
g.add_edge(node_a, node_b, 1.0)  # weight = 1.0
g.add_edge(node_b, node_c, 2.0)  # weight = 2.0
g.add_edge(node_c, node_d, 1.5)  # weight = 1.5
g.add_edge(node_d, node_a, 0.5)  # weight = 0.5

print(f"Graph has {g.edge_count()} edges")
# Output: Graph has 4 edges
```

## Querying the Graph

### Check Node and Edge Existence

```python
# Check if a node exists
if g.contains_node(node_a):
    print("Node A exists!")

# Check if an edge exists
if g.contains_edge(node_a, node_b):
    print("Edge from A to B exists!")
```

### Get Node Information

```python
# Get node degree (access the degree view and index by node)
degree_view = g.degree
degree_a = degree_view[node_a]
print(f"Node A has degree {degree_a}")

# Get node neighbors
neighbors = g.neighbors(node_a)
print(f"Node A's neighbors: {neighbors}")

# Get node attribute
attr = g.get_node_attr(node_a)
print(f"Node A's attribute: {attr}")
```

### Get All Nodes

```python
# Get all nodes from the nodes view
all_nodes = g.nodes
print(f"All nodes: {list(all_nodes)}")
```

## Running Algorithms

PyGraphina provides many graph algorithms. Here are some common ones:

### PageRank

Calculate the PageRank centrality of nodes:

```python
# PageRank(graph, damping_factor, max_iterations, tolerance)
pagerank_scores = pg.centrality.pagerank(g, 0.85, 100, 1e-6)
print(f"PageRank scores: {pagerank_scores}")
```

### Shortest Path

Find the shortest path between two nodes:

```python
# Find specific path
result = g.shortest_path(node_a, node_d)
if result:
    distance, path = result
    print(f"Shortest path from A to D: {path}")
    print(f"Total distance: {distance}")
else:
    print("No path found")
```

### Community Detection

Detect communities in the graph:

```python
# Label propagation algorithm
communities = pg.community.label_propagation(g, max_iter=100)
print(f"Communities: {communities}")
```

### Link Prediction

Predict potential future links:

```python
# Jaccard coefficient for link prediction
jaccard_scores = pg.links.jaccard_coefficient(g)
print(f"Jaccard coefficients: {jaccard_scores}")
```

## Complete Example

Here's a complete example that puts it all together:

```python
import pygraphina as pg

# Create a social network graph
def create_social_network():
    g = pg.PyGraph()

    # Add people (nodes)
    alice = g.add_node(1)
    bob = g.add_node(2)
    charlie = g.add_node(3)
    david = g.add_node(4)
    eve = g.add_node(5)

    # Add friendships (edges) with interaction strength
    g.add_edge(alice, bob, 5.0)
    g.add_edge(alice, charlie, 3.0)
    g.add_edge(bob, charlie, 4.0)
    g.add_edge(charlie, david, 2.0)
    g.add_edge(david, eve, 3.0)
    g.add_edge(eve, charlie, 1.0)

    return g, {"Alice": alice, "Bob": bob, "Charlie": charlie,
               "David": david, "Eve": eve}

# Create the network
network, people = create_social_network()

print(f"Social network: {network.node_count()} people, "
      f"{network.edge_count()} friendships")

# Find the most influential person (highest PageRank)
pagerank = pg.centrality.pagerank(network, 0.85, 100, 1e-6)
most_influential = max(pagerank, key=pagerank.get)
print(f"\nMost influential person: {most_influential}")

# Find communities (friend groups)
communities = pg.community.label_propagation(network, 100)
print(f"\nFriend groups detected: {len(set(communities.values()))}")

# Predict potential new friendships
jaccard = pg.links.jaccard_coefficient(network)
print(f"\nPotential new friendships:")
for (node1, node2), score in sorted(jaccard.items(),
                                     key=lambda x: x[1],
                                     reverse=True)[:3]:
    print(f"  Nodes {node1} and {node2}: {score:.3f}")

# Graph statistics
print(f"\nNetwork statistics:")
print(f"  Density: {network.density():.3f}")
print(f"  Average degree: {sum(network.degree[n] for n in network.nodes) / network.node_count():.2f}")
```

## Working with Directed Graphs

PyGraphina also supports directed graphs:

```python
# Create a directed graph
dg = pg.PyDiGraph()

# Add nodes and directed edges
a = dg.add_node(1)
b = dg.add_node(2)
c = dg.add_node(3)

dg.add_edge(a, b, 1.0)  # a → b
dg.add_edge(b, c, 1.0)  # b → c
dg.add_edge(c, a, 1.0)  # c → a (creates a cycle)

# Check if directed
print(f"Is directed: {dg.is_directed()}")  # True

# Get in-degree and out-degree
in_deg = dg.in_degree(b)
out_deg = dg.out_degree(b)
print(f"Node b: in-degree={in_deg}, out-degree={out_deg}")
```

## Next Steps

Now that you understand the basics, explore more features:

- [Basic Concepts](concepts.md): Learn more about PyGraphina's graph model
- [API Reference](../api/graph.md): Detailed documentation of all methods
- [Examples](../examples/basic.md): More complex examples
- [Algorithms](../examples/algorithms.md): Deep dive into specific algorithms
