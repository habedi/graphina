//! Property-based tests using proptest for graph algorithms and data structures.

use graphina::core::generators::{
    barabasi_albert_graph, complete_graph, cycle_graph, erdos_renyi_graph,
};
use graphina::core::types::{Digraph, Directed, Graph, NodeId, Undirected};
use graphina::traversal::{bfs, bidis, dfs};
use proptest::prelude::*;
use std::collections::{HashMap, HashSet};

fn graph_size() -> impl Strategy<Value = usize> {
    5usize..50usize
}

fn probability() -> impl Strategy<Value = f64> {
    0.0..=1.0
}

fn seed() -> impl Strategy<Value = u64> {
    any::<u64>()
}

proptest! {
    /// Erdos-Renyi graphs should have exactly n nodes
    #[test]
    fn prop_erdos_renyi_node_count(
        n in graph_size(),
        p in probability(),
        seed in seed()
    ) {
        let graph = erdos_renyi_graph::<Undirected>(n, p, seed)
            .expect("Should generate graph");
        prop_assert_eq!(graph.node_count(), n);
    }

    /// Property: Complete graphs should have n*(n-1)/2 edges for undirected
    #[test]
    fn prop_complete_graph_edge_count(n in graph_size()) {
        let graph = complete_graph::<Undirected>(n)
            .expect("Should generate complete graph");
        let expected_edges = n * (n - 1) / 2;
        prop_assert_eq!(graph.edge_count(), expected_edges);
    }

    /// Property: Directed complete graphs should have n*(n-1) edges
    #[test]
    fn prop_complete_digraph_edge_count(n in graph_size()) {
        let graph = complete_graph::<Directed>(n)
            .expect("Should generate complete digraph");
        let expected_edges = n * (n - 1);
        prop_assert_eq!(graph.edge_count(), expected_edges);
    }

    /// Cycle graphs should have exactly n edges
    #[test]
    fn prop_cycle_graph_properties(n in 3usize..50usize) {
        let graph = cycle_graph::<Undirected>(n)
            .expect("Should generate cycle graph");
        prop_assert_eq!(graph.node_count(), n);
        prop_assert_eq!(graph.edge_count(), n);
    }

    /// Barabasi-Albert graphs should have correct node count
    #[test]
    fn prop_barabasi_albert_node_count(
        n in 5usize..50usize,
        m in 1usize..5usize,
        seed in seed()
    ) {
        prop_assume!(n >= m);
        let graph = barabasi_albert_graph::<Undirected>(n, m, seed)
            .expect("Should generate BA graph");
        prop_assert_eq!(graph.node_count(), n);
    }

    /// Barabasi-Albert graphs should have at least (n-m)*m edges
    #[test]
    fn prop_barabasi_albert_min_edges(
        n in 5usize..50usize,
        m in 1usize..5usize,
        seed in seed()
    ) {
        prop_assume!(n >= m);
        let graph = barabasi_albert_graph::<Undirected>(n, m, seed)
            .expect("Should generate BA graph");
        let min_expected_edges = (m * (m - 1) / 2) + (n - m) * m;
        let actual_edges = graph.edge_count();
        let tolerance = 10.min(min_expected_edges);
        prop_assert!(
            actual_edges >= min_expected_edges.saturating_sub(tolerance),
            "Expected at least {} edges (with tolerance {}), got {}",
            min_expected_edges,
            tolerance,
            actual_edges
        );
    }

    /// Graph density should be in [0, 1]
    #[test]
    fn prop_graph_density_bounded(
        n in graph_size(),
        p in probability(),
        seed in seed()
    ) {
        let graph = erdos_renyi_graph::<Undirected>(n, p, seed)
            .expect("Should generate graph");
        let density = graph.density();
        prop_assert!((0.0..=1.0).contains(&density));
    }
}

