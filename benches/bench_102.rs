use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::medium::_102::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_102", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_medium_102, benchmark);
criterion_main!(group_medium_102);

