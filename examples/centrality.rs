//
// -----------------------------
// Degree Centrality
// -----------------------------
//

fn degree_centrality_example() {
    use graphina::core::types::Graph;

    use graphina::centrality::algorithms::degree_centrality;

    let mut g: Graph<i32, ()> = Graph::new();
    let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
    g.add_edge(nodes[0], nodes[1], ());
    g.add_edge(nodes[0], nodes[2], ());

    let centrality = degree_centrality(&g);
    println!("{:?}", centrality); // [2.0, 1.0, 1.0]
}

fn in_degree_centrality_example() {
    use graphina::core::types::Digraph;

    use graphina::centrality::algorithms::in_degree_centrality;

    let mut g: Digraph<i32, ()> = Digraph::new();
    let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
    g.add_edge(nodes[0], nodes[1], ());
    g.add_edge(nodes[0], nodes[2], ());

    let centrality = in_degree_centrality(&g);
    println!("{:?}", centrality); // [0.0, 1.0, 1.0]
}

fn out_degree_centrality_example() {
    use graphina::core::types::Digraph;

    use graphina::centrality::algorithms::out_degree_centrality;

    let mut g: Digraph<i32, ()> = Digraph::new();
    let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
    g.add_edge(nodes[0], nodes[1], ());
    g.add_edge(nodes[0], nodes[2], ());

    let centrality = out_degree_centrality(&g);
    println!("{:?}", centrality); // [2.0, 0.0, 0.0]
}

//
// -----------------------------
// Eigen Vector Centrality
// -----------------------------
//

fn eigenvector_centrality_example() {
    use graphina::core::types::Graph;

    use graphina::centrality::algorithms::eigenvector_centrality;

    let mut g = Graph::new();
    let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
    g.add_edge(nodes[0], nodes[1], 1.0);
    g.add_edge(nodes[0], nodes[2], 2.0);

    let centrality = eigenvector_centrality(&g, 1000, false);
    println!("{:.5?}", centrality); // [0.70711, 0.50000, 0.50000]
    let centrality = eigenvector_centrality(&g, 1000, true);
    println!("{:.5?}", centrality); // [0.70711, 0.31623, 0.63246]
}

fn eigenvector_centrality_numpy_example() {
    use graphina::centrality::algorithms::eigenvector_centrality_numpy;
    use graphina::core::types::Graph;
    let mut g = Graph::new();

    let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
    g.add_edge(nodes[0], nodes[1], 1.0);
    g.add_edge(nodes[0], nodes[2], 2.0);

    let centrality = eigenvector_centrality_numpy(&g, 1000, 1e-6_f64, false);
    println!("{:.5?}", centrality); // [0.70711, 0.50000, 0.50000]
    let centrality = eigenvector_centrality_numpy(&g, 1000, 1e-6_f64, true);
    println!("{:.5?}", centrality); // [0.70711, 0.31623, 0.63246]
}

fn eigenvector_centrality_impl_example() {
    use graphina::centrality::algorithms::eigenvector_centrality_impl;
    use graphina::core::types::Graph;

    let mut g: Graph<i32, (f64, f64)> = Graph::new();
    //                    ^^^^^^^^^^
    //                             L arbitrary type as edge
    let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
    g.add_edge(nodes[0], nodes[1], (0.0, 1.0));
    g.add_edge(nodes[0], nodes[2], (1.0, 0.0));
    let centrality = eigenvector_centrality_impl(
        &g,
        1000,
        1e-6_f64,
        |w| w.0 * 10.0 + w.1, // <-- custom evaluation for edge weight
    );
    println!("{:.5?}", centrality); // [0.70711, 0.07036, 0.70360]
}

//
// -----------------------------
// Katz Centrality
// -----------------------------
//

fn katz_centrality_example() {
    use graphina::centrality::algorithms::katz_centrality;
    use graphina::core::types::Graph;

    let mut g = Graph::new();
    let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
    g.add_edge(nodes[0], nodes[1], 1.0);
    g.add_edge(nodes[0], nodes[2], 2.0);

    let centrality = katz_centrality(&g, 0.1, 1.0, 1000, false, true);
    println!("{:.5?}", centrality); // [0.61078, 0.55989, 0.55989]
    let centrality = katz_centrality(&g, 0.01, 0.5, 1000, true, true);
    println!("{:.5?}", centrality); // [0.58301, 0.57158, 0.57741]
}