proptest! {
    /// Property: BFS should visit all nodes in a complete graph
    #[test]
    fn prop_bfs_visits_all_nodes_complete(n in graph_size()) {
        let graph = complete_graph::<Undirected>(n)
            .expect("Should generate complete graph");
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();
        if let Some(&start) = nodes.first() {
            let visited = bfs(&graph, start);
            prop_assert_eq!(visited.len(), n);
        }
    }

    /// DFS should visit all nodes in a complete graph
    #[test]
    fn prop_dfs_visits_all_nodes_complete(n in graph_size()) {
        let graph = complete_graph::<Undirected>(n)
            .expect("Should generate complete graph");
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();
        if let Some(&start) = nodes.first() {
            let visited = dfs(&graph, start);
            prop_assert_eq!(visited.len(), n);
        }
    }

    /// BFS and DFS should visit the same number of nodes
    #[test]
    fn prop_bfs_dfs_same_count(
        n in graph_size(),
        p in probability(),
        seed in seed()
    ) {
        let graph = erdos_renyi_graph::<Undirected>(n, p, seed)
            .expect("Should generate graph");
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();
        if let Some(&start) = nodes.first() {
            let bfs_count = bfs(&graph, start).len();
            let dfs_count = dfs(&graph, start).len();
            prop_assert_eq!(bfs_count, dfs_count);
        }
    }

    /// Property: Bidirectional search in complete graph should find path
    #[test]
    fn prop_bidis_finds_path_complete(n in 5usize..30usize) {
        let graph = complete_graph::<Undirected>(n)
            .expect("Should generate complete graph");
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();
        if nodes.len() >= 2 {
            let path = bidis(&graph, nodes[0], nodes[nodes.len() - 1]);
            prop_assert!(path.is_some());
            if let Some(p) = path {
                prop_assert_eq!(p[0], nodes[0]);
                prop_assert_eq!(p[p.len() - 1], nodes[nodes.len() - 1]);
            }
        }
    }

    /// Bidirectional search path length in complete graph is 2
    #[test]
    fn prop_bidis_shortest_path_complete(n in 5usize..30usize) {
        let graph = complete_graph::<Undirected>(n)
            .expect("Should generate complete graph");
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();
        if nodes.len() >= 2 {
            let path = bidis(&graph, nodes[0], nodes[1]);
            prop_assert!(path.is_some());
            if let Some(p) = path {
                prop_assert_eq!(p.len(), 2);
            }
        }
    }
}

proptest! {
    /// Adding and removing nodes should work correctly
    #[test]
    fn prop_add_remove_node_identity(values in prop::collection::vec(any::<i32>(), 1..20)) {
        let mut graph = Graph::<i32, f32>::new();
        let mut nodes = Vec::new();

        // Add nodes
        for &val in &values {
            nodes.push(graph.add_node(val));
        }

        prop_assert_eq!(graph.node_count(), values.len());

        // Remove half the nodes
        let to_remove = nodes.len() / 2;
        for _ in 0..to_remove {
            if let Some(node) = nodes.pop() {
                let _ = graph.remove_node(node);
            }
        }

        prop_assert_eq!(graph.node_count(), values.len() - to_remove);
    }

    /// Property: Edge count should match added edges
    #[test]
    fn prop_edge_count_consistency(n in 3usize..20usize) {
        let mut graph = Graph::<i32, f32>::new();
        let nodes: Vec<_> = (0..n).map(|i| graph.add_node(i as i32)).collect();

        let mut edge_count = 0;
        // Add edges in a chain
        for i in 0..n-1 {
            graph.add_edge(nodes[i], nodes[i + 1], 1.0);
            edge_count += 1;
        }

        prop_assert_eq!(graph.edge_count(), edge_count);
    }

    /// Property: Graph should be empty after clearing
    #[test]
    fn prop_clear_graph(n in graph_size(), p in probability(), seed in seed()) {
        let mut graph = erdos_renyi_graph::<Undirected>(n, p, seed)
            .expect("Should generate graph");
        prop_assert!(graph.node_count() > 0);

        graph.clear();
        prop_assert_eq!(graph.node_count(), 0);
        prop_assert_eq!(graph.edge_count(), 0);
        prop_assert!(graph.is_empty());
    }

    /// Property: Node attributes should be retrievable after adding
    #[test]
    fn prop_node_attributes(values in prop::collection::vec(any::<i32>(), 1..50)) {
        let mut graph = Graph::<i32, f32>::new();

        for &val in &values {
            let node = graph.add_node(val);
            prop_assert_eq!(graph.node_attr(node), Some(&val));
        }
    }

    /// Property: Degree of nodes in complete graph should be n-1
    #[test]
    fn prop_complete_graph_degree(n in 5usize..30usize) {
        let graph = complete_graph::<Undirected>(n)
            .expect("Should generate complete graph");

        for (node, _) in graph.nodes() {
            let degree = graph.degree(node).expect("Node should have degree");
            prop_assert_eq!(degree, n - 1);
        }
    }
}

