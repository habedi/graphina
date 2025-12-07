# Link Prediction Examples

This page demonstrates how to predict missing links in graphs using PyGraphina.

## Overview

Link prediction algorithms estimate the likelihood of future connections between nodes based on existing graph
structure. These are useful for:

- Recommender systems
- Social network friend suggestions
- Protein-protein interaction prediction
- Knowledge graph completion

## Example 1: Basic Jaccard Coefficient

The Jaccard coefficient measures similarity based on common neighbors.

```python
import pygraphina as pg

# Create a social network
g = pg.PyGraph()
nodes = {
    "Alice": g.add_node(0),
    "Bob": g.add_node(1),
    "Carol": g.add_node(2),
    "Dave": g.add_node(3),
    "Eve": g.add_node(4),
}

# Add existing friendships
friendships = [
    ("Alice", "Bob"),
    ("Alice", "Carol"),
    ("Bob", "Carol"),
    ("Bob", "Dave"),
    ("Carol", "Dave"),
    ("Dave", "Eve"),
]

for person1, person2 in friendships:
    g.add_edge(nodes[person1], nodes[person2], 1.0)

# Predict potential new links using Jaccard coefficient
predictions = pg.links.jaccard_coefficient(g)

# Sort by score (higher = more likely)
sorted_predictions = sorted(predictions.items(),
                            key=lambda x: x[1],
                            reverse=True)

print("Top link predictions (Jaccard):")
# Convert node IDs back to names
id_to_name = {v: k for k, v in nodes.items()}
for (u, v), score in sorted_predictions[:5]:
    # Only show non-existing edges
    if not g.contains_edge(u, v):
        print(f"  {id_to_name[u]} - {id_to_name[v]}: {score:.3f}")
```

## Example 2: Adamic-Adar Index

Adamic-Adar gives more weight to common neighbors with fewer connections.

```python
import pygraphina as pg

# Create a citation network
g = pg.PyGraph()
papers = [g.add_node(i) for i in range(10)]

# Add citations (undirected for simplicity)
citations = [
    (0, 1), (0, 2), (1, 2), (1, 3),
    (2, 3), (2, 4), (3, 4), (3, 5),
    (4, 5), (5, 6), (6, 7), (7, 8), (8, 9)
]

for u, v in citations:
    g.add_edge(papers[u], papers[v], 1.0)

# Predict using Adamic-Adar
predictions = pg.links.adamic_adar(g)

# Find papers that might be related
print("Potential citation predictions:")
existing = set(citations)
for (u, v), score in sorted(predictions.items(),
                            key=lambda x: x[1],
                            reverse=True)[:10]:
    u_idx = papers.index(u) if u in papers else -1
    v_idx = papers.index(v) if v in papers else -1
    if u_idx != -1 and v_idx != -1:
        pair = tuple(sorted([u_idx, v_idx]))
        if pair not in existing:
            print(f"  Paper {u_idx} <-> Paper {v_idx}: {score:.3f}")
```

## Example 3: Resource Allocation

Resource allocation models information flow through common neighbors.

```python
import pygraphina as pg

# Create a collaboration network
g = pg.PyGraph()
researchers = [g.add_node(i) for i in range(8)]

# Existing collaborations
collaborations = [
    (0, 1), (0, 2), (1, 2),  # Group 1
    (3, 4), (3, 5), (4, 5),  # Group 2
    (2, 3),  # Bridge between groups
    (5, 6), (6, 7)  # Extended connections
]

for u, v in collaborations:
    g.add_edge(researchers[u], researchers[v], 1.0)

# Predict potential collaborations
predictions = pg.links.resource_allocation(g)

# Display top predictions
print("Potential collaboration recommendations:")
existing_set = {tuple(sorted([u, v])) for u, v in collaborations}

sorted_pred = sorted(predictions.items(), key=lambda x: x[1], reverse=True)
count = 0
for (u, v), score in sorted_pred:
    pair = tuple(sorted([u, v]))
    if pair not in existing_set and score > 0:
        print(f"  Researcher {u} - Researcher {v}: {score:.3f}")
        count += 1
        if count >= 5:
            break
```

