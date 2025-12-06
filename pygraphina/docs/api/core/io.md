# Graph I/O

PyGraphina supports multiple formats for saving and loading graphs.

## Supported Formats

| Format | Read | Write | Best For |
|--------|------|-------|----------|
| Edge List | Yes | Yes | Simple, human-readable |
| JSON | Yes | Yes | Data interchange |
| Binary | Yes | Yes | Speed, compact storage |
| GraphML | No | Yes | Standard graph format |

## Edge List Format

Simple text format with edges, one per line.

```python
# Save as edge list
g.save_edge_list("graph.txt", sep=" ")

# Load from edge list
g = pg.PyGraph()
num_nodes, num_edges = g.load_edge_list("graph.txt", sep=" ")
```

File format:
```
source target weight
0 1 1.0
1 2 2.0
2 3 1.5
```

## JSON Format

Structured format with nodes and edges.

```python
# Save as JSON
g.save_json("graph.json")

# Load from JSON
g = pg.PyGraph()
g.load_json("graph.json")
```

## Binary Format

Compact, fast format for large graphs.

```python
# Save as binary
g.save_binary("graph.bin")

# Load from binary
g = pg.PyGraph()
g.load_binary("graph.bin")
```

## GraphML Format

Standard XML-based graph format.

```python
# Save as GraphML
g.save_graphml("graph.graphml")
```

Can be loaded in other tools (Gephi, Cytoscape, etc.).

## Example: Loading and Saving

```python
import pygraphina as pg

# Create and modify a graph
g = pg.PyGraph()
for i in range(100):
    g.add_node(i)

for i in range(99):
    g.add_edge(i, i+1, 1.0)

# Save in multiple formats
g.save_edge_list("graph.txt")
g.save_json("graph.json")
g.save_binary("graph.bin")
g.save_graphml("graph.graphml")

# Load in different format
g2 = pg.PyGraph()
g2.load_edge_list("graph.txt")

# Verify
assert g2.node_count() == g.node_count()
assert g2.edge_count() == g.edge_count()
```

## Separator Options for Edge Lists

Common separators:
- `" "` (space) - default, human-readable
- `","` (comma) - CSV format
- `"\t"` (tab) - tab-separated

```python
# CSV format
g.save_edge_list("graph.csv", sep=",")
g2.load_edge_list("graph.csv", sep=",")

# Tab-separated
g.save_edge_list("graph.tsv", sep="\t")
g3.load_edge_list("graph.tsv", sep="\t")
```

## Format Comparison

| Property | Edge List | JSON | Binary |
|----------|-----------|------|--------|
| Human-Readable | Yes | Yes | No |
| Compact | Moderate | No | Yes |
| Fast | Moderate | Slow | Fast |
| Standard | Yes | Yes | No |

## Tips

1. **Use edge list for portability** - Works with most tools
2. **Use binary for speed** - Fastest I/O for large graphs
3. **Use JSON for interchange** - Inspect and modify easily
4. **Use GraphML for visualization** - Standard format for graph tools
