use libfuzzer_sys::fuzz_target;
use graphina::core::types::Graph;

#[cfg(feature = "community")]
use graphina::community::louvain::louvain;

#[cfg(feature = "community")]
fuzz_target!(|data: &[u8]| {
    if data.len() < 4 {
        return;
    }

    let num_nodes = (data[0] as usize % 30) + 2; // 2-31 nodes
    let num_edges = (data[1] as usize % 50);

    let mut graph = Graph::<i32, f64>::new();
    let nodes: Vec<_> = (0..num_nodes).map(|i| graph.add_node(i as i32)).collect();

    // Add edges
    for i in 0..num_edges.min((data.len() - 2) / 2) {
        let src_idx = data[2 + i * 2] as usize % nodes.len();
        let tgt_idx = data[3 + i * 2] as usize % nodes.len();
        let weight = ((data[2 + i * 2] as f64) + 1.0).abs();

        graph.add_edge(nodes[src_idx], nodes[tgt_idx], weight);
    }

    // Remove some nodes to test the index mapping fix
    if nodes.len() > 5 && data.len() > 10 {
        let remove_idx = data[10] as usize % nodes.len();
        let _ = graph.remove_node(nodes[remove_idx]);
    }

    // Run Louvain - should not panic even with removed nodes
    let communities = louvain(&graph, Some(42));

    // Verify results are sane
    assert!(!communities.is_empty() || graph.node_count() == 0);

    // Check all nodes are accounted for
    let total_nodes: usize = communities.iter().map(|c| c.len()).sum();
    assert_eq!(total_nodes, graph.node_count());
});

#[cfg(not(feature = "community"))]
fn main() {
    eprintln!("Fuzzing louvain requires the 'community' feature");
}
use libfuzzer_sys::fuzz_target;
use graphina::core::types::Graph;

fuzz_target!(|data: &[u8]| {
    if data.len() < 4 {
        return;
    }

    // Parse the fuzzing input
    let num_nodes = (data[0] as usize % 50) + 1; // 1-50 nodes
    let num_edges = (data[1] as usize % 100); // 0-99 edges

    let mut graph = Graph::<i32, f64>::new();

    // Add nodes
    let nodes: Vec<_> = (0..num_nodes).map(|i| graph.add_node(i as i32)).collect();

    // Add edges based on fuzzing data
    for i in 0..num_edges.min((data.len() - 2) / 2) {
        let src_idx = data[2 + i * 2] as usize % nodes.len();
        let tgt_idx = data[3 + i * 2] as usize % nodes.len();
        let weight = ((data[2 + i * 2] ^ data[3 + i * 2]) as f64) / 255.0;

        graph.add_edge(nodes[src_idx], nodes[tgt_idx], weight);
    }

    // Test basic operations don't panic
    let _ = graph.node_count();
    let _ = graph.edge_count();
    let _ = graph.is_empty();
    let _ = graph.density();

    // Test all nodes have valid degrees
    for (node, _) in graph.nodes() {
        let _ = graph.degree(node);
        let _ = graph.in_degree(node);
        let _ = graph.out_degree(node);

        // Test neighbor iteration doesn't panic
        for _ in graph.neighbors(node) {}
    }

    // Test edge iteration
    for _ in graph.edges() {}

    // Test node removal (if we have nodes)
    if !nodes.is_empty() {
        let mut g_copy = graph.clone();
        let node_to_remove = nodes[data[0] as usize % nodes.len()];
        let _ = g_copy.remove_node(node_to_remove);
    }
});