fn katz_centrality_numpy_example() {
    use graphina::centrality::algorithms::katz_centrality_numpy;
    use graphina::core::types::Graph;

    let mut g = Graph::new();
    let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
    g.add_edge(nodes[0], nodes[1], 1.0);
    g.add_edge(nodes[0], nodes[2], 2.0);

    let centrality = katz_centrality_numpy(&g, 0.1, 1.0, false, true);
    println!("{:.5?}", centrality); // [0.61078, 0.55989, 0.55989]
    let centrality = katz_centrality_numpy(&g, 0.01, 0.5, true, true);
    println!("{:.5?}", centrality); // [0.58301, 0.57158, 0.57741]
}

fn katz_centrality_impl_example() {
    use graphina::centrality::algorithms::katz_centrality_impl;
    use graphina::core::types::Graph;

    let mut g: Graph<(i32, f64), (f64, f64)> = Graph::new();
    //               ^^^^^^^^^^  ^^^^^^^^^^
    //                        |           L arbitrary type as edge
    //                        L arbitrary type as node
    let nodes = [
        g.add_node((1, 2.0)),
        g.add_node((2, 3.0)),
        g.add_node((3, 2.0)),
    ];
    g.add_edge(nodes[0], nodes[1], (0.0, 1.0));
    g.add_edge(nodes[0], nodes[2], (1.0, 0.0));

    let centrality = katz_centrality_impl(
        &g,
        |_n| 0.01,                        // <-- custom alpha depend on node attribute
        |(i, f): &(i32, f64)| f.powi(*i), // <-- custom beta depend on node attribute
        1_000,
        1e-6_f64,
        true,
        |w| w.0 * 10.0 + w.1, // <-- custom evaluation for edge weight
    );
    println!("{:.5?}", centrality); // [0.23167, 0.71650, 0.65800]
}

fn closeness_centrality_example() {
    use graphina::core::types::Graph;

    use graphina::centrality::algorithms::closeness_centrality;

    let mut graph = Graph::new();
    let ids = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();
    let edges = [(0, 1, 1.0), (0, 2, 1.0), (1, 3, 1.0)];
    for (s, d, w) in edges {
        graph.add_edge(ids[s], ids[d], w);
    }

    let centrality = closeness_centrality(&graph, false).unwrap();
    println!("{:.5?}", centrality); // [0.75000, 0.75000, 0.50000, 0.50000, 0.00000]
}

fn closeness_centrality_impl_example() {
    use graphina::core::types::Graph;

    use graphina::centrality::algorithms::closeness_centrality_impl;

    let mut graph: Graph<i32, (String, f64)> = Graph::new();

    let ids = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();

    let edges = [
        (0, 1, ("friend".to_string(), 0.9)),
        (0, 2, ("family".to_string(), 0.8)),
        (1, 3, ("friend".to_string(), 0.7)),
        (2, 4, ("enemy".to_string(), 0.1)),
    ];
    for (s, d, w) in edges {
        graph.add_edge(ids[s], ids[d], w);
    }

    let eval_cost = |(s, f): &(String, f64)| match s.as_str() {
        "friend" => Some(1.0 / *f / 2.0),
        "family" => Some(1.0 / *f / 4.0),
        "enemy" => None,
        _ => Some(1.0 / *f),
    };

    let centrality = closeness_centrality_impl(&graph, eval_cost, true).unwrap();
    println!("{:.5?}", centrality); // [1.05244, 1.05244, 0.81436, 0.63088, 0.00000]
}

macro_rules! run_examples {
    ($($func:ident),* $(,)?) => {
        $(
            println!("<{}>", stringify!($func));
            $func();
            println!();
        )*
    };
}

fn main() {
    run_examples!(
        // degree centrality
        degree_centrality_example,
        in_degree_centrality_example,
        out_degree_centrality_example,
        // eigen vector centrality
        eigenvector_centrality_example,
        eigenvector_centrality_numpy_example,
        eigenvector_centrality_impl_example,
        // katz centrality
        katz_centrality_example,
        katz_centrality_numpy_example,
        katz_centrality_impl_example,
        // closeness centrality
        closeness_centrality_example,
        closeness_centrality_impl_example,
    );
}
