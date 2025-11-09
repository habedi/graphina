#![cfg_attr(not(feature = "centrality"), allow(dead_code, unused_imports))]

#[cfg(feature = "centrality")]
fn main() {
    //
    // -----------------------------
    // Degree Centrality
    // -----------------------------
    //

    fn degree_centrality_example() {
        use graphina::centrality::degree::degree_centrality;
        use graphina::core::types::Graph;

        let mut g: Graph<i32, ()> = Graph::new();
        let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
        g.add_edge(nodes[0], nodes[1], ());
        g.add_edge(nodes[0], nodes[2], ());

        let centrality = degree_centrality(&g).unwrap();
        // raw counts
        let expected = [2.0, 1.0, 1.0];
        for (i, f) in expected.into_iter().enumerate() {
            assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
        }
    }

    fn in_degree_centrality_example() {
        use graphina::centrality::degree::in_degree_centrality;
        use graphina::core::types::Digraph;

        let mut g: Digraph<i32, ()> = Digraph::new();
        let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
        g.add_edge(nodes[0], nodes[1], ());
        g.add_edge(nodes[0], nodes[2], ());

        let centrality = in_degree_centrality(&g).unwrap();
        // raw counts
        let expected = [0.0, 1.0, 1.0];
        for (i, f) in expected.into_iter().enumerate() {
            assert!((centrality[&nodes[i]] - f).abs() < 1e-5)
        }
    }

    fn out_degree_centrality_example() {
        use graphina::centrality::degree::out_degree_centrality;
        use graphina::core::types::Digraph;

        let mut g: Digraph<i32, ()> = Digraph::new();
        let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
        g.add_edge(nodes[0], nodes[1], ());
        g.add_edge(nodes[0], nodes[2], ());

        let centrality = out_degree_centrality(&g).unwrap();

        // raw counts
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
        use graphina::centrality::eigenvector::eigenvector_centrality;
        use graphina::core::types::Graph;

        let mut g = Graph::new();
        let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
        g.add_edge(nodes[0], nodes[1], 1.0);
        g.add_edge(nodes[0], nodes[2], 2.0);

        let centrality = eigenvector_centrality(&g, 1000, 1e-6_f64).expect("eigenvector failed");
        // Values depend on normalization; check basic ordering and approximate values.
        assert!(centrality[&nodes[0]] > centrality[&nodes[1]]);
        assert!(centrality[&nodes[0]] > centrality[&nodes[2]]);
    }

    //
    // -----------------------------
    // Katz Centrality
    // -----------------------------
    //

    fn katz_centrality_example() {
        use graphina::centrality::katz::katz_centrality;
        use graphina::core::types::Graph;

        let mut g = Graph::new();
        let nodes = [g.add_node(1), g.add_node(2), g.add_node(3)];
        g.add_edge(nodes[0], nodes[1], 1.0);
        g.add_edge(nodes[0], nodes[2], 2.0);

        let centrality = katz_centrality(&g, 0.1, Some(&|_| 1.0), 1000, 1e-6_f64).unwrap();
        // Node 1 connected to two others should have highest centrality.
        assert!(centrality[&nodes[0]] > centrality[&nodes[1]]);
        assert!(centrality[&nodes[0]] > centrality[&nodes[2]]);
    }

    //
    // -----------------------------
    // Closeness Centrality
    // -----------------------------
    //

    fn closeness_centrality_example() {
        use graphina::centrality::closeness::closeness_centrality;
        use graphina::core::types::Graph;
        use ordered_float::OrderedFloat;

        let mut graph: Graph<i32, OrderedFloat<f64>> = Graph::new();
        let nodes = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();
        let edges = [
            (0, 1, OrderedFloat(1.0)),
            (0, 2, OrderedFloat(1.0)),
            (1, 3, OrderedFloat(1.0)),
        ];
        for (s, d, w) in edges {
            graph.add_edge(nodes[s], nodes[d], w);
        }

        let centrality = closeness_centrality(&graph).expect("closeness failed");
        // Basic sanity: centrality of node 0 should be highest among connected nodes
        assert!(centrality[&nodes[0]] >= centrality[&nodes[1]]);
        assert!(centrality[&nodes[0]] >= centrality[&nodes[2]]);
    }

    fn pagerank_example() {
        use graphina::centrality::pagerank::pagerank;
        use graphina::core::types::Digraph;

        let mut graph = Digraph::new();
        let nodes = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();
        let edges = [(0, 1, 1.0), (0, 2, 1.0), (1, 3, 1.0)];
        for (s, d, w) in edges {
            graph.add_edge(nodes[s], nodes[d], w);
        }

        let centrality = pagerank(&graph, 0.85, 1000, 1e-6_f64).unwrap();
        // Sum close to 1, all non-negative
        let sum: f64 = nodes.iter().map(|n| centrality[n]).sum();
        assert!((sum - 1.0).abs() < 1e-6);
        for n in nodes {
            assert!(centrality[&n] >= 0.0);
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

    run_examples!(
        // degree centrality
        degree_centrality_example,
        in_degree_centrality_example,
        out_degree_centrality_example,
        // eigen vector centrality
        eigenvector_centrality_example,
        // katz centrality
        katz_centrality_example,
        // closeness centrality
        closeness_centrality_example,
        // pagerank
        pagerank_example,
    );
}

#[cfg(not(feature = "centrality"))]
fn main() {
    eprintln!(
        "This example requires the 'centrality' feature. Run with:\n  cargo run --example centrality --features centrality"
    );
}
