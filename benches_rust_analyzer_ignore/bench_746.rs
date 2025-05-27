use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::easy::_746::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_746", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_easy_746, benchmark);
criterion_main!(group_easy_746);

