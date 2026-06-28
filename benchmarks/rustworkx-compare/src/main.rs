//! Comparison harness running the same graph algorithms through graphina and
//! rustworkx-core (the pure-Rust algorithm crate behind rustworkx).
//!
//! Both libraries receive the same synthetic graph, built from one deterministic
//! edge list into a graphina `Graph` and a petgraph `UnGraph`. Each algorithm
//! runs on both, and the harness reports the median wall time per library, so it
//! doubles as a differential correctness check: the result of each algorithm is
//! normalized to a canonical, library-independent form and compared before
//! timing. Medians for an algorithm the two libraries disagree on are
//! meaningless (a library doing the wrong amount of work can look faster), so a
//! divergent algorithm is reported and not timed.
//!
//! A few comparisons need care to be meaningful across the two libraries:
//!
//! * The graph carries unit edge weights, so weighted shortest paths equal
//!   unweighted hop counts. rustworkx betweenness and closeness are structural
//!   (unweighted) while graphina's are weighted, and unit weights make the two
//!   agree.
//! * rustworkx betweenness and closeness parallelize above a node-count
//!   threshold; the harness passes `usize::MAX` to force the sequential path, so
//!   both libraries are measured single-threaded (graphina's non-parallel
//!   modules are sequential).
//! * Eigenvector centrality uses different normalization conventions in the two
//!   libraries, so its vector is L2-normalized and sign-fixed before comparison.
//!
//! Dataset sizes, degree skew, repetition counts, the per-algorithm time
//! budget, and the scale sweep come from environment variables; see
//! `Config::from_env` for the knobs and their defaults.

use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::time::{Duration, Instant};

use ordered_float::OrderedFloat;

use graphina::core::types::{Graph, NodeId};

use rustworkx_core::centrality::{
    betweenness_centrality as rwx_betweenness, closeness_centrality as rwx_closeness,
    degree_centrality as rwx_degree, edge_betweenness_centrality as rwx_edge_betweenness,
    eigenvector_centrality as rwx_eigenvector, katz_centrality as rwx_katz,
};
use rustworkx_core::connectivity::connected_components as rwx_connected_components;
use rustworkx_core::petgraph::graph::{NodeIndex, UnGraph};
use rustworkx_core::shortest_path::{
    astar as rwx_astar, bellman_ford as rwx_bellman_ford, dijkstra as rwx_dijkstra,
    distance_matrix as rwx_distance_matrix,
};
use rustworkx_core::transitivity::graph_transitivity as rwx_transitivity;
use rustworkx_core::traversal::{breadth_first_search, depth_first_search, BfsEvent, DfsEvent};

/// Zipf exponent for the skewed degree distribution. At 0.8 the hottest node
/// receives a few percent of all edge endpoints, a proper hub without saturating
/// the distinct-edge constraint.
const ZIPF_THETA: f64 = 0.8;

/// Each sweep step multiplies nodes and edges by this factor.
const SWEEP_STEP: u64 = 5;

#[derive(Clone, Copy, PartialEq)]
enum Skew {
    Uniform,
    Zipf,
}

impl Skew {
    fn as_str(self) -> &'static str {
        match self {
            Skew::Uniform => "uniform",
            Skew::Zipf => "zipf",
        }
    }
}

struct Config {
    /// Node count.
    nodes: u64,
    /// Edge count (distinct unordered pairs, no self-loops).
    edges: u64,
    /// Timed repetitions per algorithm; the median is reported.
    reps: usize,
    /// Untimed warmup runs per algorithm.
    warmups: usize,
    /// Degree distribution of the generated edges.
    skew: Skew,
    /// When set, runs the workload at base/5, base, and base*5 sizes and reports
    /// per-algorithm scaling ratios between consecutive sizes.
    sweep: bool,
    /// Time budget per algorithm per library; repetitions stop early once it is
    /// spent (at least one timed repetition always runs).
    budget: Duration,
}

