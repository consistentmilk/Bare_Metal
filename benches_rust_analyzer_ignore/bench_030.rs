use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::hard::_030::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_030", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_hard_030, benchmark);
criterion_main!(group_hard_030);

