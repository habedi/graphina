# Basic Usage Examples

This page provides practical examples of common PyGraphina operations.

## Example 1: Building a Simple Graph

```python
import pygraphina as pg

# Create an undirected graph
g = pg.PyGraph()

# Add nodes with attributes
alice = g.add_node(1)
bob = g.add_node(2)
charlie = g.add_node(3)
david = g.add_node(4)

# Add weighted edges
g.add_edge(alice, bob, 1.0)
g.add_edge(bob, charlie, 2.0)
g.add_edge(charlie, david, 1.5)
g.add_edge(alice, charlie, 3.0)

# Print graph information
print(f"Nodes: {g.node_count()}")
print(f"Edges: {g.edge_count()}")
print(f"Density: {g.density():.3f}")
print(f"Is directed: {g.is_directed()}")
```

## Example 2: Analyzing a Social Network

```python
import pygraphina as pg

# Create a friendship network
friends = pg.PyGraph()

# Add people
people = {
    "Alice": friends.add_node(1),
    "Bob": friends.add_node(2),
    "Charlie": friends.add_node(3),
    "David": friends.add_node(4),
    "Eve": friends.add_node(5),
    "Frank": friends.add_node(6),
}

# Add friendships (equal strength = 1.0)
friendships = [
    ("Alice", "Bob"),
    ("Alice", "Charlie"),
    ("Bob", "Charlie"),
    ("Bob", "David"),
    ("Charlie", "David"),
    ("Charlie", "Eve"),
    ("David", "Eve"),
    ("Eve", "Frank"),
]

for person1, person2 in friendships:
    friends.add_edge(people[person1], people[person2], 1.0)

# Find the most connected person
degrees = {name: friends.degree(node_id) for name, node_id in people.items()}
most_connected = max(degrees, key=degrees.get)
print(f"Most connected person: {most_connected} ({degrees[most_connected]} friends)")

# Calculate influence using PageRank
pagerank = pg.centrality.pagerank(friends, 0.85, 100, 1e-6)
name_to_pagerank = {name: pagerank[node_id] for name, node_id in people.items()}
most_influential = max(name_to_pagerank, key=name_to_pagerank.get)
print(f"Most influential person: {most_influential}")

# Find communities
communities = pg.community.label_propagation(friends, 100)
print(f"\nCommunities detected: {len(set(communities.values()))}")
for name, node_id in people.items():
    print(f"  {name}: Community {communities[node_id]}")
```

## Example 3: Shortest Path in a Transportation Network

```python
import pygraphina as pg

# Create a transportation network
network = pg.PyGraph()

# Add stations
stations = {
    "Station A": network.add_node(1),
    "Station B": network.add_node(2),
    "Station C": network.add_node(3),
    "Station D": network.add_node(4),
    "Station E": network.add_node(5),
}

# Add routes with travel times (in minutes)
routes = [
    ("Station A", "Station B", 5.0),
    ("Station A", "Station C", 10.0),
    ("Station B", "Station C", 3.0),
    ("Station B", "Station D", 8.0),
    ("Station C", "Station D", 2.0),
    ("Station C", "Station E", 7.0),
    ("Station D", "Station E", 4.0),
]

for start, end, time in routes:
    network.add_edge(stations[start], stations[end], time)

# Find shortest path from A to E
result = network.shortest_path(
    stations["Station A"],
    stations["Station E"]
)

if result:
    distance, path = result
    # Convert node IDs back to names
    station_names = {v: k for k, v in stations.items()}
    path_names = [station_names[node_id] for node_id in path]
    print(f"Shortest path from A to E: {' → '.join(path_names)}")
    print(f"Total travel time: {distance} minutes")
else:
    print("No path found")

# Find the most central station (best for a hub)
closeness = pg.centrality.closeness(network)
central_station_id = max(closeness, key=closeness.get)
station_names = {v: k for k, v in stations.items()}
print(f"Most central station: {station_names[central_station_id]}")
```

## Example 4: Citation Network Analysis

