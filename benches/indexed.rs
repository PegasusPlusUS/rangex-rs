use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn indexed_benchmark(c: &mut Criterion) {
    c.bench_function("sum 1 to 10000 with index", |b| {
        b.iter(|| {
            // Code to benchmark
            (1..=10000).sum::<u64>()
        })
    });
    c.bench_function("sum 1 to 100000 indexed in black_box", |b| {
        b.iter(|| {
            let _result = black_box((1..=10000).sum::<u64>());
        })
    });
}

fn custom_criterion() -> Criterion {
    Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(1))
        .sample_size(100)
}

criterion_group! {
    name = benches;
    config = custom_criterion();
    targets = indexed_benchmark
}
criterion_main!(benches);