impl Config {
    fn from_env() -> Self {
        fn var(name: &str, default: u64) -> u64 {
            std::env::var(name)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(default)
        }
        let skew = match std::env::var("RUSTWORKX_COMPARE_SKEW").as_deref() {
            Ok("zipf") => Skew::Zipf,
            Ok("uniform") | Err(_) => Skew::Uniform,
            Ok(other) => {
                panic!("RUSTWORKX_COMPARE_SKEW must be 'uniform' or 'zipf', got {other:?}")
            }
        };
        let nodes = var("RUSTWORKX_COMPARE_NODES", 2_000);
        let edges = var("RUSTWORKX_COMPARE_EDGES", 10_000);
        let reps = var("RUSTWORKX_COMPARE_REPS", 10) as usize;
        let sweep = var("RUSTWORKX_COMPARE_SWEEP", 0) != 0;
        assert!(nodes > 0, "RUSTWORKX_COMPARE_NODES must be at least 1");
        assert!(
            edges == 0 || nodes > 1,
            "RUSTWORKX_COMPARE_EDGES requires at least two nodes \
             (edges are distinct non-self-loop pairs)"
        );
        assert!(reps > 0, "RUSTWORKX_COMPARE_REPS must be at least 1");
        if sweep {
            let base_nodes = nodes / SWEEP_STEP;
            assert!(
                base_nodes > 0,
                "sweep divides the node count by {SWEEP_STEP}; \
                 RUSTWORKX_COMPARE_NODES is too small"
            );
        }
        Config {
            nodes,
            edges,
            reps,
            warmups: var("RUSTWORKX_COMPARE_WARMUPS", 3) as usize,
            skew,
            sweep,
            budget: Duration::from_secs(var("RUSTWORKX_COMPARE_BUDGET_SECS", 30)),
        }
    }
}

/// Deterministic 64-bit LCG (Knuth MMIX constants) so both libraries always see
/// the same graph and runs are reproducible without pulling in `rand`.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 16
    }

    /// Uniform sample in [0, 1) from the 48 output bits.
    fn unit(&mut self) -> f64 {
        self.next() as f64 / (1u64 << 48) as f64
    }
}

/// Cumulative Zipf distribution over node indices `0..n` with exponent
/// `ZIPF_THETA`. Skewed sampling concentrates edge endpoints on low indices,
/// producing hub nodes whose degrees follow a power law, as in real graphs;
/// uniform sampling gives every node roughly the average degree.
struct Zipf {
    cdf: Vec<f64>,
}

impl Zipf {
    fn new(n: u64) -> Self {
        let mut cdf = Vec::with_capacity(n as usize);
        let mut acc = 0.0;
        for rank in 1..=n {
            acc += 1.0 / (rank as f64).powf(ZIPF_THETA);
            cdf.push(acc);
        }
        for v in &mut cdf {
            *v /= acc;
        }
        Zipf { cdf }
    }

    fn sample(&self, u: f64) -> u64 {
        self.cdf.partition_point(|&c| c < u) as u64
    }
}

/// An undirected graph as a node count and a deduplicated edge list. Edges are
/// stored as ordered pairs `(a, b)` with `a < b`.
struct Dataset {
    nodes: u64,
    edges: Vec<(u32, u32)>,
}

fn generate(nodes: u64, edges: u64, skew: Skew) -> Dataset {
    let mut rng = Lcg(0x1554_4ED1);
    let zipf = match skew {
        Skew::Zipf => Some(Zipf::new(nodes)),
        Skew::Uniform => None,
    };
    let mut seen = HashSet::new();
    let mut out = Vec::with_capacity(edges as usize);
    // Skewed sampling rejects more duplicates around the hubs; the cap turns a
    // pathological nodes-to-edges ratio into a clear failure instead of a hang.
    let max_attempts = edges.saturating_mul(100).max(1);
    let mut attempts = 0u64;
    while (out.len() as u64) < edges {
        attempts += 1;
        assert!(
            attempts <= max_attempts,
            "edge sampling saturated; lower RUSTWORKX_COMPARE_EDGES relative to RUSTWORKX_COMPARE_NODES"
        );
        let (s, t) = match &zipf {
            Some(z) => (z.sample(rng.unit()), z.sample(rng.unit())),
            None => (rng.next() % nodes, rng.next() % nodes),
        };
        if s == t {
            continue;
        }
        let (a, b) = if s < t { (s, t) } else { (t, s) };
        if !seen.insert((a, b)) {
            continue;
        }
        out.push((a as u32, b as u32));
    }
    Dataset { nodes, edges: out }
}

