use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::medium::_079::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_079", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_medium_079, benchmark);
criterion_main!(group_medium_079);

