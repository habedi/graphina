use crate::graph::types::{GraphConstructor, GraphWrapper};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn erdos_renyi_graph<Ty: GraphConstructor<i32, f32>>(
    n: usize,
    p: f64,
    seed: u64,
) -> GraphWrapper<i32, f32, Ty> {
    let mut graph = GraphWrapper::<i32, f32, Ty>::new();
    let mut nodes = Vec::with_capacity(n);
    for i in 0..n {
        nodes.push(graph.add_node(i as i32));
    }
    let mut rng = StdRng::seed_from_u64(seed);
    if <Ty as GraphConstructor<i32, f32>>::is_directed() {
        for i in 0..n {
            for j in 0..n {
                if i != j && rng.random_bool(p) {
                    graph.add_edge(nodes[i], nodes[j], 1.0);
                }
            }
        }
    } else {
        for i in 0..n {
            for j in (i + 1)..n {
                if rng.random_bool(p) {
                    graph.add_edge(nodes[i], nodes[j], 1.0);
                }
            }
        }
    }
    graph
}

pub fn complete_graph<Ty: GraphConstructor<i32, f32>>(n: usize) -> GraphWrapper<i32, f32, Ty> {
    let mut graph = GraphWrapper::<i32, f32, Ty>::new();
    let mut nodes = Vec::with_capacity(n);
    for i in 0..n {
        nodes.push(graph.add_node(i as i32));
    }
    if <Ty as GraphConstructor<i32, f32>>::is_directed() {
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    graph.add_edge(nodes[i], nodes[j], 1.0);
                }
            }
        }
    } else {
        for i in 0..n {
            for j in (i + 1)..n {
                graph.add_edge(nodes[i], nodes[j], 1.0);
            }
        }
    }
    graph
}

pub fn bipartite_graph<Ty: GraphConstructor<i32, f32>>(
    n1: usize,
    n2: usize,
    p: f64,
    seed: u64,
) -> GraphWrapper<i32, f32, Ty> {
    let mut graph = GraphWrapper::<i32, f32, Ty>::new();
    let mut group1 = Vec::with_capacity(n1);
    let mut group2 = Vec::with_capacity(n2);
    for i in 0..n1 {
        group1.push(graph.add_node(i as i32));
    }
    for j in 0..n2 {
        group2.push(graph.add_node((n1 + j) as i32));
    }
    let mut rng = StdRng::seed_from_u64(seed);
    for &u in &group1 {
        for &v in &group2 {
            if rng.random_bool(p) {
                graph.add_edge(u, v, 1.0);
            }
        }
    }
    graph
}

pub fn star_graph<Ty: GraphConstructor<i32, f32>>(n: usize) -> GraphWrapper<i32, f32, Ty> {
    let mut graph = GraphWrapper::<i32, f32, Ty>::new();
    if n == 0 {
        return graph;
    }
    let center = graph.add_node(0);
    for i in 1..n {
        let node = graph.add_node(i as i32);
        graph.add_edge(center, node, 1.0);
    }
    graph
}

pub fn cycle_graph<Ty: GraphConstructor<i32, f32>>(n: usize) -> GraphWrapper<i32, f32, Ty> {
    let mut graph = GraphWrapper::<i32, f32, Ty>::new();
    if n == 0 {
        return graph;
    }
    let mut nodes = Vec::with_capacity(n);
    for i in 0..n {
        nodes.push(graph.add_node(i as i32));
    }
    for i in 0..n {
        let j = (i + 1) % n;
        graph.add_edge(nodes[i], nodes[j], 1.0);
    }
    graph
}

pub fn watts_strogatz_graph<Ty: GraphConstructor<i32, f32>>(
    n: usize,
    k: usize,
    beta: f64,
    seed: u64,
) -> GraphWrapper<i32, f32, Ty> {
    // Watts–Strogatz is defined for undirected graphs.
    let mut graph = GraphWrapper::<i32, f32, Ty>::new();
    let mut nodes = Vec::with_capacity(n);
    for i in 0..n {
        nodes.push(graph.add_node(i as i32));
    }
    let mut rng = StdRng::seed_from_u64(seed);
    let half_k = k / 2;
    // Create ring lattice
    for i in 0..n {
        for j in 1..=half_k {
            let neighbor = (i + j) % n;
            graph.add_edge(nodes[i], nodes[neighbor], 1.0);
        }
    }
    // Rewire each edge with probability beta.
    // For simplicity, we add new edges for rewired connections.
    for i in 0..n {
        for _ in 1..=half_k {
            if rng.random_bool(beta) {
                let mut new_target;
                loop {
                    new_target = rng.random_range(0..n);
                    if new_target != i {
                        break;
                    }
                }
                graph.add_edge(nodes[i], nodes[new_target], 1.0);
            }
        }
    }
    graph
}

pub fn barabasi_albert_graph<Ty: GraphConstructor<i32, f32>>(
    n: usize,
    m: usize,
    seed: u64,
) -> GraphWrapper<i32, f32, Ty> {
    // Barabási–Albert is defined for undirected graphs.
    let mut graph = GraphWrapper::<i32, f32, Ty>::new();
    if n < m || m == 0 {
        return graph;
    }
    // Start with a complete graph of m nodes.
    let mut nodes = Vec::with_capacity(n);
    for i in 0..m {
        nodes.push(graph.add_node(i as i32));
    }
    for i in 0..m {
        for j in (i + 1)..m {
            graph.add_edge(nodes[i], nodes[j], 1.0);
        }
    }
    let mut rng = StdRng::seed_from_u64(seed);
    let mut degrees: Vec<usize> = vec![m - 1; m];
    let mut total_degree = m * (m - 1);
    for i in m..n {
        let new_node = graph.add_node(i as i32);
        nodes.push(new_node);
        let mut targets = Vec::new();
        while targets.len() < m {
            let r = rng.random_range(0..total_degree);
            let mut cumulative = 0;
            for (idx, &deg) in degrees.iter().enumerate() {
                cumulative += deg;
                if r < cumulative {
                    if !targets.contains(&nodes[idx]) {
                        targets.push(nodes[idx]);
                    }
                    break;
                }
            }
        }
        for target in &targets {
            graph.add_edge(new_node, *target, 1.0);
            let idx = nodes.iter().position(|&x| x == *target).unwrap();
            degrees[idx] += 1;
        }
        degrees.push(m);
        total_degree += 2 * m;
    }
    graph
}
