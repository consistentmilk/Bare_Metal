use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::medium::_097::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_097", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_medium_097, benchmark);
criterion_main!(group_medium_097);