/// Highest-degree node, the source for the single-source traversals (dijkstra
/// and BFS). Picking the hub keeps the traversal non-trivial under both skews;
/// ties break toward the lowest index, so the choice is deterministic.
fn hub_node(data: &Dataset) -> usize {
    let mut degree = vec![0u64; data.nodes as usize];
    for &(a, b) in &data.edges {
        degree[a as usize] += 1;
        degree[b as usize] += 1;
    }
    degree
        .iter()
        .enumerate()
        .max_by_key(|&(_, &d)| d)
        .map(|(i, _)| i)
        .unwrap_or(0)
}

/// The three graphina graph instances. The weight type is forced by the
/// algorithms: `betweenness_centrality` requires `OrderedFloat<f64>`, the
/// generic `dijkstra` requires an integer weight (`Ord + From<u8>`), and the
/// remaining algorithms accept plain `f64`.
struct GraphinaGraphs {
    f64: Graph<(), f64>,
    of: Graph<(), OrderedFloat<f64>>,
    int: Graph<(), i64>,
    /// `NodeId`s in insertion order, so index `i` maps to the node built from id `i`.
    ids: Vec<NodeId>,
}

fn build_graphina(data: &Dataset) -> GraphinaGraphs {
    let mut g_f64 = Graph::<(), f64>::new();
    let mut g_of = Graph::<(), OrderedFloat<f64>>::new();
    let mut g_int = Graph::<(), i64>::new();
    let mut ids = Vec::with_capacity(data.nodes as usize);
    for _ in 0..data.nodes {
        let id = g_f64.add_node(());
        g_of.add_node(());
        g_int.add_node(());
        ids.push(id);
    }
    for &(a, b) in &data.edges {
        let (a, b) = (a as usize, b as usize);
        g_f64.add_edge(ids[a], ids[b], 1.0);
        g_of.add_edge(ids[a], ids[b], OrderedFloat(1.0));
        g_int.add_edge(ids[a], ids[b], 1);
    }
    GraphinaGraphs {
        f64: g_f64,
        of: g_of,
        int: g_int,
        ids,
    }
}

fn build_petgraph(data: &Dataset) -> (UnGraph<(), f64>, Vec<NodeIndex>) {
    let mut pg = UnGraph::<(), f64>::new_undirected();
    let idx: Vec<NodeIndex> = (0..data.nodes).map(|_| pg.add_node(())).collect();
    for &(a, b) in &data.edges {
        pg.add_edge(idx[a as usize], idx[b as usize], 1.0);
    }
    (pg, idx)
}

// ----------------------------------------------------------------------------
// Normalization helpers: every algorithm result is reduced to a canonical
// `Vec<i64>` so the two libraries can be compared with plain equality. Floats
// are quantized to a fixed number of decimal places, which is the comparison
// tolerance.
// ----------------------------------------------------------------------------

/// Spreads a graphina `NodeMap<f64>` into a dense vector indexed by node index.
fn map_to_vec(map: &HashMap<NodeId, f64>, n: usize) -> Vec<f64> {
    let mut v = vec![0.0; n];
    for (id, &val) in map {
        v[id.index()] = val;
    }
    v
}

/// L2-normalizes a vector and fixes its sign so the component with the largest
/// magnitude is positive. Eigenvectors are defined only up to scale and sign, so
/// this is the canonical form both libraries are compared in.
fn l2_sign_normalize(v: &mut [f64]) {
    let norm = v.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm > 0.0 {
        for x in v.iter_mut() {
            *x /= norm;
        }
    }
    let pivot = v
        .iter()
        .cloned()
        .fold(0.0_f64, |m, x| if x.abs() > m.abs() { x } else { m });
    if pivot < 0.0 {
        for x in v.iter_mut() {
            *x = -*x;
        }
    }
}

// ----------------------------------------------------------------------------
// Timing
// ----------------------------------------------------------------------------

/// Bootstrap resamples used to estimate the confidence interval of the median.
const BOOTSTRAP_RESAMPLES: usize = 2000;

#[derive(Clone, Copy)]
struct BenchStat {
    median: Duration,
    ci_lo: Duration,
    ci_hi: Duration,
    samples: usize,
}

