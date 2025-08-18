use graphina::core::types::Digraph;

use graphina::centrality::algorithms::katz_centrality;

fn main() {
    let mut graph = Digraph::new();
    let ids = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();
    let edges = [(0, 1, 1.0), (0, 2, 2.0), (1, 3, 1.0)];
    for (s, d, w) in edges {
        graph.add_edge(ids[s], ids[d], w);
    }

    let params = [
        (0.1, 1.0, 1000, false, true),
        (0.1, 1.0, 1000, false, false),
        (0.01, 0.5, 1000, true, false),
        (0.01, 0.5, 1000, true, true),
    ];

    for (alpha, beta, max_iter, weighted, normalized) in params {
        let centrality = katz_centrality(&graph, alpha, beta, max_iter, weighted, normalized);
        println!(
            "alpha: {:.3}, beta: {:.3}, max iter: {:>5}, weighted: {}, normalized: {}",
            alpha, beta, max_iter, weighted, normalized
        );
        for (n, attr) in graph.nodes() {
            println!(">> {} : {:.5}", attr, centrality[&n])
        }
        println!();
    }
}
