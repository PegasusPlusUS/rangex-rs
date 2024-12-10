use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn example_benchmark(c: &mut Criterion) {
    c.bench_function("std (1..=10000) .sum::<u16>()", |b| {
        b.iter(|| {
            // Code to benchmark
            (1..=10000).sum::<u16>()
        })
    });
    c.bench_function("inclusive u16 range 1, 10000 for +=", |b| {
        b.iter(|| {
            use rangex::basic_range::*;
            use rangex::*;

            // Code to benchmark
            let r = range_inclusive!(u16, 1, 10000, 1);
            let mut s: u16 = 0;
            for v in r {
                s += v as u16
            }
            s
        })
    });
    c.bench_function(
        "inclusive u16 range 1, 10000 into_iter().sum::<u16>()",
        |b| {
            b.iter(|| {
                use rangex::basic_range::*;
                use rangex::*;

                // Code to benchmark
                let r = range_inclusive!(u16, 1, 10000, 1);
                r.into_iter().sum::<u16>()
            })
        },
    );
    c.bench_function("std range (1..=10000).sum::<u64>() in black_box", |b| {
        b.iter(|| {
            let _result = black_box((1..=10000).sum::<u64>());
        })
    });
    c.bench_function(
        "inclusive u16 range 1, 10000 into_iter().sum::<u64>() in black_box",
        |b| {
            b.iter(|| {
                use rangex::basic_range::*;
                use rangex::*;

                let _result = black_box(
                    // Code to benchmark
                    range_inclusive!(u16, 1, 10000, 1).into_iter().sum::<u16>(),
                );
            })
        },
    );
}

fn custom_criterion() -> Criterion {
    Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(1))
        .sample_size(100)
}

criterion_group! {
    name = benches;
    config = custom_criterion();
    targets = example_benchmark
}
//criterion_group!(benches, example_benchmark);
criterion_main!(benches);