fn median_sorted(sorted: &[Duration]) -> Duration {
    sorted[sorted.len() / 2]
}

/// 95% confidence interval for the median by percentile bootstrap, with a
/// fixed-seed generator so the interval is reproducible for a given set of
/// timings.
fn bootstrap_ci95(sorted: &[Duration]) -> (Duration, Duration) {
    let n = sorted.len();
    if n <= 1 {
        let only = sorted.first().copied().unwrap_or_default();
        return (only, only);
    }
    let nanos: Vec<u128> = sorted.iter().map(Duration::as_nanos).collect();
    let mut seed: u64 = 0x9E37_79B9_7F4A_7C15 ^ (n as u64);
    for &v in &nanos {
        seed = seed.wrapping_mul(0x100_0000_01B3).wrapping_add(v as u64);
    }
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    let mut medians = Vec::with_capacity(BOOTSTRAP_RESAMPLES);
    let mut sample = vec![0u128; n];
    for _ in 0..BOOTSTRAP_RESAMPLES {
        for s in sample.iter_mut() {
            *s = nanos[(next() as usize) % n];
        }
        sample.sort_unstable();
        medians.push(sample[n / 2]);
    }
    medians.sort_unstable();
    let lo = medians[(BOOTSTRAP_RESAMPLES as f64 * 0.025) as usize];
    let hi = medians[((BOOTSTRAP_RESAMPLES as f64 * 0.975) as usize).min(BOOTSTRAP_RESAMPLES - 1)];
    (
        Duration::from_nanos(lo as u64),
        Duration::from_nanos(hi as u64),
    )
}

/// Runs `f` for up to `warmups` untimed and `reps` timed repetitions, stopping
/// each phase early once `budget` is spent, and returns the median timed
/// duration with its 95% bootstrap confidence interval.
fn bench(warmups: usize, reps: usize, budget: Duration, mut f: impl FnMut()) -> BenchStat {
    let warmup_start = Instant::now();
    for _ in 0..warmups {
        f();
        if warmup_start.elapsed() > budget {
            break;
        }
    }
    let mut times = Vec::with_capacity(reps);
    let timed_start = Instant::now();
    for _ in 0..reps {
        let start = Instant::now();
        f();
        times.push(start.elapsed());
        if timed_start.elapsed() > budget {
            break;
        }
    }
    times.sort();
    let (ci_lo, ci_hi) = bootstrap_ci95(&times);
    BenchStat {
        median: median_sorted(&times),
        ci_lo,
        ci_hi,
        samples: times.len(),
    }
}

// ----------------------------------------------------------------------------
// Workload
// ----------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum Diff {
    Match,
    Mismatch,
}

struct Row {
    name: &'static str,
    graphina: Option<BenchStat>,
    rustworkx: Option<BenchStat>,
    diff: Diff,
}

/// Two normalized result vectors agree when they have the same length and no
/// element differs by more than `eps`. Tolerance comparison rather than exact
/// equality avoids spurious mismatches from floating-point summation order and
/// rounding boundaries; for exact integer results (distances, node sets) an
/// `eps` below `1.0` keeps the comparison exact.
fn within_tolerance(a: &[f64], b: &[f64], eps: f64) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (x - y).abs() <= eps)
}

/// Runs one algorithm on both libraries, compares the normalized results, and
/// times each side only when they agree. `g_run`/`r_run` produce the raw
/// library-native result (timed); `g_norm`/`r_norm` reduce it to a canonical
/// `Vec<f64>` compared within `eps`.
#[allow(clippy::too_many_arguments)]
fn diff_and_bench<GN, RN>(
    name: &'static str,
    cfg: &Config,
    rows: &mut Vec<Row>,
    eps: f64,
    mut g_run: impl FnMut() -> GN,
    g_norm: impl Fn(&GN) -> Vec<f64>,
    mut r_run: impl FnMut() -> RN,
    r_norm: impl Fn(&RN) -> Vec<f64>,
) {
    let g_native = g_run();
    let r_native = r_run();
    let diff = if within_tolerance(&g_norm(&g_native), &r_norm(&r_native), eps) {
        Diff::Match
    } else {
        Diff::Mismatch
    };
    // `black_box` the result so the optimizer cannot elide an algorithm whose
    // return value is otherwise unused, which would report a near-zero time.
    let (graphina, rustworkx) = if diff == Diff::Match {
        (
            Some(bench(cfg.warmups, cfg.reps, cfg.budget, || {
                std::hint::black_box(g_run());
            })),
            Some(bench(cfg.warmups, cfg.reps, cfg.budget, || {
                std::hint::black_box(r_run());
            })),
        )
    } else {
        (None, None)
    };
    rows.push(Row {
        name,
        graphina,
        rustworkx,
        diff,
    });
}

