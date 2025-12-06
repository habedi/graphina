#![cfg_attr(not(feature = "centrality"), allow(dead_code, unused_imports))]

#[cfg(feature = "centrality")]
fn main() {
    use graphina::centrality::katz::katz_centrality;
    use graphina::core::types::Digraph;

    let mut graph = Digraph::new();
    let ids = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();
    let edges = [(0, 1, 1.0), (0, 2, 2.0), (1, 3, 1.0)];
    for (s, d, w) in edges {
        graph.add_edge(ids[s], ids[d], w);
    }

    // (alpha, beta_scalar, max_iter)
    let params = [(0.1, 1.0, 1000), (0.01, 0.5, 1000)];

    for (alpha, beta_scalar, max_iter) in params {
        let centrality = katz_centrality(
            &graph,
            alpha,
            Some(&move |_| beta_scalar),
            max_iter,
            1e-6_f64,
        )
        .expect("Katz centrality computation failed");
        println!(
            "alpha: {:.3}, beta: {:.3}, max iter: {:>5}",
            alpha, beta_scalar, max_iter
        );
        for (n, attr) in graph.nodes() {
            println!(">> {} : {:.5}", attr, centrality[&n])
        }
        println!();
    }
}

#[cfg(not(feature = "centrality"))]
fn main() {
    eprintln!(
        "This example requires the 'centrality' feature. Run with:\n  cargo run --example centrality_katz --features centrality"
    );
}
