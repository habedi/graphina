# PageRank Centrality

PageRank is a link analysis algorithm that assigns importance scores to nodes based on the structure of incoming links.

## Function Signature

```python
pg.centrality.pagerank(
    graph: Union[PyGraph, PyDiGraph],
    damping: float = 0.85,
    max_iters: int = 100,
    tol: float = 1e-6
) -> Dict[int, float]
```

## Parameters

- **graph**: The graph to analyze (directed or undirected)
- **damping**: Damping factor (probability of following a link vs random jump). Default: 0.85
- **max_iters**: Maximum number of iterations. Default: 100
- **tol**: Convergence tolerance. Default: 1e-6

## Returns

Dictionary mapping node IDs to PageRank scores. Scores sum to 1.0.

## Description

PageRank models the behavior of a random surfer who:

1. With probability `damping`, follows a random outgoing link
2. With probability `1 - damping`, jumps to a random node

The PageRank score represents the long-term probability of finding the surfer at each node.

### Algorithm

The algorithm iteratively updates scores using:

```
PR(u) = (1 - d) / N + d * Σ(PR(v) / out_degree(v))
```

Where the sum is over all nodes v with edges to u.

## Time Complexity

O(V + E) per iteration, typically O(k·(V + E)) where k is the number of iterations (usually < 100).

## Examples

### Basic Usage

```python
import pygraphina as pg

# Create a simple link structure
g = pg.PyDiGraph()
a, b, c, d = [g.add_node(i) for i in range(4)]

g.add_edge(a, b, 1.0)
g.add_edge(a, c, 1.0)
g.add_edge(b, c, 1.0)
g.add_edge(c, a, 1.0)
g.add_edge(d, c, 1.0)

# Calculate PageRank
scores = pg.centrality.pagerank(g, damping=0.85, max_iters=100, tol=1e-6)

print("PageRank scores:")
for node, score in sorted(scores.items(), key=lambda x: x[1], reverse=True):
    print(f"  Node {node}: {score:.4f}")
```

### Web Page Ranking

```python
import pygraphina as pg

# Create a web graph (pages linking to each other)
web = pg.PyDiGraph()

# Add pages
home = web.add_node(1)
about = web.add_node(2)
products = web.add_node(3)
blog = web.add_node(4)
contact = web.add_node(5)

# Add links (directed edges)
web.add_edge(home, about, 1.0)
web.add_edge(home, products, 1.0)
web.add_edge(home, blog, 1.0)
web.add_edge(about, contact, 1.0)
web.add_edge(products, blog, 1.0)
web.add_edge(blog, home, 1.0)
web.add_edge(blog, products, 1.0)

# Calculate page importance
pagerank = pg.centrality.pagerank(web, damping=0.85)

page_names = {home: "Home", about: "About", products: "Products",
              blog: "Blog", contact: "Contact"}

print("Page Importance Ranking:")
for node, score in sorted(pagerank.items(), key=lambda x: x[1], reverse=True):
    print(f"  {page_names[node]}: {score:.4f}")
```

### Social Network Influence

```python
import pygraphina as pg

# Create a social network (who follows whom)
social = pg.PyDiGraph()

# Add users
users = [social.add_node(i) for i in range(10)]

# Add follow relationships
follows = [
    (0, 1), (0, 2), (1, 2), (2, 3), (3, 4),
    (4, 5), (5, 0), (6, 7), (7, 8), (8, 6), (9, 0)
]

for follower, followee in follows:
    social.add_edge(follower, followee, 1.0)

# Calculate influence scores
influence = pg.centrality.pagerank(social, damping=0.85)

# Find top influencers
top_influencers = sorted(influence.items(), key=lambda x: x[1], reverse=True)[:3]

print("Top 3 Influencers:")
for rank, (user, score) in enumerate(top_influencers, 1):
    print(f"  {rank}. User {user}: {score:.4f}")
```

## Parameter Tuning

### Damping Factor

- **0.85 (default)**: Standard value used by Google
- **Higher (0.90-0.95)**: More emphasis on link structure
- **Lower (0.70-0.80)**: More random jumps, less emphasis on links