/// Loads both libraries at the given size, runs the workload, prints the result
/// table, and returns the per-algorithm timings for the sweep summary.
fn run_at(cfg: &Config, nodes: u64, edges: u64) -> Vec<Row> {
    println!(
        "dataset: {nodes} nodes, {edges} edges ({} skew); {} reps ({} warmups) per algorithm\n",
        cfg.skew.as_str(),
        cfg.reps,
        cfg.warmups
    );

    let data = generate(nodes, edges, cfg.skew);
    let n = data.nodes as usize;
    let hub = hub_node(&data);

    let gg = build_graphina(&data);
    let (pg, pidx) = build_petgraph(&data);
    let g_src = gg.ids[hub];
    let p_src = pidx[hub];
    // A fixed target for the point-to-point search, deterministic and distinct
    // from the source whenever there is more than one node.
    let target = (hub + n / 2) % n;
    let g_target = gg.ids[target];
    let p_target = pidx[target];

    let mut rows = Vec::new();

    // Single-source shortest path (unit weights, so distance == hop count).
    // Integer distances, so a sub-unit eps keeps the check exact.
    diff_and_bench(
        "dijkstra (SSSP)",
        cfg,
        &mut rows,
        0.5,
        || graphina::core::paths::dijkstra(&gg.int, g_src).expect("graphina dijkstra"),
        |m| {
            let mut v = vec![-1.0; n];
            for (id, dist) in m {
                v[id.index()] = dist.map(|d| d as f64).unwrap_or(-1.0);
            }
            v
        },
        || {
            rwx_dijkstra::<_, _, i64, Infallible, Vec<Option<i64>>>(
                &pg,
                p_src,
                None,
                |_| Ok(1),
                None,
            )
            .expect("rustworkx dijkstra")
        },
        |dist| {
            let mut v = vec![-1.0; n];
            for (i, slot) in dist.iter().enumerate().take(n) {
                v[i] = slot.map(|d| d as f64).unwrap_or(-1.0);
            }
            v
        },
    );

    // Bellman-Ford single-source shortest path (unit weights, no negative cycle).
    diff_and_bench(
        "bellman_ford (SSSP)",
        cfg,
        &mut rows,
        0.5,
        || graphina::core::paths::bellman_ford(&gg.int, g_src).expect("graphina bellman_ford"),
        |m| {
            let mut v = vec![-1.0; n];
            for (id, dist) in m {
                v[id.index()] = dist.map(|d| d as f64).unwrap_or(-1.0);
            }
            v
        },
        || {
            rwx_bellman_ford::<_, _, i64, Infallible, Vec<Option<i64>>>(&pg, p_src, |_| Ok(1), None)
                .expect("rustworkx bellman_ford")
                .expect("no negative cycle")
        },
        |dist| {
            let mut v = vec![-1.0; n];
            for (i, slot) in dist.iter().enumerate().take(n) {
                v[i] = slot.map(|d| d as f64).unwrap_or(-1.0);
            }
            v
        },
    );

    // A* point-to-point shortest path with a zero heuristic (unit weights), so
    // both libraries return the same path cost. Only the cost is compared, since
    // tie-broken paths can differ.
    diff_and_bench(
        "a_star (point-to-point)",
        cfg,
        &mut rows,
        0.5,
        || {
            graphina::core::paths::a_star(&gg.int, g_src, g_target, |_| 0i64)
                .expect("graphina a_star")
        },
        |opt| opt.iter().map(|(cost, _)| *cost as f64).collect(),
        || {
            rwx_astar(
                &pg,
                p_src,
                |n| Ok::<bool, Infallible>(n == p_target),
                |_| Ok(1i64),
                |_| Ok(0i64),
            )
            .expect("rustworkx a_star")
        },
        |opt| opt.iter().map(|(cost, _)| *cost as f64).collect(),
    );

    // All-pairs shortest path: graphina's Johnson against rustworkx's BFS-based
    // distance matrix. Both are flattened in row-major (source-major) order.
    diff_and_bench(
        "all-pairs (johnson)",
        cfg,
        &mut rows,
        0.5,
        || graphina::core::paths::johnson(&gg.int).expect("graphina johnson"),
        |outer| {
            let mut v = Vec::with_capacity(n * n);
            for i in 0..n {
                let inner = outer.get(&gg.ids[i]);
                for j in 0..n {
                    let d = inner
                        .and_then(|m| m.get(&gg.ids[j]))
                        .and_then(|d| *d)
                        .map(|d| d as f64)
                        .unwrap_or(-1.0);
                    v.push(d);
                }
            }
            v
        },
        || rwx_distance_matrix(&pg, usize::MAX, false, -1.0),
        |mat| mat.iter().copied().collect(),
    );

    // BFS reachability from the hub, compared as the set of reached nodes
    // (visitation order can differ by internal adjacency ordering).
    diff_and_bench(
        "bfs (reachable set)",
        cfg,
        &mut rows,
        0.5,
        || graphina::traversal::bfs(&gg.f64, g_src),
        |order| sorted_indices(order.iter().map(|id| id.index())),
        || {
            let mut order = Vec::new();
            breadth_first_search(&pg, Some(p_src), |event| {
                if let BfsEvent::Discover(node) = event {
                    order.push(node);
                }
            });
            order
        },
        |order| sorted_indices(order.iter().map(|idx| idx.index())),
    );

    // DFS reachability from the hub, compared as the set of reached nodes.
    diff_and_bench(
        "dfs (reachable set)",
        cfg,
        &mut rows,
        0.5,
        || graphina::traversal::dfs(&gg.f64, g_src),
        |order| sorted_indices(order.iter().map(|id| id.index())),
        || {
            let mut order = Vec::new();
            depth_first_search(&pg, Some(p_src), |event| {
                if let DfsEvent::Discover(node, _) = event {
                    order.push(node);
                }
            });
            order
        },
        |order| sorted_indices(order.iter().map(|idx| idx.index())),
    );

    // Connected components, compared as a canonical partition.
    diff_and_bench(
        "connected_components",
        cfg,
        &mut rows,
        0.5,
        || graphina::community::connected_components::connected_components(&gg.f64),
        |comps| canonical_partition(comps.iter().map(|c| c.iter().map(|id| id.index()))),
        || rwx_connected_components(&pg),
        |comps| canonical_partition(comps.iter().map(|c| c.iter().map(|idx| idx.index()))),
    );

    // Degree centrality. graphina returns raw degree counts; rustworkx divides
    // by n-1, so the graphina side is scaled to match.
    diff_and_bench(
        "degree_centrality",
        cfg,
        &mut rows,
        1e-9,
        || graphina::centrality::degree::degree_centrality(&gg.f64).expect("graphina degree"),
        |m| {
            let denom = (n.saturating_sub(1)).max(1) as f64;
            map_to_vec(m, n).iter().map(|d| d / denom).collect()
        },
        || rwx_degree(&pg, None),
        |v| v.clone(),
    );

    // Betweenness centrality, unnormalized (raw shortest-path dependency sums).
    diff_and_bench(
        "betweenness",
        cfg,
        &mut rows,
        1e-3,
        || {
            graphina::centrality::betweenness::betweenness_centrality(&gg.of, false)
                .expect("graphina betweenness")
        },
        |m| map_to_vec(m, n),
        || rwx_betweenness(&pg, false, false, usize::MAX),
        |v| opt_vec(v, n),
    );

    // Edge betweenness, unnormalized. Both sides are aligned to the generated
    // edge order: rustworkx returns a vector indexed by edge id, and graphina a
    // map keyed by the endpoint pair (stored in both directions).
    diff_and_bench(
        "edge_betweenness",
        cfg,
        &mut rows,
        1e-3,
        || {
            graphina::centrality::betweenness::edge_betweenness_centrality(&gg.of, false)
                .expect("graphina edge_betweenness")
        },
        |m| {
            data.edges
                .iter()
                .map(|&(a, b)| {
                    let (u, v) = (gg.ids[a as usize], gg.ids[b as usize]);
                    *m.get(&(u, v)).or_else(|| m.get(&(v, u))).unwrap_or(&0.0)
                })
                .collect()
        },
        || rwx_edge_betweenness(&pg, false, usize::MAX),
        |v| {
            (0..data.edges.len())
                .map(|i| v.get(i).and_then(|x| *x).unwrap_or(0.0))
                .collect()
        },
    );

    // Closeness centrality (Wasserman-Faust correction on both sides).
    diff_and_bench(
        "closeness",
        cfg,
        &mut rows,
        1e-4,
        || {
            graphina::centrality::closeness::closeness_centrality(&gg.of)
                .expect("graphina closeness")
        },
        |m| map_to_vec(m, n),
        || rwx_closeness(&pg, true, usize::MAX),
        |v| opt_vec(v, n),
    );

    // Eigenvector centrality, compared after L2 + sign normalization (the two
    // libraries use different scaling and sign conventions).
    diff_and_bench(
        "eigenvector",
        cfg,
        &mut rows,
        1e-3,
        || {
            graphina::centrality::eigenvector::eigenvector_centrality(&gg.f64, 1000, 1e-9)
                .expect("graphina eigenvector")
        },
        |m| {
            let mut v = map_to_vec(m, n);
            l2_sign_normalize(&mut v);
            v
        },
        || {
            rwx_eigenvector(
                &pg,
                |e| Ok::<f64, Infallible>(*e.weight()),
                Some(1000),
                Some(1e-9),
            )
            .expect("rustworkx eigenvector")
        },
        |opt| {
            let mut v = opt.clone().unwrap_or_default();
            l2_sign_normalize(&mut v);
            v
        },
    );

    // Katz centrality, compared after L2 + sign normalization (graphina does not
    // normalize, rustworkx L2-normalizes). A small alpha keeps both convergent.
    diff_and_bench(
        "katz_centrality",
        cfg,
        &mut rows,
        1e-3,
        || {
            graphina::centrality::katz::katz_centrality(&gg.f64, 0.01, Some(&|_| 1.0), 1000, 1e-9)
                .expect("graphina katz")
        },
        |m| {
            let mut v = map_to_vec(m, n);
            l2_sign_normalize(&mut v);
            v
        },
        || {
            rwx_katz(
                &pg,
                |_| Ok::<f64, Infallible>(1.0),
                Some(0.01),
                None,
                Some(1.0),
                Some(1000),
                Some(1e-9),
            )
            .expect("rustworkx katz")
        },
        |opt| {
            let mut v = opt.clone().unwrap_or_default();
            l2_sign_normalize(&mut v);
            v
        },
    );

    // Transitivity (global clustering), a single scalar.
    diff_and_bench(
        "transitivity",
        cfg,
        &mut rows,
        1e-6,
        || graphina::metrics::transitivity(&gg.f64),
        |x| vec![*x],
        || rwx_transitivity(&pg),
        |x| vec![*x],
    );

    print_table(cfg, &rows);
    rows
}

