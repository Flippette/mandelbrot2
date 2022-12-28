use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mandelbrot2::{config::Config, render::trace};

pub fn trace_bench(c: &mut Criterion) {
    let config = Config::default();

    c.bench_function("trace 255", |b| {
        b.iter(|| trace(black_box(0.0), black_box(0.0), &config))
    });
}

criterion_group!(benches, trace_bench);
criterion_main!(benches);
