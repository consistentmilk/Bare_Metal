use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::medium::_093::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_093", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_medium_093, benchmark);
criterion_main!(group_medium_093);