/// Sorted node indices as a `Vec<f64>`, the canonical form of a reachable set.
fn sorted_indices(indices: impl Iterator<Item = usize>) -> Vec<f64> {
    let mut v: Vec<f64> = indices.map(|i| i as f64).collect();
    v.sort_by(|a, b| a.partial_cmp(b).expect("finite index"));
    v
}

/// Canonicalizes a partition (each part sorted, parts sorted) and flattens it
/// with `-1` separators so two partitions compare equal as a `Vec<f64>`.
fn canonical_partition<I, J>(parts: I) -> Vec<f64>
where
    I: IntoIterator<Item = J>,
    J: IntoIterator<Item = usize>,
{
    let mut canon: Vec<Vec<usize>> = parts
        .into_iter()
        .map(|p| {
            let mut v: Vec<usize> = p.into_iter().collect();
            v.sort_unstable();
            v
        })
        .collect();
    canon.sort();
    let mut out = Vec::new();
    for part in canon {
        out.extend(part.into_iter().map(|x| x as f64));
        out.push(-1.0);
    }
    out
}

/// Spreads rustworkx's index-aligned `Vec<Option<f64>>` into a dense vector,
/// treating absent values as zero.
fn opt_vec(v: &[Option<f64>], n: usize) -> Vec<f64> {
    let mut out = vec![0.0; n];
    for (i, slot) in v.iter().enumerate().take(n) {
        out[i] = slot.unwrap_or(0.0);
    }
    out
}

