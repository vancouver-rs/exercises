use criterion::{black_box, criterion_group, criterion_main, Criterion};
use skeleton::parse;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("json-logs-parse", |b| b.iter(|| black_box(parse())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
