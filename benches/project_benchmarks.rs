use criterion::{Criterion, criterion_group, criterion_main};

fn benchmark_fun(_c: &mut Criterion) {
    // Your benchmarking code here
}

criterion_group!(benches, benchmark_fun,);
criterion_main!(benches);