fn print_table(cfg: &Config, rows: &[Row]) {
    println!(
        "{:<22} {:>16} {:>16} {:>14}  diff",
        "algorithm", "graphina", "rustworkx-core", "speedup"
    );
    println!(
        "(median±h, h = half-width of the 95% bootstrap CI over {} timed rounds; \
         a trailing * marks fewer rounds because the budget ran out;\n speedup = \
         rustworkx / graphina, so >1 means graphina is faster)\n",
        cfg.reps
    );
    let fmt = |b: &BenchStat| {
        let med = b.median.as_nanos().max(1) as f64;
        let half = (b.ci_hi.as_nanos() as f64 - b.ci_lo.as_nanos() as f64) / 2.0;
        let pct = (half / med * 100.0).round() as i64;
        let s = format!("{:.2?}±{pct}%", b.median);
        if b.samples < cfg.reps {
            format!("{s}*")
        } else {
            s
        }
    };
    for row in rows {
        match (row.diff, &row.graphina, &row.rustworkx) {
            (Diff::Match, Some(g), Some(r)) => {
                let ratio = r.median.as_secs_f64() / g.median.as_secs_f64().max(f64::MIN_POSITIVE);
                println!(
                    "{:<22} {:>16} {:>16} {:>13.2}x  ok",
                    row.name,
                    fmt(g),
                    fmt(r),
                    ratio
                );
            }
            _ => {
                println!(
                    "{:<22} {:>16} {:>16} {:>14}  DIFF (not timed)",
                    row.name, "-", "-", "-"
                );
            }
        }
    }
    println!();
}

