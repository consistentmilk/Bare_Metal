use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::medium::_3066::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_3066", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_medium_3066, benchmark);
criterion_main!(group_medium_3066);