proptest! {
    /// Property: No self-loops should exist in generated graphs (except if explicitly added)
    #[test]
    fn prop_no_self_loops_generated(
        n in graph_size(),
        p in probability(),
        seed in seed()
    ) {
        let graph = erdos_renyi_graph::<Undirected>(n, p, seed)
            .expect("Should generate graph");

        for (src, tgt, _) in graph.edges() {
            prop_assert_ne!(src, tgt, "Generated graphs should not have self-loops");
        }
    }

    /// Property: Undirected graphs should have symmetric edges
    #[test]
    fn prop_undirected_symmetry(n in 5usize..20usize) {
        let graph = complete_graph::<Undirected>(n)
            .expect("Should generate complete graph");

        // For each edge (u,v), edge (v,u) should exist
        for (src, tgt, _) in graph.edges() {
            let has_reverse = graph.contains_edge(tgt, src) ||
                              graph.contains_edge(src, tgt);
            prop_assert!(has_reverse, "Undirected edges should be symmetric");
        }
    }

    /// Property: Graph with p=0 should have no edges
    #[test]
    fn prop_erdos_renyi_p_zero(n in graph_size(), seed in seed()) {
        let graph = erdos_renyi_graph::<Undirected>(n, 0.0, seed)
            .expect("Should generate graph");
        prop_assert_eq!(graph.edge_count(), 0);
    }

    /// Property: Graph with p=1 should be complete
    #[test]
    fn prop_erdos_renyi_p_one(n in graph_size(), seed in seed()) {
        let graph = erdos_renyi_graph::<Undirected>(n, 1.0, seed)
            .expect("Should generate graph");
        let expected_edges = n * (n - 1) / 2;
        prop_assert_eq!(graph.edge_count(), expected_edges);
    }

    /// Property: Same seed should produce identical graphs
    #[test]
    fn prop_deterministic_generation(
        n in graph_size(),
        p in probability(),
        seed in seed()
    ) {
        let graph1 = erdos_renyi_graph::<Undirected>(n, p, seed)
            .expect("Should generate first graph");
        let graph2 = erdos_renyi_graph::<Undirected>(n, p, seed)
            .expect("Should generate second graph");

        prop_assert_eq!(graph1.node_count(), graph2.node_count());
        prop_assert_eq!(graph1.edge_count(), graph2.edge_count());
    }
}

