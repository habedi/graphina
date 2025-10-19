use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use graphina::core::generators::{barabasi_albert_graph, erdos_renyi_graph, watts_strogatz_graph};
use graphina::core::types::{Graph, Undirected};
use ordered_float::OrderedFloat;
use std::hint::black_box;

#[cfg(feature = "centrality")]
use graphina::centrality::betweenness::betweenness_centrality;
#[cfg(feature = "centrality")]
use graphina::centrality::degree::degree_centrality;
#[cfg(feature = "centrality")]
use graphina::centrality::pagerank::pagerank;

#[cfg(feature = "community")]
use graphina::community::label_propagation::label_propagation;
#[cfg(feature = "community")]
use graphina::community::louvain::louvain;

#[cfg(feature = "approximation")]
use graphina::approximation::connectivity::local_node_connectivity;

fn bench_graph_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_creation");

    for size in [100, 500, 1000, 2000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("erdos_renyi", size), size, |b, &size| {
            b.iter(|| black_box(erdos_renyi_graph::<Undirected>(size, 0.1, 42).unwrap()));
        });

        group.bench_with_input(
            BenchmarkId::new("barabasi_albert", size),
            size,
            |b, &size| {
                b.iter(|| black_box(barabasi_albert_graph::<Undirected>(size, 3, 42).unwrap()));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("watts_strogatz", size),
            size,
            |b, &size| {
                b.iter(|| black_box(watts_strogatz_graph::<Undirected>(size, 4, 0.1, 42).unwrap()));
            },
        );
    }

    group.finish();
}

#[cfg(feature = "centrality")]
fn bench_centrality_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("centrality");

    for size in [50, 100, 200].iter() {
        let graph = barabasi_albert_graph::<Undirected>(*size, 3, 42).unwrap();
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(
            BenchmarkId::new("degree_centrality", size),
            &graph,
            |b, g| {
                b.iter(|| black_box(degree_centrality(g).unwrap()));
            },
        );

        // Convert to OrderedFloat for betweenness
        let mut graph_ordered = Graph::<u32, OrderedFloat<f64>>::new();
        let node_map: std::collections::HashMap<_, _> = graph
            .nodes()
            .map(|(nid, &attr)| (nid, graph_ordered.add_node(attr)))
            .collect();

        for (src, tgt, &weight) in graph.edges() {
            graph_ordered.add_edge(node_map[&src], node_map[&tgt], OrderedFloat(weight as f64));
        }

        group.bench_with_input(
            BenchmarkId::new("pagerank", size),
            &graph_ordered,
            |b, g| {
                b.iter(|| black_box(pagerank(g, 0.85, 100, 1e-6).unwrap()));
            },
        );

        if *size <= 100 {
            // Betweenness is slow, only test on smaller graphs
            group.bench_with_input(
                BenchmarkId::new("betweenness", size),
                &graph_ordered,
                |b, g| {
                    b.iter(|| black_box(betweenness_centrality(g, false).unwrap()));
                },
            );
        }
    }

    group.finish();
}

#[cfg(feature = "community")]
fn bench_community_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("community_detection");

    for size in [100, 200, 500].iter() {
        let graph = barabasi_albert_graph::<Undirected>(*size, 4, 42).unwrap();
        group.throughput(Throughput::Elements(*size as u64));

        // Convert to f64 for louvain
        let mut graph_f64 = Graph::<u32, f64>::new();
        let node_map: std::collections::HashMap<_, _> = graph
            .nodes()
            .map(|(nid, &attr)| (nid, graph_f64.add_node(attr)))
            .collect();

        for (src, tgt, &weight) in graph.edges() {
            graph_f64.add_edge(node_map[&src], node_map[&tgt], weight as f64);
        }

        group.bench_with_input(BenchmarkId::new("louvain", size), &graph_f64, |b, g| {
            b.iter(|| black_box(louvain(g, Some(42))));
        });

        group.bench_with_input(
            BenchmarkId::new("label_propagation", size),
            &graph,
            |b, g| {
                b.iter(|| black_box(label_propagation(g, 100, Some(42))));
            },
        );
    }

    group.finish();
}

