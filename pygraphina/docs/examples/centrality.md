# Centrality Analysis Examples

This guide demonstrates how to use various centrality measures to analyze graph structure and identify important nodes.

## Example 1: Comparing Centrality Measures

Different centrality measures capture different notions of "importance". Let's compare them:

```python
import pygraphina as pg

# Create a small network with interesting structure
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Create a hub-and-spoke structure with some cross-connections
hub = 0
for i in range(1, 6):
    g.add_edge(hub, i, 1.0)

# Add a chain
for i in range(6, 9):
    g.add_edge(i, i + 1, 1.0)

# Connect hub to chain
g.add_edge(hub, 6, 1.0)

# Add some cross-connections
g.add_edge(1, 2, 1.0)
g.add_edge(3, 4, 1.0)

# Calculate different centrality measures
degree = pg.centrality.degree(g)
pagerank = pg.centrality.pagerank(g, 0.85, 100, 1e-6)
betweenness = pg.centrality.betweenness(g)
closeness = pg.centrality.closeness(g)

# Print comparison
print("Node  Degree  PageRank  Betweenness  Closeness")
print("-" * 50)
for node in range(5):  # First 5 nodes
    print(f"{node:4d}  {degree[node]:6d}  "
          f"{pagerank[node]:8.4f}  "
          f"{betweenness[node]:11.4f}  "
          f"{closeness[node]:9.4f}")
```

## Example 2: Finding Influencers in a Social Network

Identify the most influential users in a social network:

```python
import pygraphina as pg

# Create a social network
social = pg.PyDiGraph()

# Add users and follower relationships
users = {
    "alice": social.add_node(1),
    "bob": social.add_node(2),
    "charlie": social.add_node(3),
    "david": social.add_node(4),
    "eve": social.add_node(5),
    "frank": social.add_node(6),
    "grace": social.add_node(7),
    "henry": social.add_node(8),
}

# Add follow relationships (directed edges)
follows = [
    ("bob", "alice"),      # Bob follows Alice
    ("charlie", "alice"),
    ("david", "alice"),
    ("eve", "bob"),
    ("frank", "bob"),
    ("grace", "charlie"),
    ("henry", "charlie"),
    ("alice", "charlie"),
    ("bob", "david"),
    ("charlie", "eve"),
]

for follower, followee in follows:
    social.add_edge(users[follower], users[followee], 1.0)

# Calculate influence using PageRank
influence = pg.centrality.pagerank(social, 0.85, 100, 1e-6)

# Rank users by influence
ranked = sorted(
    [(name, influence[uid]) for name, uid in users.items()],
    key=lambda x: x[1],
    reverse=True
)

print("Influence Ranking (PageRank):")
for rank, (name, score) in enumerate(ranked, 1):
    followers = social.in_degree(users[name])
    print(f"{rank}. {name:8s}: {score:.4f} ({followers} followers)")

# Find connectors (high betweenness)
betweenness = pg.centrality.betweenness(social)
connector = max(users.items(), key=lambda x: betweenness[x[1]])
print(f"\nKey Connector: {connector[0]}")
```

## Example 3: Identifying Bridges in a Communication Network

Find nodes that connect different parts of the network:

```python
import pygraphina as pg

# Create a network with distinct communities
network = pg.PyGraph()
nodes = [network.add_node(i) for i in range(15)]

# Community 1 (densely connected)
for i in range(0, 5):
    for j in range(i + 1, 5):
        network.add_edge(i, j, 1.0)

# Community 2 (densely connected)
for i in range(10, 15):
    for j in range(i + 1, 15):
        network.add_edge(i, j, 1.0)

# Bridge nodes
bridge1 = 5
bridge2 = 9

# Connect communities through bridges
network.add_edge(2, bridge1, 1.0)
network.add_edge(bridge1, 6, 1.0)
network.add_edge(6, 7, 1.0)
network.add_edge(7, 8, 1.0)
network.add_edge(8, bridge2, 1.0)
network.add_edge(bridge2, 11, 1.0)

# Calculate betweenness centrality
betweenness = pg.centrality.betweenness(network)

print("Top 5 Bridge Nodes (Betweenness):")
for node, score in sorted(
    betweenness.items(),
    key=lambda x: x[1],
    reverse=True
)[:5]:
    print(f"  Node {node}: {score:.4f}")

# Verify they connect communities
communities = pg.community.label_propagation(network, 100)
print(f"\nTotal communities: {len(set(communities.values()))}")
print(f"Bridge1 ({bridge1}) community: {communities[bridge1]}")
print(f"Bridge2 ({bridge2}) community: {communities[bridge2]}")
```

