use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::medium::_1352::ProductOfNumbers;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_1352", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_medium_1352, benchmark);
criterion_main!(group_medium_1352);