```python
import pygraphina as pg

# Create a directed citation network (paper A cites paper B)
citations = pg.PyDiGraph()

# Add papers
papers = {
    "Paper 1": citations.add_node(1),
    "Paper 2": citations.add_node(2),
    "Paper 3": citations.add_node(3),
    "Paper 4": citations.add_node(4),
    "Paper 5": citations.add_node(5),
    "Paper 6": citations.add_node(6),
}

# Add citations (directed edges)
cites = [
    ("Paper 4", "Paper 1"),  # Paper 4 cites Paper 1
    ("Paper 4", "Paper 2"),
    ("Paper 5", "Paper 1"),
    ("Paper 5", "Paper 3"),
    ("Paper 6", "Paper 2"),
    ("Paper 6", "Paper 3"),
    ("Paper 6", "Paper 4"),
]

for citing, cited in cites:
    citations.add_edge(papers[citing], papers[cited], 1.0)

# Find most cited papers (highest in-degree)
paper_citations = {
    name: citations.in_degree(node_id)
    for name, node_id in papers.items()
}
most_cited = max(paper_citations, key=paper_citations.get)
print(f"Most cited paper: {most_cited} ({paper_citations[most_cited]} citations)")

# Find most important papers using PageRank
pagerank = pg.centrality.pagerank(citations, 0.85, 100, 1e-6)
paper_importance = {name: pagerank[node_id] for name, node_id in papers.items()}
most_important = max(paper_importance, key=paper_importance.get)
print(f"Most important paper (PageRank): {most_important}")

# Find papers that are bridges between research areas
betweenness = pg.centrality.betweenness(citations)
paper_bridging = {name: betweenness[node_id] for name, node_id in papers.items()}
bridge_paper = max(paper_bridging, key=paper_bridging.get)
print(f"Bridge paper: {bridge_paper}")
```

## Example 5: Link Prediction in a Collaboration Network

```python
import pygraphina as pg

# Create a collaboration network
collabs = pg.PyGraph()

# Add researchers
researchers = [collabs.add_node(i) for i in range(8)]

# Add existing collaborations
existing = [
    (0, 1), (0, 2), (1, 2), (1, 3),
    (2, 4), (3, 4), (5, 6), (6, 7)
]

for r1, r2 in existing:
    collabs.add_edge(r1, r2, 1.0)

# Predict future collaborations using different methods
print("Link Prediction Scores:\n")

# Method 1: Jaccard Coefficient
jaccard = pg.links.jaccard_coefficient(collabs)
print("Top 3 by Jaccard Coefficient:")
for (n1, n2), score in sorted(jaccard.items(), key=lambda x: x[1], reverse=True)[:3]:
    if not collabs.contains_edge(n1, n2):  # Only non-existing edges
        print(f"  Researchers {n1} and {n2}: {score:.3f}")

# Method 2: Adamic-Adar Index
adamic_adar = pg.links.adamic_adar_index(collabs)
print("\nTop 3 by Adamic-Adar:")
for (n1, n2), score in sorted(adamic_adar.items(), key=lambda x: x[1], reverse=True)[:3]:
    if not collabs.contains_edge(n1, n2):
        print(f"  Researchers {n1} and {n2}: {score:.3f}")

# Method 3: Resource Allocation
resource = pg.links.resource_allocation_index(collabs)
print("\nTop 3 by Resource Allocation:")
for (n1, n2), score in sorted(resource.items(), key=lambda x: x[1], reverse=True)[:3]:
    if not collabs.contains_edge(n1, n2):
        print(f"  Researchers {n1} and {n2}: {score:.3f}")
```

## Example 6: Graph Metrics and Properties

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Add edges to create an interesting structure
edges = [
    (0, 1), (0, 2), (1, 2), (1, 3), (2, 3),
    (3, 4), (4, 5), (5, 6), (6, 7), (7, 8),
    (8, 9), (9, 6)
]

for n1, n2 in edges:
    g.add_edge(n1, n2, 1.0)

# Calculate various metrics
print("Graph Metrics:")
print(f"  Nodes: {g.node_count()}")
print(f"  Edges: {g.edge_count()}")
print(f"  Density: {g.density():.3f}")

# Graph-level metrics
try:
    diameter = pg.metrics.diameter(g)
    print(f"  Diameter: {diameter}")
except:
    print("  Diameter: Could not calculate (disconnected?)")