```python
# Compare different damping factors
for damping in [0.70, 0.85, 0.95]:
    scores = pg.centrality.pagerank(g, damping=damping)
    top_node = max(scores, key=scores.get)
    print(f"Damping {damping}: Top node {top_node} = {scores[top_node]:.4f}")
```

### Convergence

Adjust `max_iters` and `tol` for speed vs accuracy:

```python
# Fast approximation (fewer iterations)
fast = pg.centrality.pagerank(g, max_iters=10, tol=1e-3)

# High precision (more iterations, tighter tolerance)
precise = pg.centrality.pagerank(g, max_iters=200, tol=1e-9)
```

## Comparison with Eigenvector Centrality

PageRank and eigenvector centrality are similar but have key differences:

| Feature             | PageRank           | Eigenvector      |
|---------------------|--------------------|------------------|
| Damping             | Yes (random jumps) | No               |
| Disconnected graphs | Handles well       | May fail         |
| Convergence         | Always converges   | May not converge |
| Use case            | Web ranking        | Social influence |

```python
# Compare the two
pagerank_scores = pg.centrality.pagerank(g, 0.85, 100, 1e-6)
eigenvector_scores = pg.centrality.eigenvector(g, 100, 1e-6)

print("Comparison:")
for node in g.nodes():
    print(f"Node {node}: PR={pagerank_scores[node]:.4f}, "
          f"EV={eigenvector_scores[node]:.4f}")
```

## Common Pitfalls

### 1. Using on Undirected Graphs

PageRank works on both directed and undirected graphs, but interpretation differs:

- **Directed**: Models link following behavior
- **Undirected**: All edges are bidirectional

### 2. Dangling Nodes

Nodes with no outgoing edges (sinks) are handled automatically by the algorithm.

### 3. Disconnected Components

PageRank handles disconnected graphs gracefully due to the random jump mechanism.

## Performance

For large graphs, consider:

1. **Parallel implementation**: Use `pg.parallel.pagerank_parallel()`
2. **Early stopping**: Use larger `tol` for faster (approximate) results
3. **Fewer iterations**: Reduce `max_iters` if exact convergence isn't critical

```python
# For large graphs
import pygraphina as pg

# Standard (sequential)
scores = pg.centrality.pagerank(large_graph, 0.85, 100, 1e-6)

# Parallel (faster for large graphs)
scores = pg.parallel.pagerank_parallel(large_graph, 0.85, 100, 1e-6)
```

## Applications

- **Web search**: Ranking web pages
- **Social networks**: Finding influential users
- **Citation networks**: Identifying important papers
- **Recommendation systems**: Ranking items
- **Protein networks**: Finding critical proteins
- **Knowledge graphs**: Entity importance

## Personalized PageRank { #personalized }

Personalized PageRank (PPR) allows you to bias the random jumps toward specific nodes, making the results more relevant
to a particular starting point.

!!! note "Current Implementation"
PyGraphina currently provides standard PageRank. For personalized results, you can use the following workaround.

### Workaround for Personalized PageRank

You can approximate Personalized PageRank by computing PageRank on a modified graph:

```python
import pygraphina as pg

# Create a graph
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(10)]

# Add edges
for i in range(9):
    g.add_edge(i, i + 1, 1.0)
g.add_edge(9, 0, 1.0)

# To personalize to a specific node, extract its ego graph
# and compute PageRank on that subgraph
personalization_node = 0
ego = g.ego_graph(personalization_node, radius=3)

# Compute PageRank on the ego graph
local_scores = pg.centrality.pagerank(ego, 0.85, 100, 1e-6)
print(f"Local importance around node {personalization_node}: {local_scores}")
```

## See Also

- [Eigenvector Centrality](eigenvector.md): Similar algorithm without damping
- [Personalized PageRank](#personalized): PageRank with custom start nodes
- [Katz Centrality](katz.md): Weighted path-based centrality
- [All Centrality Measures](index.md): Overview of centrality algorithms
