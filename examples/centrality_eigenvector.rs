use graphina::core::types::Graph;

use graphina::centrality::algorithms::eigenvector_centrality;

fn main() {
    let mut graph = Graph::new();
    let ids = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();
    let edges = [(0, 1, 1.0), (0, 2, 2.0), (1, 3, 1.0)];
    for (s, d, w) in edges {
        graph.add_edge(ids[s], ids[d], w);
    }

    let centrality = eigenvector_centrality(&graph, 1000, false);
    println!("Unweighted",);
    for (n, attr) in graph.nodes() {
        println!(">> {} : {:.5}", attr, centrality[&n])
    }
    println!();
    let centrality = eigenvector_centrality(&graph, 1000, true);
    println!("Weighted",);
    for (n, attr) in graph.nodes() {
        println!(">> {} : {:.5}", attr, centrality[&n])
    }
    println!();
}
