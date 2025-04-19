use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::hard::_023::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_023", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_hard_023, benchmark);
criterion_main!(group_hard_023);

