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
    let expected = [2.0, 1.0, 1.0];
    for (i, f) in expected.into_iter().enumerate() {
        assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
    }
}

fn in_degree_centrality_example() {
    use graphina::core::types::Digraph;

    use graphina::centrality::algorithms::in_degree_centrality;

    let mut g: Digraph<i32, ()> = Digraph::new();
    let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
    g.add_edge(nodes[0], nodes[1], ());
    g.add_edge(nodes[0], nodes[2], ());

    let centrality = in_degree_centrality(&g);
    let expected = [0.0, 1.0, 1.0];
    for (i, f) in expected.into_iter().enumerate() {
        assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
    }
}

fn out_degree_centrality_example() {
    use graphina::core::types::Digraph;

    use graphina::centrality::algorithms::out_degree_centrality;

    let mut g: Digraph<i32, ()> = Digraph::new();
    let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
    g.add_edge(nodes[0], nodes[1], ());
    g.add_edge(nodes[0], nodes[2], ());

    let centrality = out_degree_centrality(&g);

    let expected = [2.0, 0.0, 0.0];
    for (i, f) in expected.into_iter().enumerate() {
        assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
    }
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
    let expected = [0.70711, 0.50000, 0.50000];
    for (i, f) in expected.into_iter().enumerate() {
        assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
    }

    let centrality = eigenvector_centrality(&g, 1000, true);
    let expected = [0.70711, 0.31623, 0.63246];
    for (i, f) in expected.into_iter().enumerate() {
        assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
    }
}

fn eigenvector_centrality_numpy_example() {
    use graphina::centrality::algorithms::eigenvector_centrality_numpy;
    use graphina::core::types::Graph;
    let mut g = Graph::new();

    let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
    g.add_edge(nodes[0], nodes[1], 1.0);
    g.add_edge(nodes[0], nodes[2], 2.0);

    let centrality = eigenvector_centrality_numpy(&g, 1000, 1e-6_f64, false);
    let expected = [0.70711, 0.50000, 0.50000];
    for (i, f) in expected.into_iter().enumerate() {
        assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
    }

    let centrality = eigenvector_centrality_numpy(&g, 1000, 1e-6_f64, true);
    let expected = [0.70711, 0.31623, 0.63246];
    for (i, f) in expected.into_iter().enumerate() {
        assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
    }
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
    let expected = [0.70711, 0.07036, 0.70360];
    for (i, f) in expected.into_iter().enumerate() {
        assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
    }
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
    let expected = [0.61078, 0.55989, 0.55989];
    for (i, f) in expected.into_iter().enumerate() {
        assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
    }

    let centrality = katz_centrality(&g, 0.01, 0.5, 1000, true, true);
    let expected = [0.58301, 0.57158, 0.57741];
    for (i, f) in expected.into_iter().enumerate() {
        assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
    }
}

fn katz_centrality_numpy_example() {
    use graphina::centrality::algorithms::katz_centrality_numpy;
    use graphina::core::types::Graph;

    let mut g = Graph::new();
    let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
    g.add_edge(nodes[0], nodes[1], 1.0);
    g.add_edge(nodes[0], nodes[2], 2.0);

    let centrality = katz_centrality_numpy(&g, 0.1, 1.0, false, true);
    let expected = [0.61078, 0.55989, 0.55989];
    for (i, f) in expected.into_iter().enumerate() {
        assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
    }

    let centrality = katz_centrality_numpy(&g, 0.01, 0.5, true, true);
    let expected = [0.58301, 0.57158, 0.57741];
    for (i, f) in expected.into_iter().enumerate() {
        assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
    }
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
    let expected = [0.23167, 0.71650, 0.65800];
    for (i, f) in expected.into_iter().enumerate() {
        assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
    }
}

fn closeness_centrality_example() {
    use graphina::core::types::Graph;

    use graphina::centrality::algorithms::closeness_centrality;

    let mut graph = Graph::new();
    let nodes = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();
    let edges = [(0, 1, 1.0), (0, 2, 1.0), (1, 3, 1.0)];
    for (s, d, w) in edges {
        graph.add_edge(nodes[s], nodes[d], w);
    }

    let centrality = closeness_centrality(&graph, false).unwrap();
    let expected = [0.75000, 0.75000, 0.50000, 0.50000, 0.00000];
    for (i, f) in expected.into_iter().enumerate() {
        assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
    }
}

fn closeness_centrality_impl_example() {
    use graphina::core::types::Graph;

    use graphina::centrality::algorithms::closeness_centrality_impl;

    let mut graph: Graph<i32, (String, f64)> = Graph::new();

    let nodes = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();

    let edges = [
        (0, 1, ("friend".to_string(), 0.9)),
        (0, 2, ("family".to_string(), 0.8)),
        (1, 3, ("friend".to_string(), 0.7)),
        (2, 4, ("enemy".to_string(), 0.1)),
    ];
    for (s, d, w) in edges {
        graph.add_edge(nodes[s], nodes[d], w);
    }

    let eval_cost = |(s, f): &(String, f64)| match s.as_str() {
        "friend" => Some(1.0 / *f / 2.0),
        "family" => Some(1.0 / *f / 4.0),
        "enemy" => None,
        _ => Some(1.0 / *f),
    };

    let centrality = closeness_centrality_impl(&graph, eval_cost, true).unwrap();
    let expected = [1.05244, 1.05244, 0.81436, 0.63088, 0.00000];
    for (i, f) in expected.into_iter().enumerate() {
        assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
    }
}

fn pagerank_example() {
    use graphina::core::types::Digraph;

    use graphina::centrality::algorithms::pagerank;

    let mut graph = Digraph::new();
    let nodes = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();
    let edges = [(0, 1, 1.0), (0, 2, 1.0), (1, 3, 1.0)];
    for (s, d, w) in edges {
        graph.add_edge(nodes[s], nodes[d], w);
    }

    let centrality = pagerank(&graph, 0.85, 1000);
    let expected = [0.14161, 0.20180, 0.20180, 0.31315, 0.14161];
    for (i, f) in expected.into_iter().enumerate() {
        assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
    }
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
        // pagerank
        pagerank_example,
    );
}