try:
    avg_clustering = pg.metrics.average_clustering_coefficient(g)
    print(f"  Average clustering: {avg_clustering:.3f}")
except:
    print("  Average clustering: Could not calculate")

# Node-level analysis
print("\nNode Analysis:")
for node in nodes[:5]:  # First 5 nodes
    degree = g.degree(node)
    neighbors = g.neighbors(node)
    print(f"  Node {node}: degree={degree}, neighbors={neighbors}")
```

## Example 7: Working with Graph Generators

```python
import pygraphina as pg

# Generate an Erdős-Rényi random graph
er_graph = pg.core.erdos_renyi(n=50, p=0.1, seed=42)
print(f"Erdős-Rényi: {er_graph.node_count()} nodes, {er_graph.edge_count()} edges")

# Generate a Barabási-Albert scale-free network
ba_graph = pg.core.barabasi_albert(n=50, m=2, seed=42)
print(f"Barabási-Albert: {ba_graph.node_count()} nodes, {ba_graph.edge_count()} edges")

# Generate a Watts-Strogatz small-world network
ws_graph = pg.core.watts_strogatz(n=50, k=4, beta=0.1, seed=42)
print(f"Watts-Strogatz: {ws_graph.node_count()} nodes, {ws_graph.edge_count()} edges")

# Generate a complete graph
complete = pg.core.complete_graph(n=10)
print(f"Complete: {complete.node_count()} nodes, {complete.edge_count()} edges")
print(f"Complete graph density: {complete.density():.3f}")  # Should be 1.0

# Compare properties
print("\nComparing PageRank distributions:")
for name, graph in [("ER", er_graph), ("BA", ba_graph), ("WS", ws_graph)]:
    pr = pg.centrality.pagerank(graph, 0.85, 100, 1e-6)
    max_pr = max(pr.values())
    min_pr = min(pr.values())
    print(f"  {name}: max={max_pr:.4f}, min={min_pr:.4f}, ratio={max_pr/min_pr:.2f}")
```

## Example 8: Combining Multiple Algorithms

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(20)]

# Add edges (create a connected graph)
for i in range(19):
    g.add_edge(i, i + 1, 1.0)
# Add some shortcuts
g.add_edge(0, 10, 1.0)
g.add_edge(5, 15, 1.0)
g.add_edge(10, 19, 1.0)

# Comprehensive analysis
print("Comprehensive Graph Analysis\n")

# 1. Basic properties
print("1. Basic Properties:")
print(f"   Nodes: {g.node_count()}, Edges: {g.edge_count()}")
print(f"   Density: {g.density():.3f}")

# 2. Community structure
print("\n2. Community Detection:")
communities = pg.community.label_propagation(g, 100)
num_communities = len(set(communities.values()))
print(f"   Communities found: {num_communities}")

# 3. Important nodes
print("\n3. Node Importance:")
pagerank = pg.centrality.pagerank(g, 0.85, 100, 1e-6)
betweenness = pg.centrality.betweenness(g)
top_pr = max(pagerank, key=pagerank.get)
top_bet = max(betweenness, key=betweenness.get)
print(f"   Most important (PageRank): Node {top_pr}")
print(f"   Most important (Betweenness): Node {top_bet}")

# 4. Link prediction
print("\n4. Link Prediction (top 3 missing links):")
jaccard = pg.links.jaccard_coefficient(g)
missing_links = [(n1, n2, s) for (n1, n2), s in jaccard.items()
                 if not g.contains_edge(n1, n2)]
for n1, n2, score in sorted(missing_links, key=lambda x: x[2], reverse=True)[:3]:
    print(f"   ({n1}, {n2}): {score:.3f}")

# 5. Paths
print("\n5. Shortest Paths:")
result = g.shortest_path(0, 19)
if result:
    distance, path = result
    print(f"   Path from 0 to 19: {path}")
    print(f"   Path length: {len(path) - 1}")

print("\nAnalysis complete!")
```

## Next Steps

- [Centrality Analysis Examples](centrality.md)
- [Community Detection Examples](community.md)
- [Link Prediction Examples](links.md)
- [Advanced Graph Algorithms](algorithms.md)