## Example 4: Preferential Attachment

Preferential attachment favors high-degree nodes.

```python
import pygraphina as pg

# Create a social network following power-law degree distribution
g = pg.core.barabasi_albert_graph(20, 2, seed=42)

# Calculate preferential attachment scores
predictions = pg.links.preferential_attachment(g)

# Analyze predictions
print("Preferential Attachment Analysis:")

# Get node degrees
degrees = {n: g.degree(n) for n in g.nodes()}

# Top predictions
sorted_pred = sorted(predictions.items(), key=lambda x: x[1], reverse=True)

print("\nTop 10 predictions:")
for i, ((u, v), score) in enumerate(sorted_pred[:10]):
    if not g.contains_edge(u, v):
        deg_u = degrees[u]
        deg_v = degrees[v]
        print(f"  {i + 1}. Nodes {u}-{v}: score={score:.0f} "
              f"(degrees: {deg_u}, {deg_v})")
```

## Example 5: Common Neighbors

Simple count of shared neighbors.

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(6)]

# Add edges forming triangles and paths
g.add_edge(nodes[0], nodes[1], 1.0)
g.add_edge(nodes[0], nodes[2], 1.0)
g.add_edge(nodes[1], nodes[2], 1.0)  # Triangle

g.add_edge(nodes[2], nodes[3], 1.0)
g.add_edge(nodes[3], nodes[4], 1.0)
g.add_edge(nodes[4], nodes[5], 1.0)

# Predict using common neighbors
predictions = pg.links.common_neighbors(g)

# Display predictions with counts
print("Common neighbor predictions:")
for (u, v), count in sorted(predictions.items(),
                            key=lambda x: x[1],
                            reverse=True):
    if count > 0 and not g.contains_edge(u, v):
        print(f"  {u} - {v}: {count} common neighbors")
```

## Example 6: Comparing Link Prediction Methods

```python
import pygraphina as pg

# Create a test network
g = pg.core.erdos_renyi_graph(30, 0.15, seed=42)

# Apply multiple algorithms
algorithms = {
    "Jaccard": pg.links.jaccard_coefficient,
    "Adamic-Adar": pg.links.adamic_adar,
    "Resource Allocation": pg.links.resource_allocation,
    "Preferential Attachment": pg.links.preferential_attachment,
    "Common Neighbors": pg.links.common_neighbors,
}

# Get predictions from each algorithm
print("Link Prediction Comparison")
print("=" * 60)

for name, algo in algorithms.items():
    predictions = algo(g)

    # Count non-zero predictions for non-existing edges
    non_existing = 0
    positive_pred = 0

    for (u, v), score in predictions.items():
        if not g.contains_edge(u, v):
            non_existing += 1
            if score > 0:
                positive_pred += 1

    print(f"\n{name}:")
    print(f"  Total predictions: {len(predictions)}")
    print(f"  Non-existing edges: {non_existing}")
    print(f"  Positive predictions: {positive_pred}")

    # Show top 3
    top_3 = sorted(predictions.items(), key=lambda x: x[1], reverse=True)[:3]
    print(f"  Top 3 scores:")
    for (u, v), score in top_3:
        if not g.contains_edge(u, v):
            print(f"    {u}-{v}: {score:.4f}")
```

## Example 7: Temporal Link Prediction

Predict links based on network evolution.

```python
import pygraphina as pg

# Simulate network growth over time
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(20)]

# Time step 1: Initial network
edges_t1 = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]
for u, v in edges_t1:
    g.add_edge(nodes[u], nodes[v], 1.0)

# Predict links for time step 2
predictions_t1 = pg.links.jaccard_coefficient(g)

# Get top predictions
top_predictions = sorted(predictions_t1.items(),
                         key=lambda x: x[1],
                         reverse=True)[:5]

print("Predictions before growth:")
for (u, v), score in top_predictions:
    if not g.contains_edge(u, v):
        print(f"  {u}-{v}: {score:.3f}")

