use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::easy::_3168::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_3168", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_easy_3168, benchmark);
criterion_main!(group_easy_3168);