## Example 4: Finding Central Hubs in a Transportation Network

Identify the most central locations in a transportation network:

```python
import pygraphina as pg

# Create a transportation network (cities and routes)
transport = pg.PyGraph()

cities = {
    "NYC": transport.add_node(1),
    "BOS": transport.add_node(2),
    "PHI": transport.add_node(3),
    "DC": transport.add_node(4),
    "CHI": transport.add_node(5),
    "DET": transport.add_node(6),
    "ATL": transport.add_node(7),
    "MIA": transport.add_node(8),
    "DEN": transport.add_node(9),
    "LA": transport.add_node(10),
}

# Add routes (travel time in hours)
routes = [
    ("NYC", "BOS", 4), ("NYC", "PHI", 2), ("PHI", "DC", 2),
    ("NYC", "DC", 4), ("NYC", "CHI", 12), ("CHI", "DET", 5),
    ("DC", "ATL", 8), ("ATL", "MIA", 10), ("CHI", "DEN", 14),
    ("DEN", "LA", 16), ("LA", "MIA", 40),
]

for c1, c2, time in routes:
    transport.add_edge(cities[c1], cities[c2], time)

# Calculate closeness centrality (which city is most accessible?)
closeness = pg.centrality.closeness(transport)
city_closeness = {
    name: closeness[city_id]
    for name, city_id in cities.items()
}

print("Most Accessible Cities (Closeness):")
for city, score in sorted(
    city_closeness.items(),
    key=lambda x: x[1],
    reverse=True
)[:5]:
    print(f"  {city}: {score:.4f}")

# Calculate harmonic centrality (alternative that handles disconnection better)
harmonic = pg.centrality.harmonic(transport)
city_harmonic = {
    name: harmonic[city_id]
    for name, city_id in cities.items()
}

print("\nMost Central Cities (Harmonic):")
for city, score in sorted(
    city_harmonic.items(),
    key=lambda x: x[1],
    reverse=True
)[:5]:
    print(f"  {city}: {score:.4f}")
```

## Example 5: Using Eigenvector Centrality for Quality Assessment

Find nodes connected to other important nodes:

```python
import pygraphina as pg

# Create a citation network
citations = pg.PyDiGraph()

papers = [citations.add_node(i) for i in range(20)]

# Simulate citations (papers cite earlier papers)
# High-quality papers are cited by other high-quality papers
import random
random.seed(42)

for paper in range(5, 20):
    # Each paper cites 2-4 earlier papers
    num_citations = random.randint(2, 4)
    cited = random.sample(range(paper), num_citations)
    for c in cited:
        citations.add_edge(paper, c, 1.0)

# Calculate centrality
pagerank = pg.centrality.pagerank(citations, 0.85, 100, 1e-6)
eigenvector = pg.centrality.eigenvector(citations, 100, 1e-6)

print("Top 5 Papers by PageRank:")
for paper, score in sorted(
    pagerank.items(),
    key=lambda x: x[1],
    reverse=True
)[:5]:
    citations_count = citations.in_degree(paper)
    print(f"  Paper {paper}: {score:.4f} ({citations_count} citations)")

print("\nTop 5 Papers by Eigenvector:")
for paper, score in sorted(
    eigenvector.items(),
    key=lambda x: x[1],
    reverse=True
)[:5]:
    citations_count = citations.in_degree(paper)
    print(f"  Paper {paper}: {score:.4f} ({citations_count} citations)")
```

