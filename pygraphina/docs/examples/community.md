# Community Detection Examples

This page demonstrates how to detect communities in graphs using PyGraphina.

## Example 1: Basic Label Propagation

Label Propagation is a fast, parameter-free community detection algorithm.

```python
import pygraphina as pg

# Create a graph with clear community structure
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(12)]

# Community 1: nodes 0-3 (densely connected)
for i in range(4):
    for j in range(i+1, 4):
        g.add_edge(nodes[i], nodes[j], 1.0)

# Community 2: nodes 4-7 (densely connected)
for i in range(4, 8):
    for j in range(i+1, 8):
        g.add_edge(nodes[i], nodes[j], 1.0)

# Community 3: nodes 8-11 (densely connected)
for i in range(8, 12):
    for j in range(i+1, 12):
        g.add_edge(nodes[i], nodes[j], 1.0)

# Add a few inter-community edges
g.add_edge(nodes[3], nodes[4], 1.0)
g.add_edge(nodes[7], nodes[8], 1.0)

# Detect communities
communities = pg.community.label_propagation(g, max_iters=100)

# Group nodes by community
from collections import defaultdict
comm_groups = defaultdict(list)
for node, comm_id in communities.items():
    comm_groups[comm_id].append(node)

print(f"Detected {len(comm_groups)} communities:")
for comm_id, members in comm_groups.items():
    print(f"  Community {comm_id}: {sorted(members)}")
```

## Example 2: Louvain Method on Karate Club

The Louvain method optimizes modularity to find communities.

```python
import pygraphina as pg

# Load Zachary's Karate Club graph
g = pg.core.karate_club_graph()

# Apply Louvain algorithm
# Resolution parameter: higher values find more, smaller communities
communities = pg.community.louvain(g, resolution=1.0)

# Calculate modularity
# Group nodes by community
from collections import defaultdict
comm_nodes = defaultdict(list)
for node, comm in communities.items():
    comm_nodes[comm].append(node)

print(f"Found {len(comm_nodes)} communities:")
for comm_id, members in sorted(comm_nodes.items()):
    print(f"  Community {comm_id}: {len(members)} members")
    print(f"    Nodes: {sorted(members)[:10]}{'...' if len(members) > 10 else ''}")

# Analyze community structure
for comm_id, members in comm_nodes.items():
    # Create subgraph for this community
    subg = g.subgraph(members)
    density = subg.density()
    print(f"\n  Community {comm_id} density: {density:.3f}")
```

## Example 3: Girvan-Newman Algorithm

Girvan-Newman finds communities by iteratively removing edges with high betweenness.

```python
import pygraphina as pg

# Create a graph with two connected clusters
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(8)]

# Cluster 1
g.add_edge(nodes[0], nodes[1], 1.0)
g.add_edge(nodes[1], nodes[2], 1.0)
g.add_edge(nodes[2], nodes[0], 1.0)
g.add_edge(nodes[1], nodes[3], 1.0)

# Bridge edge (will be detected as between-community)
g.add_edge(nodes[3], nodes[4], 1.0)

# Cluster 2
g.add_edge(nodes[4], nodes[5], 1.0)
g.add_edge(nodes[5], nodes[6], 1.0)
g.add_edge(nodes[6], nodes[4], 1.0)
g.add_edge(nodes[5], nodes[7], 1.0)

# Detect communities
communities = pg.community.girvan_newman(g, num_communities=2)

# Display results
from collections import defaultdict
groups = defaultdict(list)
for node, comm in communities.items():
    groups[comm].append(node)

for comm_id, members in groups.items():
    print(f"Community {comm_id}: {sorted(members)}")
```

## Example 4: Spectral Clustering

Spectral clustering uses the graph Laplacian to find communities.

```python
import pygraphina as pg

# Create a graph with three communities
g = pg.PyGraph()

# Add nodes in groups
community_sizes = [5, 6, 4]
nodes = []
for i in range(sum(community_sizes)):
    nodes.append(g.add_node(i))

# Build communities with high internal connectivity
offset = 0
for size in community_sizes:
    # Connect nodes within community
    for i in range(size):
        for j in range(i+1, size):
            g.add_edge(nodes[offset+i], nodes[offset+j], 1.0)
    offset += size

# Add sparse inter-community edges
g.add_edge(nodes[4], nodes[5], 0.5)    # Between community 0 and 1
g.add_edge(nodes[10], nodes[11], 0.5)  # Between community 1 and 2

# Apply spectral clustering
k = 3  # Number of communities
communities = pg.community.spectral_clustering(g, k)

# Analyze results
from collections import Counter
comm_counts = Counter(communities.values())
print(f"Community distribution: {dict(comm_counts)}")
```

