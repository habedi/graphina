# Visualization

The visualization module provides tools for creating static and interactive visualizations of graphs.

## Interactive Visualization

### save_as_html

```python
pg.visualization.save_as_html(
    graph: Union[PyGraph, PyDiGraph],
path: str,
layout: str = "force_directed",
width: int = 800,
height: int = 600,
show_labels: bool = True
)
```

Save graph as interactive HTML file with D3.js visualization.

**Parameters:**

- `graph`: The input graph (PyGraph or PyDiGraph).
- `path`: Output file path (e.g., "graph.html").
- `layout`: Layout algorithm name. Options: "force_directed", "circular", "hierarchical", "grid", "random".
- `width`: Canvas width in pixels.
- `height`: Canvas height in pixels.
- `show_labels`: Whether to show node labels.

**Example:**

```python
import pygraphina as pg

g = pg.core.erdos_renyi_graph(50, 0.1)
pg.visualization.save_as_html(g, "graph.html", layout="circular")
```

---

### to_d3_json

```python
pg.visualization.to_d3_json(graph: Union[PyGraph, PyDiGraph]) -> str
```

Export graph to D3.js-compatible JSON format.

**Returns:**

- JSON string compatible with D3.js force-directed graphs.

## Layout Algorithms

### compute_layout

```python
pg.visualization.compute_layout(
    graph: Union[PyGraph, PyDiGraph],
algorithm: str = "force_directed",
width: float = 800.0,
height: float = 600.0
) -> Dict[int, Tuple[float, float]]
```

Compute node positions for graph visualization.

**Parameters:**

- `graph`: Input graph.
- `algorithm`: Layout algorithm name.
    - `"force_directed"`: Physics-based force simulation (default).
    - `"circular"`: Nodes arranged in a circle.
    - `"hierarchical"`: Tree-like layout (Sugiyama).
    - `"grid"`: Regular grid arrangement.
    - `"random"`: Random placement.
- `width`: Canvas width.
- `height`: Canvas height.

**Returns:**

- Dictionary mapping node IDs to (x, y) coordinates.

---

## Terminal Visualization

### to_ascii_art

```python
pg.visualization.to_ascii_art(graph: Union[PyGraph, PyDiGraph]) -> str
```

Generate ASCII art representation of the graph. Useful for quick debugging of small graphs.

**Example:**

```python
g = pg.core.path_graph(3)
print(pg.visualization.to_ascii_art(g))
# 0 --- 1 --- 2
```
