use criterion::{criterion_group, criterion_main, Criterion};



fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("perft 6", |b| b.iter(|| chess::debug::perft(&chess::board::Board::default(), 6)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
