## Graphina Feature Roadmap

This document includes the roadmap for the Graphina project.
It outlines features to be implemented and their current status.

> [!IMPORTANT]
> This roadmap is a work in progress and is subject to change.

- **Core Library (Graph Types, I/O, Validation)**
    - [x] Directed and undirected graph types (weighted and unweighted)
    - [x] NodeId and EdgeId wrappers, NodeMap, EdgeMap, and OrderedNodeMap
    - [x] Graph builders (advanced/topology builders)
    - [x] Validation utilities (connectivity, DAG, bipartite, negative weights, and self-loops)
    - [x] Simple I/O: edge and adjacency list formats
    - [x] Serialization: JSON, binary, and GraphML
    - [x] Experimental memory pooling utilities

- **Shortest Paths and Traversal**
    - [x] Dijkstra (single-source) and Dijkstra path tracing
    - [x] Bellman–Ford (negative weights) and path tracing
    - [x] Floyd–Warshall (all-pairs)
    - [x] Johnson’s algorithm (all-pairs on sparse graphs)
    - [x] A* and IDA*
    - [x] BFS, DFS, IDDFS, and bidirectional search

- **Centrality (Node/Edge Importance)**
    - [x] Degree, Closeness, Betweenness (node and edge)
    - [x] Eigenvector, PageRank, Katz, and Harmonic centralities
    - [x] Personalized PageRank (vector plus NodeMap facade API)
    - [x] Local and Global reaching centralities
    - [x] Laplacian centrality
    - [x] VoteRank (seed selector)
    - [ ] Percolation centrality

- **Community Detection and Clustering**
    - [x] Label propagation
    - [x] Louvain method
    - [x] Girvan–Newman
    - [x] Spectral embeddings and spectral clustering
    - [x] Infomap (simplified version)
    - [x] Personalized PageRank (community usage)
    - [x] Connected components

- **Minimum Spanning Trees (MST)**
    - [x] Prim’s algorithm
    - [x] Kruskal’s algorithm
    - [x] Borůvka’s algorithm

- **Subgraphs and Links**
    - [x] Subgraph extraction (induced, ego graph, and k-hop)
    - [x] Filter nodes and edges; get the component subgraph
    - [x] Link prediction (RA, Jaccard, Adamic–Adar, CN, SH variants, WIC, and CNC)

- **Approximation and Heuristics**
    - [x] Maximum independent set (greedy)
    - [x] Maximum clique (greedy heuristic), clique removal
    - [x] Large clique size
    - [x] Approx. clustering coefficient (sampling)
    - [x] Densest subgraph (greedy peeling)
    - [x] Diameter lower bound
    - [x] Minimum vertex cover (greedy)
    - [x] Minimum maximal matching (greedy)
    - [x] Ramsey R(2, t) approximation
    - [x] TSP approximations (greedy, SA, TA, and Christofides)
    - [x] Treewidth decompositions (min-degree and min fill-in)

- **Parallel Algorithms**
    - [x] Parallel BFS
    - [x] Parallel degree and clustering coefficients
    - [x] Parallel triangles counting
    - [x] Parallel PageRank
    - [x] Parallel shortest paths
    - [x] Parallel connected components

- **Visualization**
    - [x] ASCII visualization
    - [x] Force-directed, circular, grid, hierarchical layouts
    - [x] HTML generation, SVG export, PNG export
    - [x] D3.js JSON

- **API and Developer Experience**
    - [x] Unified error handling via `GraphinaError` and `Result` returns (community and many centrality algorithms)
    - [x] Public re-exports and facades for consistent entry points (like personalized PageRank)
    - [x] Gate low-level modules (like `core::pool`) behind feature flags; marked experimental
    - [x] Crate-level documentation summarizing modules, features, and conventions
    - [ ] Expanded conversion helpers (vector <-> NodeMap, typed adapters) across modules
    - [ ] Finer-grained error variants for convergence vs invalid argument
    - [ ] Stability policy and semver guarantees for public APIs

- **Ecosystem and Bindings**
    - [x] Python bindings (`PyGraphina`) with community and centrality coverage
    - [ ] Ensure parity and ergonomic exceptions across bindings
    - [ ] Interoperability helpers (like import and export formats compatible with `NetworkX` and `rustworkx`)
    - [ ] WebAssembly support

- **Data and Real-World Graphs**
    - [x] Integration tests referencing public datasets
    - [ ] Optional dataset helper scripts with docs for getting the test datasets

- **Benchmarks and Performance**
    - [x] Benchmarks for algorithms, graphs, and project-level scenarios
    - [ ] Micro-benchmarks for pooling and hot paths under `--features pool`
    - [ ] Profiling guides and performance tuning tips in docs

- **Code Quality, CI, and Documentation**
    - [x] CI for builds and tests; code coverage reporting