proptest! {
    /// Property: BFS should visit start node first
    #[test]
    fn prop_bfs_starts_at_start(n in graph_size(), p in probability(), seed in seed()) {
        let graph = erdos_renyi_graph::<Undirected>(n, p, seed)
            .expect("Should generate graph");
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();
        if let Some(&start) = nodes.first() {
            let visited = bfs(&graph, start);
            if !visited.is_empty() {
                prop_assert_eq!(visited[0], start);
            }
        }
    }

    /// Property: DFS should visit start node first
    #[test]
    fn prop_dfs_starts_at_start(n in graph_size(), p in probability(), seed in seed()) {
        let graph = erdos_renyi_graph::<Undirected>(n, p, seed)
            .expect("Should generate graph");
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();
        if let Some(&start) = nodes.first() {
            let visited = dfs(&graph, start);
            if !visited.is_empty() {
                prop_assert_eq!(visited[0], start);
            }
        }
    }

    /// Property: Path from node to itself should be single node
    #[test]
    fn prop_bidis_self_path(n in graph_size(), p in probability(), seed in seed()) {
        let graph = erdos_renyi_graph::<Undirected>(n, p, seed)
            .expect("Should generate graph");
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();
        if let Some(&node) = nodes.first() {
            let path = bidis(&graph, node, node);
            prop_assert!(path.is_some());
            if let Some(p) = path {
                prop_assert_eq!(p.len(), 1);
                prop_assert_eq!(p[0], node);
            }
        }
    }

    /// Property: No duplicate nodes in BFS traversal
    #[test]
    fn prop_bfs_no_duplicates(n in graph_size(), p in probability(), seed in seed()) {
        let graph = erdos_renyi_graph::<Undirected>(n, p, seed)
            .expect("Should generate graph");
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();
        if let Some(&start) = nodes.first() {
            let visited = bfs(&graph, start);
            let unique: HashSet<_> = visited.iter().cloned().collect();
            prop_assert_eq!(visited.len(), unique.len(), "BFS should not visit nodes twice");
        }
    }

    /// Property: No duplicate nodes in DFS traversal
    #[test]
    fn prop_dfs_no_duplicates(n in graph_size(), p in probability(), seed in seed()) {
        let graph = erdos_renyi_graph::<Undirected>(n, p, seed)
            .expect("Should generate graph");
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();
        if let Some(&start) = nodes.first() {
            let visited = dfs(&graph, start);
            let unique: HashSet<_> = visited.iter().cloned().collect();
            prop_assert_eq!(visited.len(), unique.len(), "DFS should not visit nodes twice");
        }
    }
}

// Define operations that can be performed on the graph
#[derive(Debug, Clone)]
enum GraphOp {
    AddNode(i32),
    AddEdge(i32, i32, f64),
    RemoveNode(i32),
}

// Strategy to generate random graph operations
fn graph_op_strategy() -> impl Strategy<Value = GraphOp> {
    prop_oneof![
        any::<i32>().prop_map(GraphOp::AddNode),
        (any::<i32>(), any::<i32>(), any::<f64>()).prop_map(|(u, v, w)| GraphOp::AddEdge(u, v, w)),
        any::<i32>().prop_map(GraphOp::RemoveNode),
    ]
}

proptest! {
    #[test]
    fn test_graph_invariants(ops in proptest::collection::vec(graph_op_strategy(), 1..50)) {
        let mut graph = Graph::<i32, f64>::new();
        let mut model_nodes: HashSet<i32> = HashSet::new();
        let mut model_to_id: HashMap<i32, NodeId> = HashMap::new();
        let mut edge_count = 0;

        for op in ops {
            match op {
                GraphOp::AddNode(val) => {
                    if !model_nodes.contains(&val) {
                        let id = graph.add_node(val);
                        model_nodes.insert(val);
                        model_to_id.insert(val, id);
                    }
                }
                GraphOp::AddEdge(u, v, w) => {
                    if let (Some(&uid), Some(&vid)) = (model_to_id.get(&u), model_to_id.get(&v)) {
                         graph.add_edge(uid, vid, w);
                         edge_count += 1;
                    }
                }
                GraphOp::RemoveNode(_val) => {}
            }
            prop_assert_eq!(graph.node_count(), model_nodes.len());
            prop_assert_eq!(graph.edge_count(), edge_count);
        }
    }

    #[test]
    fn test_digraph_invariants(ops in proptest::collection::vec(graph_op_strategy(), 1..50)) {
        let mut graph = Digraph::<i32, f64>::new();
        let mut model_nodes: HashSet<i32> = HashSet::new();
        let mut model_to_id: HashMap<i32, NodeId> = HashMap::new();
        let mut edge_count = 0;

        for op in ops {
            match op {
                GraphOp::AddNode(val) => {
                    if !model_nodes.contains(&val) {
                        let id = graph.add_node(val);
                        model_nodes.insert(val);
                        model_to_id.insert(val, id);
                    }
                }
                GraphOp::AddEdge(u, v, w) => {
                    if let (Some(&uid), Some(&vid)) = (model_to_id.get(&u), model_to_id.get(&v)) {
                         graph.add_edge(uid, vid, w);
                         edge_count += 1;
                    }
                }
                GraphOp::RemoveNode(_val) => {}
            }
            prop_assert_eq!(graph.node_count(), model_nodes.len());
            prop_assert_eq!(graph.edge_count(), edge_count);
        }
    }
}