fn main() {
    let cfg = Config::from_env();
    println!("graphina vs rustworkx-core algorithm comparison\n");

    if !cfg.sweep {
        run_at(&cfg, cfg.nodes, cfg.edges);
        return;
    }

    let sizes = [
        (cfg.nodes / SWEEP_STEP, cfg.edges / SWEEP_STEP),
        (cfg.nodes, cfg.edges),
        (
            cfg.nodes.saturating_mul(SWEEP_STEP),
            cfg.edges.saturating_mul(SWEEP_STEP),
        ),
    ];
    let mut all = Vec::new();
    for (i, &(n, m)) in sizes.iter().enumerate() {
        println!("==== sweep size {} of {} ====", i + 1, sizes.len());
        all.push(run_at(&cfg, n, m));
    }

    println!("==== scaling (median ratio between consecutive {SWEEP_STEP}x sizes) ====");
    println!(
        "(a ratio above {SWEEP_STEP}.0 is superlinear in dataset size; gr = graphina, rwx = rustworkx-core)\n"
    );
    println!(
        "{:<22} {:>20} {:>20}",
        "algorithm", "small->mid", "mid->large"
    );
    let names: Vec<&'static str> = all[0].iter().map(|r| r.name).collect();
    for (idx, name) in names.iter().enumerate() {
        let cell = |from: usize, to: usize| -> String {
            let a = all[from][idx]
                .graphina
                .as_ref()
                .zip(all[from][idx].rustworkx.as_ref());
            let b = all[to][idx]
                .graphina
                .as_ref()
                .zip(all[to][idx].rustworkx.as_ref());
            match (a, b) {
                (Some((ag, ar)), Some((bg, br))) => {
                    let gr =
                        bg.median.as_secs_f64() / ag.median.as_secs_f64().max(f64::MIN_POSITIVE);
                    let rwx =
                        br.median.as_secs_f64() / ar.median.as_secs_f64().max(f64::MIN_POSITIVE);
                    format!("gr {gr:.1} / rwx {rwx:.1}")
                }
                _ => "-".to_string(),
            }
        };
        println!("{:<22} {:>20} {:>20}", name, cell(0, 1), cell(1, 2));
    }
}