# Time step 2: Add new edges (simulating actual growth)
edges_t2 = [(1, 3), (2, 4), (0, 5), (5, 6)]
for u, v in edges_t2:
    if v < len(nodes):
        g.add_edge(nodes[u], nodes[v], 1.0)

# Check prediction accuracy
print("\nActual new edges added:")
for u, v in edges_t2:
    print(f"  {nodes[u]}-{nodes[v]}")

print("\nChecking prediction accuracy:")
predicted_pairs = {(u, v) for (u, v), _ in top_predictions}
actual_pairs = {(nodes[u], nodes[v]) for u, v in edges_t2 if v < len(nodes)}

# Calculate how many predictions were correct
correct = len(predicted_pairs & actual_pairs)
print(f"  Correct predictions: {correct}/{len(top_predictions)}")
```

## Example 8: Centrality-Based Link Prediction

Use centrality measures to predict links.

```python
import pygraphina as pg

# Create a network
g = pg.core.watts_strogatz_graph(30, 4, 0.1, seed=42)

# Calculate centrality-based predictions
predictions = pg.links.centrality_based(g)

# Get node centralities for analysis
pagerank = pg.centrality.pagerank(g, 0.85, 100, 1e-6)

print("Centrality-based link predictions:")

# Sort predictions
sorted_pred = sorted(predictions.items(), key=lambda x: x[1], reverse=True)

# Show top predictions with centrality info
for i, ((u, v), score) in enumerate(sorted_pred[:10]):
    if not g.contains_edge(u, v):
        pr_u = pagerank.get(u, 0)
        pr_v = pagerank.get(v, 0)
        print(f"  {u}-{v}: score={score:.4f} "
              f"(PageRank: {pr_u:.4f}, {pr_v:.4f})")
```

## Example 9: Link Prediction Evaluation

Evaluate prediction quality using held-out edges.

```python
import pygraphina as pg
import random

# Create a test graph
g_full = pg.core.barabasi_albert_graph(50, 3, seed=42)

# Split edges: 80% training, 20% test
all_edges = list(g_full.edges())
random.seed(42)
random.shuffle(all_edges)

split_point = int(0.8 * len(all_edges))
train_edges = all_edges[:split_point]
test_edges = all_edges[split_point:]

# Create training graph
g_train = pg.PyGraph()
for node in g_full.nodes():
    g_train.add_node(node)

for u, v in train_edges:
    g_train.add_edge(u, v, 1.0)

# Make predictions
predictions = pg.links.adamic_adar(g_train)

# Evaluate on test set
test_set = {tuple(sorted([u, v])) for u, v in test_edges}

# Rank all predictions
sorted_pred = sorted(predictions.items(), key=lambda x: x[1], reverse=True)

# Calculate metrics
top_k = 20
top_k_pairs = {tuple(sorted([u, v])) for (u, v), _ in sorted_pred[:top_k]}

hits = len(top_k_pairs & test_set)
precision = hits / top_k if top_k > 0 else 0
recall = hits / len(test_set) if len(test_set) > 0 else 0

print(f"Link Prediction Evaluation:")
print(f"  Training edges: {len(train_edges)}")
print(f"  Test edges: {len(test_edges)}")
print(f"  Top-{top_k} Precision: {precision:.3f}")
print(f"  Top-{top_k} Recall: {recall:.3f}")
print(f"  Hits: {hits}/{top_k}")
```

## Use Cases Summary

| Algorithm               | Best For                  | Complexity              |
|-------------------------|---------------------------|-------------------------|
| Jaccard Coefficient     | General purpose, balanced | O(V²·d)                 |
| Adamic-Adar             | Social networks           | O(V²·d)                 |
| Resource Allocation     | Information networks      | O(V²·d)                 |
| Preferential Attachment | Scale-free networks       | O(V²)                   |
| Common Neighbors        | Quick baseline            | O(V²·d)                 |
| Centrality-based        | Influence networks        | O(V²) + centrality cost |

Where d is average degree.
