use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::hard::_025::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_025", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_hard_025, benchmark);
criterion_main!(group_hard_025);