## Example 6: Using Katz Centrality with Custom Parameters

Katz centrality allows fine-tuning of influence propagation:

```python
import pygraphina as pg

# Create a simple network
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(8)]

edges = [
    (0, 1), (0, 2), (1, 3), (2, 3),
    (3, 4), (4, 5), (5, 6), (6, 7)
]

for a, b in edges:
    g.add_edge(a, b, 1.0)

# Compare different alpha values (attenuation factor)
print("Comparing Katz Centrality with different alpha:")
for alpha in [0.05, 0.1, 0.2]:
    katz = pg.centrality.katz(g, alpha=alpha, beta=1.0, max_iters=100, tol=1e-6)
    top_node = max(katz, key=katz.get)
    print(f"\nAlpha = {alpha:.2f}:")
    print(f"  Top node: {top_node} (score: {katz[top_node]:.4f})")
    print(f"  Score range: {min(katz.values()):.4f} - {max(katz.values()):.4f}")
```

## Example 7: Parallel Centrality for Large Graphs

Use parallel implementations for better performance on large graphs:

```python
import pygraphina as pg
import time

# Create a larger random graph
large_graph = pg.core.erdos_renyi(n=1000, p=0.01, seed=42)

print(f"Graph: {large_graph.node_count()} nodes, {large_graph.edge_count()} edges")

# Sequential PageRank
start = time.time()
pr_seq = pg.centrality.pagerank(large_graph, 0.85, 100, 1e-6)
time_seq = time.time() - start

# Parallel PageRank
start = time.time()
pr_par = pg.parallel.parallel_pagerank(large_graph, 0.85, 100, 1e-6)
time_par = time.time() - start

print(f"\nSequential: {time_seq:.3f}s")
print(f"Parallel:   {time_par:.3f}s")
print(f"Speedup:    {time_seq/time_par:.2f}x")

# Results should be nearly identical
top_5_seq = sorted(pr_seq.items(), key=lambda x: x[1], reverse=True)[:5]
top_5_par = sorted(pr_par.items(), key=lambda x: x[1], reverse=True)[:5]

print("\nTop 5 nodes (both methods):")
for (n1, s1), (n2, s2) in zip(top_5_seq, top_5_par):
    print(f"  Node {n1}: {s1:.6f} (parallel: {s2:.6f})")
```

## Best Practices

### 1. Choose the Right Centrality

- **Degree**: Quick, simple, good for initial analysis
- **PageRank**: General-purpose, handles directed graphs well
- **Betweenness**: Find bridges and bottlenecks (slower)
- **Closeness**: Find central nodes (requires connected graph)
- **Eigenvector**: Connections to important nodes matter
- **Katz**: Tunable influence propagation

### 2. Parameter Tuning

For iterative algorithms (PageRank, Eigenvector, Katz):

```python
# Quick approximation
quick = pg.centrality.pagerank(g, 0.85, max_iters=10, tol=1e-3)

# High precision
precise = pg.centrality.pagerank(g, 0.85, max_iters=200, tol=1e-9)
```

### 3. Handle Disconnected Graphs

Some centrality measures require connected graphs. Use harmonic centrality as an alternative to closeness:

```python
# Closeness may fail on disconnected graphs
try:
    closeness = pg.centrality.closeness(g)
except Exception as e:
    print(f"Closeness failed: {e}")
    # Use harmonic instead
    harmonic = pg.centrality.harmonic(g)
```

### 4. Normalize When Comparing

When comparing centrality scores across different graphs, consider normalization:

```python
def normalize_scores(scores):
    min_s = min(scores.values())
    max_s = max(scores.values())
    range_s = max_s - min_s
    return {
        node: (score - min_s) / range_s if range_s > 0 else 0
        for node, score in scores.items()
    }

normalized = normalize_scores(pagerank)
```

## See Also

- [Centrality API Reference](../api/centrality/index.md)
- [PageRank Documentation](../api/centrality/pagerank.md)
- [Community Detection Examples](community.md)
- [Basic Examples](basic.md)