fn bench_graph_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_operations");

    for size in [100, 500, 1000].iter() {
        let mut graph = Graph::<u32, f32>::new();
        let nodes: Vec<_> = (0..*size).map(|i| graph.add_node(i)).collect();

        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("add_nodes", size), size, |b, &size| {
            b.iter(|| {
                let mut g = Graph::<u32, f32>::new();
                for i in 0..size {
                    black_box(g.add_node(i));
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("add_edges", size), &graph, |b, g| {
            b.iter(|| {
                let mut g = g.clone();
                let nodes: Vec<_> = g.nodes().map(|(id, _)| id).collect();
                for i in 0..nodes.len().min(100) {
                    for j in (i + 1)..nodes.len().min(100) {
                        black_box(g.add_edge(nodes[i], nodes[j], 1.0));
                    }
                }
            });
        });

        // Add some edges for traversal benchmarks
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len().min(i + 5) {
                graph.add_edge(nodes[i], nodes[j], 1.0);
            }
        }

        group.bench_with_input(BenchmarkId::new("neighbors", size), &graph, |b, g| {
            b.iter(|| {
                let nodes: Vec<_> = g.nodes().map(|(id, _)| id).collect();
                for node in &nodes[..nodes.len().min(10)] {
                    black_box(g.neighbors(*node).count());
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("degree", size), &graph, |b, g| {
            b.iter(|| {
                let nodes: Vec<_> = g.nodes().map(|(id, _)| id).collect();
                for node in &nodes {
                    black_box(g.degree(*node));
                }
            });
        });
    }

    group.finish();
}

#[cfg(feature = "approximation")]
fn bench_approximation_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("approximation");

    for size in [50, 100, 200].iter() {
        let mut graph = Graph::<u32, OrderedFloat<f64>>::new();
        let nodes: Vec<_> = (0..*size).map(|i| graph.add_node(i)).collect();

        // Create a connected graph
        for i in 0..(nodes.len() - 1) {
            graph.add_edge(nodes[i], nodes[i + 1], OrderedFloat(1.0));
        }

        group.throughput(Throughput::Elements(*size as u64));

        if *size <= 100 {
            // Only test on smaller graphs due to potential slowness
            group.bench_with_input(
                BenchmarkId::new("local_connectivity", size),
                &graph,
                |b, g| {
                    let nodes: Vec<_> = g.nodes().map(|(id, _)| id).collect();
                    if nodes.len() >= 2 {
                        b.iter(|| {
                            black_box(local_node_connectivity(g, nodes[0], nodes[nodes.len() / 2]))
                        });
                    }
                },
            );
        }
    }

    group.finish();
}

// Conditional criterion_group based on features
#[cfg(all(
    feature = "centrality",
    feature = "community",
    feature = "approximation"
))]
criterion_group!(
    benches,
    bench_graph_creation,
    bench_graph_operations,
    bench_centrality_algorithms,
    bench_community_detection,
    bench_approximation_algorithms,
);

#[cfg(all(
    feature = "centrality",
    feature = "community",
    not(feature = "approximation")
))]
criterion_group!(
    benches,
    bench_graph_creation,
    bench_graph_operations,
    bench_centrality_algorithms,
    bench_community_detection,
);

#[cfg(all(
    feature = "centrality",
    not(feature = "community"),
    not(feature = "approximation")
))]
criterion_group!(
    benches,
    bench_graph_creation,
    bench_graph_operations,
    bench_centrality_algorithms,
);

#[cfg(not(any(
    feature = "centrality",
    feature = "community",
    feature = "approximation"
)))]
criterion_group!(benches, bench_graph_creation, bench_graph_operations,);

criterion_main!(benches);
