# Core Module Bugs and API Inconsistencies

## Critical Issues Found

### 1. **Bidirectional Search Bug - Incorrect Path Reconstruction** ⚠️ HIGH SEVERITY

**Location:** `src/core/traversal.rs` - `bidis()` function (lines 330-360)

**Issue:** The bidirectional search has a logic error in path reconstruction. The function checks for intersection after expanding each frontier, but the intersection check can find nodes that weren't actually part of the shortest path meeting point.

**Problem Code:**
```rust
// After expanding forward frontier
if let Some(&meet) = forward_visited.intersection(&backward_visited).next() {
    meeting_node = Some(meet);
    break;
}
```

**Impact:** May return suboptimal or incorrect paths when the meeting node is discovered in the visited set but not necessarily the shortest meeting point.

**Recommendation:** Track the actual frontier (nodes added in current iteration) separately from visited set.

---

### 2. **Watts-Strogatz Graph Generator - Duplicate Edges Not Prevented** ⚠️ MEDIUM SEVERITY

**Location:** `src/core/generators.rs` - `watts_strogatz_graph()` function

**Issue:** When rewiring edges, the algorithm can create duplicate edges (multiple edges between the same pair of nodes) because it doesn't check if an edge already exists before adding a new one.

**Problem Code:**
```rust
loop {
    new_target = rng.random_range(0..n);
    if new_target != i {
        break;
    }
}
graph.add_edge(nodes[i], nodes[new_target], 1.0);
```

**Impact:** Generated graphs may have multiple edges between node pairs, which is incorrect for simple graphs.

**Fix Needed:** Check if edge already exists before adding:
```rust
if graph.find_edge(nodes[i], nodes[new_target]).is_none() {
    graph.add_edge(nodes[i], nodes[new_target], 1.0);
}
```

---

### 3. **Barabási-Albert Graph Generator - Inefficient and Potentially Infinite Loop** ⚠️ HIGH SEVERITY

**Location:** `src/core/generators.rs` - `barabasi_albert_graph()` function

**Issue:** The target selection loop can run indefinitely if the random selection keeps picking nodes already in the targets list.

**Problem Code:**
```rust
while targets.len() < m {
    let r = rng.random_range(0..total_degree);
    let mut cumulative = 0;
    for (idx, &deg) in degrees.iter().enumerate() {
        cumulative += deg;
        if r < cumulative {
            if !targets.contains(&nodes[idx]) {  // <-- Can retry forever
                targets.push(nodes[idx]);
            }
            break;
        }
    }
}
```

**Impact:** 
- Infinite loop possible with high probability as graph grows
- Very inefficient even when it terminates
- May hang applications

**Fix Needed:** Use rejection sampling with fallback or weighted selection without replacement.

---

### 4. **Get Backward Neighbors - Inefficient for Directed Graphs** ⚠️ MEDIUM SEVERITY

**Location:** `src/core/traversal.rs` - `get_backward_neighbors()` function

**Issue:** For directed graphs, this function iterates through ALL edges in the graph to find incoming edges to a single node. This is O(E) per call.

**Problem Code:**
```rust
graph
    .edges()
    .filter(|(_, tgt, _)| *tgt == node)
    .map(|(src, _, _)| src)
    .collect()
```

**Impact:** Bidirectional search on directed graphs has O(E * V) complexity instead of O(E + V).

**Recommendation:** Add reverse edge index to graph type or cache reverse edges in bidirectional search.

---

### 5. **I/O Adjacency List - Weight Parsing Ambiguity** ⚠️ LOW SEVERITY

**Location:** `src/core/io.rs` - `read_adjacency_list()` function

**Issue:** When processing adjacency lists, if the number of tokens is odd (excluding the source), the last neighbor will get default weight 1.0, which might be unintentional.

**Problem Code:**
```rust
let weight: f32 = if i + 1 < tokens.len() {
    tokens[i + 1].parse()?
} else {
    1.0  // <-- Silent default
};
```

**Impact:** Data loss - missing weights are silently defaulted instead of reporting an error.

**Recommendation:** Return error or warning when weight is missing in the middle of the list.

---

## API Inconsistencies

### 1. **Inconsistent Return Types for "Not Found" Cases**

**Issue:** Different functions use different approaches for "not found":
- `bfs()` / `dfs()` - Return complete traversal (no failure case)
- `iddfs()` - Returns `Option<Vec<NodeId>>`
- `bidis()` - Returns `Option<Vec<NodeId>>`
- `dijkstra()` - Returns `Result<Vec<Option<W>>>`

**Recommendation:** For alpha stage, this is acceptable but should be documented as design decision.

---

### 2. **Graph Generator Type Constraint Inconsistency**

**Issue:** All graph generators hardcode types to `BaseGraph<u32, f32, Ty>`, but user code typically uses other types.

**Example:**
```rust
pub fn erdos_renyi_graph<Ty: GraphConstructor<u32, f32>>(
    n: usize, p: f64, seed: u64
) -> Result<BaseGraph<u32, f32, Ty>, GraphinaException>
```

**Impact:** Users must convert or work with u32/f32 types even if their domain uses different types.

**Recommendation:** Consider making generators generic over node/edge types with default trait bounds.

---

### 3. **Missing Validation in Traversal Functions**

**Issue:** BFS and DFS don't validate that the start node exists in the graph.

**Impact:** If user passes invalid NodeId, behavior is undefined (likely returns empty traversal).

**Recommendation:** Add node existence validation or document preconditions.

---

### 4. **Borůvka MST Uses Rayon but Others Don't**

**Issue:** Only Borůvka's algorithm uses parallel processing (Rayon), while Prim's and Kruskal's are sequential.

**Impact:** Inconsistent performance characteristics; users might not expect parallelism.

**Recommendation:** Document parallelism clearly or make it optional via feature flag.

---

## Design Improvements for Alpha Stage

### 1. **Add Graph Validation Helper**
```rust
pub fn validate_node(&self, node: NodeId) -> Result<(), GraphinaException> {
    if self.node_attr(node).is_none() {
        return Err(GraphinaException::new("Node does not exist"));
    }
    Ok(())
}
```

### 2. **Add Builder Pattern for Graph Generators**
```rust
GraphGenerator::erdos_renyi()
    .nodes(100)
    .probability(0.1)
    .seed(42)
    .build()
```

### 3. **Consistent Error Handling**
Consider standardizing on either:
- `Option` for simple cases
- `Result` for all fallible operations
- Both with clear naming (`find_*` vs `try_*`)

### 4. **Add Precondition Checks**
For algorithms that require specific graph properties:
```rust
pub fn dijkstra<A, W, Ty>(
    graph: &BaseGraph<A, W, Ty>,
    source: NodeId,
) -> Result<Vec<Option<W>>, GraphinaException> {
    // Validate source exists
    if graph.node_attr(source).is_none() {
        return Err(GraphinaException::new("Source node not found"));
    }
    // ... rest of implementation
}
```

---

## Summary Statistics

**Critical Bugs:** 2 (Barabási-Albert infinite loop, bidirectional search path)
**Medium Severity:** 2 (Watts-Strogatz duplicates, backward neighbors inefficiency)
**Low Severity:** 1 (I/O weight parsing)
**API Inconsistencies:** 4

**Total Issues:** 9

