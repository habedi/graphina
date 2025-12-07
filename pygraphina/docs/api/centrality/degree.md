# Degree Centrality

Degree centrality measures node importance based on the number of connections.

!!! important "Centrality Scores vs Raw Degree"
    This function returns **normalized centrality scores** (0.0 to 1.0), NOT raw degree counts.

    - **Centrality score**: `degree / (n - 1)` where n = number of nodes
    - **Raw degree count**: Use `g._degree(node)` instead

## Function

```python
pg.centrality.degree(graph: Union[PyGraph, PyDiGraph]) -> Dict[int, float]
```

Returns a dictionary mapping node IDs to their degree centrality scores (normalized between 0 and 1).

**Parameters:**
- `graph`: The input graph (PyGraph or PyDiGraph)

**Returns:**
- `Dict[int, float]`: Mapping of node IDs to degree centrality scores

## Understanding Degree Centrality

### What is Degree Centrality?

Degree centrality is a normalized measure of connectivity:

- **Score = 1.0**: Node is connected to all other nodes (maximum centrality)
- **Score = 0.5**: Node is connected to half of all nodes
- **Score = 0.0**: Node has no connections (isolated)

Formula: `centrality(v) = degree(v) / (n - 1)`

Where:
- `degree(v)` = number of edges connected to node v
- `n` = total number of nodes in the graph
- `n - 1` = maximum possible degree

### Centrality Scores vs Raw Degree

| Measure | Function | Returns | Use Case |
|---------|----------|---------|----------|
| **Degree Centrality** | `pg.centrality.degree(g)` | `Dict[int, float]` (0.0-1.0) | Comparing importance across different graphs |
| **Raw Degree** | `g._degree(node)` | `int` (count) | Local analysis, exact connection counts |

## Examples

### Basic Usage

```python
import pygraphina as pg

# Create a star graph (one central node connected to others)
g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(5)]

# Node 0 is the center, connected to all others
for i in range(1, 5):
    g.add_edge(nodes[0], nodes[i], 1.0)

# Get degree centrality (normalized scores)
centrality = pg.centrality.degree(g)
print(centrality)
# {0: 1.0, 1: 0.25, 2: 0.25, 3: 0.25, 4: 0.25}

# Node 0 has centrality 1.0 (connected to all 4 other nodes: 4/(5-1) = 1.0)
# Other nodes have centrality 0.25 (connected to 1 node: 1/(5-1) = 0.25)
```

### Comparing with Raw Degree

```python
import pygraphina as pg

g = pg.PyGraph()
nodes = [g.add_node(i) for i in range(5)]
g.add_edge(nodes[0], nodes[1], 1.0)
g.add_edge(nodes[0], nodes[2], 1.0)
g.add_edge(nodes[1], nodes[2], 1.0)

# Degree centrality (normalized)
centrality = pg.centrality.degree(g)
print(f"Centrality of node 0: {centrality[nodes[0]]:.2f}")  # 0.50

# Raw degree (count)
raw_degree = g._degree(nodes[0])
print(f"Raw degree of node 0: {raw_degree}")  # 2

# Explanation:
# Node 0 has 2 connections out of maximum 4 possible (5-1)
# Centrality = 2 / 4 = 0.5
```

### When to Use Each

**Use Degree Centrality** when:
- Comparing nodes across graphs of different sizes
- Need normalized importance scores
- Publishing results or comparing to papers
- Analyzing relative importance

**Use Raw Degree** when:
- Need exact connection counts
- Implementing custom algorithms
- Local network analysis
- Graph statistics (average degree, etc.)

## For Directed Graphs

For directed graphs, use:
- `pg.centrality.degree(g)`: Total degree (in + out)
- `pg.centrality.in_degree(g)`: In-degree centrality
- `pg.centrality.out_degree(g)`: Out-degree centrality

## Properties

- **Fast**: O(V) where V is the number of nodes
- **Simple**: Easy to interpret and compute
- **Local**: Only considers immediate neighbors
- **Normalized**: Scores are comparable across graphs

Simple, fast, and effective for many applications.
