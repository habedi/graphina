#![cfg_attr(not(feature = "centrality"), allow(dead_code, unused_imports))]

#[cfg(feature = "centrality")]
fn main() {
    use graphina::centrality::eigenvector::eigenvector_centrality;
    use graphina::core::types::Graph;

    let mut graph = Graph::new();
    let ids = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();
    let edges = [(0, 1, 1.0), (0, 2, 2.0), (1, 3, 1.0)];
    for (s, d, w) in edges {
        graph.add_edge(ids[s], ids[d], w);
    }

    let centrality =
        eigenvector_centrality(&graph, 1000, 1e-6_f64).expect("eigenvector centrality failed");
    println!("Eigenvector centrality (tolerance=1e-6):");
    for (n, attr) in graph.nodes() {
        println!(">> {} : {:.5}", attr, centrality[&n])
    }
    println!();
}

#[cfg(not(feature = "centrality"))]
fn main() {
    eprintln!(
        "This example requires the 'centrality' feature. Run with:\n  cargo run --example centrality_eigenvector --features centrality"
    );
}
