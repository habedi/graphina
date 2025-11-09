fn line() {
    use graphina::core::types::Graph;

    use graphina::core::paths::dijkstra_path_f64;

    let mut graph = Graph::new();
    let ids = (0..5).map(|i| graph.add_node(i)).collect::<Vec<_>>();
    let edges = [(0, 1, 1.0), (1, 2, 1.0), (2, 3, 2.0), (3, 4, 1.0)];
    for (s, d, w) in edges {
        graph.add_edge(ids[s], ids[d], w);
    }

    let (cost, trace) = dijkstra_path_f64(&graph, ids[0], None).unwrap();

    println!("cost : {:?}", cost);
    println!("trace: {:?}", trace);
    let expected_cost = [Some(0.0), Some(1.0), Some(2.0), Some(4.0), Some(5.0)];
    let expected_trace = [None, Some(ids[0]), Some(ids[1]), Some(ids[2]), Some(ids[3])];

    for id in ids {
        assert_eq!(cost[&id], expected_cost[id.index()]);
        assert_eq!(trace[&id], expected_trace[id.index()]);
    }
}

fn flight() {
    use graphina::core::types::Digraph;

    use graphina::core::paths::dijkstra_path_impl;

    let mut graph: Digraph<String, (f64, String)> = Digraph::new();
    //                             ^^^^^^^^^^^^^
    //                                         L arbitrary type as edge

    let cities = ["ATL", "PEK", "LHR", "HND", "CDG", "FRA", "HKG"];

    let ids = cities
        .iter()
        .map(|s| graph.add_node(s.to_string()))
        .collect::<Vec<_>>();

    let edges = [
        //
        ("ATL", "PEK", (900.0, "boeing")),
        ("ATL", "LHR", (500.0, "airbus")),
        ("ATL", "HND", (700.0, "airbus")),
        //
        ("PEK", "LHR", (800.0, "boeing")),
        ("PEK", "HND", (100.0, "airbus")),
        ("PEK", "HKG", (100.0, "airbus")),
        //
        ("LHR", "CDG", (100.0, "airbus")),
        ("LHR", "FRA", (200.0, "boeing")),
        ("LHR", "HND", (600.0, "airbus")),
        //
        ("HND", "ATL", (700.0, "airbus")),
        ("HND", "FRA", (600.0, "airbus")),
        ("HND", "HKG", (100.0, "airbus")),
        //
    ];

    for (s, d, w) in edges {
        let depart = cities.iter().position(|city| s == *city).unwrap();
        let destin = cities.iter().position(|city| d == *city).unwrap();
        graph.add_edge(ids[depart], ids[destin], (w.0, w.1.to_string()));
    }

    // function for evaluating possible cost for the edge
    // Some(f64) for cost
    // None for impassable
    let eval_cost = |(price, manufactuer): &(f64, String)| match manufactuer.as_str() {
        "boeing" => None,  // avoid boeing plane
        _ => Some(*price), // return price as the cost
    };

    let (cost, trace) = dijkstra_path_impl(&graph, ids[0], Some(1000.0), eval_cost).unwrap();
    println!("cost : {:?}", cost);
    println!("trace: {:?}", trace);

    let expected_cost = [
        Some(0.0),
        None,
        Some(500.0),
        Some(700.0),
        Some(600.0),
        None,
        Some(800.0),
    ];
    let expected_trace = [
        None,
        None,
        Some(ids[0]),
        Some(ids[0]),
        Some(ids[2]),
        None,
        Some(ids[3]),
    ];

    for id in ids {
        assert_eq!(cost[&id], expected_cost[id.index()]);
        assert_eq!(trace[&id], expected_trace[id.index()]);
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
    run_examples!(line, flight);
}