## Example 5: Connected Components

For disconnected graphs, connected components form natural communities.

```python
import pygraphina as pg

# Create a graph with multiple disconnected components
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(15)]

# Component 1: nodes 0-4
edges_1 = [(0,1), (1,2), (2,3), (3,4), (4,0)]
for u, v in edges_1:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Component 2: nodes 5-9
edges_2 = [(5,6), (6,7), (7,8), (8,9), (9,5)]
for u, v in edges_2:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Component 3: nodes 10-14
edges_3 = [(10,11), (11,12), (12,13), (13,14)]
for u, v in edges_3:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Find connected components
components = pg.community.connected_components(g)

# Group by component
from collections import defaultdict
comp_groups = defaultdict(list)
for node, comp_id in components.items():
    comp_groups[comp_id].append(node)

print(f"Found {len(comp_groups)} components:")
for comp_id, members in sorted(comp_groups.items()):
    print(f"  Component {comp_id}: {sorted(members)}")
```

## Example 6: Comparing Community Detection Methods

```python
import pygraphina as pg
import time

# Create a test graph
g = pg.core.barabasi_albert_graph(100, 3, seed=42)

# Test different algorithms
algorithms = [
    ("Label Propagation", lambda: pg.community.label_propagation(g, 100)),
    ("Louvain", lambda: pg.community.louvain(g, 1.0)),
    ("Connected Components", lambda: pg.community.connected_components(g)),
]

results = {}
for name, algo in algorithms:
    start = time.time()
    communities = algo()
    elapsed = time.time() - start

    # Count communities
    num_communities = len(set(communities.values()))

    results[name] = {
        'communities': num_communities,
        'time': elapsed
    }

    print(f"{name}:")
    print(f"  Communities: {num_communities}")
    print(f"  Time: {elapsed:.4f}s")
```

## Example 7: Hierarchical Community Detection

```python
import pygraphina as pg

# Create a hierarchical structure
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(20)]

# Level 1: Two major groups
# Group A: nodes 0-9
for i in range(10):
    for j in range(i+1, 10):
        if (i % 5) == (j % 5) or abs(i - j) == 1:
            g.add_edge(nodes[i], nodes[j], 1.0)

# Group B: nodes 10-19
for i in range(10, 20):
    for j in range(i+1, 20):
        if ((i-10) % 5) == ((j-10) % 5) or abs(i - j) == 1:
            g.add_edge(nodes[i], nodes[j], 1.0)

# Weak inter-group connection
g.add_edge(nodes[9], nodes[10], 0.1)

# First level: detect major communities
major_communities = pg.community.louvain(g, resolution=0.5)

# Second level: detect sub-communities within each major community
from collections import defaultdict
major_groups = defaultdict(list)
for node, comm in major_communities.items():
    major_groups[comm].append(node)

print("Hierarchical community structure:")
for major_id, members in major_groups.items():
    print(f"\nMajor community {major_id} ({len(members)} nodes):")

    # Create subgraph and detect sub-communities
    subg = g.subgraph(members)
    if subg.node_count() > 1:
        sub_communities = pg.community.louvain(subg, resolution=1.5)
        sub_groups = defaultdict(list)
        for node, sub_comm in sub_communities.items():
            sub_groups[sub_comm].append(node)

        for sub_id, sub_members in sub_groups.items():
            print(f"  Sub-community {sub_id}: {sorted(sub_members)}")
```

## Example 8: Community Quality Metrics

```python
import pygraphina as pg

# Create a graph
g = pg.core.erdos_renyi_graph(50, 0.1, seed=42)

# Detect communities
communities = pg.community.louvain(g, 1.0)

# Calculate community quality metrics
from collections import defaultdict

# Group nodes by community
comm_nodes = defaultdict(list)
for node, comm in communities.items():
    comm_nodes[comm].append(node)

print("Community Quality Metrics:")
print("=" * 50)

for comm_id, members in comm_nodes.items():
    # Create subgraph for this community
    subg = g.subgraph(members)

    # Internal edges
    internal_edges = subg.edge_count()

    # Possible internal edges
    n = len(members)
    possible_edges = n * (n - 1) // 2

    # Internal density
    density = internal_edges / possible_edges if possible_edges > 0 else 0

    # Average clustering
    clustering = subg.average_clustering()

    print(f"\nCommunity {comm_id}:")
    print(f"  Size: {len(members)} nodes")
    print(f"  Internal edges: {internal_edges}")
    print(f"  Density: {density:.3f}")
    print(f"  Avg clustering: {clustering:.3f}")
```
