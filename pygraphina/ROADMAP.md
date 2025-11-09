## PyGraphina Feature Roadmap

This file includes the roadmap for the Python bindings (`PyGraphina`) of the Graphina project.
It shows the features to be implemented and their current status.

> [!IMPORTANT]
> This roadmap is a work in progress and is subject to change.

- **Core Binding Infrastructure**
    - [x] Basic Graph and Digraph construction (add or remove nodes and edges)
    - [x] Node and edge indexing mapping (internal <-> Python IDs)
    - [x] Safe error propagation (Rust `GraphinaError` -> Python `ValueError` and `RuntimeError`)
    - [x] Serialization passthrough (JSON, binary, GraphML via Rust core)
    - [ ] Direct Python-side graph construction helpers (builder-style API)
    - [ ] Rich repr and pretty-print for graphs (size, density, sample nodes)
    - [ ] Optional validation toggles (lazy and strict)

- **Community Detection Coverage**
    - [x] Louvain (returns list-of-lists of Python node IDs)
    - [x] Label propagation (returns dict[node] -> label)
    - [x] Girvan–Newman
    - [x] Spectral clustering
    - [x] Infomap (simplified)
    - [x] Personalized PageRank (vector)
    - [ ] NodeMap-style APIs exposed as dicts uniformly (standardize output forms)
    - [ ] Error-type differentiation for invalid params vs convergence (custom Python exceptions)
    - [ ] Percolation centrality (after Rust implementation)

- **Centrality and Metrics**
    - [x] Degree, closeness, betweenness (node) centralities
    - [x] Eigenvector, PageRank, Katz, harmonic centralities
    - [x] Local and global reaching, Laplacian, VoteRank
    - [ ] Edge betweenness exposure (currently only node-level in Python)
    - [ ] Personalized PageRank NodeMap wrapper (dict[node] -> score)
    - [ ] Uniform return type policy (dict vs list) documented and consolidated

- **Paths and Traversal**
    - [ ] Expose Dijkstra single-source distances (dict) and predecessors
    - [ ] Expose Bellman–Ford distances and negative cycle indicator
    - [ ] Expose BFS and DFS traversal order APIs (list[int])
    - [ ] A* and IDA* wrappers (with optional heuristic callbacks from Python)
    - [ ] Floyd–Warshall and Johnson’s (dense and sparse APSP) with memory-conscious streaming option

- **Subgraphs and Link Prediction**
    - [ ] Subgraph (induced) extraction returning a new PyGraph
    - [ ] Ego graph and k-hop extraction
    - [ ] Link prediction scores (RA, Jaccard, Adamic–Adar, CN, SH variants, WIC, and CNC)
    - [ ] Batched link scoring API (like list[(u,v)] -> scores) for performance

- **Approximation and Heuristics**
    - [ ] Greedy max clique, MIS, vertex cover, matching wrappers
    - [ ] Densest subgraph approximate routine
    - [ ] Treewidth heuristics (min-degree, min fill-in)
    - [ ] TSP approximation suite bindings (with path and length return)

- **Parallelism and Performance Tuning**
    - [x] Basic use of Rust parallel algorithms (using rayon internally)
    - [ ] Configurable thread limits from Python (`set_threads(n)`) where safe
    - [ ] Benchmarks comparing Python vs pure-Rust invocation overhead
    - [ ] Optional memory pooling enabling (if `pool` feature compiled) with explicit opt-in

- **Visualization**
    - [ ] Layout algorithms (force-directed and circular) returning coordinates
    - [ ] Export helpers (for SVG and PNG) directly callable from Python
    - [ ] `D3.js` JSON export wrapper
    - [ ] An HTML embedding helper

- **Packaging and Distribution**
    - [x] pyproject.toml based build using maturin/pyo3
    - [x] Version alignment with core crate
    - [x] Basic README and examples
    - [ ] Type hints (`.pyi` or inline) for all public APIs
    - [ ] Wheel builds for major platforms (Windows, macOS, and Linux)
    - [ ] Optional lite build (exclude heavy features like `visualization` and `parallel`)

- **Developer Experience**
    - [x] Consistent error conversion to Python exceptions
    - [ ] More granular Python exception classes (for example, `GraphinaInvalidGraphError` and
      `GraphinaConvergenceError`)
    - [ ] Docstring generation from Rust doc comments
    - [ ] MkDocs (Python-specific) user guide
    - [ ] Interactive Jupyter examples notebooks

- **Testing and Quality**
    - [x] Pytest integration for existing functionality
    - [ ] Property-based tests (hypothesis) for round-trip conversions
    - [ ] Fuzzing harness (optional) for edge list parsing
    - [ ] Coverage reports for Python bindings

- **Ecosystem Integration**
    - [ ] NetworkX interop utilities (read and export to `networkx.Graph`)
    - [ ] Pandas DataFrame import and export (edges and nodes)
    - [ ] Polars and Apache Arrow integration (optional feature)

- **Performance Benchmarks**
    - [ ] Benchmarks comparing PyGraphina with NetworkX (for centrality, etc.)
    - [ ] Memory allocation impact with and without pooling
    - [ ] Parallel scaling plots (threads vs throughput)

- **Documentation Enhancements**
    - [ ] Roadmap sync automation (derive from Rust core roadmap)
    - [ ] FAQ and troubleshooting section
    - [ ] Error catalogue (mapping Rust errors to Python exceptions)
