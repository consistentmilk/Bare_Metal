use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::medium::_1726::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_1726", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_medium_1726, benchmark);
criterion_main!(group_medium_1726);

